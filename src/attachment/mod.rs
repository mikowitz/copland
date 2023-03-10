pub mod attachable;
pub mod direction;
pub mod position;

pub use attachable::{ArpeggioStyle, Attachable, Components};
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
    #[must_use]
    pub const fn new(attachable: Attachable) -> Self {
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

    #[must_use]
    pub fn prepared_components(&self) -> Components {
        let (before, after) = self.attachable.components();
        (
            prepare_components(&before, self.direction, self.attachable.has_direction()),
            prepare_components(&after, self.direction, self.attachable.has_direction()),
        )
    }
}

fn prepare_components(
    components: &[String],
    direction: Option<Direction>,
    has_direction: bool,
) -> Vec<String> {
    components
        .iter()
        .map(|c| with_direction(c, direction, has_direction))
        .collect()
}

fn with_direction(component: &str, direction: Option<Direction>, has_direction: bool) -> String {
    match (has_direction, direction) {
        (false, _) => component.to_string(),
        (true, None) => format!("- {component}"),
        (true, Some(Direction::Up)) => format!("^ {component}"),
        (true, Some(Direction::Down)) => format!("_ {component}"),
    }
}

#[cfg(test)]
mod tests;
