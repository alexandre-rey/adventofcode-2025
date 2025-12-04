use std::fs;

use crate::part_1::Part1;

mod part_1;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let part_1 = Part1::new(&input);
    part_1.run();
}
