pub mod ellipsis;
pub mod material;
pub mod planets;
pub mod shaders;
pub mod ship;

use crate::{color::Color, light::Light, vertex::Vertex};
use nalgebra_glm::{dot, vec2, vec3_to_vec2, Vec2, Vec3};

pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub intensity: f32,
    pub depth: f32,
    pub vertex_position: Vec3,
    pub texture_position: Vec2,
}

impl Fragment {
    pub fn new(position: Vec2, color: Color, depth: f32, vertex_position: Vec3) -> Self {
        Fragment {
            position,
            color,
            depth,
            vertex_position,
            intensity: 1.0,
            texture_position: Vec2::zeros(),
        }
    }

    pub fn new_with_intensity(
        position: Vec2,
        color: Color,
        depth: f32,
        vertex_position: Vec3,
        intensity: f32,
        texture_position: Vec2,
    ) -> Self {
        Fragment {
            position,
            color,
            intensity,
            depth,
            vertex_position,
            texture_position,
        }
    }

    // pub fn apply<T>(self, uniforms: &Uniforms, func: T) -> Fragment
    // where
    //     T: Fn(Fragment, &Uniforms) -> Fragment,
    // {
    //     func(self, uniforms)
    // }
}

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = vec![];
    // let distance = nalgebra_glm::distance(&b.transformed_position, &a.transformed_position);
    // let step_size = 1.0 / (10.0 / 2.0 * distance);
    let step_size = 1.0e-3;
    let direction = b.screen_position - a.screen_position;

    // println!(
    //     "From {:?} to {:?}, DIR={direction:?}",
    //     b.transformed_position, a.transformed_position
    // );

    let mut accum = 0.0;
    while accum <= 1.0 {
        let new_position = a.screen_position + accum * direction;
        // println!("POINT: {new_position:?} t={accum}");
        fragments.push(Fragment::new(
            vec3_to_vec2(&new_position),
            Color::pink(),
            0.0,
            new_position,
        ));
        accum += step_size;
    }

    fragments
}

pub fn wireframe_triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    line(v1, v2)
        .into_iter()
        .chain(line(v2, v3))
        .chain(line(v3, v1))
        .collect()
}

pub fn triangle(
    v1: &Vertex,
    v2: &Vertex,
    v3: &Vertex,
    camera_direction: Option<&Vec3>,
    use_screen_position: &bool,
    lights: &[Light],
    custom_depth: Option<f32>,
) -> Vec<Fragment> {
    // let mut fragments = wireframe_triangle(v1, v2, v3);
    // let mut fragments = vec![];

    let (a, b, c) = (v1.screen_position, v2.screen_position, v3.screen_position);

    let triangle_area = edge_function(&a, &b, &vec3_to_vec2(&c));
    let (min, max) = calculate_bounding_box(&a, &b, &c);

    let base_color = Color::new(100, 100, 100);

    let possible_fragment_count = (max.1 - min.1) * (max.0 - min.0);
    let mut fragments = Vec::with_capacity(possible_fragment_count as usize);

    (min.1..=max.1).for_each(|y| {
        (min.0..=max.0).for_each(|x| {
            let point = vec2(x as f32 + 0.5, y as f32 + 0.5);
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);

            // If the point is inside the triangle...
            if (0.0..=1.0).contains(&w1) && (0.0..=1.0).contains(&w2) && (0.0..=1.0).contains(&w3) {
                // Interpolated normal...
                let normal = w1 * v1.normal + w2 * v2.normal + w3 * v3.normal;
                let normal = normal.normalize();
                // FIXME: For now the normal is fine, but this should ideally be
                // a position using barycentrics
                // Interpolated position...
                let position = if *use_screen_position {
                    v1.screen_position
                } else {
                    w1 * v1.model_position + w2 * v2.model_position + w3 * v3.model_position
                };

                // Interpolated texture coords...
                let tex_cords = w1 * v1.tex_coords + w2 * v2.tex_coords + w3 * v3.tex_coords;

                if let Some(camera_direction) = camera_direction {
                    let camera_intensity = dot(&normal, camera_direction);
                    if camera_intensity >= 0.0 {
                        // If the camera is not looking at the fragment, don't compute it!
                        return;
                    }
                }

                // Interpolated depth...
                let depth = if let Some(d) = custom_depth {
                    d
                } else {
                    // w1 * v1.position.z + w2 * v2.position.z + w3 * v3.position.z
                    position.z
                };

                // let position = if use_normal {
                //     normal
                // } else {
                //     w1 * v1.position + w2 * v2.position + w3 * v3.position
                // };

                let mut intensity = 0.0;
                for light_source in lights {
                    let light_dir = (position - light_source.position).normalize();
                    intensity += dot(&light_dir, &normal) * light_source.intensity;
                }

                intensity = intensity.clamp(0.0, 1.0);

                fragments.push(Fragment::new_with_intensity(
                    point, base_color, depth, position, intensity, tex_cords,
                ));
            }
        })
    });

    fragments
}

pub fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> ((i32, i32), (i32, i32)) {
    let minx = v1.x.min(v2.x).min(v3.x).floor();
    let miny = v1.y.min(v2.y).min(v3.y).floor();

    let maxx = v1.x.max(v2.x).max(v3.x).ceil();
    let maxy = v1.y.max(v2.y).max(v3.y).ceil();

    ((minx as i32, miny as i32), (maxx as i32, maxy as i32))
}

fn barycentric_coordinates(p: &Vec2, a: &Vec3, b: &Vec3, c: &Vec3, area: f32) -> (f32, f32, f32) {
    let w1 = edge_function(b, c, p) / area;
    let w2 = edge_function(c, a, p) / area;
    let w3 = edge_function(a, b, p) / area;

    (w1, w2, w3)
}
fn edge_function(a: &Vec3, b: &Vec3, c: &Vec2) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}
