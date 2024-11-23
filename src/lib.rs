pub mod bmp;
pub mod camera;
pub mod color;
pub mod fragment;
pub mod framebuffer;
pub mod light;
pub mod obj;
pub mod render;
pub mod vertex;

use camera::Camera;
use color::{blenders::BlendMode, Color};
use nalgebra_glm::{Mat4, Vec2, Vec3};
use obj::Obj;
use vertex::shader::{create_model_matrix, ShaderType, Uniforms};

pub fn equal(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() < eps
}

pub fn clamp_with_universe(original: Vec2, mapped: Vec2, value: f32) -> f32 {
    let mapped_range = mapped.max() - mapped.min();

    (value - original.min()) / original.max() * mapped_range + mapped.min()
}

pub enum Message {
    RotateCamera(f32, f32),
    ZoomCamera(f32),
    UpdateTime(f32),
    ChangePlanet(Entity),
    Advance(f32),
}

pub type EntityShader = (ShaderType, Vec<Color>, BlendMode);

pub struct EntityModel {
    pub rotation: Vec3,
    pub scale: f32,
    pub translation: Vec3,
}

pub struct Entity {
    pub objs: Vec<Obj>,
    pub shaders: Vec<EntityShader>,
    pub model_matrix: Mat4,
    /// Optimizes the rendering of triangles,
    /// may cause some triangles to not render correctly.
    pub optimize: bool,
    /// Whether or not to use vertex_normal instead of vertex_position
    pub use_normal: bool,
    pub model: EntityModel,
}

impl Entity {
    pub fn modify_model(&mut self, new_model: EntityModel) {
        let model_matrix =
            create_model_matrix(new_model.translation, new_model.scale, new_model.rotation);
        self.model = new_model;
        self.model_matrix = model_matrix;
    }
}

pub struct Model {
    pub entities: Vec<Entity>,
    pub ship: Entity,
    pub uniforms: Uniforms,
    pub camera: Camera,
}
