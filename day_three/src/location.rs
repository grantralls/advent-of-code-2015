use std::hash;
use std::fmt;

#[derive(Eq, Debug, Copy, Clone)]
pub struct Location {
    pub x: i32,
    pub y: i32,
    pub times_visited: i32,
}

impl hash::Hash for Location {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {}, y: {}, times_visited: {}",
            self.x, self.y, self.times_visited
        )
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

#[cfg(test)]
mod tests {
    use std::collections;
    use super::*;

    #[test]
    fn equality_hash_works() {
        let mut loc_hash = collections::HashSet::new();

        let first_loc = Location {
            x: 0,
            y: 0,
            times_visited: 1,
        };
        let second_loc = Location {
            x: 0,
            y: 0,
            times_visited: 2,
        };

        loc_hash.insert(first_loc);
        loc_hash.insert(second_loc);

        assert_eq!(loc_hash.len(), 1);
    }

    #[test]
    fn equality_works() {
        let first_loc = Location {
            x: 0,
            y: 0,
            times_visited: 1,
        };
        let second_loc = Location {
            x: 0,
            y: 0,
            times_visited: 2,
        };

        assert_eq!(first_loc, second_loc);
    }
}