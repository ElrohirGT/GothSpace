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
            BlendMode::IgnoreWhiteReplace,
        ),
        (
            ShaderType::CellularShader {
                zoom: 200.0,
                speed: 0.0,
                fractal: crate::vertex::shader::FractalConfig {
                    f_type: fastnoise_lite::FractalType::None,
                    octaves: 4,
                    lacunarity: 0.5,
                    gain: 1.0,
                    weighted_strength: 0.0,
                },
                cellular: crate::vertex::shader::CellularConfig {
                    distance_func: fastnoise_lite::CellularDistanceFunction::EuclideanSq,
                    return_type: fastnoise_lite::CellularReturnType::Distance,
                    jitter: 1.0,
                },
            },
            vec![0xff002b.into()],
            BlendMode::IgnoreWhiteAdd,
        ),
    ];

    let scale = 0.2;
    let rotation = ORIGINAL_ROTATION;
    let translation = initial_world_position;
    let optimizations = EntityOptimizations {
        camera_direction: false,
        frustum_cutting: false,
    };

    let entity = Entity {
        wireframe_color: Some(Color::white()),
        ellipsis: None,
        objs: ship_obj,
        use_screen_position: true,
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
        wireframe_color: Some(Color::white()),
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
