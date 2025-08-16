pub mod connection;
pub mod interface;
pub mod serde_types;

use iced::{
    Element, Executor, Length, Size, Subscription, Task,
    futures::{SinkExt, Stream, StreamExt, channel::mpsc::unbounded},
};
use iced_core::layout::Limits;
use iced_layershell::reexport::NewLayerShellSettings;
use iced_winit::program::{Message, Renderer};

use crate::{
    applet::{
        connection::applet_sub,
        interface::{AppletCore, AppletCoreRequest, AppletMessage, Margin, limits_from_grid},
    },
    runtime,
    theme::Theme,
    widget::autosize::SizeBox,
};

/// An interactive, native, cross-platform, multi-windowed application.
///
/// A [`Program`] can execute asynchronous actions by returning a
/// [`Task`] in some of its methods.
pub trait Applet: Sized {
    /// The message of the program.
    type Message: Message + 'static;

    /// The renderer of the program.
    type Renderer: Renderer;

    /// The executor of the program.
    type Executor: Executor;

    fn core(&self) -> &AppletCore;

    fn core_mut(&mut self) -> &mut AppletCore;

    /// Returns the unique name of the [`Program`].
    fn name() -> &'static str;

    fn new() -> (Self, Task<Self::Message>);

    fn update(&mut self, message: Self::Message) -> Task<Self::Message>;

    fn applet_view(
        &self,
        window: iced::window::Id,
    ) -> (Limits, Element<'_, Self::Message, Theme, Self::Renderer>);

    fn window_view(
        &self,
        _window: iced::window::Id,
    ) -> Element<'_, Self::Message, Theme, Self::Renderer> {
        None::<Element<Self::Message, Theme, Self::Renderer>>.into()
    }

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

#[iced_layershell::to_layer_message(multi)]
#[derive(Debug)]
enum AzaleaAppMessage<M: Sync + Send + 'static> {
    Ignore,
    ThemeUpdated,
    AppletUpdate(M),
    AppletMessage(AppletMessage),
    CoreRequest(AppletCoreRequest),
    AppletLayout(Size<f32>),
}

struct Instance<A: Applet> {
    program: A,
}

impl<A> Instance<A>
where
    A: Applet,
    A::Message: 'static,
{
}

impl<A> Applet for Instance<A>
where
    A: Applet,
    A::Message: 'static,
{
    type Message = AzaleaAppMessage<A::Message>;

    type Renderer = A::Renderer;

    type Executor = A::Executor;

    fn core(&self) -> &AppletCore {
        self.program.core()
    }

    fn core_mut(&mut self) -> &mut AppletCore {
        self.program.core_mut()
    }

    fn name() -> &'static str {
        A::name()
    }

    /// Creates a new [`Instance`] of the given [`Program`].
    fn new() -> (Self, Task<AzaleaAppMessage<A::Message>>) {
        let (program, task) = A::new();

        (Self { program }, task.map(AzaleaAppMessage::AppletUpdate))
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
            AzaleaAppMessage::AppletUpdate(message) => self
                .program
                .update(message)
                .map(AzaleaAppMessage::AppletUpdate),
            AzaleaAppMessage::CoreRequest(request) => match request {
                AppletCoreRequest::SendLimits(limits) => {
                    let core = self.core_mut();
                    core.limits = Some(limits);
                    if let Some(sender) = core.applet_sender.clone() {
                        Task::future(async move {
                            sender
                                .send(interface::AppletRequest::Limits(limits))
                                .unwrap();
                        })
                        .map(|_| Self::Message::Ignore)
                    } else {
                        Task::none()
                    }
                }
                AppletCoreRequest::Ready(sender) => {
                    let core = self.core_mut();
                    core.sender = Some(sender);
                    let mut task = Task::none();
                    for request in core.pending_requests.drain(..) {
                        task = task.chain(Task::done(request))
                    }
                    task.map(Self::Message::CoreRequest)
                }
                AppletCoreRequest::NewLayershell { id, settings } => {
                    Task::done(AzaleaAppMessage::NewLayerShell { settings, id })
                }
                AppletCoreRequest::NewWindow { id, settings } => {
                    Task::done(AzaleaAppMessage::NewBaseWindow { settings, id })
                }
                AppletCoreRequest::NewPopup { id, settings } => {
                    Task::done(AzaleaAppMessage::NewPopUp { settings, id })
                }
                AppletCoreRequest::NewMenuPopup { id, settings } => {
                    Task::done(AzaleaAppMessage::NewMenu { settings, id })
                }
                AppletCoreRequest::SetInputRegion { id, callback } => {
                    Task::done(AzaleaAppMessage::SetInputRegion { id, callback })
                }
                AppletCoreRequest::ChangeAnchor { id, anchor } => {
                    Task::done(AzaleaAppMessage::AnchorChange { anchor, id })
                }
                AppletCoreRequest::ChangeSize { id, size } => {
                    Task::done(AzaleaAppMessage::SizeChange {
                        size: (size.width, size.height),
                        id,
                    })
                }
                AppletCoreRequest::ChangeAnchorAndSiize { id, anchor, size } => {
                    Task::done(AzaleaAppMessage::AnchorSizeChange {
                        size: (size.width, size.height),
                        anchor,
                        id,
                    })
                }
                AppletCoreRequest::ChangeMargin { id, margin } => {
                    Task::done(AzaleaAppMessage::MarginChange {
                        margin: (margin.top, margin.right, margin.bottom, margin.left),
                        id,
                    })
                }
                AppletCoreRequest::ChangeExclusiveZone { id, zone_size } => {
                    Task::done(AzaleaAppMessage::ExclusiveZoneChange { id, zone_size })
                }
                AppletCoreRequest::Close { id } => Task::done(AzaleaAppMessage::RemoveWindow(id)),
            },
            AzaleaAppMessage::AppletMessage(message) => match message {
                AppletMessage::Ready(applet_sender) => {
                    self.core_mut().applet_sender = Some(applet_sender);
                    Task::none()
                }
                AppletMessage::Event(applet_event) => match applet_event {
                    interface::AppletEvent::GridSize(size) => {
                        self.core_mut().grid_size = size;
                        Task::none()
                    }
                    interface::AppletEvent::Size(width, height) => {
                        let core = self.core_mut();
                        core.size = Size::new(width, height);
                        if let Some(id) = core.applet_id {
                            Task::done(AzaleaAppMessage::AnchorSizeChange {
                                id,
                                anchor: core.direction.to_applet_anchor(),
                                size: (width, height),
                            })
                        } else {
                            Task::none()
                        }
                    }
                    interface::AppletEvent::Margin {
                        top,
                        right,
                        bottom,
                        left,
                    } => {
                        let core = self.core_mut();
                        core.margin = Margin {
                            top,
                            right,
                            bottom,
                            left,
                        };
                        if let Some(id) = core.applet_id {
                            Task::done(AzaleaAppMessage::MarginChange {
                                id,
                                margin: (top, right, bottom, left),
                            })
                        } else {
                            Task::none()
                        }
                    }
                    interface::AppletEvent::Direction(direction) => {
                        let core = self.core_mut();
                        core.direction = direction;
                        if let Some(id) = core.applet_id {
                            Task::done(AzaleaAppMessage::AnchorSizeChange {
                                id,
                                anchor: core.direction.to_applet_anchor(),
                                size: (core.size.width, core.size.height),
                            })
                        } else {
                            Task::none()
                        }
                    }
                    interface::AppletEvent::Show => {
                        let core = self.core_mut();
                        core.visible = true;
                        if core.applet_id.is_none() {
                            let id = iced::window::Id::unique();
                            core.applet_id = Some(id);
                            Task::done(AzaleaAppMessage::NewLayerShell {
                                settings: NewLayerShellSettings {
                                    size: Some((core.size.width, core.size.height)),
                                    layer: core.layer,
                                    anchor: core.direction.to_applet_anchor(),
                                    exclusive_zone: Some(-1),
                                    margin: Some((
                                        core.margin.top,
                                        core.margin.right,
                                        core.margin.bottom,
                                        core.margin.left,
                                    )),
                                    ..Default::default()
                                },
                                id,
                            })
                        } else {
                            Task::none()
                        }
                    }
                    interface::AppletEvent::Hide => {
                        let core = self.core_mut();
                        core.visible = false;
                        if let Some(id) = core.applet_id.take() {
                            Task::done(AzaleaAppMessage::RemoveWindow(id))
                        } else {
                            Task::none()
                        }
                    }
                },
            },
            AzaleaAppMessage::AppletLayout(size) => {
                let core = self.core_mut();
                if Some(size) != core.intrinsic {
                    if let Some(sender) = &mut core.applet_sender {
                    sender
                        .send(interface::AppletRequest::Intrinsic(size))
                        .unwrap()
                }
                }
                core.intrinsic = Some(size);
                Task::none()
            }
            _ => Task::none(),
        }
    }

    /// Produces the current widget tree of the [`Instance`].
    fn applet_view(
        &self,
        window: iced::window::Id,
    ) -> (Limits, Element<'_, Self::Message, Theme, A::Renderer>) {
        let (limits, view) = self.program.applet_view(window);
        if Some(limits) != self.core().limits {
            if let Some(mut sender) = self.core().sender.clone() {
                runtime().spawn(async move {
                    sender
                        .send(AppletCoreRequest::SendLimits(limits))
                        .await
                        .unwrap()
                });
            }
        }
        let intrinsic = self.core().intrinsic.clone();
        let sized_box = SizeBox::new(view.map(Self::Message::AppletUpdate))
            .limits(limits_from_grid(&limits, self.core().grid_size))
            .on_layout(move |rect| {
                let size = rect.size();
                if Some(size) != intrinsic {
                    Some(AzaleaAppMessage::AppletLayout(size))
                } else {
                    None
                }
            })
            .width(Length::Shrink)
            .height(Length::Shrink);
        (limits, sized_box.into())
    }

    /// Produces the current widget tree of the [`Instance`].
    fn window_view(
        &self,
        window: iced::window::Id,
    ) -> Element<'_, Self::Message, Theme, A::Renderer> {
        self.program
            .window_view(window)
            .map(Self::Message::AppletUpdate)
    }

    /// Returns the current [`Subscription`] of the [`Instance`].
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            Subscription::run(Theme::subscribe).map(|_| Self::Message::ThemeUpdated),
            self.program.subscription().map(Self::Message::AppletUpdate),
            Subscription::run(core_requests_subscribe).map(Self::Message::CoreRequest),
            Subscription::run(applet_sub).map(Self::Message::AppletMessage),
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

impl<A> Instance<A>
where
    A: Applet,
    A::Message: 'static,
{
    pub fn view(
        &self,
        id: iced::window::Id,
    ) -> Element<AzaleaAppMessage<A::Message>, Theme, A::Renderer> {
        if Some(id) == self.core().applet_id {
            let (limits, view) = self.applet_view(id);
            if Some(limits) != self.core().limits {
                if let Some(sender) = self.core().applet_sender.clone() {
                    sender
                        .send(interface::AppletRequest::Limits(limits))
                        .unwrap()
                };
            }
            view
        } else {
            self.window_view(id)
        }
    }
}

pub fn run<A>(settings: iced_layershell::Settings) -> iced_layershell::Result
where
    A: Applet + 'static,
    A::Message: Sync + Send + 'static,
{
    iced_layershell::build_pattern::daemon(
        Instance::<A>::new,
        || Instance::<A>::name().into(),
        Instance::<A>::update,
        Instance::<A>::view,
    )
    .subscription(Instance::<A>::subscription)
    .title(|state, id| Some(Instance::<A>::title(state, id)))
    .theme(Instance::<A>::theme)
    .settings(settings)
    .scale_factor(Instance::<A>::scale_factor)
    .run()
}

pub fn core_requests_subscribe() -> impl Stream<Item = AppletCoreRequest> {
    iced::stream::channel(100, async move |mut tx| {
        let (sender, mut receiver) = unbounded();
        tx.send(AppletCoreRequest::Ready(sender.clone()))
            .await
            .unwrap();
        loop {
            if let Some(request) = receiver.next().await {
                tx.send(request).await.unwrap();
            };
        }
    })
}
