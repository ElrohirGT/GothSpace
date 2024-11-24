use std::f32::consts::PI;

use fastnoise_lite::{CellularDistanceFunction, CellularReturnType, FastNoiseLite, FractalType};
use nalgebra_glm::{vec3, vec4, Mat4, Vec3};

use crate::{texture::Textures, vertex::Vertex};

pub enum ShaderType {
    Stripe {
        stripe_width: f32,
    },
    MovingStripes {
        speed: f32,
        stripe_width: f32,
    },
    GlowShader {
        stripe_width: f32,
        glow_size: f32,
        red: f32,
        blue: f32,
    },
    AliveCheckerboard,
    Intensity,
    BaseColor,
    FBmShader {
        zoom: f32,
        speed: f32,
        fractal: FractalConfig,
    },
    CellularShader {
        zoom: f32,
        speed: f32,
        fractal: FractalConfig,
        cellular: CellularConfig,
    },
    Texture {
        texture: Textures,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct FractalConfig {
    pub octaves: i32,
    pub lacunarity: f32,
    pub gain: f32,
    pub weighted_strength: f32,
    pub f_type: FractalType,
}

#[derive(Debug, Clone, Copy)]
pub struct CellularConfig {
    pub distance_func: CellularDistanceFunction,
    pub return_type: CellularReturnType,
    pub jitter: f32,
}

pub struct Uniforms {
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: f32,
}

pub fn create_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(7082003);

    noise.set_fractal_lacunarity(Some(0.530));

    noise
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms, model_matrix: &Mat4) -> Vertex {
    let Uniforms {
        view_matrix,
        projection_matrix,
        viewport_matrix,
        ..
    } = uniforms;

    let position = vec4(
        vertex.model_position.x,
        vertex.model_position.y,
        vertex.model_position.z,
        1.0,
    );
    let transformed = projection_matrix * view_matrix * model_matrix * position;

    let w = transformed.w;
    let ndc_position = vec4(transformed.x / w, transformed.y / w, transformed.z / w, 1.0);

    let screen_position = viewport_matrix * ndc_position;
    let transformed_position = vec3(screen_position.x, screen_position.y, screen_position.z);

    // Transform normal
    let vertex_normal = vec4(vertex.normal.x, vertex.normal.y, vertex.normal.z, 1.0);
    let normal_matrix = model_matrix
        .try_inverse()
        .unwrap_or(Mat4::identity())
        .transpose();
    let transformed_normal = normal_matrix * vertex_normal;
    let w = transformed_normal.w;
    let transformed_normal = vec3(
        transformed_normal.x / w,
        transformed_normal.y / w,
        transformed_normal.z / w,
    );

    Vertex {
        screen_position: transformed_position,
        normal: transformed_normal,
        frustum_position: ndc_position,
        ..*vertex
    }
}

pub fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sinx, cosx) = rotation.x.sin_cos();
    let (siny, cosy) = rotation.y.sin_cos();
    let (sinz, cosz) = rotation.z.sin_cos();

    #[rustfmt::skip]
    let rotation_x = Mat4::new(
        1.0,    0.0,    0.0,    0.0,
        0.0,    cosx,   -sinx,  0.0,
        0.0,    sinx,   cosx,   0.0,
        0.0,    0.0,    0.0,    1.0,
    );

    #[rustfmt::skip]
    let rotation_y = Mat4::new(
        cosy,   0.0,    siny,   0.0,
        0.0,    1.0,    0.0,    0.0,
        -siny,  0.0,    cosy,   0.0,
        0.0,    0.0,    0.0,    1.0,
    );

    #[rustfmt::skip]
    let rotation_z = Mat4::new(
        cosz,   -sinz,  0.0,    0.0,
        sinz,   cosz,   0.0,    0.0,
        0.0,    0.0,    1.0,    0.0,
        0.0,    0.0,    0.0,    1.0,
    );

    let rotation_matrix = rotation_z * rotation_y * rotation_x;

    #[rustfmt::skip]
    let matrix = Mat4::new(
        scale,  0.0,    0.0,    translation.x,
        0.0,    scale,  0.0,    translation.y,
        0.0,    0.0,    scale,  translation.z,
        0.0,    0.0,    0.0,    1.0,
    ) * rotation_matrix;

    matrix
}

pub fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    nalgebra_glm::look_at(&eye, &center, &up)
}

pub fn create_projection_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 500.0;

    nalgebra_glm::perspective(fov, aspect_ratio, near, far)
}

pub fn create_viewport_matrix(framebuffer_width: f32, framebuffer_height: f32) -> Mat4 {
    #[rustfmt::skip]
    let matrix = Mat4::new(
        framebuffer_width / 2.0,    0.0,                        0.0,    framebuffer_width / 2.0,
        0.0,                        -framebuffer_height / 2.0,  0.0,    framebuffer_height / 2.0,
        0.0,                        0.0,                        1.0,    0.0,
        0.0,                        0.0,                        0.0,    1.0);

    matrix
}
