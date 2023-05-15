use day_two::part_one;
use day_two::part_two;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading the input file");

    part_one::run(&input);
    part_two::run(&input);
}
