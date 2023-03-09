pub mod attachable;
pub mod direction;
pub mod position;

pub use attachable::{Attachable, Components};
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

    pub fn prepared_components(&self) -> Components {
        let (before, after) = self.attachable.components();
        (
            prepare_components(before, self.direction),
            prepare_components(after, self.direction),
        )
    }
}

fn prepare_components(components: Vec<String>, direction: Option<Direction>) -> Vec<String> {
    components
        .iter()
        .map(|c| with_direction(c, direction))
        .collect()
}

fn with_direction(component: &str, direction: Option<Direction>) -> String {
    match direction {
        None => format!("- {component}"),
        Some(Direction::Up) => format!("^ {component}"),
        Some(Direction::Down) => format!("_ {component}"),
    }
}

#[cfg(test)]
mod tests;
