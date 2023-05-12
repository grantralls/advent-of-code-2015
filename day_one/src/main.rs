use day_one::{part_one, part_two};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    part_one::run(&input);
    part_two::run(&input);
}
