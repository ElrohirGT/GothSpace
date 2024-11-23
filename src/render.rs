use fastnoise_lite::FastNoiseLite;
use nalgebra_glm::{Mat4, Vec3};
use rayon::prelude::*;

use crate::{
    fragment::{shaders::fragment_shader, triangle, Fragment},
    framebuffer::Framebuffer,
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
        ..
    } = data;

    let mut render_entities = Vec::with_capacity(1 + entities.len());
    render_entities.push(ship);
    for e in entities.iter() {
        render_entities.push(e);
    }

    for entity in render_entities {
        let Entity {
            objs,
            shaders,
            model_matrix,
            optimize,
            use_normal,
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
            let triangles = assembly(&new_vertices);
            // println!("Assembly done!");

            // Rasterization
            // println!("Applying rasterization...");
            let camera_direction = &camera.direction();
            let fragments = rasterize(
                triangles,
                if *optimize {
                    Some(camera_direction)
                } else {
                    None
                },
                *use_normal,
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

fn apply_shaders(vertices: &[Vertex], uniforms: &Uniforms, model_matrix: &Mat4) -> Vec<Vertex> {
    vertices
        .iter()
        .map(|v| vertex_shader(v, uniforms, model_matrix))
        .collect()
}

fn assembly(vertices: &[Vertex]) -> Vec<&[Vertex]> {
    vertices.chunks(3).collect()
    // let mut triangles = Vec::new();
    // for i in (0..vertices.len()).step_by(3) {
    //     if i + 2 < vertices.len() {
    //         triangles.push(&[
    //             vertices[i].clone(),
    //             vertices[i + 1].clone(),
    //             vertices[i + 2].clone(),
    //         ]);
    //     }
    // }
    // triangles
}

fn rasterize(
    triangles: Vec<&[Vertex]>,
    camera_direction: Option<&Vec3>,
    use_normal: bool,
) -> Vec<Fragment> {
    triangles
        .iter()
        .flat_map(|tri| triangle(&tri[0], &tri[1], &tri[2], camera_direction, use_normal))
        .collect()
}

fn paint_fragments(fragments: Vec<Fragment>, framebuffer: &mut Framebuffer) {
    for fragment in fragments {
        framebuffer.set_current_color(fragment.color);
        let _ = framebuffer.paint_point(fragment.position, fragment.depth);
    }
}
