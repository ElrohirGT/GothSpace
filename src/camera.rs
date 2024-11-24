use std::{f32::consts::PI, ops::Neg};

use nalgebra_glm::{rotate_vec3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// Determines whether or not the camera has changed.
    has_changed: bool,

    /// Camera position in the world space.
    pub eye: Vec3,

    /// Point the camera is looking at.
    pub center: Vec3,

    /// What's the up vector of the camera.
    pub up: Vec3,

    /// The max zoom in of the camera.
    pub max_zoom: f32,

    /// The amount of cumulative zoom deltas it currently has.
    accum_zoom: f32,
}

impl Camera {
    /// Creates a new Camera with the given parameters.
    ///
    /// * `eye`: Camera position in the world space.
    /// * `center`: Point the camera is looking at.
    /// * `up`: What's the up vector of the camera.
    /// * `max_zoom`: The max zoom deltas can sum up to.
    pub fn new(eye: Vec3, center: Vec3, up: Vec3, max_zoom: f32) -> Self {
        Camera {
            eye,
            center,
            up,
            has_changed: true,
            accum_zoom: 0.0,
            max_zoom,
        }
    }

    /// Advances the camera by a certain delta in the direction is facing.
    pub fn advance_camera(&mut self, delta: f32) {
        let delta_vec = self.direction() * delta;

        self.eye += delta_vec;
        self.center += delta_vec;
        self.has_changed = true;
    }

    /// Makes the camera rotate it's center by a given yaw and pitch.
    ///
    /// * `delta_yaw`: Rotates the camera from left to right.
    /// * `delta_pitch`: Rotates the camera up and down.
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);

        let radius_xz =
            (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let new_eye = self.center
            + Vec3::new(
                radius * new_yaw.cos() * new_pitch.cos(),
                -radius * new_pitch.sin(),
                radius * new_yaw.sin() * new_pitch.cos(),
            );

        self.eye = new_eye;
        self.has_changed = true;
    }

    /// Moves the point the camera is looking at.
    ///
    /// * `direction`: The direction to move the camera by.
    /// * `rotation_speed`: The speed to which move the camera by.
    pub fn move_center(&mut self, direction: Vec3, rotation_speed: f32) {
        let radius_vector = self.center - self.eye;
        let radius = radius_vector.magnitude();

        let angle_x = direction.x * rotation_speed; // Adjust this factor to control rotation speed
        let angle_y = direction.y * rotation_speed;

        let rotated = rotate_vec3(&radius_vector, angle_x, &Vec3::new(0.0, 1.0, 0.0));

        let right = rotated.cross(&self.up).normalize();
        let final_rotated = rotate_vec3(&rotated, angle_y, &right);

        self.center = self.eye + final_rotated.normalize() * radius;
        self.has_changed = true;
    }

    /// Zooms in and zooms out the camera by a given delta.
    pub fn zoom(&mut self, delta: f32) {
        let Camera {
            has_changed,
            max_zoom,
            accum_zoom,
            ..
        } = self;
        *has_changed = true;

        *accum_zoom += delta;
        let delta = match (accum_zoom < &mut max_zoom.neg(), accum_zoom > max_zoom) {
            (true, _) => {
                *accum_zoom = -*max_zoom;
                0.0
            }
            (_, true) => {
                *accum_zoom = *max_zoom;
                0.0
            }
            (false, false) => delta,
        };

        let forward_dir = self.direction();
        self.eye += forward_dir * delta;
    }

    /// Rotates the Camera in place, by a given delta_yaw and pitch
    ///
    /// * `delta_yaw`: Rotates cam from left to right.
    /// * `delta_pitch`: Rotates cam up and down.
    pub fn rotate_cam(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.has_changed = true;
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);

        let radius_xz =
            (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();

        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        // Keep between [0, PI/2.0]
        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        // Keep it between slightly below (-PI/2, PI/2)
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let new_eye = self.center
            + Vec3::new(
                radius * new_yaw.cos() * new_pitch.cos(),
                -radius * new_pitch.sin(),
                radius * new_yaw.sin() * new_pitch.cos(),
            );

        self.eye = new_eye;
    }

    pub fn has_changed(&self) -> bool {
        self.has_changed
    }

    pub fn reset_change(&mut self) {
        self.has_changed = false;
    }

    pub fn move_focus(&mut self, delta_pos: Vec3) {
        self.has_changed = true;
        self.center += delta_pos;
    }

    /// Get's the direction of the camera, normalized.
    pub fn direction(&self) -> Vec3 {
        (self.center - self.eye).normalize()
    }
}
