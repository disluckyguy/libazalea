pub mod button;
pub mod container;
pub mod text;

use color::ColorSpace;
use iced::Color;

#[derive(Debug, Clone)]
#[expect(dead_code)]
pub struct Theme {
    primary: Color,
    on_primary: Color,
    primary_container: Color,
    on_primary_container: Color,
    secondary: Color,
    on_secondary: Color,
    secondary_container: Color,
    on_secondary_container: Color,
    tertiary: Color,
    on_tertiary: Color,
    tertiary_container: Color,
    on_tertiary_container: Color,
    danger: Color,
    on_danger: Color,
    danger_container: Color,
    on_danger_container: Color,
    warning: Color,
    on_warning: Color,
    warning_container: Color,
    on_warning_container: Color,
    surface_dim: Color,
    surface: Color,
    surface_bright: Color,
    on_surface: Color,
    surface_container_lowest: Color,
    surface_container_low: Color,
    surface_container: Color,
    surface_container_high: Color,
    surface_container_highest: Color,
    on_surface_container: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::from_rgb8(170, 199, 255),
            on_primary: Color::BLACK,
            primary_container: Color::from_rgb8(40, 71, 119),
            on_primary_container: Color::WHITE,
            secondary: Color::from_rgb8(190, 198, 220),
            on_secondary: Color::BLACK,
            secondary_container: Color::from_rgb8(62, 71, 89),
            on_secondary_container: Color::WHITE,
            tertiary: Color::from_rgb8(221, 188, 224),
            on_tertiary: Color::BLACK,
            tertiary_container: Color::from_rgb8(87, 62, 92),
            on_tertiary_container: Color::WHITE,
            danger: Color::from_rgb8(255, 180, 171),
            on_danger: Color::from_rgb8(105, 0, 5),
            danger_container: Color::from_rgb8(147, 0, 10),
            on_danger_container: Color::from_rgb8(255, 218, 214),
            warning: Color::from_rgb8(217, 179, 140),
            on_warning: Color::from_rgb8(130, 77, 23),
            warning_container: Color::from_rgb8(204, 102, 0),
            on_warning_container: Color::from_rgb8(255, 230, 204),
            surface_dim: Color::from_rgb8(25, 28, 32),
            surface: Color::from_rgb8(17, 19, 24),
            surface_bright: Color::from_rgb8(55, 57, 62),
            on_surface: Color::WHITE,
            surface_container_lowest: Color::from_rgb8(12, 14, 19),
            surface_container_low: Color::from_rgb8(25, 28, 32),
            surface_container: Color::from_rgb8(29, 32, 36),
            surface_container_high: Color::from_rgb8(40, 42, 47),
            surface_container_highest: Color::from_rgb8(51, 53, 58),
            on_surface_container: Color::WHITE,

        }
    }
}

impl iced::application::DefaultStyle for Theme {
    fn default_style(&self) -> iced::daemon::Appearance {
        iced::daemon::Appearance {
            background_color: self.surface,
            text_color: self.on_surface
        }
    }
}

pub struct Hsl {
    h: f32,
    s: f32,
    l: f32,
}

impl Hsl {
    pub fn lighter(self) -> Self {
        Self {
            l: (self.l + 10.).min(100.),
            ..self
        }
    }

    pub fn lighter_by(self, value: f32) -> Self {
        Self {
            l: (self.l + value).min(100.),
            ..self
        }
    }

    pub fn lighten(&mut self) {
        self.l = (self.l + 10.).min(100.);
    }

    pub fn lighten_by(&mut self, value: f32) {
        self.l = (self.l + value).min(100.);
    }

    pub fn darker(self) -> Self {
        Self {
            l: (self.l - 10.).max(0.),
            ..self
        }
    }

    pub fn darker_by(self, value: f32) -> Self {
        Self {
            l: (self.l - value).max(0.),
            ..self
        }
    }

    pub fn darken(&mut self) {
        self.l = (self.l - 10.).max(0.);
    }

    pub fn darken_by(&mut self, value: f32) {
        self.l = (self.l - value).max(0.);
    }
}

impl From<Color> for Hsl {
    fn from(value: Color) -> Self {
        let linear = value.into_linear();
        let rgb = [linear[0], linear[1], linear[2]];
        let [h, s, l] = color::Hsl::from_linear_srgb(rgb);
        Hsl { h, s, l }
    }
}

impl From<Hsl> for Color {
    fn from(value: Hsl) -> Self {
        let [r, g, b] = color::Hsl::to_linear_srgb([value.h, value.s, value.l]);
        Color::from_linear_rgba(r, g, b, 1.0)
    }
}
