use super::*;

#[test]
fn new() {
    let attachment = Attachment::new(Attachable::Tie);

    assert_eq!(attachment.attachable, Attachable::Tie);
    assert_eq!(attachment.priority, 0);
    assert_eq!(attachment.direction, None);
    assert_eq!(attachment.position, None);
}

#[test]
fn set_direction() {
    let mut attachment = Attachment::new(Attachable::Tie);
    attachment.set_direction(Direction::Up);

    assert_eq!(attachment.attachable, Attachable::Tie);
    assert_eq!(attachment.direction, Some(Direction::Up));
}

#[test]
fn reset_direction() {
    let mut attachment = Attachment::new(Attachable::Tie);
    attachment.set_direction(Direction::Up);
    attachment.reset_direction();

    assert_eq!(attachment.attachable, Attachable::Tie);
    assert_eq!(attachment.direction, None);
}

#[test]
fn prepared_components() {
    let attachment = Attachment::new(Attachable::Tie);

    assert_eq!(
        attachment.prepared_components(),
        (vec![], vec![String::from("- ~")])
    );
}
