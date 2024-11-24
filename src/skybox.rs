use core::f32;
use nalgebra_glm::{vec2, Vec3, Vec4};
use rand::prelude::*;
use std::f32::consts::PI;

use crate::{framebuffer::Framebuffer, vertex::shader::Uniforms};

pub struct Star {
    position: Vec3,
    brightness: f32,
    size: u8,
}

pub struct Skybox {
    stars: Vec<Star>,
}

impl Skybox {
    pub fn new(star_count: usize, star_radius: f32) -> Self {
        let mut rng = rand::thread_rng();
        let mut stars = Vec::with_capacity(star_count);

        for _ in 0..star_count {
            // Generate random spherical coordinates
            let left_right_angle = rng.gen::<f32>() * 2.0 * PI;
            let up_down_ange = rng.gen::<f32>() * PI;

            // Convert spherical to Cartesian coordinates
            let x = star_radius * up_down_ange.sin() * left_right_angle.cos();
            let y = star_radius * up_down_ange.cos();
            let z = star_radius * up_down_ange.sin() * left_right_angle.sin();

            // Random brightness between 0.0 and 1.0
            let brightness = rng.gen::<f32>();
            let size: u8 = rng.gen_range(1..=3);

            stars.push(Star {
                position: Vec3::new(x, y, z),
                brightness,
                size,
            });
        }

        Skybox { stars }
    }

    pub fn render(
        &self,
        framebuffer: &mut Framebuffer,
        uniforms: &Uniforms,
        camera_position: &Vec3,
    ) {
        // let mut rng = rand::thread_rng();

        for star in &self.stars {
            // Calculate star position relative to camera
            let position = star.position + camera_position;

            // Project the star position to screen space
            let pos_vec4 = Vec4::new(position.x, position.y, position.z, 1.0);
            let projected = uniforms.projection_matrix * uniforms.view_matrix * pos_vec4;

            // Perform perspective division
            if projected.w <= 0.0 {
                continue;
            }
            let ndc = projected / projected.w;

            // Apply viewport transform
            let screen_pos = uniforms.viewport_matrix * Vec4::new(ndc.x, ndc.y, ndc.z, 1.0);

            // Check if star is in front of camera and within screen bounds
            if screen_pos.z < 0.0 {
                continue;
            }

            let x = screen_pos.x as usize;
            let y = screen_pos.y as usize;

            if x < framebuffer.width && y < framebuffer.height {
                // Random chance for star to blink
                /*
                let blink_chance = rng.gen::<f32>();
                let blink_amount = if blink_chance < 0.3 {
                    (rng.gen::<f32>() - 0.5) * 0.2 // This gives us -0.1 to 0.1 variation
                } else {
                    0.0
                };
                let adjusted_brightness = (star.brightness + blink_amount).clamp(0.0, 1.0);
                */
                let intensity = (star.brightness * 255.0) as u8;
                let color = (intensity as u32) << 16 | (intensity as u32) << 8 | intensity as u32;

                framebuffer.set_current_color(color);
                // framebuffer.point(x, y, 1000.0);  // depth is high so things render in front

                let x = x as f32;
                let y = y as f32;
                let depth = -1000.0;
                let _ = match star.size {
                    1 => framebuffer.paint_point(vec2(x, y), depth),
                    2 => framebuffer
                        .paint_point(vec2(x, y), depth)
                        .and_then(|_| framebuffer.paint_point(vec2(x + 1.0, y), depth))
                        .and_then(|_| framebuffer.paint_point(vec2(x, y + 1.0), depth))
                        .and_then(|_| framebuffer.paint_point(vec2(x + 1.0, y + 1.0), depth)),
                    3 => framebuffer
                        .paint_point(vec2(x, y), depth)
                        .and_then(|_| framebuffer.paint_point(vec2(x - 1.0, y), depth))
                        .and_then(|_| framebuffer.paint_point(vec2(x + 1.0, y), depth))
                        .and_then(|_| framebuffer.paint_point(vec2(x, y - 1.0), depth))
                        .and_then(|_| framebuffer.paint_point(vec2(x, y + 1.0), depth)),
                    _ => Ok(()),
                };
            }
        }
    }
}
