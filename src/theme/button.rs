use iced::{Border, Color, Shadow, Vector, widget::button};

use crate::theme::{Oklch, Theme};

#[derive(Debug, Clone)]
pub enum ButtonShape {
    Pill,
    Regular,
    Circular,
}

impl From<ButtonShape> for Border {
    fn from(value: ButtonShape) -> Self {
        match value {
            ButtonShape::Pill => Border::default().rounded(10.),
            ButtonShape::Regular => Border::default().rounded(8.),
            ButtonShape::Circular => Border::default().rounded(i32::MAX),
        }
    }
}

impl button::Catalog for Theme {
    type Class<'a> = button::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(regular)
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        class(self, status)
    }
}

pub fn regular(theme: &Theme, status: button::Status) -> button::Style {
    styled_color(
        theme.surface_container,
        theme.surface_container_high,
        theme.surface_container_highest,
        theme.on_surface,
        status,
    )
}

pub fn flat(theme: &Theme, status: button::Status) -> button::Style {
    styled_flat(
        Color::TRANSPARENT,
        theme.surface_container_high,
        theme.surface_container_highest,
        theme.on_surface,
        status,
    )
}

pub fn suggested(theme: &Theme, status: button::Status) -> button::Style {
    styled_color(
        theme.primary_container,
        theme.primary_container.lighter(),
        theme.primary_container.darker(),
        theme.on_primary_container,
        status,
    )
}

pub fn secondary(theme: &Theme, status: button::Status) -> button::Style {
    styled_color(
        theme.secondary_container,
        theme.secondary_container.lighter(),
        theme.secondary_container.darker(),
        theme.on_secondary_container,
        status,
    )
}

pub fn tertiary(theme: &Theme, status: button::Status) -> button::Style {
    styled_color(
        theme.tertiary_container,
        theme.tertiary_container.lighter(),
        theme.tertiary_container.darker(),
        theme.on_tertiary_container,
        status,
    )
}

pub fn warning(theme: &Theme, status: button::Status) -> button::Style {
    styled_color(
        theme.warning_container,
        theme.warning_container.lighter(),
        theme.warning_container.darker(),
        theme.on_warning_container,
        status,
    )
}

pub fn danger(theme: &Theme, status: button::Status) -> button::Style {
    styled_color(
        theme.danger_container,
        theme.danger_container.lighter(),
        theme.danger_container.darker(),
        theme.on_danger_container,
        status,
    )
}

pub fn styled_color(
    background: Color,
    hover_background: Color,
    focus_background: Color,
    text: Color,
    status: button::Status,
) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(iced::Background::Color(background)),
            text_color: text,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 2.,
                color: Color::BLACK,
                offset: Vector::new(0., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(hover_background)),
            text_color: text,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 2.,
                color: Color::BLACK,
                offset: Vector::new(0., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(focus_background)),
            text_color: text,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 2.,
                color: Color::BLACK,
                offset: Vector::new(0., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(background.darker())),
            text_color: text.darker().darker(),
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 2.,
                offset: Vector::new(0., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

pub fn styled_flat(
    background: Color,
    hover_background: Color,
    focus_background: Color,
    text: Color,
    status: button::Status,
) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(iced::Background::Color(background)),
            text_color: text,
            border: ButtonShape::Regular.into(),
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(hover_background)),
            text_color: text,
            border: ButtonShape::Regular.into(),
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(focus_background)),
            text_color: text,
            border: ButtonShape::Regular.into(),
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(background.darker())),
            text_color: text.darker().darker(),
            border: ButtonShape::Regular.into(),
            ..Default::default()
        },
    }
}
