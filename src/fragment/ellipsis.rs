use nalgebra_glm::{vec3, Vec3};

/// Contains all the parameters to make an ellipsis.
/// The formula for an ellipsis centered at (h,k) is:
/// (x-h)^2 / a^2 + (z-k)^2 / b^2 = 1
pub struct Ellipsis {
    pub center: Vec3,
    pub a: f32,
    pub b: f32,
    pub y_max: f32,
    pub velocity: f32,
}

pub fn next_point_in_ellipsis(radians: f32, ellipsis: &Ellipsis) -> Vec3 {
    let Ellipsis {
        a,
        b,
        y_max,
        center,
        ..
    } = ellipsis;

    let (sin, cos) = radians.sin_cos();
    let x = a * cos;
    let z = b * sin;
    let y = y_max * cos;

    center + vec3(x, y, z)
}
