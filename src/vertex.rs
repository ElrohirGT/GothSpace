pub mod shader;

use crate::color::Color;
use nalgebra_glm::{Vec2, Vec3, Vec4};

#[derive(Clone, Debug)]
pub struct Vertex {
    /// The position of the vertex inside the 3D model.
    pub model_position: Vec3,
    /// The position of the vertex inside the screen.
    pub screen_position: Vec3,
    /// The position of the vertex inside the camera frustum.
    /// Useful for checking if is inside the camera FOV.
    pub frustum_position: Vec4,
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub color: Color,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Vertex {
            screen_position: position,
            model_position: position,
            normal,
            tex_coords,
            color: Color::black(),
            frustum_position: Vec4::zeros(),
        }
    }

    pub fn new_with_color(position: Vec3, color: Color) -> Self {
        Vertex {
            screen_position: position,
            model_position: position,
            normal: Vec3::new(0.0, 0.0, 0.0),
            tex_coords: Vec2::new(0.0, 0.0),
            color,
            frustum_position: Vec4::zeros(),
        }
    }

    pub fn set_screen_position(&mut self, position: Vec3, normal: Vec3) {
        self.screen_position = position;
        self.normal = normal;
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            screen_position: Vec3::zeros(),
            normal: Vec3::zeros(),
            tex_coords: Vec2::zeros(),
            color: Color::black(),
            frustum_position: Vec4::zeros(),
            model_position: Vec3::zeros(),
        }
    }
}
