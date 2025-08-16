use std::fmt::Debug;

use iced::{Executor, Settings, Subscription, Task};

use iced_winit::program::{Message, Renderer};

use iced_core::Element;

use crate::theme::Theme;

/// An interactive, native, cross-platform, multi-windowed application.
///
/// A [`Program`] can execute asynchronous actions by returning a
/// [`Task`] in some of its methods.
#[allow(missing_docs)]
pub trait Application: Sized {
    /// The message of the program.
    type Message: Message + 'static;

    /// The renderer of the program.
    type Renderer: Renderer;

    /// The executor of the program.
    type Executor: Executor;

    /// Returns the unique name of the [`Program`].
    fn name() -> &'static str;

    fn new() -> (Self, Task<Self::Message>);

    fn update(&mut self, message: Self::Message) -> Task<Self::Message>;

    fn view(&self, window: iced::window::Id) -> Element<'_, Self::Message, Theme, Self::Renderer>;

    fn title(&self, _window: iced::window::Id) -> String {
        let mut title = String::new();

        for (i, part) in Self::name().split("_").enumerate() {
            use std::borrow::Cow;

            let part = match part {
                "a" | "an" | "of" | "in" | "and" => Cow::Borrowed(part),
                _ => {
                    let mut part = part.to_owned();

                    if let Some(first_letter) = part.get_mut(0..1) {
                        first_letter.make_ascii_uppercase();
                    }

                    Cow::Owned(part)
                }
            };

            if i > 0 {
                title.push(' ');
            }

            title.push_str(&part);
        }

        format!("{title} - Iced")
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn theme(&self, _window: iced::window::Id) -> Theme {
        <Theme as Default>::default()
    }

    fn style(&self, theme: &Theme) -> iced::theme::Style {
        iced::theme::Base::base(theme)
    }

    fn scale_factor(&self, _window: iced::window::Id) -> f64 {
        1.0
    }
}

#[derive(Debug)]
enum AzaleaAppMessage<M>
where
    M: Debug,
{
    ThemeUpdated,
    Application(M),
}

struct Instance<A: Application> {
    program: A,
}

impl<A> Instance<A>
where
    A: Application,
    A::Message: 'static,
{
}

impl<A> Application for Instance<A>
where
    A: Application,
    A::Message: 'static,
{
    type Message = AzaleaAppMessage<A::Message>;

    type Renderer = A::Renderer;

    type Executor = A::Executor;

    fn name() -> &'static str {
        A::name()
    }

    /// Creates a new [`Instance`] of the given [`Program`].
    fn new() -> (Self, Task<AzaleaAppMessage<A::Message>>) {
        let (program, task) = A::new();

        (Self { program }, task.map(AzaleaAppMessage::Application))
    }
    /// Returns the current title of the [`Instance`].
    fn title(&self, window: iced::window::Id) -> String {
        self.program.title(window)
    }

    /// Processes the given message and updates the [`Instance`].
    fn update(
        &mut self,
        message: AzaleaAppMessage<A::Message>,
    ) -> Task<AzaleaAppMessage<A::Message>> {
        match message {
            AzaleaAppMessage::ThemeUpdated => Task::none(),
            AzaleaAppMessage::Application(message) => self
                .program
                .update(message)
                .map(AzaleaAppMessage::Application),
        }
    }

    /// Produces the current widget tree of the [`Instance`].
    fn view(&self, window: iced::window::Id) -> Element<'_, Self::Message, Theme, A::Renderer> {
        self.program.view(window).map(Self::Message::Application)
    }

    /// Returns the current [`Subscription`] of the [`Instance`].
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            Subscription::run(Theme::subscribe).map(|_| Self::Message::ThemeUpdated),
            self.program.subscription().map(Self::Message::Application),
        ])
    }

    /// Returns the current theme of the [`Instance`].
    fn theme(&self, window: iced::window::Id) -> Theme {
        self.program.theme(window)
    }

    /// Returns the current [`theme::Style`] of the [`Instance`].
    fn style(&self, theme: &Theme) -> iced::theme::Style {
        self.program.style(theme)
    }

    /// Returns the current scale factor of the [`Instance`].
    fn scale_factor(&self, window: iced::window::Id) -> f64 {
        self.program.scale_factor(window)
    }
}

pub fn run<A>(settings: Settings) -> Result<(), iced::Error>
where
    A: Application + 'static,
    A::Message: 'static,
{
    iced::daemon(
        Instance::<A>::new,
        Instance::<A>::update,
        Instance::<A>::view,
    )
    .subscription(Instance::<A>::subscription)
    .title(Instance::<A>::title)
    .theme(Instance::<A>::theme)
    .settings(settings)
    .scale_factor(Instance::<A>::scale_factor)
    .run()
}
