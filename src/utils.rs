use crate::{
    applet::interface::Direction,
    reexports::{iced::Size, iced_core::layout::Limits},
};

pub fn size_from_direction(direction: &Direction, size: u32) -> (u32, u32) {
    match direction {
        Direction::Left | Direction::Right => (size, 0),
        Direction::Top | Direction::Bottom => (0, size),
    }
}

pub fn limits_from_grid(limits: &Limits, size: u32) -> Limits {
    let max = limits.max();
    let min = limits.min();
    Limits::new(
        Size::new(min.width * size as f32, min.height * size as f32),
        Size::new(max.width * size as f32, max.height * size as f32),
    )
}
