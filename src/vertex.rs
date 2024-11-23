pub mod shader;

use crate::{color::Color, texture::Textures};
use nalgebra_glm::{Vec2, Vec3, Vec4};

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub frustum_position: Vec4,
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub color: Color,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
            color: Color::black(),
            frustum_position: Vec4::zeros(),
        }
    }

    pub fn new_with_color(position: Vec3, color: Color) -> Self {
        Vertex {
            position,
            normal: Vec3::new(0.0, 0.0, 0.0),
            tex_coords: Vec2::new(0.0, 0.0),
            color,
            frustum_position: Vec4::zeros(),
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
            frustum_position: Vec4::zeros(),
        }
    }
}
