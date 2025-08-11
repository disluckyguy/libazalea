use iced::{
    Alignment, Border, Element, Length, Padding, Pixels, Shadow, alignment,
    widget::{Column, Container, Rule, container},
};

use crate::theme::Theme;

pub struct BoxedList<'a, Message> {
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    align: Alignment,
    clip: bool,
    children: Vec<Element<'a, Message, Theme>>,
}

impl<'a, Message> BoxedList<'a, Message> {
    /// Creates an empty [`BoxedList`].
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    /// Creates a [`BoxedList`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// Creates a [`BoxedList`] with the given elements.
    pub fn with_children(children: impl IntoIterator<Item = Element<'a, Message, Theme>>) -> Self {
        let iterator = children.into_iter();

        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// Creates a [`BoxedList`] from an already allocated [`Vec`].
    ///
    /// Keep in mind that the [`BoxedList`] will not inspect the [`Vec`], which means
    /// it won't automatically adapt to the sizing strategy of its contents.
    ///
    /// If any of the children have a [`Length::Fill`] strategy, you will need to
    /// call [`Row::width`] or [`Row::height`] accordingly.
    pub fn from_vec(children: Vec<Element<'a, Message, Theme>>) -> Self {
        Self {
            spacing: 4.0,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
            align: Alignment::Start,
            clip: true,
            children,
        }
    }

    /// Sets the horizontal spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = amount.into().0;
        self
    }

    /// Sets the [`Padding`] of the [`BoxedList`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`BoxedList`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`BoxedList`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the maximum width of the [`BoxedList`].
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// Sets the maximum height of the [`BoxedList`].
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = max_height.into().0;
        self
    }

    /// Sets the vertical alignment of the contents of the [`BoxedList`] .
    pub fn align(mut self, align: impl Into<alignment::Vertical>) -> Self {
        self.align = Alignment::from(align.into());
        self
    }

    /// Sets whether the contents of the [`BoxedList`] should be clipped on
    /// overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Adds an [`Element`] to the [`BoxedList`].
    pub fn push(mut self, child: impl Into<Element<'a, Message, Theme>>) -> Self {
        let child = child.into();
        let child_size = child.as_widget().size_hint();

        self.width = self.width.enclose(child_size.width);
        self.height = self.height.enclose(child_size.height);

        self.children.push(child);
        self
    }

    /// Adds an element to the [`BoxedList`], if `Some`.
    pub fn push_maybe(self, child: Option<impl Into<Element<'a, Message, Theme>>>) -> Self {
        if let Some(child) = child {
            self.push(child)
        } else {
            self
        }
    }

    /// Extends the [`BoxedList`] with the given children.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message, Theme>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }
}

impl<'a, Message: 'a> From<BoxedList<'a, Message>> for Element<'a, Message, Theme> {
    fn from(value: BoxedList<'a, Message>) -> Self {
        let mut col = Column::new()
            .spacing(value.spacing)
            .width(Length::Fill)
            .align_x(value.align);
        let length = value.children.len();
        for (i, child) in value.children.into_iter().enumerate() {
            col = col.push(child);
            if i != length - 1 {
                col = col.push(Rule::horizontal(4.));
            }
        }
        Container::new(col)
            .center_x(value.width)
            .center_y(value.height)
            .width(value.width)
            .height(value.height)
            .max_width(value.max_width)
            .max_height(value.max_height)
            .clip(value.clip)
            .style(boxed_list_style)
            .into()
    }
}

pub fn boxed_list_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(theme.surface_container)),
        shadow: Shadow {
            blur_radius: 8.0,
            ..Default::default()
        },
        border: Border::default().rounded(8.),
        ..Default::default()
    }
}
