use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Child, Command},
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::UnboundedSender;

use super::serde_types::{LimitsDef, SizeDef};
use iced::{
    Size, Task,
    futures::{self, SinkExt},
    window,
};
use iced_core::layout::Limits;
use iced_layershell::{
    actions::{ActionCallback, IcedNewMenuSettings, IcedNewPopupSettings, IcedXdgWindowSettings},
    reexport::{Anchor, NewLayerShellSettings},
};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast::Sender, mpsc};

#[derive(Debug, Clone)]
pub enum RuntimeRequest {
    Launch {
        info: AppletInfo,
        order: u32,
        position: AppletPosition,
    },
}

#[derive(Debug, Clone)]
pub enum RuntimeMessage {
    Ready(UnboundedSender<RuntimeRequest>),
    New(Applet),
    Request(AppletRequest, u32),
    Shutdown(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppletEvent {
    GridSize(u32),
    Size(u32, u32),
    Margin {
        top: i32,
        right: i32,
        bottom: i32,
        left: i32,
    },
    Direction(Direction),
    Show,
    Hide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppletRequest {
    #[serde(with = "LimitsDef")]
    Limits(Limits),
    #[serde(with = "SizeDef")]
    Intrinsic(Size),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppletPosition {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone)]
pub struct Applet {
    pub id: u32,
    pub order: u32,
    pub info: AppletInfo,
    pub sender: Sender<AppletEvent>,
    pub limits: Option<iced::advanced::layout::Limits>,
    pub intrinsic: Option<Size>,
    pub size: Option<Size>,
    pub margin: (i32, i32, i32, i32),
    pub process: Arc<Mutex<Child>>,
    pub position: AppletPosition,
}

impl PartialEq for Applet {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.order == other.order
            && self.info == other.info
            && self.limits == other.limits
            && self.intrinsic == other.intrinsic
            && self.position == other.position
    }
}

impl PartialOrd for Applet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.position.cmp(&other.position) {
            std::cmp::Ordering::Equal => {}
            ord => return Some(ord),
        }
        Some(self.order.cmp(&other.order))
    }
}

pub fn limits_from_grid(limits: &Limits, size: u32) -> Limits {
    let max = limits.max();
    let min = limits.min();
    Limits::new(
        Size::new(min.width * size as f32, min.height * size as f32),
        Size::new(max.width * size as f32, max.height * size as f32),
    )
}

#[derive(Debug, Clone)]
pub enum AppletMessage {
    Ready(mpsc::UnboundedSender<AppletRequest>),
    Event(AppletEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppletInfo {
    pub id: String,
    pub name: String,
    pub keywords: Vec<String>,
    pub exec: String,
    pub icon: PathBuf,
}

impl AppletInfo {
    pub fn all() -> Result<Vec<Self>, Box<dyn Error>> {
        let applets_dir = dirs::home_dir()
            .ok_or("No home directory")?
            .join(Path::new(".config/azalea/applets"));
        let dir = fs::read_dir(applets_dir)?;
        Ok(dir
            .into_iter()
            .filter_map(|e| {
                let entry = e.ok()?;
                let content = fs::read_to_string(entry.path()).ok()?;
                toml::from_str(&content).ok()
            })
            .collect())
    }

    pub fn launch(&self) -> Result<Child, Box<dyn Error>> {
        let args: Vec<_> = self.exec.split_whitespace().collect();
        let child = Command::new(args.get(0).ok_or("invalid index")?)
            .args(args.get(1..).unwrap_or_default())
            .spawn()?;
        Ok(child)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let toml: Self = toml::from_str(&content)?;
        Ok(toml)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

impl Direction {
    pub fn to_applet_anchor(self) -> Anchor {
        match self {
            Direction::Top => Anchor::Left | Anchor::Top,
            Direction::Left => Anchor::Top | Anchor::Left,
            Direction::Right => Anchor::Top | Anchor::Right,
            Direction::Bottom => Anchor::Left | Anchor::Bottom,
        }
    }
}

impl From<Direction> for Anchor {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Top => Anchor::Left | Anchor::Right | Anchor::Top,
            Direction::Left => Anchor::Top | Anchor::Left | Anchor::Bottom,
            Direction::Right => Anchor::Top | Anchor::Right | Anchor::Bottom,
            Direction::Bottom => Anchor::Left | Anchor::Right | Anchor::Bottom,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AppletDirection {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct AppletCore {
    pub(crate) sender: Option<futures::channel::mpsc::UnboundedSender<AppletCoreRequest>>,
    pub(crate) applet_sender: Option<UnboundedSender<AppletRequest>>,
    pub(crate) pending_requests: Vec<AppletCoreRequest>,
    pub(crate) applet_id: Option<iced::window::Id>,
    pub(crate) intrinsic: Option<Size<f32>>,
    pub(crate) limits: Option<Limits>,
    pub margin: Margin,
    pub grid_size: u32,
    pub size: Size<u32>,
    pub direction: Direction,
    pub visible: bool,
    pub layer: iced_layershell::reexport::Layer,
}

impl AppletCore {
    pub fn new() -> Self {
        Self {
            sender: None,
            applet_sender: None,
            applet_id: None,
            intrinsic: None,
            limits: None,
            pending_requests: Vec::new(),
            margin: Margin::default(),
            grid_size: 50,
            size: Size::new(50, 50),
            direction: Direction::Bottom,
            layer: iced_layershell::reexport::Layer::Top,
            visible: false,
        }
    }
    pub fn new_layershell(
        &mut self,
        id: window::Id,
        settings: NewLayerShellSettings,
    ) -> Task<window::Id> {
        let mut task = Task::none();
        let request = AppletCoreRequest::NewLayershell { settings, id };
        if let Some(sender) = &self.sender {
            let mut sender = sender.clone();
            task = task.chain(Task::future(async move {
                sender.send(request).await.unwrap();
                id
            }))
        } else {
            self.pending_requests.push(request);
        }
        task
    }

    pub fn new_window(
        &mut self,
        id: window::Id,
        settings: IcedXdgWindowSettings,
    ) -> Task<window::Id> {
        let mut task = Task::none();
        let request = AppletCoreRequest::NewWindow { settings, id };
        if let Some(sender) = &self.sender {
            let mut sender = sender.clone();
            task = task.chain(Task::future(async move {
                sender.send(request).await.unwrap();
                id
            }))
        } else {
            self.pending_requests.push(request);
        }
        task
    }

    pub fn new_popup(
        &mut self,
        id: window::Id,
        settings: IcedNewPopupSettings,
    ) -> Task<window::Id> {
        let mut task = Task::none();
        let request = AppletCoreRequest::NewPopup { id, settings };
        if let Some(sender) = &self.sender {
            let mut sender = sender.clone();
            task = task.chain(Task::future(async move {
                sender.send(request).await.unwrap();
                id
            }))
        } else {
            self.pending_requests.push(request);
        }
        task
    }

    pub fn new_menu_popup(
        &mut self,
        id: window::Id,
        settings: IcedNewMenuSettings,
    ) -> Task<window::Id> {
        let mut task = Task::none();
        let request = AppletCoreRequest::NewMenuPopup { id, settings };
        if let Some(sender) = &self.sender {
            let mut sender = sender.clone();
            task = task.chain(Task::future(async move {
                sender.send(request).await.unwrap();
                id
            }))
        } else {
            self.pending_requests.push(request);
        }
        task
    }

    pub fn set_input_region(
        &mut self,
        id: window::Id,
        callback: ActionCallback,
    ) -> Task<window::Id> {
        let mut task = Task::none();
        let request = AppletCoreRequest::SetInputRegion { id, callback };
        if let Some(sender) = &self.sender {
            let mut sender = sender.clone();
            task = task.chain(Task::future(async move {
                sender.send(request).await.unwrap();
                id
            }))
        } else {
            self.pending_requests.push(request);
        }
        task
    }

    pub fn change_achor(&mut self, id: window::Id, anchor: Anchor) -> Task<window::Id> {
        let mut task = Task::none();
        let request = AppletCoreRequest::ChangeAnchor { id, anchor };
        if let Some(sender) = &self.sender {
            let mut sender = sender.clone();
            task = task.chain(Task::future(async move {
                sender.send(request).await.unwrap();
                id
            }))
        } else {
            self.pending_requests.push(request);
        }
        task
    }
    pub fn change_size(&mut self, id: window::Id, size: Size<u32>) -> Task<window::Id> {
        let mut task = Task::none();
        let request = AppletCoreRequest::ChangeSize { id, size };
        if let Some(sender) = &self.sender {
            let mut sender = sender.clone();
            task = task.chain(Task::future(async move {
                sender.send(request).await.unwrap();
                id
            }))
        } else {
            self.pending_requests.push(request);
        }
        task
    }
    pub fn change_achor_and_size(
        &mut self,
        id: window::Id,
        anchor: Anchor,
        size: Size<u32>,
    ) -> Task<window::Id> {
        let mut task = Task::none();
        let request = AppletCoreRequest::ChangeAnchorAndSiize { id, anchor, size };
        if let Some(sender) = &self.sender {
            let mut sender = sender.clone();
            task = task.chain(Task::future(async move {
                sender.send(request).await.unwrap();
                id
            }))
        } else {
            self.pending_requests.push(request);
        }
        task
    }
    pub fn change_margin(&mut self, id: window::Id, margin: Margin) -> Task<window::Id> {
        let mut task = Task::none();
        let request = AppletCoreRequest::ChangeMargin { id, margin };
        if let Some(sender) = &self.sender {
            let mut sender = sender.clone();
            task = task.chain(Task::future(async move {
                sender.send(request).await.unwrap();
                id
            }))
        } else {
            self.pending_requests.push(request);
        }
        task
    }

    pub fn change_exclusive_zone(&mut self, id: window::Id, zone_size: i32) -> Task<window::Id> {
        let mut task = Task::none();
        let request = AppletCoreRequest::ChangeExclusiveZone { id, zone_size };
        if let Some(sender) = &self.sender {
            let mut sender = sender.clone();
            task = task.chain(Task::future(async move {
                sender.send(request).await.unwrap();
                id
            }))
        } else {
            self.pending_requests.push(request);
        }
        task
    }

    pub fn close(&mut self, id: window::Id) -> Task<window::Id> {
        let mut task = Task::none();
        let request = AppletCoreRequest::Close { id };
        if let Some(sender) = &self.sender {
            let mut sender = sender.clone();
            task = task.chain(Task::future(async move {
                sender.send(request).await.unwrap();
                id
            }))
        } else {
            self.pending_requests.push(request);
        }
        task
    }
}

#[derive(Debug, Clone, Default)]
pub struct Margin {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

#[derive(Debug, Clone)]
pub enum AppletCoreRequest {
    Ready(iced::futures::channel::mpsc::UnboundedSender<AppletCoreRequest>),
    NewLayershell {
        id: window::Id,
        settings: NewLayerShellSettings,
    },
    NewWindow {
        id: window::Id,
        settings: IcedXdgWindowSettings,
    },
    NewPopup {
        id: window::Id,
        settings: IcedNewPopupSettings,
    },
    NewMenuPopup {
        id: window::Id,
        settings: IcedNewMenuSettings,
    },
    SetInputRegion {
        id: window::Id,
        callback: ActionCallback,
    },
    ChangeAnchor {
        id: window::Id,
        anchor: Anchor,
    },
    ChangeSize {
        id: window::Id,
        size: Size<u32>,
    },
    ChangeAnchorAndSiize {
        id: window::Id,
        anchor: Anchor,
        size: Size<u32>,
    },
    ChangeMargin {
        id: window::Id,
        margin: Margin,
    },
    ChangeExclusiveZone {
        id: window::Id,
        zone_size: i32,
    },
    Close {
        id: window::Id,
    },
    SendLimits(Limits)
}

#[derive(Debug, Clone)]
pub enum AppletCoreEvent {
    Ready(AppletCore),
}
