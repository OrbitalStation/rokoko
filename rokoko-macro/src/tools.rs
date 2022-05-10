//!
//! This module provides some function to be used all
//! across the crate
//!

///
/// Capitalize the string.
///
/// # Examples
///
/// ```norun
/// assert_eq!(capitalize("pork"), "Pork".to_string())
/// ```
///
pub fn capitalize(s: &str) -> String {
    let (first, rest) = s.split_at(s.chars().next().unwrap().len_utf8());
    first.to_uppercase() + rest
}

///
/// Convert string from snake to upper case
///
/// # Examples
///
/// ```norun
/// assert_eq!(snake_to_upper_case("snaky_snake"), "SnakySnake".to_string())
/// ```
///
pub fn snake_to_upper_case(snake: &str) -> String {
    snake.split('_').map(capitalize).collect::<Vec <_>>().join("")
}
