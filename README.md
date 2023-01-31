# include Doc

[![tests](https://github.com/simon-bourne/include-doc/actions/workflows/tests.yml/badge.svg)](https://github.com/simon-bourne/include-doc/actions/workflows/tests.yml)
[![crates.io](https://img.shields.io/crates/v/include-doc.svg)](https://crates.io/crates/include-doc)
[![Documentation](https://docs.rs/include-doc/badge.svg)](https://docs.rs/include-doc)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/include-doc)](./LICENSE-APACHE)

Include examples in your Rustdocs.

This crate allows you to include example files in Rustdoc.

## Features

- Write and maintain your examples in a normal Rust file, with full editor support. See [this example](https://github.com/simon-bourne/include-doc/blob/v0.0.2/example/src/doc_with_example.rs).
- Automatically hide imports.
- Put many examples into one file, avoiding duplicated boilerplate for setup code. See [this example](https://github.com/simon-bourne/include-doc/blob/v0.0.2/example/src/doc_with_tests.rs).
