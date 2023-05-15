use crate::parser::make_presents;

pub fn run(input: &str) {
    let presents = make_presents(input);

    let mut total = 0;
    presents.iter().for_each(|present| {
        total += present.total_wrapping_paper();
    });

    println!("Total squared area: {}", total);
}
