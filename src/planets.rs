use nalgebra_glm::{Mat4, Vec3};

use crate::{
    blenders::BlendMode,
    color::Color,
    obj::load_objs,
    shader::{create_model_matrix, FractalConfig, ShaderType},
    Entity,
};

pub fn create_default_planet_model_matrix() -> Mat4 {
    create_model_matrix(Vec3::zeros(), 1.0, Vec3::zeros())
}

pub fn create_disco_planet() -> Entity {
    let planet_obj = load_objs("sphere.obj").unwrap();
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

    Entity {
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
    }
}

pub fn create_ocean_planet() -> Entity {
    let planet_obj = load_objs("sphere.obj").unwrap();
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

    Entity {
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
    }
}

pub fn create_gas_giant() -> Entity {
    let planet_obj = load_objs("sphere.obj").unwrap();
    let shaders = vec![
        (
            ShaderType::BaseColor,
            vec![0xc2e9ed.into()],
            BlendMode::Replace,
        ),
        (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    Entity {
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
    }
}

pub fn create_face_planet() -> Entity {
    let planet_obj = load_objs("sphere.obj").unwrap();
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
                cellular: crate::shader::CellularConfig {
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

    Entity {
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
    }
}

pub fn create_snow_planet() -> Entity {
    let planet_obj = load_objs("sphere.obj").unwrap();
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

    Entity {
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
    }
}

pub fn create_sun() -> Entity {
    let planet_obj = load_objs("sphere.obj").unwrap();
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
                cellular: crate::shader::CellularConfig {
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

    Entity {
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
    }
}

pub fn create_green_planet() -> Entity {
    let planet_obj = load_objs("sphere.obj").unwrap();
    let shaders = vec![
        (
            ShaderType::BaseColor,
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
        // (
        //     ShaderType::FBmShader {
        //         zoom: 200.0,
        //         speed: 0.0,
        //         fractal: FractalConfig {
        //             octaves: 4,
        //             lacunarity: 2.0,
        //             gain: 0.5,
        //             weighted_strength: 0.0,
        //             f_type: fastnoise_lite::FractalType::None,
        //         },
        //     },
        //     vec![Color::new(0, 0, 100)],
        //     BlendMode::Subtract,
        // ),
        // (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    Entity {
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
    }
}
