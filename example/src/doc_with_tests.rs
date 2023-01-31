/// # Examples
///
/// Examples from `tests/doc.rs`:
///
/// ## First Example
///
/// ```
#[doc = include_doc::function_body!("tests/doc.rs", my_first_example, [MyFirstStruct, setup_first_example])]
/// ```
/// 
/// ## Second Example
/// ```
#[doc = include_doc::function_body!("tests/doc.rs", my_second_example, [MySecondStruct, setup_second_example])]
/// ```
pub fn my_function() {}
