use nalgebra_glm::{Mat4, Vec3};

use crate::{
    color::{blenders::BlendMode, Color},
    obj::load_objs,
    vertex::shader::{create_model_matrix, CellularConfig, FractalConfig, ShaderType},
    Entity, EntityModel, EntityOptimizations,
};

use super::ellipsis::Ellipsis;

pub fn create_default_planet_model_matrix() -> Mat4 {
    let model = create_default_planet_model();
    create_model_matrix(model.translation, model.scale, model.rotation)
}

pub fn create_default_planet_model() -> EntityModel {
    EntityModel {
        rotation: Vec3::zeros(),
        translation: Vec3::zeros(),
        scale: 1.0,
    }
}

const SPHERE_OBJ: &str = "assets/models/sphere.obj";
const OPTIMIZATIONS: EntityOptimizations = EntityOptimizations {
    camera_direction: false,
    frustum_cutting: true,
};

// const BASE_SLOWDOWN: f32 = 1e-2;
const BASE_SLOWDOWN: f32 = 1e-4;

pub fn create_disco_planet() -> Entity {
    let planet_obj = load_objs(SPHERE_OBJ).unwrap();
    let shaders = vec![
        (
            ShaderType::MovingStripes {
                speed: 1e-3,
                stripe_width: 0.1,
            },
            vec![Color::pink(), Color::green()],
            BlendMode::Replace,
        ),
        (
            ShaderType::MovingStripes {
                speed: 1e-4,
                stripe_width: 0.1,
            },
            vec![Color::black(), Color::blue()],
            BlendMode::Normal,
        ),
        (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    let ellipsis = Some(Ellipsis {
        center: Vec3::zeros(),
        a: 10.0,
        b: 10.0,
        y_max: 0.0,
        velocity: 10.0 * BASE_SLOWDOWN,
    });

    Entity {
        wireframe_color: None,
        ellipsis,
        custom_depth: None,
        model: create_default_planet_model(),
        objs: planet_obj,
        use_screen_position: false,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
        optimizations: OPTIMIZATIONS,
    }
}

pub fn create_ocean_planet() -> Entity {
    let planet_obj = load_objs(SPHERE_OBJ).unwrap();
    let shaders = vec![
        (
            ShaderType::MovingStripes {
                speed: 1e-4,
                stripe_width: 0.1,
            },
            vec![Color::new(0, 0, 240), Color::blue()],
            BlendMode::Replace,
        ),
        (
            ShaderType::FBmShader {
                zoom: 600.0,
                speed: 4e-2,
                fractal: FractalConfig {
                    octaves: 4,
                    lacunarity: 2.0,
                    gain: 0.8,
                    weighted_strength: 0.0,
                    f_type: fastnoise_lite::FractalType::FBm,
                },
            },
            vec![Color::new(230, 230, 230)],
            BlendMode::Screen,
        ),
        (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    let ellipsis = Some(Ellipsis {
        center: Vec3::zeros(),
        a: 5.0,
        b: 60.0,
        y_max: 100.0,
        velocity: 3.0 * BASE_SLOWDOWN,
    });

    Entity {
        wireframe_color: None,
        ellipsis,
        custom_depth: None,
        model: create_default_planet_model(),
        use_screen_position: false,
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
        optimizations: OPTIMIZATIONS,
    }
}

pub fn create_gas_giant() -> Entity {
    let planet_obj = load_objs(SPHERE_OBJ).unwrap();
    let shaders = vec![
        (
            ShaderType::BaseColor,
            vec![0xc2e9ed.into()],
            BlendMode::Replace,
        ),
        (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    let ellipsis = Some(Ellipsis {
        center: Vec3::zeros(),
        a: 80.0,
        b: 90.0,
        y_max: 0.0,
        velocity: 6.0 * BASE_SLOWDOWN,
    });

    Entity {
        ellipsis,
        wireframe_color: None,
        custom_depth: None,
        model: create_default_planet_model(),
        use_screen_position: false,
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
        optimizations: OPTIMIZATIONS,
    }
}

pub fn create_face_planet() -> Entity {
    let planet_obj = load_objs(SPHERE_OBJ).unwrap();
    let shaders = vec![
        (
            ShaderType::CellularShader {
                zoom: 200.0,
                speed: 0.0,
                fractal: FractalConfig {
                    octaves: 3,
                    lacunarity: 2.0,
                    gain: 1.26,
                    weighted_strength: 0.0,
                    f_type: fastnoise_lite::FractalType::FBm,
                },
                cellular: CellularConfig {
                    distance_func: fastnoise_lite::CellularDistanceFunction::EuclideanSq,
                    return_type: fastnoise_lite::CellularReturnType::Distance2Div,
                    jitter: 1.0,
                },
            },
            vec![Color::red()],
            BlendMode::Replace,
        ),
        (
            ShaderType::BaseColor,
            vec![0xff7900.into()],
            BlendMode::Overlay,
        ),
        // (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    let ellipsis = Some(Ellipsis {
        center: Vec3::zeros(),
        a: 45.0,
        b: 60.0,
        y_max: 40.0,
        velocity: 2.0 * BASE_SLOWDOWN,
    });

    Entity {
        wireframe_color: None,
        ellipsis,
        custom_depth: None,
        model: create_default_planet_model(),
        use_screen_position: false,
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
        optimizations: OPTIMIZATIONS,
    }
}

pub fn create_snow_planet() -> Entity {
    let planet_obj = load_objs(SPHERE_OBJ).unwrap();
    let shaders = vec![
        (
            ShaderType::FBmShader {
                zoom: 500.0,
                speed: 0.2,
                fractal: FractalConfig {
                    octaves: 3,
                    lacunarity: 0.5,
                    gain: 1.0,
                    weighted_strength: 0.0,
                    f_type: fastnoise_lite::FractalType::FBm,
                },
            },
            vec![0xc2e9ed.into()],
            BlendMode::Add,
        ),
        (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    let ellipsis = Some(Ellipsis {
        center: Vec3::zeros(),
        a: 22.0,
        b: 35.0,
        y_max: 0.0,
        velocity: BASE_SLOWDOWN,
    });

    Entity {
        wireframe_color: None,
        ellipsis,
        custom_depth: None,
        model: create_default_planet_model(),
        use_screen_position: false,
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
        optimizations: OPTIMIZATIONS,
    }
}

pub fn create_sun(starting_position: Vec3) -> Entity {
    let planet_obj = load_objs(SPHERE_OBJ).unwrap();
    let shaders = vec![
        (
            ShaderType::BaseColor,
            vec![0xf55e08.into()],
            BlendMode::Replace,
        ),
        (
            ShaderType::CellularShader {
                zoom: 2000.0,
                speed: 0.2,
                fractal: FractalConfig {
                    octaves: 4,
                    lacunarity: 0.5,
                    gain: 1.0,
                    weighted_strength: 0.0,
                    f_type: fastnoise_lite::FractalType::PingPong,
                },
                cellular: CellularConfig {
                    distance_func: fastnoise_lite::CellularDistanceFunction::EuclideanSq,
                    return_type: fastnoise_lite::CellularReturnType::Distance,
                    jitter: 1.0,
                },
            },
            vec![0xc2e9ed.into()],
            BlendMode::Add,
        ),
        // (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    let mut model = create_default_planet_model();
    model.translation = starting_position;
    model.scale *= 4.0;

    Entity {
        wireframe_color: None,
        ellipsis: None,
        custom_depth: None,
        model_matrix: create_model_matrix(model.translation, model.scale, model.rotation),
        model,
        use_screen_position: false,
        objs: planet_obj,
        shaders,
        optimizations: OPTIMIZATIONS,
    }
}

pub fn create_green_planet() -> Entity {
    let planet_obj = load_objs(SPHERE_OBJ).unwrap();
    let shaders = vec![
        (
            ShaderType::BaseColor,
            // vec![Color::new(100, 100, 100)],
            vec![Color::blue()],
            BlendMode::Replace,
        ),
        (
            ShaderType::FBmShader {
                zoom: 200.0,
                speed: 0.1,
                fractal: FractalConfig {
                    octaves: 4,
                    lacunarity: 2.0,
                    gain: 0.5,
                    weighted_strength: 0.0,
                    f_type: fastnoise_lite::FractalType::None,
                },
            },
            vec![0x087227.into()],
            BlendMode::Difference,
        ),
        (
            ShaderType::FBmShader {
                zoom: 200.0,
                speed: 0.0,
                fractal: FractalConfig {
                    octaves: 4,
                    lacunarity: 2.0,
                    gain: 0.5,
                    weighted_strength: 0.0,
                    f_type: fastnoise_lite::FractalType::None,
                },
            },
            vec![Color::new(0, 0, 100)],
            BlendMode::Subtract,
        ),
        // (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    let ellipsis = Some(Ellipsis {
        center: Vec3::zeros(),
        a: 50.0,
        b: 12.0,
        y_max: 12.0,
        velocity: 5.0 * BASE_SLOWDOWN,
    });

    Entity {
        wireframe_color: None,
        ellipsis,
        custom_depth: None,
        model: create_default_planet_model(),
        objs: planet_obj,
        use_screen_position: false,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
        optimizations: OPTIMIZATIONS,
    }
}
