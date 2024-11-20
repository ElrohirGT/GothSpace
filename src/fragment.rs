use crate::{color::Color, shader::Uniforms, vertex::Vertex};
use nalgebra_glm::{dot, vec3_to_vec2, Vec2, Vec3};

pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub intensity: f32,
    pub depth: f32,
    pub vertex_position: Vec3,
}

impl Fragment {
    pub fn new(position: Vec2, color: Color, depth: f32, vertex_position: Vec3) -> Self {
        Fragment {
            position,
            color,
            depth,
            vertex_position,
            intensity: 1.0,
        }
    }

    pub fn new_with_intensity(
        position: Vec2,
        color: Color,
        depth: f32,
        vertex_position: Vec3,
        intensity: f32,
    ) -> Self {
        Fragment {
            position,
            color,
            intensity,
            depth,
            vertex_position,
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
    let direction = b.position - a.position;

    // println!(
    //     "From {:?} to {:?}, DIR={direction:?}",
    //     b.transformed_position, a.transformed_position
    // );

    let mut accum = 0.0;
    while accum <= 1.0 {
        let new_position = a.position + accum * direction;
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

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, camera_direction: &Vec3) -> Vec<Fragment> {
    // let mut fragments = wireframe_triangle(v1, v2, v3);
    // let mut fragments = vec![];

    let (a, b, c) = (v1.position, v2.position, v3.position);

    let triangle_area = edge_function(&a, &b, &vec3_to_vec2(&c));
    let (min, max) = calculate_bounding_box(&a, &b, &c);

    let light_dir = Vec3::new(0.0, 0.5, 1.0).normalize();
    let base_color = Color::new(100, 100, 100);

    let step_size = 5e-1;
    let y_step_count = ((max.y - min.y) / step_size).ceil() as u32;
    let x_step_count = ((max.x - min.x) / step_size).ceil() as u32;

    let fragments: Vec<Fragment> = (0..y_step_count)
        .flat_map(|y_idx| {
            let currenty = min.y + step_size * (y_idx as f32);

            (0..x_step_count).filter_map(move |x_idx| {
                let currentx = min.x + step_size * (x_idx as f32);

                let point = Vec2::new(currentx, currenty);
                let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);

                if (0.0..=1.0).contains(&w1)
                    && (0.0..=1.0).contains(&w2)
                    && (0.0..=1.0).contains(&w3)
                {
                    // Interpolated normal...
                    let normal = w1 * v1.normal + w2 * v2.normal + w3 * v3.normal;
                    let normal = normal.normalize();
                    let camera_intensity = dot(&normal, camera_direction);
                    if camera_intensity >= 0.0 {
                        // If the camera is not looking at the fragment, don't compute it!
                        return None;
                    }

                    let intensity = dot(&light_dir, &normal).clamp(0.0, 1.0);
                    // if intensity <= 0.0 {
                    //     println!("The intensity is {intensity}! {light_dir:?} dot {normal:?}");
                    // }

                    // Interpolated depth...
                    let depth = w1 * a.z + w2 * b.z + w3 * c.z;

                    // Interpolated position...
                    // let position = a;
                    // FIXME: For now the normal is fine, but this should ideally be
                    // a position using barycentrics
                    let position = normal;
                    // let position = a * w1 + b * w2 + c * w3;
                    Some(Fragment::new_with_intensity(
                        point, base_color, depth, position, intensity,
                    ))
                } else {
                    None
                }
            })
        })
        .collect();

    fragments
}

pub fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (Vec2, Vec2) {
    let minx = v1.x.min(v2.x).min(v3.x);
    let miny = v1.y.min(v2.y).min(v3.y);

    let maxx = v1.x.max(v2.x).max(v3.x);
    let maxy = v1.y.max(v2.y).max(v3.y);

    (Vec2::new(minx, miny), Vec2::new(maxx, maxy))
}

// pub fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> (f32, f32, f32) {
//     let pa = a - p;
//     let ab = b - a;
//     let ac = c - a;
//
//     let v = (pa.y * ab.x - pa.x * ab.y) / (ac.x * ab.y - ac.y * ab.x);
//     let u = -(v * ac.y + pa.y) / ab.y;
//     let w = 1.0 - u - v;
//
//     (u, v, w)
// }

fn barycentric_coordinates(p: &Vec2, a: &Vec3, b: &Vec3, c: &Vec3, area: f32) -> (f32, f32, f32) {
    let w1 = edge_function(b, c, p) / area;
    let w2 = edge_function(c, a, p) / area;
    let w3 = edge_function(a, b, p) / area;

    (w1, w2, w3)
}
fn edge_function(a: &Vec3, b: &Vec3, c: &Vec2) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}
