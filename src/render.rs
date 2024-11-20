use fastnoise_lite::FastNoiseLite;
use nalgebra_glm::{Mat4, Vec3};
use rayon::prelude::*;

use crate::{
    fragment::{triangle, Fragment},
    framebuffer::Framebuffer,
    shader::{fragment_shader, vertex_shader, Uniforms},
    vertex::Vertex,
    Entity, Model,
};

pub fn render(framebuffer: &mut Framebuffer, data: &Model, noise: &mut FastNoiseLite) {
    let Model {
        render_entities,
        uniforms,
        camera,
        ..
    } = data;

    for entity in render_entities {
        let Entity {
            objs,
            shaders,
            model_matrix,
        } = entity;

        for vertex_array in objs {
            // Vertex Shader
            // println!("Applying shaders...");
            let new_vertices = apply_shaders(vertex_array, uniforms, model_matrix);
            // println!("Vertex shader applied!");

            // Primitive assembly
            // println!("Assembly...");
            let triangles = assembly(&new_vertices);
            // println!("Assembly done!");

            // Rasterization
            // println!("Applying rasterization...");
            let fragments = rasterize(triangles, &camera.direction());
            // println!("Rasterization applied!");

            // println!("Applying fragment shaders...");
            let fragments = fragments
                .into_iter()
                .map(|f| fragment_shader(f, shaders, uniforms, noise))
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
}

fn rasterize(triangles: Vec<&[Vertex]>, camera_direction: &Vec3) -> Vec<Fragment> {
    triangles
        .iter()
        .flat_map(|tri| triangle(&tri[0], &tri[1], &tri[2], camera_direction))
        .collect()
}

fn paint_fragments(fragments: Vec<Fragment>, framebuffer: &mut Framebuffer) {
    for fragment in fragments {
        framebuffer.set_current_color(fragment.color);
        let _ = framebuffer.paint_point(fragment.position, fragment.depth);
    }
}
