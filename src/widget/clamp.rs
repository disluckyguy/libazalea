use core::f32;

use iced::{Element, Length, Pixels, widget::Container};

use crate::theme::Theme;

pub struct Clamp<'a, Message> {
    content: Element<'a, Message, Theme>,
    max_width: f32,
    max_height: f32,
    content_max_width: f32,
    content_max_height: f32,
    width: Length,
    height: Length,
}

impl<'a, Message> Clamp<'a, Message> {
    pub fn new(content: impl Into<Element<'a, Message, Theme>>) -> Self {
        Self {
            content: content.into(),
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
            content_max_width: f32::INFINITY,
            content_max_height: f32::INFINITY,
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = max_height.into().0;
        self
    }

    pub fn content_max_width(mut self, content_max_width: impl Into<Pixels>) -> Self {
        self.content_max_width = content_max_width.into().0;
        self
    }

    pub fn content_max_height(mut self, content_max_height: impl Into<Pixels>) -> Self {
        self.content_max_height = content_max_height.into().0;
        self
    }
}

impl<'a, Message: 'a> From<Clamp<'a, Message>> for Element<'a, Message, Theme> {
    fn from(value: Clamp<'a, Message>) -> Self {
        Container::new(
            Container::new(value.content)
                .width(Length::Fill)
                .height(Length::Fill)
                .max_width(value.content_max_width)
                .max_height(value.content_max_height),
        )
        .center_x(value.width)
        .center_y(value.height)
        .max_width(value.max_width)
        .max_height(value.max_height)
        .into()
    }
}
