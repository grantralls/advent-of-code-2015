use crate::present::Present;

const MESSAGE: &str = "Expected a number in string form";

pub fn make_presents(input: &str) -> Vec<Present> {
    let presents_vec = input.split("\n");

    return presents_vec
        .map(|present| {
            let values: Vec<&str> = present.split("x").collect();

            let new_present = Present::new(
                values[0].parse().expect(MESSAGE),
                values[1].parse().expect(MESSAGE),
                values[2].parse().expect(MESSAGE),
            );

            return new_present;
        })
        .collect();
}
