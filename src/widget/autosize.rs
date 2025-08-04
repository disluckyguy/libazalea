use iced_core::alignment::{self, Alignment};
use iced_core::border::Border;
use iced_core::event::{self, Event};
use iced_core::gradient::{self, Gradient};
use iced_core::layout::{self, Limits};
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::widget::tree::{self, Tree};
use iced_core::widget::{self, Operation};
use iced_core::{
    self, Background, Clipboard, Color, Element, Layout, Length, Padding, Pixels, Point, Rectangle,
    Shadow, Shell, Size, Theme, Vector, Widget,
};
use iced_runtime::task::{self, Task};

pub struct SizeBox<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: Catalog,
    Renderer: iced_core::Renderer,
{
    id: Option<Id>,
    layout: Option<Box<dyn Fn(Rectangle) -> Option<Message> + 'a>>,
    limits: Limits,
    padding: Padding,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    clip: bool,
    content: Element<'a, Message, Theme, Renderer>,
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> SizeBox<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: iced_core::Renderer,
    Message: Clone,
{
    /// Creates a [`SizeBox`] with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        let content = content.into();
        let size = content.as_widget().size_hint();

        SizeBox {
            id: None,
            layout: None,
            limits: Limits::NONE,
            padding: Padding::ZERO,
            width: size.width.fluid(),
            height: size.height.fluid(),
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            clip: false,
            class: Theme::default(),
            content,
        }
    }

    /// Sets the [`Id`] of the [`SizeBox`].
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the [`Padding`] of the [`SizeBox`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`SizeBox`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`SizeBox`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the maximum width of the [`SizeBox`].
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// Sets the maximum height of the [`SizeBox`].
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = max_height.into().0;
        self
    }

    /// Sets the width of the [`SizeBox`] and centers its contents horizontally.
    pub fn center_x(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(alignment::Horizontal::Center)
    }

    /// Sets the height of the [`SizeBox`] and centers its contents vertically.
    pub fn center_y(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(alignment::Vertical::Center)
    }

    /// Centers the contents in both the horizontal and vertical axes of the
    /// [`SizeBox`].
    ///
    /// This is equivalent to chaining [`center_x`] and [`center_y`].
    ///
    /// [`center_x`]: Self::center_x
    /// [`center_y`]: Self::center_y
    pub fn center(self, length: impl Into<Length>) -> Self {
        let length = length.into();

        self.center_x(length).center_y(length)
    }

    /// Aligns the contents of the [`SizeBox`] to the left.
    pub fn align_left(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(alignment::Horizontal::Left)
    }

    /// Aligns the contents of the [`SizeBox`] to the right.
    pub fn align_right(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(alignment::Horizontal::Right)
    }

    /// Aligns the contents of the [`SizeBox`] to the top.
    pub fn align_top(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(alignment::Vertical::Top)
    }

    /// Aligns the contents of the [`SizeBox`] to the bottom.
    pub fn align_bottom(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(alignment::Vertical::Bottom)
    }

    /// Sets the content alignment for the horizontal axis of the [`SizeBox`].
    pub fn align_x(mut self, alignment: impl Into<alignment::Horizontal>) -> Self {
        self.horizontal_alignment = alignment.into();
        self
    }

    /// Sets the content alignment for the vertical axis of the [`SizeBox`].
    pub fn align_y(mut self, alignment: impl Into<alignment::Vertical>) -> Self {
        self.vertical_alignment = alignment.into();
        self
    }

    /// Sets whether the contents of the [`SizeBox`] should be clipped on
    /// overflow.ew(Point { x: 0., y: 0. }, Size::ZERO)
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Sets the style of the [`SizeBox`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`SizeBox`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    pub fn bounds(&self) -> Option<Rectangle> {
        self.state().downcast_ref::<State>().bounds
    }

    pub fn on_layout(mut self, message: impl Fn(Rectangle) -> Option<Message> + 'a) -> Self {
        self.layout = Some(Box::new(message));
        self
    }

    pub fn limits(mut self, limits: Limits) -> Self {
        self.limits = limits;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct State {
    bounds: Option<Rectangle>,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for SizeBox<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: iced_core::Renderer,
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.content.as_widget().children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.content.as_widget().diff(tree);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let bounds = iced_core::layout::sized(&self.limits, Length::Shrink, Length::Shrink, |_| {
            self.content
                .as_widget()
                .layout(tree, renderer, &self.limits)
                .size()
        });
        let state = tree.state.downcast_mut::<State>();
        state.bounds = Some(bounds.bounds());
        layout(
            limits,
            self.width,
            self.height,
            self.max_width,
            self.max_height,
            self.padding,
            self.horizontal_alignment,
            self.vertical_alignment,
            |limits| self.content.as_widget().layout(tree, renderer, limits),
        )
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(
            self.id.as_ref().map(|id| &id.0),
            layout.bounds(),
            &mut |operation| {
                self.content.as_widget().operate(
                    tree,
                    layout.children().next().unwrap(),
                    renderer,
                    operation,
                );
            },
        );
    }

    // fn update(
    //     &mut self,
    //     tree: &mut Tree,
    //     event: &Event,
    //     layout: Layout<'_>,
    //     cursor: mouse::Cursor,
    //     renderer: &Renderer,
    //     clipboard: &mut dyn Clipboard,
    //     shell: &mut Shell<'_, Message>,
    //     viewport: &Rectangle,
    // ) {
    //     if let Some(layout) = &self.layout
    //         && let Some(bounds) = tree.state.downcast_ref::<State>().bounds
    //         && let Some(message) = layout(bounds)
    //     {
    //         shell.publish(message);
    //     }
    //     self.content.as_widget_mut().update(
    //         tree,
    //         event,
    //         layout.children().next().unwrap(),
    //         cursor,
    //         renderer,
    //         clipboard,
    //         shell,
    //         viewport,
    //     );
    // }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if let Some(layout) = &self.layout
            && let Some(bounds) = tree.state.downcast_ref::<State>().bounds
            && let Some(message) = layout(bounds)
        {
            shell.publish(message);
        }
        self.content.as_widget_mut().on_event(
            tree,
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    // fn mouse_interaction(
    //     &self,
    //     tree: &Tree,
    //     layout: Layout<'_>,
    //     cursor: mouse::Cursor,
    //     viewport: &Rectangle,
    //     renderer: &Renderer,
    // ) -> mouse::Interaction {
    //     self.content.as_widget().mouse_interaction(
    //         tree,
    //         layout.children().next().unwrap(),
    //         cursor,
    //         viewport,
    //         renderer,
    //     )
    // }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        renderer_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let style = theme.style(&self.class);

        if let Some(clipped_viewport) = bounds.intersection(viewport) {
            draw_background(renderer, &style, bounds);

            self.content.as_widget().draw(
                tree,
                renderer,
                theme,
                &renderer::Style {
                    text_color: style.text_color.unwrap_or(renderer_style.text_color),
                },
                layout.children().next().unwrap(),
                cursor,
                if self.clip {
                    &clipped_viewport
                } else {
                    viewport
                },
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            tree,
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<SizeBox<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(
        column: SizeBox<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(column)
    }
}

/// Computes the layout of a [`SizeBox`].
pub fn layout(
    limits: &layout::Limits,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    padding: Padding,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    layout_content: impl FnOnce(&layout::Limits) -> layout::Node,
) -> layout::Node {
    layout::positioned(
        &limits.max_width(max_width).max_height(max_height),
        width,
        height,
        padding,
        |limits| layout_content(&limits.loose()),
        |content, size| {
            content.align(
                Alignment::from(horizontal_alignment),
                Alignment::from(vertical_alignment),
                size,
            )
        },
    )
}

/// Draws the background of a [`SizeBox`] given its [`Style`] and its `bounds`.
pub fn draw_background<Renderer>(renderer: &mut Renderer, style: &Style, bounds: Rectangle)
where
    Renderer: iced_core::Renderer,
{
    if style.background.is_some() || style.border.width > 0.0 || style.shadow.color.a > 0.0 {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: style.border,
                shadow: style.shadow,
                //snap: true,
            },
            style
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );
    }
}

/// The identifier of a [`SizeBox`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(widget::Id);

impl Id {
    /// Creates a custom [`Id`].
    pub fn new(id: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        Self(widget::Id::new(id))
    }

    /// Creates a unique [`Id`].
    ///
    /// This function produces a different [`Id`] every time it is called.
    pub fn unique() -> Self {
        Self(widget::Id::unique())
    }
}

impl From<Id> for widget::Id {
    fn from(id: Id) -> Self {
        id.0
    }
}

/// Produces a [`Task`] that queries the visible screen bounds of the
/// [`SizeBox`] with the given [`Id`].
pub fn visible_bounds(id: impl Into<Id>) -> Task<Option<Rectangle>> {
    let id = id.into();

    struct VisibleBounds {
        target: widget::Id,
        depth: usize,
        scrollables: Vec<(Vector, Rectangle, usize)>,
        bounds: Option<Rectangle>,
    }

    impl Operation<Option<Rectangle>> for VisibleBounds {
        fn scrollable(
            &mut self,
            _state: &mut dyn widget::operation::Scrollable,
            _id: Option<&widget::Id>,
            bounds: Rectangle,
            _content_bounds: Rectangle,
            translation: Vector,
        ) {
            match self.scrollables.last() {
                Some((last_translation, last_viewport, _depth)) => {
                    let viewport = last_viewport
                        .intersection(&(bounds - *last_translation))
                        .unwrap_or(Rectangle::new(Point::ORIGIN, Size::ZERO));

                    self.scrollables
                        .push((translation + *last_translation, viewport, self.depth));
                }
                None => {
                    self.scrollables.push((translation, bounds, self.depth));
                }
            }
        }

        fn container(
            &mut self,
            id: Option<&widget::Id>,
            bounds: Rectangle,
            operate_on_children: &mut dyn FnMut(&mut dyn Operation<Option<Rectangle>>),
        ) {
            if self.bounds.is_some() {
                return;
            }

            if id == Some(&self.target) {
                match self.scrollables.last() {
                    Some((translation, viewport, _)) => {
                        self.bounds = viewport.intersection(&(bounds - *translation));
                    }
                    None => {
                        self.bounds = Some(bounds);
                    }
                }

                return;
            }

            self.depth += 1;

            operate_on_children(self);

            self.depth -= 1;

            match self.scrollables.last() {
                Some((_, _, depth)) if self.depth == *depth => {
                    let _ = self.scrollables.pop();
                }
                _ => {}
            }
        }

        fn finish(&self) -> widget::operation::Outcome<Option<Rectangle>> {
            widget::operation::Outcome::Some(self.bounds)
        }
    }

    task::widget(VisibleBounds {
        target: id.into(),
        depth: 0,
        scrollables: Vec::new(),
        bounds: None,
    })
}

/// The appearance of a container.
#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    /// The text [`Color`] of the container.
    pub text_color: Option<Color>,
    /// The [`Background`] of the container.
    pub background: Option<Background>,
    /// The [`Border`] of the container.
    pub border: Border,
    /// The [`Shadow`] of the container.
    pub shadow: Shadow,
}

impl Style {
    /// Updates the text color of the [`Style`].
    pub fn color(self, color: impl Into<Color>) -> Self {
        Self {
            text_color: Some(color.into()),
            ..self
        }
    }

    /// Updates the border of the [`Style`].
    pub fn border(self, border: impl Into<Border>) -> Self {
        Self {
            border: border.into(),
            ..self
        }
    }

    /// Updates the background of the [`Style`].
    pub fn background(self, background: impl Into<Background>) -> Self {
        Self {
            background: Some(background.into()),
            ..self
        }
    }

    /// Updates the shadow of the [`Style`].
    pub fn shadow(self, shadow: impl Into<Shadow>) -> Self {
        Self {
            shadow: shadow.into(),
            ..self
        }
    }
}

impl From<Color> for Style {
    fn from(color: Color) -> Self {
        Self::default().background(color)
    }
}

impl From<Gradient> for Style {
    fn from(gradient: Gradient) -> Self {
        Self::default().background(gradient)
    }
}

impl From<gradient::Linear> for Style {
    fn from(gradient: gradient::Linear) -> Self {
        Self::default().background(gradient)
    }
}

/// The theme catalog of a [`SizeBox`].
pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// A styling function for a [`SizeBox`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl<'a, Theme> From<Style> for StyleFn<'a, Theme> {
    fn from(style: Style) -> Self {
        Box::new(move |_theme| style)
    }
}

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// A transparent [`SizeBox`].
pub fn transparent<Theme>(_theme: &Theme) -> Style {
    Style::default()
}
