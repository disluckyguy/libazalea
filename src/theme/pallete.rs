use crate::theme::Oklch;
use iced::Color;

#[derive(Debug, Clone)]
pub struct Tones {
    pub color0: Color,
    pub color5: Color,
    pub color10: Color,
    pub color15: Color,
    pub color20: Color,
    pub color25: Color,
    pub color30: Color,
    pub color35: Color,
    pub color40: Color,
    pub color45: Color,
    pub color50: Color,
    pub color55: Color,
    pub color60: Color,
    pub color65: Color,
    pub color70: Color,
    pub color75: Color,
    pub color80: Color,
    pub color85: Color,
    pub color90: Color,
    pub color95: Color,
    pub color98: Color,
    pub color99: Color,
    pub color100: Color,
}

impl Tones {
    pub fn from_color(color: Color) -> Self {
        Tones {
            color0: color.with_lightness(toe_inv(0.0)),
            color5: color.with_lightness(toe_inv(0.5)),
            color10: color.with_lightness(toe_inv(0.10)),
            color15: color.with_lightness(toe_inv(0.15)),
            color20: color.with_lightness(toe_inv(0.20)),
            color25: color.with_lightness(toe_inv(0.25)),
            color30: color.with_lightness(toe_inv(0.30)),
            color35: color.with_lightness(toe_inv(0.35)),
            color40: color.with_lightness(toe_inv(0.40)),
            color45: color.with_lightness(toe_inv(0.45)),
            color50: color.with_lightness(toe_inv(0.50)),
            color55: color.with_lightness(toe_inv(0.5)),
            color60: color.with_lightness(toe_inv(0.60)),
            color65: color.with_lightness(toe_inv(0.65)),
            color70: color.with_lightness(toe_inv(0.70)),
            color75: color.with_lightness(toe_inv(0.75)),
            color80: color.with_lightness(toe_inv(0.80)),
            color85: color.with_lightness(toe_inv(0.85)),
            color90: color.with_lightness(toe_inv(0.90)),
            color95: color.with_lightness(toe_inv(0.95)),
            color98: color.with_lightness(toe_inv(0.98)),
            color99: color.with_lightness(toe_inv(0.99)),
            color100: color.with_lightness(toe_inv(1.)),
        }
    }
}

pub fn toe_inv(x: f32) -> f32 {
    let k_1 = 0.206;
    let k_2 = 0.03;
    let k_3 = (1. + k_1) / (1. + k_2);
    return (x * x + k_1 * x) / (k_3 * (x + k_2));
}
