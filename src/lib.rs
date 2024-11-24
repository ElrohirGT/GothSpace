pub mod bmp;
pub mod camera;
pub mod color;
pub mod fragment;
pub mod framebuffer;
pub mod light;
pub mod obj;
pub mod render;
pub mod skybox;
pub mod texture;
pub mod vertex;

use camera::Camera;
use color::{blenders::BlendMode, Color};
use light::Light;
use nalgebra_glm::{Mat4, Vec2, Vec3};
use obj::Obj;
use skybox::Skybox;
use texture::GameTextures;
use vertex::shader::{create_model_matrix, ShaderType, Uniforms};

pub fn equal(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() < eps
}

pub fn clamp_with_universe(original: Vec2, mapped: Vec2, value: f32) -> f32 {
    let mapped_range = mapped.max() - mapped.min();

    (value - original.min()) / original.max() * mapped_range + mapped.min()
}

pub enum Message {
    ZoomCamera(f32),
    RotateCamera(f32, f32),
    UpdateTime(f32),
    ChangePlanet(Entity),
    Advance(f32),
    ResizeWindow((usize, usize)),
}

pub type EntityShader = (ShaderType, Vec<Color>, BlendMode);

pub struct EntityModel {
    pub rotation: Vec3,
    pub scale: f32,
    pub translation: Vec3,
}

pub struct EntityOptimizations {
    /// Don't render triangles whose normal is in the same direction as the camera is looking
    /// May cause some X-ray vision bugs...
    pub camera_direction: bool,
    /// Don't render the triangle if the camera isn't looking in that direction
    pub frustum_cutting: bool,
}

pub struct Entity {
    pub objs: Vec<Obj>,
    pub shaders: Vec<EntityShader>,
    pub model_matrix: Mat4,
    pub optimizations: EntityOptimizations,
    /// If true, will use the screen position instead of the model position of each vertex for its
    /// shaders.
    pub use_screen_position: bool,
    pub model: EntityModel,
    /// Lower depth means it will be rendered on top other stuff
    pub custom_depth: Option<f32>,
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
    pub textures: GameTextures,
    pub lights: Vec<Light>,
    pub skybox: Skybox,
}
