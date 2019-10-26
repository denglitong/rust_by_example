// The primary way of documenting a Rust project is through annotating the source code.
// Document comments are written in markdown and support code blocks in them.
// Rust takes care about correctness, so these code blocks are compiled and used as tests.

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern create <createname>`. Assume we're testing `documents` crate:
///
/// ```
/// use crate::testing::add;
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
///
/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// use crate::testing::divide_non_zero_result;
/// let result = divide_non_zero_result(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// use crate::testing::divide_non_zero_result;
/// // panics on division by zero
/// divide_non_zero_result(10, 0);
/// ```
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero")
    }
    a / b
}

/// Using hidden `try_main` in doc tests.
///
/// ```
/// # // hidden lines start with `#` symbol,  but they're still compileable!
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// let res = crate::testing::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #   try_main().unwrap(); // calling try_main and unwrapping
/// #                        // so that test will panic in case of error
/// # }
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}
