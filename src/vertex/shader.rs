use std::f32::consts::PI;

use fastnoise_lite::{CellularDistanceFunction, CellularReturnType, FastNoiseLite, FractalType};
use nalgebra_glm::{vec2, vec3, vec4, Mat4, Vec3};

use crate::{clamp_with_universe, color::Color, fragment::Fragment, vertex::Vertex, EntityShader};

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

    let position = vec4(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed = viewport_matrix * projection_matrix * view_matrix * model_matrix * position;
    // println!("{position:?} TURNED INTO {transformed:?}");

    let w = transformed.w;
    let transformed_position = vec3(transformed.x / w, transformed.y / w, transformed.z / w);

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
    // let transformed_normal = vertex.normal;
    // println!("{normal_matrix:?} -> {transformed_normal:?}");

    Vertex {
        position: transformed_position,
        normal: transformed_normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
    }
}

pub fn fragment_shader(
    fragment: Fragment,
    inputs: &[EntityShader],
    uniforms: &Uniforms,
    noise: &mut FastNoiseLite,
) -> Fragment {
    let color = inputs.iter().fold(fragment.color, |acc, current| {
        let color = match current.0 {
            ShaderType::Stripe { stripe_width } => {
                stripes_shader(&fragment, stripe_width, &current.1)
            }
            ShaderType::MovingStripes {
                stripe_width,
                speed,
            } => moving_stripes(&fragment, stripe_width, speed, &current.1, uniforms),
            ShaderType::AliveCheckerboard => todo!(),
            ShaderType::Intensity => intensity_shader(&fragment, &acc),
            ShaderType::BaseColor => current.1[0],
            ShaderType::GlowShader {
                stripe_width,
                glow_size,
                red,
                blue,
            } => glowing_shader(&fragment, stripe_width, glow_size, red, blue),
            ShaderType::FBmShader {
                zoom,
                speed,
                fractal,
            } => fbm_shader(
                &fragment, uniforms, &current.1, speed, zoom, &fractal, noise,
            ),
            ShaderType::CellularShader {
                zoom,
                speed,
                fractal,
                cellular,
            } => cellular_shader(
                &fragment, uniforms, &current.1, speed, zoom, &fractal, &cellular, noise,
            ),
        };

        acc.blend(&color, &current.2)
    });

    Fragment { color, ..fragment }
}

fn cellular_shader(
    fragment: &Fragment,
    uniforms: &Uniforms,
    colors: &[Color],
    speed: f32,
    zoom: f32,
    fractal: &FractalConfig,
    cellular: &CellularConfig,
    noise: &mut FastNoiseLite,
) -> Color {
    let Uniforms { time, .. } = uniforms;
    let FractalConfig {
        octaves,
        lacunarity,
        gain,
        weighted_strength,
        f_type,
    } = *fractal;
    let CellularConfig {
        distance_func,
        return_type,
        jitter,
    } = *cellular;

    let x = fragment.vertex_position.x * zoom + speed * time;
    let y = fragment.vertex_position.y * zoom;

    noise.set_noise_type(Some(fastnoise_lite::NoiseType::Cellular));
    noise.set_fractal_octaves(Some(octaves));
    noise.set_fractal_gain(Some(gain));
    noise.set_fractal_weighted_strength(Some(weighted_strength));
    noise.set_fractal_type(Some(f_type));
    noise.set_fractal_lacunarity(Some(lacunarity));

    noise.set_cellular_distance_function(Some(distance_func));
    noise.set_cellular_return_type(Some(return_type));
    noise.set_cellular_jitter(Some(jitter));

    let noise_value = noise.get_noise_2d(x, y);
    let intensity = clamp_with_universe(vec2(-1.0, 1.0), vec2(0.0, 1.0), noise_value);

    colors[0] * intensity
}

fn intensity_shader(fragment: &Fragment, current_color: &Color) -> Color {
    let Fragment { intensity, .. } = fragment;

    *current_color * *intensity
}

fn stripes_shader(fragment: &Fragment, stripe_width: f32, colors: &[Color]) -> Color {
    let y = fragment.vertex_position.y;
    // let y = fragment.position.y as usize;

    let stripe_idx = (y / stripe_width).abs() as usize % colors.len();
    colors[stripe_idx]
}

fn interesting_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let color1 = Color::red();
    let color2 = Color::green();
    let color3 = Color::blue();

    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let frequency = 10.0;

    let wave1 = (x * 7.0 * frequency + y * 5.0 * frequency).sin() * 0.5 + 0.5;
    let wave2 = (x * 5.0 * frequency - y * 8.0 * frequency + PI / 3.0).sin() * 0.5 + 0.5;
    let wave3 = (y * 6.0 * frequency + x * 4.0 * frequency + 2.0 * PI / 3.0).sin() * 0.5 + 0.5;

    // TODO: Keep implementing...

    color1
        .lerp(&color2, wave1)
        .lerp(&color3, wave2)
        .lerp(&color1, wave3)
}

fn glowing_shader(
    fragment: &Fragment,
    stripe_width: f32,
    glow_size: f32,
    red: f32,
    blue: f32,
) -> Color {
    let y = fragment.vertex_position.y;

    let distance_to_center = (y % stripe_width - stripe_width / 2.0).abs();
    let glow_intensity = ((1.0 - (distance_to_center / glow_size).min(1.0)) * PI / 2.0).sin();

    Color::new(
        (red * glow_intensity * 255.0) as u8,
        (blue * glow_intensity * 255.0) as u8,
        (glow_intensity * 255.0) as u8,
    )
}

fn moving_stripes(
    fragment: &Fragment,
    stripe_width: f32,
    speed: f32,
    colors: &[Color],
    uniforms: &Uniforms,
) -> Color {
    let color1 = colors[0];
    let color2 = colors[1];

    let moving_y = fragment.vertex_position.y + uniforms.time * speed;

    let stripe_factor = ((moving_y / stripe_width) * PI).sin() * 0.5 + 0.5;
    color1.lerp(&color2, stripe_factor)
}

fn fbm_shader(
    fragment: &Fragment,
    uniforms: &Uniforms,
    colors: &[Color],
    speed: f32,
    zoom: f32,
    fractal: &FractalConfig,
    noise: &mut FastNoiseLite,
) -> Color {
    let Uniforms { time, .. } = uniforms;
    let FractalConfig {
        octaves,
        lacunarity,
        gain,
        weighted_strength,
        f_type,
    } = *fractal;

    let x = fragment.vertex_position.x * zoom + speed * time;
    let y = fragment.vertex_position.y * zoom;

    noise.set_fractal_octaves(Some(octaves));
    noise.set_fractal_gain(Some(gain));
    noise.set_fractal_weighted_strength(Some(weighted_strength));
    noise.set_noise_type(Some(fastnoise_lite::NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(f_type));
    noise.set_fractal_lacunarity(Some(lacunarity));

    let noise_value = noise.get_noise_2d(x, y);
    let intensity = clamp_with_universe(vec2(-1.0, 1.0), vec2(0.0, 1.0), noise_value);

    colors[0] * intensity
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
    let far = 1000.0;

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
