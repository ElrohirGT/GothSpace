use nalgebra_glm::{Vec2, Vec3};
use tobj;

use crate::vertex::Vertex;

pub type Obj = Vec<Vertex>;

pub fn load_objs(filename: &str) -> Result<Vec<Obj>, tobj::LoadError> {
    let (models, _) = tobj::load_obj(
        filename,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    )?;

    let objs = models
        .into_iter()
        .map(|m| m.mesh)
        .map(|mesh| {
            let vertices: Vec<Vec3> = mesh
                .positions
                .chunks(3)
                .map(|v| Vec3::new(v[0], v[1], v[2]))
                .collect();

            let normals: Vec<Vec3> = mesh
                .normals
                .chunks(3)
                .map(|n| Vec3::new(n[0], n[1], n[2]))
                .collect();

            let texcoords: Vec<Vec2> = mesh
                .texcoords
                .chunks(2)
                .map(|t| Vec2::new(t[0], t[1]))
                .collect();

            let indices: Vec<usize> = mesh.indices.iter().map(|idx| *idx as usize).collect();
            get_vertex_array(indices, vertices, normals, texcoords)
        })
        .collect();

    Ok(objs)
}

fn get_vertex_array(
    indices: Vec<usize>,
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords: Vec<Vec2>,
) -> Vec<Vertex> {
    (indices.iter().map(|idx| {
        let position = *vertices.get(*idx).unwrap();
        let normal = *normals.get(*idx).unwrap_or(&Vec3::new(0.4, 0.3, 0.3));
        let tex_cords = *texcoords.get(*idx).unwrap_or(&Vec2::new(0.0, 0.0));
        Vertex::new(position, normal, tex_cords)
    }))
    .collect()
}
