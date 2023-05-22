use md5;

pub fn find_smallest_md5_with_five_zeroes(input: &str,) -> i32 {
    let mut counter = 0;

    let mut result = format!("{:x}", md5::compute(input.to_owned() + &counter.to_string()));

    while !has_five_zeroes(&result) {
        counter += 1;

        result = format!("{:x}", md5::compute(input.to_owned() + &counter.to_string()));
    }

    return counter;
}

pub fn find_smallest_md5_with_six_zeroes(input: &str) -> i32 {
    let mut counter = 0;

    let mut result = format!("{:x}", md5::compute(input.to_owned() + &counter.to_string()));

    while !has_six_zeroes(&result) {
        counter += 1;

        result = format!("{:x}", md5::compute(input.to_owned() + &counter.to_string()));
    }

    return counter;
}

fn has_five_zeroes(input: &str) -> bool {
    if input.len() < 5 {
        return false;
    }

    let first_five_chars = &input[..5];

    return first_five_chars == "00000";
}

fn has_six_zeroes(input: &str) -> bool {
    if input.len() < 6 {
        return false;
    }

    let first_five_chars = &input[..6];

    return first_five_chars == "000000";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_five_zeroes_test() {
        let input = "00000abdefg";
        assert_eq!(has_five_zeroes(input), true);
        assert_eq!(has_five_zeroes("1"), false);
        assert_eq!(has_five_zeroes("11111111"), false);
    }

    #[test]
    fn has_six_zeroes_test() {
        let input = "000000abdefg";
        assert_eq!(has_five_zeroes(input), true);
        assert_eq!(has_five_zeroes("1"), false);
        assert_eq!(has_five_zeroes("111111111"), false);
    }

    #[test]
    fn find_smallest_md5_test() {
        let result = find_smallest_md5_with_five_zeroes("abcdef");

        assert_eq!(result, 609043);
    }
}