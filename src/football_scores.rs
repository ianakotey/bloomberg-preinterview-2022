/*
 * Complete the 'counts' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY teamA
 *  2. INTEGER_ARRAY teamB
 */

pub fn counts(team_a: &[i32], team_b: &[i32]) -> Vec<i32> {
    let mut sorted = Vec::from(team_a);
    sorted.sort_unstable();

    team_b
        .iter()
        .map(|score| sorted.iter().filter(|s| s <= &score).count() as i32)
        .collect()
}
