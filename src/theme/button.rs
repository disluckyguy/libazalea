use iced::{Border, Color, Shadow, widget::button};

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
    match status {
        button::Status::Active => button::Style {
            background: Some(iced::Background::Color(theme.surface_container_highest)),
            text_color: theme.on_surface_container,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(
                theme.surface_container_highest.lighter(),
            )),
            text_color: theme.on_surface_container,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(
                theme.surface_container_highest.lighter().lighter(),
            )),
            text_color: theme.on_surface_container,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(
                theme.surface_container_highest.darker(),
            )),
            text_color: theme.on_surface_container.darker().darker(),
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

pub fn suggested(theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(iced::Background::Color(theme.primary_container)),
            text_color: theme.on_primary_container,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(theme.primary_container.lighter())),
            text_color: theme.on_primary_container,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(
                theme.primary_container.lighter().lighter(),
            )),
            text_color: theme.on_primary_container,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(theme.primary_container.darker())),
            text_color: theme.on_primary_container.darker().darker(),
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

pub fn warning(theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(iced::Background::Color(theme.warning_container)),
            text_color: theme.on_warning_container,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(
                theme.warning_container.lighter().into(),
            )),
            text_color: theme.on_warning_container,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(
                theme.warning_container.lighter().lighter(),
            )),
            text_color: theme.on_warning_container,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                color: Color::BLACK,
                blur_radius: 8.,
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(theme.warning_container.darker())),
            text_color: theme.on_warning_container.darker().darker(),
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

pub fn danger(theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(iced::Background::Color(theme.danger_container)),
            text_color: theme.on_danger_container,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(theme.danger_container.lighter())),
            text_color: theme.on_danger_container,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(
                theme.danger_container.lighter().lighter(),
            )),
            text_color: theme.on_danger_container,
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(theme.danger_container)),
            text_color: theme.on_danger_container.darker().darker(),
            border: ButtonShape::Regular.into(),
            shadow: Shadow {
                blur_radius: 8.,
                ..Default::default()
            },
            ..Default::default()
        },
    }
}
