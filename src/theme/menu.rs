use iced::{Border, overlay::menu};

use crate::theme::Theme;

impl menu::Catalog for Theme {
    type Class<'a> = menu::StyleFn<'a, Self>;

    fn default<'a>() -> <Self as menu::Catalog>::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &<Self as menu::Catalog>::Class<'_>) -> menu::Style {
        class(self)
    }
}

pub fn default(theme: &Theme) -> menu::Style {
    menu::Style {
        background: iced::Background::Color(theme.surface_container),
        border: Border::default().rounded(8.),
        text_color: theme.on_surface,
        selected_text_color: theme.on_surface,
        selected_background: iced::Background::Color(theme.surface_container_highest),
    }
}
