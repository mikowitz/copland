pub mod attachable;
pub mod direction;
pub mod position;

pub use attachable::Attachable;
pub use direction::Direction;
pub use position::Position;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Attachment {
    attachable: Attachable,
    direction: Option<Direction>,
    position: Option<Position>,
    priority: i32,
}

impl Attachment {
    pub fn new(attachable: Attachable) -> Self {
        let (direction, position, priority) = Attachable::defaults(attachable);
        Self {
            attachable,
            direction,
            position,
            priority,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = Some(direction);
    }

    pub fn reset_direction(&mut self) {
        let (direction, _, _) = Attachable::defaults(self.attachable);
        self.direction = direction;
    }
}

#[cfg(test)]
mod tests;
