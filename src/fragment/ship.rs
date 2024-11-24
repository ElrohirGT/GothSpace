use std::f32::consts::PI;

use nalgebra_glm::{vec3, Vec3};

use crate::{
    camera::Camera,
    color::{blenders::BlendMode, Color},
    obj::load_objs,
    vertex::shader::{create_model_matrix, ShaderType},
    Entity, EntityModel, EntityOptimizations, Ship,
};

pub const ORIGINAL_ROTATION: Vec3 = Vec3::new(0.0, PI, 0.0);

pub fn create_ship(initial_world_position: Vec3) -> Ship {
    let ship_obj = load_objs("assets/models/BlueFalcon.obj").unwrap();

    let shaders = vec![
        (
            ShaderType::BaseColor,
            vec![Color::new(0, 0, 255)],
            BlendMode::Replace,
        ),
        // (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    let scale = 0.1;
    let rotation = ORIGINAL_ROTATION;
    let translation = initial_world_position;
    let optimizations = EntityOptimizations {
        camera_direction: false,
        frustum_cutting: false,
    };

    let entity = Entity {
        ellipsis: None,
        objs: ship_obj,
        use_screen_position: false,
        shaders,
        optimizations,
        model_matrix: create_model_matrix(translation, scale, rotation),
        model: crate::EntityModel {
            rotation,
            scale,
            translation,
        },
        custom_depth: None,
    };

    Ship {
        acceleration: Vec3::zeros(),
        velocity: Vec3::zeros(),
        entity,
    }
}

pub fn create_ship_from(other_ship: &Ship) -> Ship {
    let ship_obj = load_objs("assets/models/BlueFalcon.obj").unwrap();

    let shaders = vec![
        (
            ShaderType::BaseColor,
            vec![Color::new(0, 0, 255)],
            BlendMode::Replace,
        ),
        // (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    let optimizations = EntityOptimizations {
        camera_direction: false,
        frustum_cutting: false,
    };

    let EntityModel {
        rotation,
        scale,
        translation,
    } = other_ship.entity.model;

    let model_matrix = create_model_matrix(translation, scale, rotation);

    let model = EntityModel {
        rotation,
        scale,
        translation,
    };

    let entity = Entity {
        model,
        model_matrix,
        ellipsis: None,
        objs: ship_obj,
        use_screen_position: false,
        shaders,
        optimizations,
        custom_depth: None,
    };

    Ship {
        acceleration: other_ship.acceleration,
        velocity: other_ship.velocity,
        entity,
    }
}

pub fn translation_from_camera(camera: &Camera) -> Vec3 {
    camera.center + vec3(0.0, -3.0, 0.0)
}
