use iced::{Border, widget::container};

use crate::theme::Theme;

impl container::Catalog for Theme {
    type Class<'a> = container::StyleFn<'a, Theme>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(container::transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        class(self)
    }
}

pub fn card(theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(theme.on_surface_container),
        background: Some(iced::Background::Color(theme.surface_container)),
        border: Border::default().rounded(8.),
        ..Default::default()
    }
}
