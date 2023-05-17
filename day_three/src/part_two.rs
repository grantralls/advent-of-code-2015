use day_three::mover::Mover;
use crate::santa::Santa;

pub fn run(input: &str) {
    let mut santa = Santa { mover: Mover::new() };
    let mut robo_santa = Santa { mover: Mover::new() };

    input.chars().enumerate().for_each(|(i, val)| {
        if i % 2 == 1 {
            santa.move_santa(&val);
        } else {
            robo_santa.move_santa(&val);
        }
    });

    santa.mover.previous_locations.extend(robo_santa.mover.previous_locations);

    println!("{} houses received at least one preset", santa.mover.previous_locations.len());
}