pub fn run(input: &str) {
    if let Some(i) = pos_of_first_basement_char(input) {
        println!("First basement char position: {}", i);
    } else {
        println!("Never entered basement");
    }
}

fn pos_of_first_basement_char(input: &str) -> Option<usize> {
    let mut current_floor = 0;

    for (i, ch) in input.chars().enumerate() {
        if ch == crate::UP {
            current_floor += 1;
        } else if ch == crate::DOWN {
            current_floor -= 1;
        } else {
            panic!("Invalid character, expected either ')' or '(': {}", ch);
        }

        if current_floor == -1 {
            return Some(i + 1);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_none_if_never_enters_basement() {
        assert_eq!(pos_of_first_basement_char("()()()"), None);
    }

    #[test]
    fn it_returns_position_of_first_basement_char() {
        assert_eq!(pos_of_first_basement_char("()())"), Some(5));
    }

    #[test]
    fn enter_basement_at_one() {
        assert_eq!(pos_of_first_basement_char(")"), Some(1));
    }
}
