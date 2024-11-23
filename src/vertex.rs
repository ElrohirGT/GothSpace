pub mod shader;

use crate::{
    color::Color,
    texture::{self, Textures},
};
use nalgebra_glm::{Vec2, Vec3};

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub color: Color,
    pub texture: Option<Textures>,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
            color: Color::black(),
            texture: None,
        }
    }

    pub fn new_with_color(position: Vec3, color: Color) -> Self {
        Vertex {
            position,
            normal: Vec3::new(0.0, 0.0, 0.0),
            tex_coords: Vec2::new(0.0, 0.0),
            color,
            texture: None,
        }
    }

    pub fn new_with_texture(position: Vec3, texture: Textures) -> Self {
        Vertex {
            position,
            normal: Vec3::new(0.0, 0.0, 0.0),
            tex_coords: Vec2::new(0.0, 0.0),
            color: Color::black(),
            texture: Some(texture),
        }
    }

    pub fn set_transformed(&mut self, position: Vec3, normal: Vec3) {
        self.position = position;
        self.normal = normal;
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            tex_coords: Vec2::new(0.0, 0.0),
            color: Color::black(),
            texture: None,
        }
    }
}
