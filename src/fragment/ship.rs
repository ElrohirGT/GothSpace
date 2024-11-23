use core::f32;
use std::f32::consts::PI;

use nalgebra_glm::{rotate_vec3, vec3, Vec3};

use crate::{
    camera::Camera,
    color::{blenders::BlendMode, Color},
    obj::load_objs,
    vertex::shader::{create_model_matrix, ShaderType},
    Entity,
};

const ORIGINAL_ROTATION: Vec3 = Vec3::new(0.0, PI, 0.0);

pub fn create_ship(camera: &Camera) -> Entity {
    let ship_obj = load_objs("BlueFalcon.obj").unwrap();

    let shaders = vec![
        (
            ShaderType::BaseColor,
            vec![Color::new(100, 100, 100)],
            BlendMode::Replace,
        ),
        (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    let scale = 0.1;
    let rotation = ORIGINAL_ROTATION;
    let translation = translation_from_camera(camera);

    Entity {
        objs: ship_obj,
        use_normal: false,
        shaders,
        optimize: false,
        model_matrix: create_model_matrix(translation, scale, rotation),
        model: crate::EntityModel {
            rotation,
            scale,
            translation,
        },
    }
}

pub fn translation_from_camera(camera: &Camera) -> Vec3 {
    camera.center + vec3(0.0, -3.0, 0.0)
}
