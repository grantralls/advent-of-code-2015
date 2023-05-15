use std::vec;

pub struct Present {
    length: u32,
    width: u32,
    height: u32,
    sorted_sides: Vec<u32>,
}

impl Present {
    pub fn new(l: u32, w: u32, h: u32) -> Present {
        let mut measurements = vec![l, w, h];
        measurements.sort();

        return Present {
            length: l,
            width: w,
            height: h,
            sorted_sides: measurements,
        };
    }

    pub fn required_ribbon(&self) -> u32 {
        let measurements = &self.sorted_sides;

        return (2 * measurements[0])
            + (2 * measurements[1])
            + (self.length * self.width * self.height);
    }

    pub fn total_wrapping_paper(&self) -> u32 {
        return self.area() + self.extra_slack();
    }

    fn area(&self) -> u32 {
        return (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length);
    }

    fn extra_slack(&self) -> u32 {
        let measurements = &self.sorted_sides;

        return measurements[0] * measurements[1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let first_present = Present::new(2, 3, 4);

        assert_eq!(first_present.area(), 52);
        assert_eq!(first_present.extra_slack(), 6);
        assert_eq!(first_present.total_wrapping_paper(), 58);
    }

    #[test]
    fn second_example() {
        let second_present = Present::new(1, 1, 10);

        assert_eq!(second_present.area(), 42);
        assert_eq!(second_present.extra_slack(), 1);
        assert_eq!(second_present.total_wrapping_paper(), 43);
    }

    #[test]
    fn required_ribbon_one() {
        let present = Present::new(2, 3, 4);

        assert_eq!(present.required_ribbon(), 34)
    }

    #[test]
    fn required_ribbon_two() {
        let present = Present::new(1, 1, 10);

        assert_eq!(present.required_ribbon(), 14)
    }
}
