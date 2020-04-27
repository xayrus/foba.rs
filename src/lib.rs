pub mod hashmap_collection;
pub mod vec_collection;

pub use self::vec_collection::number_vec_collection;

/// 将给定的数字加一
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = hoyeungw_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}