mod part_one;
mod part_two;
mod santa;

fn main() {
    let input = include_str!("../input.txt");
    
    part_one::run(input);
    part_two::run(input);
}
