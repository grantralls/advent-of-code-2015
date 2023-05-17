use std::collections;
use crate::location::Location;

pub struct Mover {
    pub current_location: Location,
    pub previous_locations: collections::HashSet<Location>,
}

impl Mover {
    pub fn new() -> Mover {
        let mut my_hash = collections::HashSet::new();

        my_hash.insert(Location {
            x: 0,
            y: 0,
            times_visited: 1,
        });

        return Mover {
            current_location: Location {
                x: 0,
                y: 0,
                times_visited: 1,
            },
            previous_locations: my_hash,
        };
    }

    pub fn change_position(&mut self, new_pos: (i32, i32)) {
        let mut new_loc = Location {
            x: new_pos.0,
            y: new_pos.1,
            times_visited: 1,
        };

        if let Some(v) = self.previous_locations.take(&new_loc) {
            new_loc.times_visited = v.times_visited + 1;
        };

        self.current_location = new_loc;
        self.previous_locations.insert(new_loc);

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_two_locations() {
        let mut mover = Mover::new();

        mover.change_position((1, 0));

        assert_eq!(mover.previous_locations.len(), 2);
    }

    #[test]
    fn marks_the_same_location_twice() {
        let mut mover = Mover::new();

        mover.change_position((1, 0));
        mover.change_position((1, 0));

        assert_eq!(mover.previous_locations.len(), 2)
    }
}