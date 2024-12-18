use std::{fs::File, io::BufReader};

use image::{
    codecs::gif::GifDecoder, AnimationDecoder, Frame, GenericImageView, ImageDecoder, ImageReader,
    Pixel,
};

use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub enum Textures {
    Space,
    Instructions,
}

pub struct GameTextures {
    pub space: Texture,
    pub instructions: Texture,
}

impl GameTextures {
    pub fn new(asset_dir: &str) -> Self {
        let space = format!("{}{}", asset_dir, "space.png");
        let instructions = format!("{}{}", asset_dir, "instructions.jpg");

        let space = Texture::new(&space);
        let instructions = Texture::new(&instructions);

        GameTextures {
            space,
            instructions,
        }
    }

    pub fn get_texture(&self, id: Textures) -> &Texture {
        match id {
            Textures::Space => &self.space,
            Textures::Instructions => &self.instructions,
        }
    }
}

pub struct Texture {
    pub width: u32,
    pub height: u32,
    colors: Vec<Color>,
}

pub struct AnimatedTexture {
    pub width: u32,
    pub height: u32,
    frames: Vec<Frame>,
    pub frame_count: usize,
}

impl AnimatedTexture {
    pub fn new(file_path: &str) -> Self {
        let file_in = BufReader::new(File::open(file_path).unwrap());
        let decoder = GifDecoder::new(file_in).unwrap();
        let (width, height) = decoder.dimensions();
        let frames = decoder.into_frames();
        let frames = frames.collect_frames().expect("error decoding gif");
        let frame_count = frames.len();

        Self {
            width,
            height,
            frames,
            frame_count,
        }
    }

    /// Get's the color of the pixel positioned on the frame `t`.
    pub fn get_pixel_color(&self, t: usize, x: u32, y: u32) -> Color {
        let pixel = self.frames[t].buffer().get_pixel(x, y).to_rgb();
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        Color { r, g, b }
    }
}

impl Texture {
    pub fn new(file_path: &str) -> Self {
        let image = ImageReader::open(file_path).unwrap().decode().unwrap();
        let width = image.width();
        let height = image.height();

        let size = width * height;
        let mut colors = vec![0xffffff.into(); size as usize];

        // If I use flatmap and all that this get's reordered...
        // I don't know why
        for x in 0..width {
            for y in 0..height {
                let pixel = image.get_pixel(x, y).to_rgb();
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];

                let idx = y * width + x;
                colors[idx as usize] = Color { r, g, b };
            }
        }

        Texture {
            width,
            height,
            colors,
        }
    }

    pub fn get_pixel_color(&self, u: f32, v: f32) -> Color {
        let x = (u * self.width as f32) as u32;
        let y = (v * self.height as f32) as u32;

        // println!("({}, {}) -> ({}, {})", u, v, x, y);

        let idx = y * self.width + x;
        self.colors[idx as usize]
    }
}
