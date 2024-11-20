use crate::color::Color;

pub enum BlendMode {
    Normal,
    Add,
    Subtract,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    Dodge,
    Burn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Replace,
}

impl Color {
    pub fn blend(&self, blend: &Color, strategy: &BlendMode) -> Self {
        match strategy {
            BlendMode::Normal => self.blend_normal(blend),
            BlendMode::Add => self.blend_add(blend),
            BlendMode::Subtract => self.blend_subtract(blend),
            BlendMode::Multiply => self.blend_multiply(blend),
            BlendMode::Screen => self.blend_screen(blend),
            BlendMode::Overlay => self.blend_overlay(blend),
            BlendMode::Darken => self.blend_darken(blend),
            BlendMode::Lighten => self.blend_lighten(blend),
            BlendMode::Dodge => self.blend_color_dodge(blend),
            BlendMode::Burn => self.blend_color_burn(blend),
            BlendMode::HardLight => self.blend_hard_light(blend),
            BlendMode::SoftLight => self.blend_soft_light(blend),
            BlendMode::Difference => self.blend_difference(blend),
            BlendMode::Exclusion => self.blend_exclusion(blend),
            BlendMode::Replace => *blend,
        }
    }

    pub fn blend_normal(&self, blend: &Color) -> Self {
        if blend.is_black() {
            *self
        } else {
            *blend
        }
    }

    pub fn blend_add(&self, blend: &Color) -> Self {
        *self + *blend
    }

    pub fn blend_subtract(&self, blend: &Color) -> Self {
        *self - *blend
    }

    pub fn blend_multiply(&self, blend: &Color) -> Self {
        Color::new(
            ((self.r as f32 * blend.r as f32) / 255.0) as u8,
            ((self.g as f32 * blend.g as f32) / 255.0) as u8,
            ((self.b as f32 * blend.b as f32) / 255.0) as u8,
        )
    }

    pub fn blend_screen(&self, blend: &Color) -> Color {
        Color::new(
            255 - ((255 - self.r as u16) * (255 - blend.r as u16) / 255) as u8,
            255 - ((255 - self.g as u16) * (255 - blend.g as u16) / 255) as u8,
            255 - ((255 - self.b as u16) * (255 - blend.b as u16) / 255) as u8,
        )
    }

    pub fn blend_overlay(&self, blend: &Color) -> Color {
        // Overlay: Combines Multiply and Screen blend modes
        fn overlay_channel(base: u8, blend: u8) -> u8 {
            if base < 128 {
                ((2 * base as u16 * blend as u16) / 255) as u8
            } else {
                (255 - 2 * (255 - base as u16) * (255 - blend as u16) / 255) as u8
            }
        }
        Color::new(
            overlay_channel(self.r, blend.r),
            overlay_channel(self.g, blend.g),
            overlay_channel(self.b, blend.b),
        )
    }

    pub fn blend_darken(&self, blend: &Color) -> Color {
        Color::new(
            self.r.min(blend.r),
            self.g.min(blend.g),
            self.b.min(blend.b),
        )
    }

    pub fn blend_lighten(&self, blend: &Color) -> Color {
        Color::new(
            self.r.max(blend.r),
            self.g.max(blend.g),
            self.b.max(blend.b),
        )
    }

    pub fn blend_color_dodge(&self, blend: &Color) -> Color {
        fn dodge_channel(base: u8, blend: u8) -> u8 {
            if blend == 255 {
                255
            } else {
                ((base as u16 * 255) / (255 - blend as u16)).min(255) as u8
            }
        }
        Color::new(
            dodge_channel(self.r, blend.r),
            dodge_channel(self.g, blend.g),
            dodge_channel(self.b, blend.b),
        )
    }

    pub fn blend_color_burn(&self, blend: &Color) -> Color {
        fn burn_channel(base: u8, blend: u8) -> u8 {
            if blend == 0 {
                0
            } else {
                255 - ((255 - base as u16) * 255 / blend as u16).min(255) as u8
            }
        }
        Color::new(
            burn_channel(self.r, blend.r),
            burn_channel(self.g, blend.g),
            burn_channel(self.b, blend.b),
        )
    }

    pub fn blend_hard_light(&self, blend: &Color) -> Color {
        // Hard Light: Similar to Overlay, but with blend and base colors swapped
        self.blend_overlay(blend)
    }

    pub fn blend_soft_light(&self, blend: &Color) -> Color {
        fn soft_light_channel(base: u8, blend: u8) -> u8 {
            let b = base as f32 / 255.0;
            let s = blend as f32 / 255.0;
            if s < 0.5 {
                (b - (1.0 - 2.0 * s) * b * (1.0 - b)) * 255.0
            } else {
                (b + (2.0 * s - 1.0) * (((b - 0.5).abs() * 16.0 + 12.0) * b - 3.0)) * 255.0
            }
            .round() as u8
        }
        Color::new(
            soft_light_channel(self.r, blend.r),
            soft_light_channel(self.g, blend.g),
            soft_light_channel(self.b, blend.b),
        )
    }

    pub fn blend_difference(&self, blend: &Color) -> Color {
        Color::new(
            (self.r as i16 - blend.r as i16).unsigned_abs() as u8,
            (self.g as i16 - blend.g as i16).unsigned_abs() as u8,
            (self.b as i16 - blend.b as i16).unsigned_abs() as u8,
        )
    }

    pub fn blend_exclusion(&self, blend: &Color) -> Color {
        Color::new(
            (self.r as u16 + blend.r as u16 - 2 * self.r as u16 * blend.r as u16 / 255) as u8,
            (self.g as u16 + blend.g as u16 - 2 * self.g as u16 * blend.g as u16 / 255) as u8,
            (self.b as u16 + blend.b as u16 - 2 * self.b as u16 * blend.b as u16 / 255) as u8,
        )
    }
}
