use core::f32;

use crate::{bmp::write_bmp_file, color::Color};

type Buffer = Vec<u32>;

#[derive(Debug)]
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Buffer,
    background_color: Color,
    current_color: Color,
    empty_buffer: Buffer,
    z_buffer: Vec<f32>,
    empty_z_buffer: Vec<f32>,
}

fn create_filled_buffer(width: &usize, height: &usize, color: &Color) -> Buffer {
    let color_hex: u32 = color.into();

    (0..(width * height)).map(|_| color_hex).collect()
}

fn create_filled_z_buffer(width: &usize, height: &usize, default: f32) -> Vec<f32> {
    (0..(width * height)).map(|_| default).collect()
}

#[derive(Debug)]
pub enum PaintPointErrors {
    XTooLarge,
    XTooSmall,
    YTooLarge,
    YTooSmall,
}
impl std::fmt::Display for PaintPointErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
impl std::error::Error for PaintPointErrors {}

#[derive(Debug)]
pub enum GetColorErrors {
    XTooLarge,
    YTooLarge,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let background_color = Color::default();
        let current_color = Color::white();
        let empty_buffer = create_filled_buffer(&width, &height, &Color::default());
        let buffer = empty_buffer.clone();
        let z_buffer = create_filled_z_buffer(&width, &height, f32::NEG_INFINITY);
        let empty_z_buffer = z_buffer.clone();

        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color,
            empty_buffer,
            z_buffer,
            empty_z_buffer,
        }
    }

    /// Creates an empty buffer according to the corresponding `background_color`.
    ///
    /// The implementation of this method assumes the background color will not change that much.
    pub fn clear(&mut self) {
        self.buffer.clone_from(&self.empty_buffer);
        self.z_buffer.clone_from(&self.empty_z_buffer);
    }

    /// Saves the current framebuffer as a background.
    /// This makes it so every time we clear it get's cleared with this instead.
    pub fn save_as_background(&mut self) {
        self.empty_buffer.clone_from(&self.buffer)
    }

    /// Colors a point in the given location. Rounds x and y.
    /// If either x or y are exactly half between integers then the value is rounded up.
    ///
    /// The paint origin is located on the top left corner of the window.
    ///
    /// The color used is the one provided by `current_color`.
    pub fn paint_point(
        &mut self,
        point: nalgebra_glm::Vec2,
        depth: f32,
    ) -> Result<(), PaintPointErrors> {
        let Framebuffer {
            width,
            height,
            buffer,
            current_color,
            z_buffer,
            ..
        } = self;
        let x = point.x;
        let y = point.y;

        if x < 0.0 {
            Err(PaintPointErrors::XTooSmall)?
        }

        if y < 0.0 {
            Err(PaintPointErrors::YTooSmall)?
        }

        let x = x.round() as usize;
        let y = y.round() as usize;

        match (x < *width, y < *height) {
            (false, _) => Err(PaintPointErrors::XTooLarge),
            (_, false) => Err(PaintPointErrors::YTooLarge),
            _ => {
                let idx = y * *width + x;
                if z_buffer[idx] < depth {
                    z_buffer[idx] = depth;
                    buffer[idx] = current_color.into();
                }
                Ok(())
            }
        }
    }

    /// Gets the color of a point in the buffer.
    pub fn get_color(&self, x: usize, y: usize) -> Result<Color, GetColorErrors> {
        let Framebuffer {
            width,
            height,
            buffer,
            ..
        } = self;

        match (x <= *width, y <= *height) {
            (_, false) => Err(GetColorErrors::YTooLarge),
            (false, _) => Err(GetColorErrors::XTooLarge),
            _ => Ok(buffer[y * *width + x].into()),
        }
    }

    /// Sets the `background_color` property.
    /// This method regenerates the framebuffer used as background.
    ///
    /// * `new_color`: The color to apply.
    pub fn set_background_color(&mut self, new_color: impl Into<Color>) {
        let Framebuffer {
            width,
            height,
            background_color,
            empty_buffer,
            ..
        } = self;

        *background_color = new_color.into();
        *empty_buffer = create_filled_buffer(width, height, background_color);
    }

    /// Sets the `current_color` property.
    ///
    /// * `new_color`: The color to apply.
    pub fn set_current_color(&mut self, new_color: impl Into<Color>) {
        self.current_color = new_color.into();
    }

    /// Saves the pixel data into a .bmp located in the given `file_path`.
    pub fn save(&self, file_path: &str) -> std::io::Result<()> {
        let Framebuffer {
            width,
            height,
            buffer,
            ..
        } = self;

        write_bmp_file(file_path, buffer, *width, *height)
    }
}
