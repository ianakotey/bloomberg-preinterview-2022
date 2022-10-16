

use libbloom22::prelude::*;

fn main() {
    let test_arr_1 = [1234, 4321];
    let test_arr_2 = [2345, 3214];

    dbg!(minimum_moves(&test_arr_2, &test_arr_1));

    let team_a = [1, 2, 3];
    let team_b = [2, 4];

    println!("{:?}", counts(&team_a, &team_b));
}
