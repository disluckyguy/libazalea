use iced::{Size, advanced::layout::Limits};
use serde::{Deserialize, Serialize};

/// A set of size constraints for layouting.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Limits")]
pub(crate) struct LimitsDef {
    #[serde(getter = "Limits::min")]
    #[serde(with = "SizeDef")]
    min: Size,
    #[serde(getter = "Limits::max")]
    #[serde(with = "SizeDef")]
    max: Size,
}

impl From<LimitsDef> for Limits {
    fn from(def: LimitsDef) -> Limits {
        Limits::new(def.min, def.max)
    }
}

/// An amount of space in 2 dimensions.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Size")]
pub(crate) struct SizeDef<T = f32> {
    /// The width.
    pub width: T,
    /// The height.
    pub height: T,
}
