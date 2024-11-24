use std::f32::consts::PI;

use fastnoise_lite::FastNoiseLite;
use nalgebra_glm::vec2;

use crate::{
    clamp_with_universe,
    color::Color,
    texture::{GameTextures, Textures},
    vertex::shader::{CellularConfig, FractalConfig, ShaderType, Uniforms},
    EntityShader,
};

use super::Fragment;

pub fn fragment_shader(
    fragment: Fragment,
    inputs: &[EntityShader],
    uniforms: &Uniforms,
    noise: &mut FastNoiseLite,
    textures: &GameTextures,
) -> Fragment {
    let color = inputs.iter().fold(
        fragment.color,
        |acc, (shader_type, colors, blend_strategy)| {
            let color = match shader_type {
                ShaderType::Stripe { stripe_width } => {
                    stripes_shader(&fragment, *stripe_width, colors)
                }
                ShaderType::MovingStripes {
                    stripe_width,
                    speed,
                } => moving_stripes(&fragment, *stripe_width, *speed, colors, uniforms),
                ShaderType::Intensity => intensity_shader(&fragment, &acc),
                ShaderType::BaseColor => colors[0],
                ShaderType::GlowShader {
                    stripe_width,
                    glow_size,
                    red,
                    blue,
                } => glowing_shader(&fragment, *stripe_width, *glow_size, *red, *blue),
                ShaderType::FBmShader {
                    zoom,
                    speed,
                    fractal,
                } => fbm_shader(&fragment, uniforms, colors, *speed, *zoom, fractal, noise),
                ShaderType::CellularShader {
                    zoom,
                    speed,
                    fractal,
                    cellular,
                } => cellular_shader(
                    &fragment, uniforms, colors, *speed, *zoom, fractal, cellular, noise,
                ),
                ShaderType::Texture { texture } => texture_shader(&fragment, textures, *texture),
            };

            acc.blend(&color, blend_strategy)
        },
    );

    Fragment { color, ..fragment }
}

fn texture_shader(fragment: &Fragment, textures: &GameTextures, texture: Textures) -> Color {
    let texture = textures.get_texture(texture);
    texture.get_pixel_color(fragment.texture_position.x, fragment.texture_position.y)
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

fn stripes_shader(fragment: &Fragment, stripe_width: f32, colors: &[Color]) -> Color {
    let y = fragment.vertex_position.y;
    // let y = fragment.position.y as usize;

    let stripe_idx = (y / stripe_width).abs() as usize % colors.len();
    colors[stripe_idx]
}

fn intensity_shader(fragment: &Fragment, current_color: &Color) -> Color {
    let Fragment { intensity, .. } = fragment;

    *current_color * *intensity
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
