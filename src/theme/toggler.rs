use iced::{Color, widget::toggler};

use crate::theme::{Oklch, Theme};

impl toggler::Catalog for Theme {
    type Class<'a> = toggler::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(regular)
    }

    fn style(&self, class: &Self::Class<'_>, status: toggler::Status) -> toggler::Style {
        class(self, status)
    }
}

pub fn regular(theme: &Theme, status: toggler::Status) -> toggler::Style {
    styled_color(
        theme.surface_container_highest,
        theme.primary_fixed,
        theme.outline,
        theme.on_primary,
        status,
    )
}

pub fn secondary(theme: &Theme, status: toggler::Status) -> toggler::Style {
    styled_color(
        theme.surface_container_highest,
        theme.secondary,
        theme.outline,
        theme.on_secondary,
        status,
    )
}

pub fn tertiary(theme: &Theme, status: toggler::Status) -> toggler::Style {
    styled_color(
        theme.surface_container_highest,
        theme.tertiary,
        theme.outline,
        theme.on_tertiary,
        status,
    )
}

pub fn warning(theme: &Theme, status: toggler::Status) -> toggler::Style {
    styled_color(
        theme.surface_container_highest,
        theme.warning,
        theme.outline,
        theme.on_warning,
        status,
    )
}

pub fn danger(theme: &Theme, status: toggler::Status) -> toggler::Style {
    styled_color(
        theme.surface_container_highest,
        theme.danger,
        theme.outline,
        theme.on_danger,
        status,
    )
}

pub fn styled_color(
    background: Color,
    toggled_background: Color,
    foreground: Color,
    toggled_foreground: Color,
    status: toggler::Status,
) -> toggler::Style {
    match status {
        toggler::Status::Active { is_toggled } => toggler::Style {
            background: if is_toggled {
                toggled_background
            } else {
                background
            },
            background_border_color: Color::TRANSPARENT,
            background_border_width: 0.,
            foreground: if is_toggled {
                toggled_foreground
            } else {
                foreground
            },
            foreground_border_color: foreground,
            foreground_border_width: 0.,
        },
        toggler::Status::Hovered { is_toggled } => toggler::Style {
            background: if is_toggled {
                toggled_background
            } else {
                background
            },
            background_border_color: Color::TRANSPARENT,
            background_border_width: 0.,
            foreground: if is_toggled {
                toggled_foreground
            } else {
                foreground
            },
            foreground_border_color: foreground,
            foreground_border_width: 0.,
        },
        toggler::Status::Disabled => toggler::Style {
            background:  background.darker(),
            background_border_color: Color::TRANSPARENT,
            background_border_width: 0.,
            foreground,
            foreground_border_color: foreground.darker(),
            foreground_border_width: 0.,
        },
    }
}
