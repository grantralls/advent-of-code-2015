use crate::santa::Santa;
use day_three::mover::Mover;

pub fn run(input: &str) {
    let mut santa = Santa {
        mover: Mover::new()
    };

    input.chars().into_iter().for_each(|direction| {
        santa.move_santa(&direction);
    });

    println!("Santa visited {} houses", santa.mover.previous_locations.len());
}