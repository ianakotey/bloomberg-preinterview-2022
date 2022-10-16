use std::ops::Div;

/// Get the digit of a base-10 number at a particular index.
/// Indexes are zero-based. Numbers are expected to be little-endian

///
/// # Examples
///
/// ```
/// use libbloom22::get_digit;
/// assert_eq!(get_digit(1234, 3), 1);
/// ```
pub fn get_digit(num: i32, pos: u8) -> i8 {
    (num % 10_i32.pow((pos + 1) as u32)) // get rid of leading digits
        .div(10_i32.pow(pos as u32)) // remove following digits
        as i8
}

/*
 * Complete the 'minimumMoves' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY arr1
 *  2. INTEGER_ARRAY arr2
 */
pub fn minimum_moves(arr1: &[i32], arr2: &[i32]) -> i32 {
    arr1.iter()
        .zip(arr2.iter())
        .flat_map(|(a, b)| (0..=7).map(|n| (get_digit(*a, n), get_digit(*b, n))))
        .map(|(a, b)| (a - b).abs() as i32)
        .sum()
}
