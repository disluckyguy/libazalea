use iced::{Font, Pixels};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Weight {
    Emphasized,
    #[default]
    Regular,
}

impl From<Weight> for iced::font::Weight {
    fn from(value: Weight) -> Self {
        match value {
            Weight::Emphasized => iced::font::Weight::ExtraBold,
            Weight::Regular => iced::font::Weight::Medium,
        }
    }
}

impl From<Weight> for iced::font::Font {
    fn from(value: Weight) -> Self {
        Font {
            weight: value.into(),
            ..Default::default()
        }
    }
}

pub enum Label {
    Small = 11,
    Medium = 12,
    Large = 14,
}

impl From<Label> for Pixels {
    fn from(value: Label) -> Self {
        Pixels::from(value as u32)
    }
}

pub enum Body {
    Small = 12,
    Medium = 14,
    Large = 16,
}

impl From<Body> for Pixels {
    fn from(value: Body) -> Self {
        Pixels::from(value as u32)
    }
}

pub enum Title {
    Small = 14,
    Medium = 16,
    Large = 22,
}

impl From<Title> for Pixels {
    fn from(value: Title) -> Self {
        Pixels::from(value as u32)
    }
}

pub enum Headline {
    Small = 24,
    Medium = 28,
    Large = 32,
}

impl From<Headline> for Pixels {
    fn from(value: Headline) -> Self {
        Pixels::from(value as u32)
    }
}

pub enum Display {
    Small = 36,
    Medium = 45,
    Large = 57,
}

impl From<Display> for Pixels {
    fn from(value: Display) -> Self {
        Pixels::from(value as u32)
    }
}
