use iced::widget::text;

use crate::theme::Theme;

impl text::Catalog for Theme {
    type Class<'a> = text::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_| text::Style::default())
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        class(self)
    }
}
