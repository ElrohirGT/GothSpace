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
use fastnoise_lite::FastNoiseLite;
use nalgebra_glm::{Mat4, Vec2, Vec3};
use obj::Obj;
use vertex::shader::{ShaderType, Uniforms};

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
}

pub type EntityShader = (ShaderType, Vec<Color>, BlendMode);

pub struct Entity {
    pub objs: Vec<Obj>,
    pub shaders: Vec<EntityShader>,
    pub model_matrix: Mat4,
    /// Optimizes the rendering of triangles,
    /// may cause some triangles to not render correctly.
    pub optimize: bool,
    /// Whether or not to use vertex_normal instead of vertex_position
    pub use_normal: bool,
}

pub struct Model {
    pub entities: Vec<Entity>,
    pub render_entities: Vec<Entity>,
    pub uniforms: Uniforms,
    pub camera: Camera,
}
