use iced_layershell::reexport::Anchor;
use serde::{Deserialize, Serialize};

pub mod connection;
pub mod runtime;
pub mod serde_types;

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Child, Command},
    sync::{Arc, Mutex},
};

use iced::Size;
use iced_core::layout::Limits;
use serde_types::{LimitsDef, SizeDef};
use tokio::sync::{broadcast::Sender, mpsc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppletEvent {
    PanelSize(u32),
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
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
