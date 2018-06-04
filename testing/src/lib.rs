/// This awesome function allows you to sum two integers
/// this function is so awesome you can even overflow \o/
///
/// # Examples
///
/// ```
///extern crate testing;
/// use testing::sum;
///
/// # fn main() {
/// // Create two integers
/// let two = 2;
/// let four = 4;
/// let sum = sum(two, four);
/// assert_eq!(5, sum);
/// # }
/// ```
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn two_plus_three_is_five() -> () {
    // Setup
    let expected_result = 5;
    let a = 2;
    let b = 3;

    assert_eq!(expected_result, sum(a, b));
}
