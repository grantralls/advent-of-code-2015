use crate::parser::make_presents;

pub fn run(input: &str) {
    let presents = make_presents(input);

    let mut total_feet = 0;

    presents.iter().for_each(|present| {
        total_feet += present.required_ribbon();
    });

    println!("Required Ribbon: {}", total_feet);
}
