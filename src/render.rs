use fastnoise_lite::FastNoiseLite;
use nalgebra_glm::{Mat4, Vec3};
use rayon::prelude::*;

use crate::{
    color::Color,
    fragment::{shaders::fragment_shader, triangle, Fragment},
    framebuffer::Framebuffer,
    light::Light,
    vertex::{
        shader::{vertex_shader, Uniforms},
        Vertex,
    },
    Entity, Model,
};

pub fn render(framebuffer: &mut Framebuffer, data: &Model, noise: &mut FastNoiseLite) {
    let Model {
        entities,
        uniforms,
        camera,
        ship,
        textures,
        lights,
        skybox,
        view_type,
        game_window,
        ..
    } = data;

    match game_window {
        crate::GameWindow::Controls => {}
        crate::GameWindow::Simulation => {
            skybox.render(framebuffer, uniforms, &camera.eye);

            let mut render_entities = Vec::with_capacity(1 + entities.len());
            if matches!(view_type, crate::ViewType::FirstPerson) {
                render_entities.push(&ship.entity);
            }

            for e in entities.iter() {
                render_entities.push(e);
            }

            for entity in render_entities {
                let Entity {
                    objs,
                    shaders,
                    model_matrix,
                    optimizations,
                    use_screen_position,
                    custom_depth,
                    wireframe_color: color_of_lines,
                    ..
                } = entity;

                for vertex_array in objs {
                    // Vertex Shader
                    // println!("Applying shaders...");
                    let new_vertices = apply_shaders(vertex_array, uniforms, model_matrix);
                    // println!("Vertex shader applied!");
                    // for vertex in new_vertices.iter().take(25) {
                    //     println!("Transformed vertex: {:?}", vertex);
                    // }

                    // Primitive assembly
                    // println!("Assembly...");
                    let triangles = assembly(&new_vertices, optimizations.frustum_cutting);
                    // println!("Assembly done!");

                    // Rasterization
                    // println!("Applying rasterization...");
                    let camera_direction = &camera.direction();
                    let fragments = rasterize(
                        triangles,
                        if optimizations.camera_direction {
                            Some(camera_direction)
                        } else {
                            None
                        },
                        use_screen_position,
                        lights,
                        *custom_depth,
                        color_of_lines,
                    );
                    // println!("Rasterization applied!");

                    // println!("Applying fragment shaders...");
                    let fragments = fragments
                        .into_iter()
                        .map(|f| fragment_shader(f, shaders, uniforms, noise, textures))
                        .collect();
                    // println!("Fragment shaders applied!");

                    // Fragment Processing
                    // println!("Painting fragments...");
                    paint_fragments(fragments, framebuffer);
                    // println!("Fragments painted!");
                }
            }
        }
    }
}

fn apply_shaders(vertices: &[Vertex], uniforms: &Uniforms, model_matrix: &Mat4) -> Vec<Vertex> {
    vertices
        .par_iter()
        .map(|v| vertex_shader(v, uniforms, model_matrix))
        .collect()
}

fn assembly(vertices: &[Vertex], should_optimize: bool) -> Vec<&[Vertex]> {
    let triangles = vertices.chunks(3);

    if should_optimize {
        triangles
            .filter(|triangle_vertices| {
                let range = -1.0..1.0;
                let a = &triangle_vertices[0];
                let b = &triangle_vertices[1];
                let c = &triangle_vertices[2];
                let a_in_range = range.contains(&a.frustum_position.x)
                    && range.contains(&a.frustum_position.y)
                    && range.contains(&a.frustum_position.z);
                let b_in_range = range.contains(&b.frustum_position.x)
                    && range.contains(&b.frustum_position.y)
                    && range.contains(&b.frustum_position.z);
                let c_in_range = range.contains(&c.frustum_position.x)
                    && range.contains(&c.frustum_position.y)
                    && range.contains(&c.frustum_position.z);

                a_in_range || b_in_range || c_in_range
            })
            .collect()
    } else {
        triangles.collect()
    }
}

fn rasterize(
    triangles: Vec<&[Vertex]>,
    camera_direction: Option<&Vec3>,
    use_screen_position: &bool,
    lights: &[Light],
    custom_depth: Option<f32>,
    wirefragme_color: &Option<Color>,
) -> Vec<Fragment> {
    triangles
        .par_iter()
        .flat_map(|tri| {
            triangle(
                &tri[0],
                &tri[1],
                &tri[2],
                camera_direction,
                use_screen_position,
                lights,
                custom_depth,
                wirefragme_color,
            )
        })
        .collect()
}

fn paint_fragments(fragments: Vec<Fragment>, framebuffer: &mut Framebuffer) {
    for fragment in fragments {
        framebuffer.set_current_color(fragment.color);
        let _ = framebuffer.paint_point(fragment.position, fragment.depth);
    }
}
