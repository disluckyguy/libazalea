pub mod button;
pub mod catalogs;
pub mod container;
pub mod menu;
pub mod pallete;
pub mod scrollable;
pub mod text;
pub mod text_input;
pub mod toggler;
pub mod svg;

use std::{
    sync::{Arc, LazyLock},
};

use arc_swap::ArcSwap;
use iced::Color;
use mundy::{Interest, Preferences};
use palette::convert::FromColorUnclamped;

use crate::theme::pallete::{Tones, toe_inv};

#[derive(Debug, Clone)]
pub enum ThemeType {
    Dark,
    Light,
    System,
    Custom(String),
}

pub static SYSTEM_PREFERENCES: LazyLock<ArcSwap<Preferences>> = LazyLock::new(|| {
    use std::time::Duration;
    ArcSwap::new(Arc::new(
        Preferences::once_blocking(
            Interest::ColorScheme | Interest::Contrast | Interest::AccentColor,
            Duration::from_millis(1200),
        )
        .unwrap_or_default(),
    ))
});

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub theme_type: ThemeType,
    pub is_dark: bool,
    pub primary: Color,
    pub on_primary: Color,
    pub primary_container: Color,
    pub on_primary_container: Color,
    pub primary_fixed: Color,
    pub primary_fixed_dim: Color,
    pub on_primary_fixed: Color,
    pub on_primary_fixed_variant: Color,
    pub secondary: Color,
    pub on_secondary: Color,
    pub secondary_container: Color,
    pub on_secondary_container: Color,
    pub secondary_fixed: Color,
    pub secondary_fixed_dim: Color,
    pub on_secondary_fixed: Color,
    pub on_secondary_fixed_variant: Color,
    pub tertiary: Color,
    pub on_tertiary: Color,
    pub tertiary_container: Color,
    pub on_tertiary_container: Color,
    pub tertiary_fixed: Color,
    pub tertiary_fixed_dim: Color,
    pub on_tetriary_fixed: Color,
    pub on_tetriary_fixed_variant: Color,
    pub danger: Color,
    pub on_danger: Color,
    pub danger_container: Color,
    pub on_danger_container: Color,
    pub warning: Color,
    pub on_warning: Color,
    pub warning_container: Color,
    pub on_warning_container: Color,
    pub surface_dim: Color,
    pub surface: Color,
    pub surface_bright: Color,
    pub on_surface: Color,
    pub on_surface_variant: Color,
    pub surface_container_lowest: Color,
    pub surface_container_low: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,
    pub outline: Color,
    pub outline_variant: Color,
    pub scrim: Color,
    pub shadow: Color,
}

impl Theme {
    pub fn system() -> Self {
        let seed = SYSTEM_PREFERENCES
            .load()
            .accent_color
            .0
            .map_or(Color::from_rgb8(118, 156, 223), |c| {
                Color::from_rgba(c.red as _, c.green as _, c.blue as _, c.alpha as _)
            });

        Self::from_seed(seed, ThemeType::System)
    }

    pub fn from_seed(seed: Color, theme_type: ThemeType) -> Self {
        let primary = seed.with_chroma(0.13);
        let tertiary = {
            let mut oklch = seed.to_oklch();
            oklch[2] += 70.;
            Color::from_oklch(oklch).with_chroma(0.13)
        };
        let secondary = seed.with_chroma(0.03);
        Self::from_colors(primary, secondary, tertiary, theme_type)
    }

    pub fn from_colors(primary: Color, secondary: Color, tertiary: Color, theme_type: ThemeType) -> Self {
        let primary_tones = Tones::from_color(primary);
        let tertiary_tones = Tones::from_color(tertiary);

        let secondary_tones = Tones::from_color(secondary);
        let neutral_tones = Tones::from_color(primary.with_chroma(0.011));

        let neutral_variant_tones = Tones::from_color(primary.with_chroma(0.024));

        let danger_tones = Tones::from_color(Color::from_rgb(0.896, 0.0145, 0.007));
        let warning_tones = Tones::from_color(Color::from_rgb(0.81, 0.67, 0.));
        println!("{:?}", SYSTEM_PREFERENCES.load().color_scheme);
        match theme_type {
            ThemeType::Dark => Theme::dark_from_tones(primary_tones, secondary_tones, tertiary_tones, neutral_tones, neutral_variant_tones, warning_tones, danger_tones),
            ThemeType::Light => Theme::light_from_tones(primary_tones, secondary_tones, tertiary_tones, neutral_tones, neutral_variant_tones, warning_tones, danger_tones),
            ThemeType::System => match SYSTEM_PREFERENCES.load().color_scheme {
                mundy::ColorScheme::NoPreference => Theme::light_from_tones(primary_tones, secondary_tones, tertiary_tones, neutral_tones, neutral_variant_tones, warning_tones, danger_tones),
                mundy::ColorScheme::Light => Theme::light_from_tones(primary_tones, secondary_tones, tertiary_tones, neutral_tones, neutral_variant_tones, warning_tones, danger_tones),
                mundy::ColorScheme::Dark => Theme::dark_from_tones(primary_tones, secondary_tones, tertiary_tones, neutral_tones, neutral_variant_tones, warning_tones, danger_tones),
            }
            _ => todo!()
        }
    }

    fn dark_from_tones(primary: Tones, secondary: Tones, tertiary: Tones, neutral: Tones, neutral_variant: Tones, warning: Tones, danger: Tones) -> Self {
        Self {
            theme_type: ThemeType::Dark,
            name: String::from("Dark"),
            is_dark: true,
            primary: primary.color80,
            on_primary: primary.color20,
            primary_container: primary.color30,
            on_primary_container: primary.color90,
            primary_fixed: primary.color90,
            primary_fixed_dim: primary.color80,
            on_primary_fixed: primary.color10,
            on_primary_fixed_variant: primary.color30,
            secondary: secondary.color80,
            on_secondary: secondary.color20,
            secondary_container: secondary.color30,
            on_secondary_container: secondary.color90,
            secondary_fixed: secondary.color90,
            secondary_fixed_dim: secondary.color80,
            on_secondary_fixed: secondary.color10,
            on_secondary_fixed_variant: secondary.color30,
            tertiary: tertiary.color80,
            on_tertiary: tertiary.color20,
            tertiary_container: tertiary.color30,
            on_tertiary_container: tertiary.color90,
            tertiary_fixed: tertiary.color90,
            tertiary_fixed_dim: tertiary.color80,
            on_tetriary_fixed: tertiary.color10,
            on_tetriary_fixed_variant: tertiary.color30,
            danger: danger.color80,
            on_danger: danger.color20,
            danger_container: danger.color30,
            on_danger_container: danger.color90,
            warning: warning.color80,
            on_warning: warning.color20,
            warning_container: warning.color30,
            on_warning_container: warning.color90,
            surface_dim: neutral.color10.with_lightness(toe_inv(0.06)),
            surface: neutral.color10.with_lightness(toe_inv(0.06)),
            surface_bright: neutral.color40.with_lightness(toe_inv(0.24)),
            on_surface: neutral.color90,
            on_surface_variant: neutral_variant.color80,
            surface_container_lowest: neutral.color20.with_lightness(toe_inv(0.04)),
            surface_container_low: neutral.color30.with_lightness(toe_inv(0.10)),
            surface_container: neutral.color40.with_lightness(toe_inv(0.12)),
            surface_container_high: neutral.color45.with_lightness(toe_inv(0.17)),
            surface_container_highest: neutral.color20,
            outline: neutral_variant.color60,
            outline_variant: neutral_variant.color30,
            scrim: neutral.color0,
            shadow: neutral.color0,
        }
    }

    fn light_from_tones(primary: Tones, secondary: Tones, tertiary: Tones, neutral: Tones, neutral_variant: Tones, warning: Tones, danger: Tones) -> Self {
        Self {
            theme_type: ThemeType::Light,
            name: String::from("Light"),
            is_dark: true,
            primary: primary.color40,
            on_primary: primary.color100,
            primary_container: primary.color90,
            on_primary_container: primary.color10,
            primary_fixed: primary.color90,
            primary_fixed_dim: primary.color80,
            on_primary_fixed: primary.color10,
            on_primary_fixed_variant: primary.color30,
            secondary: secondary.color40,
            on_secondary: secondary.color100,
            secondary_container: secondary.color90,
            on_secondary_container: secondary.color10,
            secondary_fixed: secondary.color90,
            secondary_fixed_dim: secondary.color80,
            on_secondary_fixed: secondary.color10,
            on_secondary_fixed_variant: secondary.color30,
            tertiary: tertiary.color40,
            on_tertiary: tertiary.color100,
            tertiary_container: tertiary.color90,
            on_tertiary_container: tertiary.color10,
            tertiary_fixed: tertiary.color90,
            tertiary_fixed_dim: tertiary.color80,
            on_tetriary_fixed: tertiary.color10,
            on_tetriary_fixed_variant: tertiary.color30,
            danger: danger.color40,
            on_danger: danger.color100,
            danger_container: danger.color90,
            on_danger_container: danger.color10,
            warning: warning.color40,
            on_warning: warning.color100,
            warning_container: warning.color90,
            on_warning_container: warning.color10,
            surface_dim: neutral.color10.with_lightness(toe_inv(0.87)),
            surface: neutral.color10.with_lightness(toe_inv(0.98)),
            surface_bright: neutral.color40.with_lightness(toe_inv(0.98)),
            on_surface: neutral.color10,
            on_surface_variant: neutral_variant.color30,
            surface_container_lowest: neutral.color100,
            surface_container_low: neutral.color30.with_lightness(toe_inv(0.96)),
            surface_container: neutral.color40.with_lightness(toe_inv(0.94)),
            surface_container_high: neutral.color45.with_lightness(toe_inv(0.92)),
            surface_container_highest: neutral.color90,
            outline: neutral_variant.color50,
            outline_variant: neutral_variant.color80,
            scrim: neutral.color0,
            shadow: neutral.color0,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::system()
    }
}

impl iced::application::DefaultStyle for Theme {
    fn default_style(&self) -> iced::daemon::Appearance {
        iced::daemon::Appearance {
            background_color: self.surface,
            text_color: self.on_surface,
        }
    }
}

pub trait Oklch {
    fn from_oklch(oklch: [f32; 3]) -> Self;

    fn to_oklch(self) -> [f32; 3];

    fn with_lightness(self, value: f32) -> Self;

    fn with_chroma(self, value: f32) -> Self;

    fn with_hue(self, value: f32) -> Self;

    fn lighter(self) -> Self;

    fn lighter_by(self, value: f32) -> Self;

    fn lighten(&mut self);

    fn lighten_by(&mut self, value: f32);

    fn darker(self) -> Self;

    fn darker_by(self, value: f32) -> Self;

    fn darken(&mut self);

    fn darken_by(&mut self, value: f32);
}

impl Oklch for Color {
    fn from_oklch(oklch: [f32; 3]) -> Self {
        let color: palette::rgb::LinSrgb =
            palette_gamut_mapping::gamut_map(palette::Oklch::new(oklch[0], oklch[1], oklch[2]));
        Self::from_linear_rgba(color.red, color.green, color.blue, 1.)
    }

    fn to_oklch(self) -> [f32; 3] {
        let [r, g, b, _] = self.into_linear();
        let oklch = palette::Oklch::from_color_unclamped(palette::LinSrgb::new(r, g, b));
        let (l, c, h) = oklch.into_components();
        [l, c, h.into_inner()]
    }

    fn lighter(self) -> Self {
        let mut oklch = self.to_oklch();
        oklch[0] += 0.03f32.min(1.);
        Self::from_oklch(oklch)
    }

    fn lighter_by(self, value: f32) -> Self {
        let mut oklch = self.to_oklch();
        oklch[0] += value.min(1.);
        Self::from_oklch(oklch)
    }

    fn lighten(&mut self) {
        let mut oklch = self.to_oklch();
        oklch[0] += 0.03f32.min(1.);
        *self = Self::from_oklch(oklch);
    }

    fn lighten_by(&mut self, value: f32) {
        let mut oklch = self.to_oklch();
        oklch[0] += value.min(1.);
        *self = Self::from_oklch(oklch);
    }

    fn darker(self) -> Self {
        let mut oklch = self.to_oklch();
        oklch[0] -= 0.03f32.max(0.);
        Self::from_oklch(oklch)
    }

    fn darker_by(self, value: f32) -> Self {
        let mut oklch = self.to_oklch();
        oklch[0] -= value.max(0.);
        Self::from_oklch(oklch)
    }

    fn darken(&mut self) {
        let mut oklch = self.to_oklch();
        oklch[0] -= 0.03f32.max(0.);
        *self = Self::from_oklch(oklch);
    }

    fn darken_by(&mut self, value: f32) {
        let mut oklch = self.to_oklch();
        oklch[0] -= value.max(0.);
        *self = Self::from_oklch(oklch);
    }

    fn with_lightness(self, value: f32) -> Self {
        let mut oklch = self.to_oklch();
        oklch[0] = value;
        Self::from_oklch(oklch)
    }

    fn with_chroma(self, value: f32) -> Self {
        let mut oklch = self.to_oklch();
        oklch[1] = value;
        Self::from_oklch(oklch)
    }

    fn with_hue(self, value: f32) -> Self {
        let mut oklch = self.to_oklch();
        oklch[2] = value;
        Self::from_oklch(oklch)
    }
}
