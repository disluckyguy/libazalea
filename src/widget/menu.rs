use color::{ColorSpace, Hsl};
use iced::{
    Alignment, Background, Border, Color, Element, Length,
    widget::{self, Button, Column, Container, Text, row},
};
use iced_aw::{Menu, menu::Item};

pub fn standard<'a, Message>(
    items: Vec<Item<'a, Message, iced::Theme, iced::Renderer>>,
) -> Menu<'a, Message, iced::Theme, iced::Renderer> {
    Menu::new(items).width(100).spacing(-4.0)
}

pub struct MenuItem<'a, Message: Clone> {
    leading: Option<Element<'a, Message>>,
    title: Element<'a, Message>,
    trailing: Option<Element<'a, Message>>,
    on_press: Option<OnPress<'a, Message>>,
}

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

impl<'a, Message: Clone + 'a> OnPress<'a, Message> {
    fn get(&self) -> Message {
        match self {
            OnPress::Direct(message) => message.clone(),
            OnPress::Closure(f) => f(),
        }
    }
}

impl<'a, Message: Clone + 'a> MenuItem<'a, Message> {
    pub fn new(title: Element<'a, Message>) -> Self {
        Self {
            leading: None,
            title,
            trailing: None,
            on_press: None,
        }
    }

    pub fn leading(mut self, content: Element<'a, Message>) -> Self {
        self.leading = Some(content);
        self
    }

    pub fn trailing(mut self, content: Element<'a, Message>) -> Self {
        self.trailing = Some(content);
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// Unless `on_press` is called, the [`Button`] will be disabled.
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// This is analogous to [`Button::on_press`], but using a closure to produce
    /// the message.
    ///
    /// This closure will only be called when the [`Button`] is actually pressed and,
    /// therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed,
    /// if `Some`.
    ///
    /// If `None`, the [`Button`] will be disabled.
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press.map(OnPress::Direct);
        self
    }
}

impl<'a, Message: Clone + 'a> From<MenuItem<'a, Message>> for Element<'a, Message> {
    fn from(value: MenuItem<'a, Message>) -> Element<'a, Message> {
        let mut row = row![].width(Length::Fill).spacing(6);

        if let Some(leading) = value.leading {
            row = row.push(leading);
        }
        row = row.push(Container::new(value.title).width(Length::Fill));
        if let Some(trailing) = value.trailing {
            row = row.push(trailing);
        }
        Button::new(row)
            .on_press_maybe(value.on_press.map(|op| op.get()))
            .padding(4)
            .height(30)
            .style(|theme, status| match status {
                widget::button::Status::Active => widget::button::Style {
                    background: Some(Background::Color(theme.palette().background)),
                    text_color: theme.palette().text,
                    border: Border::default().rounded(8),
                    ..Default::default()
                },
                widget::button::Status::Hovered => widget::button::Style {
                    background: Some(Background::Color(mix(theme.palette().primary))),
                    text_color: theme.palette().text,
                    border: Border::default().rounded(8),
                    ..Default::default()
                },
                widget::button::Status::Pressed => widget::button::Style {
                    background: Some(Background::Color(mix(theme.palette().primary))),
                    text_color: theme.palette().text,
                    border: Border::default()
                        .rounded(8)
                        .color(theme.palette().primary)
                        .width(2),
                    ..Default::default()
                },
                widget::button::Status::Disabled => widget::button::Style {
                    background: Some(Background::Color(theme.palette().background)),
                    text_color: theme.palette().text.scale_alpha(0.5),
                    border: Border::default(),
                    ..Default::default()
                },
            })
            .into()
    }
}

pub struct Section<'a, Message: Clone> {
    title: Option<String>,
    items: Vec<MenuItem<'a, Message>>,
    max_width: f32,
    width: Length,
    height: Length,
}

impl<'a, Message: Clone + 'a> Section<'a, Message> {
    /// Creates an empty [`Row`].
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
    /// Creates a [`Row`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// Creates a [`Row`] with the given elements.
    pub fn with_children(children: impl IntoIterator<Item = MenuItem<'a, Message>>) -> Self {
        let iterator = children.into_iter();

        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// Creates a [`Row`] from an already allocated [`Vec`].
    ///
    /// Keep in mind that the [`Row`] will not inspect the [`Vec`], which means
    /// it won't automatically adapt to the sizing strategy of its contents.
    ///
    /// If any of the children have a [`Length::Fill`] strategy, you will need to
    /// call [`Row::width`] or [`Row::height`] accordingly.
    pub fn from_vec(items: Vec<MenuItem<'a, Message>>) -> Self {
        Self {
            title: None,
            items,
            width: Length::Fill,
            height: Length::Shrink,
            max_width: f32::INFINITY,
        }
    }
    /// Sets the width of the [`Row`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Row`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Adds an [`Element`] to the [`Row`].
    pub fn push(mut self, child: impl Into<MenuItem<'a, Message>>) -> Self {
        self.items.push(child.into());
        self
    }

    /// Adds an element to the [`Row`], if `Some`.
    pub fn push_maybe(self, child: Option<impl Into<MenuItem<'a, Message>>>) -> Self {
        if let Some(child) = child {
            self.push(child.into())
        } else {
            self
        }
    }

    /// Extends the [`Row`] with the given children.
    pub fn extend(self, children: impl IntoIterator<Item = MenuItem<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }
}

impl<'a, Message: Clone + 'a> From<Section<'a, Message>> for  Element<'a, Message> {
    fn from(value: Section<'a, Message>) -> Element<'a, Message> {
        Container::new(
            Column::new()
                .push_maybe(value.title.map(|title| {
                    Container::new(Text::new(title).size(16).font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    }))
                    .align_y(Alignment::Center)
                }))
                .padding(2)
                .extend(value.items.into_iter().map(|e| e.into())),
        )
        .width(value.width)
        .height(value.height)
        .max_width(value.max_width)
        .padding(8)
        .into()
    }
}

pub fn mix(cb: Color) -> Color {
    let linear = cb.into_linear();
    let srgb = [linear[0], linear[1], linear[2]];
    let mut foreground = Hsl::from_linear_srgb(srgb);
    foreground[1] = 20.;
    foreground[2] = 40.;
    let [r, g, b] = Hsl::to_linear_srgb(foreground);
    Color::from_linear_rgba(r, g, b, 1.0)
}
