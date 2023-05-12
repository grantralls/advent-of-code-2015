pub fn run(input: &str) {
    println!("Final floor: {}", final_floor(input));
}

fn final_floor(input: &str) -> i32 {
    let mut final_floor = 0;

    input.chars().for_each(|ch| {
        if ch == crate::UP {
            final_floor += 1;
        } else if ch == crate::DOWN {
            final_floor -= 1;
        } else {
            panic!("Invalid character, expected either ')' or '(': {}", ch);
        }
    });

    final_floor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_goes_negative() {
        assert_eq!(final_floor(")))"), -3);
    }

    #[test]
    fn it_goes_positive() {
        assert_eq!(final_floor("((("), 3);
    }

    #[test]
    fn it_goes_up_and_down() {
        assert_eq!(final_floor("()())"), -1);
    }

    #[test]
    fn it_returns_to_zero() {
        assert_eq!(final_floor("()()()"), 0);
    }
}
