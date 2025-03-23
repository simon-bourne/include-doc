use std::{collections::HashSet, env, fmt::Display, fs, path::Path};

use error::Tokens;
use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use ra_ap_syntax::{
    ast::{self, HasModuleItem, HasName, Type},
    AstNode, NodeOrToken, SourceFile,
};
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Comma,
    Error, Ident, LitStr, Result, Token,
};

mod error;

macro_rules! call_site_error {
    ($($message:tt)*) => {
        Err($crate::error::call_site(format!($($message)*)))
    }
}

macro_rules! error {
    ($span:expr, $($message:tt)*) => {
        Err($crate::Error::new($span, format!($($message)*)))
    }
}

#[proc_macro]
pub fn source_file(input: TokenStream) -> TokenStream {
    let file: LitStr = parse_macro_input!(input);

    doc_function_body(file, Ident::new("main", Span::call_site()), None).tokens()
}

#[proc_macro]
pub fn function_body(input: TokenStream) -> TokenStream {
    let args: FunctionBodyArgs = parse_macro_input!(input);
    let function_body = args.function_body.to_string();
    let mut dependencies = HashSet::new();

    dependencies.extend(args.dependencies.iter().map(Ident::to_string));

    if dependencies.contains(&function_body) {
        return Error::new(
            args.function_body.span(),
            "Function body can't be in dependencies",
        )
        .into_compile_error()
        .into();
    }

    doc_function_body(args.file, args.function_body, Some(&dependencies)).tokens()
}

struct FunctionBodyArgs {
    file: LitStr,
    function_body: Ident,
    dependencies: Vec<Ident>,
}

impl Parse for FunctionBodyArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let file = input.parse()?;
        input.parse::<Comma>()?;
        let function_body = input.parse()?;
        input.parse::<Comma>()?;
        let dependencies;
        bracketed!(dependencies in input);
        let dependencies = dependencies
            .parse_terminated(Ident::parse, Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            file,
            function_body,
            dependencies,
        })
    }
}

fn doc_function_body(
    file: LitStr,
    function_body_ident: Ident,
    deps: Option<&HashSet<String>>,
) -> Result<proc_macro2::TokenStream> {
    let source = parse_file(&file)?;

    let mut found_body = false;
    let function_body = function_body_ident.to_string();
    let mut track_deps = HashSet::new();

    let parts = source.items().filter_map(|item| match item {
        ast::Item::Use(use_item) => Some(hide_in_doc(use_item)),
        ast::Item::Fn(function) => function.name().and_then(|name| {
            let name = name.text();

            if name.as_str() == function_body {
                found_body = true;
                extract_function_body(&function)
            } else if is_dependency(&name, deps, &mut track_deps) {
                include_always(&function)
            } else {
                None
            }
        }),
        ast::Item::Const(item) => include_if_dependency(&item, deps, &mut track_deps),
        ast::Item::Enum(item) => include_if_dependency(&item, deps, &mut track_deps),
        ast::Item::ExternBlock(item) => include_always(&item),
        ast::Item::ExternCrate(item) => include_always(&item),
        ast::Item::Impl(item) => {
            if is_type_dependency(&item.self_ty(), deps, &mut track_deps)
                || is_type_dependency(&item.trait_(), deps, &mut track_deps)
            {
                include_always(&item)
            } else {
                None
            }
        }
        ast::Item::MacroCall(item) => include_always(&item),
        ast::Item::MacroRules(item) => include_if_dependency(&item, deps, &mut track_deps),
        ast::Item::MacroDef(item) => include_if_dependency(&item, deps, &mut track_deps),
        ast::Item::Module(item) => include_if_dependency(&item, deps, &mut track_deps),
        ast::Item::Static(item) => include_if_dependency(&item, deps, &mut track_deps),
        ast::Item::Struct(item) => include_if_dependency(&item, deps, &mut track_deps),
        ast::Item::Trait(item) => include_if_dependency(&item, deps, &mut track_deps),
        ast::Item::TypeAlias(item) => include_if_dependency(&item, deps, &mut track_deps),
        ast::Item::Union(item) => include_if_dependency(&item, deps, &mut track_deps),
        ast::Item::TraitAlias(item) => include_if_dependency(&item, deps, &mut track_deps),
    });

    let doc = parts.collect::<Vec<String>>().join("\n");

    if let Some(deps) = deps {
        let missing_deps = deps.difference(&track_deps).join(", ");

        if !missing_deps.is_empty() {
            call_site_error!("Not all dependencies were found: [{missing_deps}]")?;
        }
    }

    if !found_body {
        error!(function_body_ident.span(), "{function_body} not found")?;
    }

    Ok(quote!(#doc))
}

fn include_always<T: Display>(node: &T) -> Option<String> {
    Some(format!("{node}\n"))
}

fn include_if_dependency<T: HasName + Display>(
    node: &T,
    dependencies: Option<&HashSet<String>>,
    dependency_tracker: &mut HashSet<String>,
) -> Option<String> {
    node.name().and_then(|name| {
        let name = name.text();

        if is_dependency(&name, dependencies, dependency_tracker) {
            Some(format!("{node}\n"))
        } else {
            None
        }
    })
}

fn is_type_dependency(
    ty: &Option<Type>,
    dependencies: Option<&HashSet<String>>,
    dependency_tracker: &mut HashSet<String>,
) -> bool {
    let Some(ty) = ty else {
        return false;
    };

    ty.syntax()
        .descendants_with_tokens()
        .any(|token| match token {
            NodeOrToken::Node(_) => false,
            NodeOrToken::Token(token) => {
                is_dependency(token.text(), dependencies, dependency_tracker)
            }
        })
}

fn is_dependency(
    name: impl AsRef<str>,
    dependencies: Option<&HashSet<String>>,
    dependency_tracker: &mut HashSet<String>,
) -> bool {
    dependencies.map(|deps| {
        let name = name.as_ref();
        let is_dep = deps.contains(name);

        if is_dep {
            dependency_tracker.insert(name.to_string());
        }

        is_dep
    }) != Some(false)
}

fn extract_function_body(function: &ast::Fn) -> Option<String> {
    function.body().map(|body| {
        if function.async_token().is_some() {
            format!("async {body};\n")
        } else {
            remove_indent(
                body.to_string()
                    .trim()
                    .trim_start_matches('{')
                    .trim_end_matches('}'),
            ) + "\n"
        }
    })
}

fn remove_indent(text: &str) -> String {
    let min_indent = text.lines().filter_map(indent_size).min().unwrap_or(0);

    text.lines()
        .map(|line| {
            if line.len() > min_indent {
                &line[min_indent..]
            } else {
                ""
            }
        })
        .join("\n")
        .trim_matches('\n')
        .to_string()
}

fn indent_size(text: &str) -> Option<usize> {
    if text.trim().is_empty() {
        None
    } else {
        text.find(|c: char| c != ' ' && c != '\t')
    }
}

fn parse_file(file_expr: &LitStr) -> Result<SourceFile> {
    let source_code = read_file(file_expr)?;
    let parse = SourceFile::parse(&source_code);
    let source = parse.tree();

    if !parse.errors().is_empty() {
        error!(file_expr.span(), "Errors in source file")?;
    }

    Ok(source)
}

fn read_file(file_expr: &LitStr) -> Result<String> {
    let file = file_expr.value();

    let dir = env::var("CARGO_MANIFEST_DIR").map_err(error::call_site)?;
    let path = Path::new(&dir).join(file);
    fs::read_to_string(path).map_err(|e| Error::new(file_expr.span(), e))
}

fn hide_in_doc(item: impl Display) -> String {
    // We need the extra `"\n#"` as otherwise rustdoc won't include attributes after
    // hidden items. e.g.
    //
    // ```
    // # use blah
    // #[attribute_will_also_be_hidden]
    // ```
    format!("# {}\n", item.to_string().lines().format("")) + "#"
}
