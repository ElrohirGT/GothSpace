use nalgebra_glm::Vec3;

use crate::{
    color::blenders::BlendMode,
    obj::load_objs,
    texture::Textures,
    vertex::shader::{create_model_matrix, ShaderType},
    Entity, EntityModel, EntityOptimizations,
};

pub fn create_skybox() -> Entity {
    let sphere = load_objs("assets/models/sphere.obj").unwrap();

    let shaders = vec![(
        ShaderType::Texture {
            texture: Textures::Space,
        },
        vec![],
        BlendMode::Replace,
    )];

    let scale = 100.0;
    let rotation = Vec3::zeros();
    let translation = Vec3::zeros();
    let optimizations = EntityOptimizations {
        camera_direction: false,
        frustum_cutting: true,
    };

    Entity {
        objs: sphere,
        use_normal: true,
        shaders,
        optimizations,
        model_matrix: create_model_matrix(translation, scale, rotation),
        model: EntityModel {
            rotation,
            scale,
            translation,
        },
    }
}
