use iced::{Border, Color, border::Radius, widget::text_input};

use crate::theme::{Oklch, Theme};

impl text_input::Catalog for Theme {
    type Class<'a> = text_input::StyleFn<'a, Theme>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(regular)
    }

    fn style(&self, class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        class(self, status)
    }
}

pub fn regular(theme: &Theme, status: text_input::Status) -> text_input::Style {
    styled_color(
        theme.surface_container_highest,
        theme.on_surface,
        theme.on_surface_variant,
        theme.primary,
        theme.primary.scale_alpha(0.5),
        status,
    )
}

pub fn suggested(theme: &Theme, status: text_input::Status) -> text_input::Style {
    styled_color(
        theme.surface_container_highest,
        theme.on_surface,
        theme.on_surface_variant,
        theme.primary,
        theme.primary.scale_alpha(0.5),
        status,
    )
}

pub fn secondary(theme: &Theme, status: text_input::Status) -> text_input::Style {
    styled_color(
        theme.surface_container_highest,
        theme.on_surface,
        theme.on_surface_variant,
        theme.secondary,
        theme.secondary.scale_alpha(0.5),
        status,
    )
}

pub fn tertiary(theme: &Theme, status: text_input::Status) -> text_input::Style {
    styled_color(
        theme.surface_container_highest,
        theme.on_surface,
        theme.on_surface_variant,
        theme.tertiary,
        theme.tertiary.scale_alpha(0.5),
        status,
    )
}

pub fn warning(theme: &Theme, status: text_input::Status) -> text_input::Style {
    styled_color(
        theme.surface_container_highest,
        theme.on_surface,
        theme.on_surface_variant,
        theme.warning,
        theme.warning.scale_alpha(0.5),
        status,
    )
}

pub fn danger(theme: &Theme, status: text_input::Status) -> text_input::Style {
    styled_color(
        theme.surface_container_highest,
        theme.on_surface,
        theme.on_surface_variant,
        theme.danger,
        theme.danger.scale_alpha(0.5),
        status,
    )
}

pub fn styled_color(
    background: Color,
    value: Color,
    place_holder: Color,
    border: Color,
    selection: Color,
    status: text_input::Status,
) -> text_input::Style {
    match status {
        text_input::Status::Active => text_input::Style {
            background: iced::Background::Color(background),
            icon: value,
            border: Border::default().rounded(8.),
            placeholder: place_holder,
            value: value,
            selection: selection,
        },
        text_input::Status::Hovered => text_input::Style {
            background: iced::Background::Color(background),
            icon: value,
            border: Border {
                color: border,
                width: 1.,
                radius: Radius::new(8.),
            },
            placeholder: place_holder,
            value: value,
            selection: selection,
        },
        text_input::Status::Focused { is_hovered: _ } => text_input::Style {
            background: iced::Background::Color(background),
            icon: value,
            border: Border {
                color: border,
                width: 2.,
                radius: Radius::new(8.),
            },
            placeholder: place_holder,
            value: value,
            selection: selection,
        },
        text_input::Status::Disabled => text_input::Style {
            background: iced::Background::Color(background.darker()),
            icon: value.darker(),
            border: Border::default(),
            placeholder: place_holder.darker(),
            value: value.darker(),
            selection: selection,
        },
    }
}
