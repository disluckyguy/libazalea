use crate::theme::Theme;
use iced::{
    Alignment, Element, Length, Padding, Pixels, alignment,
    widget::{Column, Container, Row, Text, container},
};
pub struct ListItem<'a, Message> {
    padding: Padding,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    clip: bool,
    leading: Option<Element<'a, Message, Theme>>,
    title: Text<'a, Theme>,
    subtitle: Option<Text<'a, Theme>>,
    trailing: Option<Element<'a, Message, Theme>>,
}

impl<'a, Message> ListItem<'a, Message> {
    /// Creates a [`Container`] with the given content.
    pub fn new(title: impl Into<Text<'a, Theme>>) -> Self {
        ListItem {
            padding: Padding::ZERO,
            width: Length::Fill,
            height: Length::Shrink,
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            clip: false,
            leading: None,
            title: title.into(),
            subtitle: None,
            trailing: None,
        }
    }

    /// Sets the [`Padding`] of the [`Container`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Container`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Container`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the maximum width of the [`Container`].
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// Sets the maximum height of the [`Container`].
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = max_height.into().0;
        self
    }

    /// Sets the width of the [`Container`] and centers its contents horizontally.
    pub fn center_x(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(alignment::Horizontal::Center)
    }

    /// Sets the height of the [`Container`] and centers its contents vertically.
    pub fn center_y(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(alignment::Vertical::Center)
    }

    /// Centers the contents in both the horizontal and vertical axes of the
    /// [`Container`].
    ///
    /// This is equivalent to chaining [`center_x`] and [`center_y`].
    ///
    /// [`center_x`]: Self::center_x
    /// [`center_y`]: Self::center_y
    pub fn center(self, length: impl Into<Length>) -> Self {
        let length = length.into();

        self.center_x(length).center_y(length)
    }

    /// Aligns the contents of the [`Container`] to the left.
    pub fn align_left(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(alignment::Horizontal::Left)
    }

    /// Aligns the contents of the [`Container`] to the right.
    pub fn align_right(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(alignment::Horizontal::Right)
    }

    /// Aligns the contents of the [`Container`] to the top.
    pub fn align_top(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(alignment::Vertical::Top)
    }

    /// Aligns the contents of the [`Container`] to the bottom.
    pub fn align_bottom(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(alignment::Vertical::Bottom)
    }

    /// Sets the content alignment for the horizontal axis of the [`Container`].
    pub fn align_x(mut self, alignment: impl Into<alignment::Horizontal>) -> Self {
        self.horizontal_alignment = alignment.into();
        self
    }

    /// Sets the content alignment for the vertical axis of the [`Container`].
    pub fn align_y(mut self, alignment: impl Into<alignment::Vertical>) -> Self {
        self.vertical_alignment = alignment.into();
        self
    }

    /// Sets whether the contents of the [`Container`] should be clipped on
    /// overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    pub fn leading(mut self, leading: Element<'a, Message, Theme>) -> Self {
        self.leading = Some(leading);
        self
    }

    pub fn trailing(mut self, trailing: Element<'a, Message, Theme>) -> Self {
        self.trailing = Some(trailing);
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<Text<'a, Theme>>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }
}

impl<'a, Message: 'a> From<ListItem<'a, Message>> for Element<'a, Message, Theme> {
    fn from(value: ListItem<'a, Message>) -> Self {
        let row = Row::new()
            .push(value.leading)
            .push(
                Column::new()
                    .push(value.title)
                    .push(value.subtitle)
                    .width(Length::Fill),
            )
            .push(value.trailing)
            .align_y(Alignment::Center)
            .width(Length::Fill);
        Container::new(row)
            .center_x(value.width)
            .center_y(value.height)
            .width(value.width)
            .height(value.height)
            .max_width(value.max_width)
            .max_height(value.max_height)
            .clip(value.clip)
            .padding(12)
            .style(|theme| container::Style {
                text_color: Some(theme.on_surface),
                ..Default::default()
            })
            .into()
    }
}
