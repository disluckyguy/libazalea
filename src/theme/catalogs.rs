use iced::{
    border::Radius, widget::{checkbox, rule}, Color
};

use crate::theme::{Oklch, Theme, button::ButtonShape};

impl checkbox::Catalog for Theme {
    type Class<'a> = checkbox::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: checkbox::Status) -> checkbox::Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: checkbox::Status) -> checkbox::Style {
    styled_color(
        theme.surface_container,
        theme.primary_container,
        theme.on_surface,
        theme.on_primary_container,
        status,
    )
}

pub fn styled_color(
    unchecked_background: Color,
    checked_background: Color,
    unchecked_text: Color,
    checked_text: Color,
    status: checkbox::Status,
) -> checkbox::Style {
    match status {
        checkbox::Status::Active { is_checked } => {
            let color = if is_checked {
                checked_background
            } else {
                unchecked_background
            };
            let text_color = if is_checked {
                checked_text
            } else {
                unchecked_text
            };
            checkbox::Style {
                background: iced::Background::Color(color),
                text_color: Some(text_color),
                border: ButtonShape::Regular.into(),
                icon_color: text_color,
            }
        }
        checkbox::Status::Hovered { is_checked } => {
            let color = if is_checked {
                checked_background
            } else {
                unchecked_background
            };
            let text_color = if is_checked {
                checked_text
            } else {
                unchecked_text
            };
            checkbox::Style {
                background: iced::Background::Color(color.lighter()),
                text_color: Some(text_color),
                border: ButtonShape::Regular.into(),
                icon_color: text_color,
            }
        }
        checkbox::Status::Disabled { is_checked } => {
            let color = if is_checked {
                checked_background
            } else {
                unchecked_background
            };
            let text_color = if is_checked {
                checked_text
            } else {
                unchecked_text
            };
            checkbox::Style {
                background: iced::Background::Color(color.darker()),
                text_color: Some(text_color.darker().darker()),
                border: ButtonShape::Regular.into(),
                icon_color: text_color,
            }
        }
    }
}

impl rule::Catalog for Theme {
    type Class<'a> = rule::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default_rule)
    }

    fn style(&self, class: &Self::Class<'_>) -> rule::Style {
        class(self)
    }
}

pub fn default_rule(theme: &Theme) -> rule::Style {
    rule::Style {
        color: theme.surface_container_highest,
        width: 2,
        radius: Radius::new(2.),
        fill_mode: rule::FillMode::Padded(6),
    }
}