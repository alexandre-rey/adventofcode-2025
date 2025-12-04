use std::fs;

use crate::{part_1::Part1, part_2::Part2};

mod part_1;
mod part_2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let part_1 = Part1::new(&input);
    part_1.run();

    let part_2 = Part2::new(&input);
    part_2.run();
}
