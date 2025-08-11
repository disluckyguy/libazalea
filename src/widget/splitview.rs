use core::f32;

use iced::{
    Background, Element, Length, Pixels,
    widget::{Container, Row, Stack, container},
};

use crate::theme::Theme;

pub struct SplitView<'a, Message> {
    content: Element<'a, Message, Theme>,
    sidebar: Element<'a, Message, Theme>,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    min_sidebar_width: f32,
    max_sidebar_width: f32,
    sidebar_collapsed: bool,
    sidebar_visible: bool,
    sidebar_position: SidebarPosition,
}

pub enum SidebarPosition {
    Start,
    End,
}

impl<'a, Message> SplitView<'a, Message> {
    pub fn new(
        content: impl Into<Element<'a, Message, Theme>>,
        sidebar: impl Into<Element<'a, Message, Theme>>,
    ) -> Self {
        Self {
            content: content.into(),
            sidebar: sidebar.into(),
            width: Length::Fill,
            height: Length::Fill,
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
            min_sidebar_width: 150.,
            max_sidebar_width: 250.,
            sidebar_collapsed: false,
            sidebar_visible: true,
            sidebar_position: SidebarPosition::Start,
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

    pub fn min_sidebar_width(mut self, min_sidebar_width: impl Into<Pixels>) -> Self {
        self.min_sidebar_width = min_sidebar_width.into().0;
        self
    }

    pub fn max_sidebar_height(mut self, max_sidebar_width: impl Into<Pixels>) -> Self {
        self.max_sidebar_width = max_sidebar_width.into().0;
        self
    }

    pub fn sidebar_collapsed(mut self, collapsed: bool) -> Self {
        self.sidebar_collapsed = collapsed;
        self
    }

    pub fn sidebar_visible(mut self, visible: bool) -> Self {
        self.sidebar_visible = visible;
        self
    }

    pub fn sidebar_position(mut self, position: SidebarPosition) -> Self {
        self.sidebar_position = position;
        self
    }
}

impl<'a, Message: 'a> From<SplitView<'a, Message>> for Element<'a, Message, Theme> {
    fn from(value: SplitView<'a, Message>) -> Self {
        let max_sidebar_width = value.max_sidebar_width.clone();
        let sidebar = Container::new(value.sidebar)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .max_width(max_sidebar_width)
            .style(|theme: &Theme| {
                container::Style::default().background(Background::Color(theme.surface_container))
            });
        let content = Container::new(value.content)
            .width(Length::FillPortion(3))
            .height(Length::Fill);
        let mut vec = vec![];
        if value.sidebar_visible {
            match value.sidebar_position {
                SidebarPosition::Start => {
                    vec.extend(vec![sidebar.into(), content.into()]);
                }
                SidebarPosition::End => {
                    vec.extend(vec![content.into(), sidebar.into()]);
                }
            }
        } else {
            vec.push(content.into());
        }
        Container::new(if value.sidebar_collapsed {
            Element::from(Stack::from_vec(vec).width(value.width).height(value.height))
        } else {
            Element::from(Row::from_vec(vec).width(value.width).height(value.height))
        })
        .max_width(value.max_width)
        .max_height(value.max_height)
        .into()
    }
}
