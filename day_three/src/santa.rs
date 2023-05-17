use day_three::mover::Mover;

const NORTH: &char = &'^';
const EAST: &char = &'>';
const SOUTH: &char = &'v';
const WEST: &char = &'<';

pub struct Santa {
    pub mover: Mover
}

impl Santa {
    pub fn move_santa(&mut self, direction: &char) {

        let mut new_pos = (self.mover.current_location.x, self.mover.current_location.y);
                
        if direction == NORTH {
            new_pos.1 += 1;
        } else if direction == SOUTH {
            new_pos.1 -= 1;
        } else if direction == WEST {
            new_pos.0 -= 1;
        } else if direction == EAST {
            new_pos.0 += 1;
        }

        self.mover.change_position(new_pos);
    }
}