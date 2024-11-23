use std::f32::consts::PI;

use nalgebra_glm::{vec3, Vec3};

use crate::{
    color::{blenders::BlendMode, Color},
    obj::load_objs,
    vertex::shader::{create_model_matrix, ShaderType},
    Entity,
};

pub fn create_ship() -> Entity {
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
    let rotation = vec3(0.0, PI, 0.0);
    let translation = vec3(0.0, -2.0, 0.0);

    Entity {
        objs: ship_obj,
        use_normal: false,
        shaders,
        optimize: false,
        model_matrix: create_model_matrix(translation, scale, rotation),
    }
}
