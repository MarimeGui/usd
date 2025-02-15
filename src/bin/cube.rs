use std::{fs::File, io::Write};

use usd::{
    prim::{set_face_vertex_counts, set_face_vertex_indices, set_points, Prim, PrimType},
    stage::Stage,
    Axis,
};

fn main() {
    // Mesh data
    let vertices = vec![
        [1.0, 1.0, -1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, -1.0, -1.0],
        [1.0, -1.0, -1.0],
        [1.0, 1.0, 1.0],
        [-1.0, 1.0, 1.0],
        [-1.0, -1.0, 1.0],
        [1.0, -1.0, 1.0],
    ];
    let indices = vec![
        2, 1, 0, 3, 4, 5, 6, 7, 7, 6, 2, 3, 6, 5, 1, 2, 5, 4, 0, 1, 4, 7, 3, 0,
    ];
    let vertex_count = vec![4; indices.len() / 4];

    // Setup Mesh prim
    let mut mesh_prim = Prim::new(PrimType::Mesh, "CubeMesh".to_string());
    set_points(&mut mesh_prim, vertices);
    set_face_vertex_indices(&mut mesh_prim, indices);
    set_face_vertex_counts(&mut mesh_prim, vertex_count);

    // Setup Object prim (not really necessary here)
    let mut object_prim = Prim::new(PrimType::Xform, "Cube".to_string());
    object_prim.push_sub(mesh_prim);

    // Setup stage
    let mut stage = Stage::default();
    stage.push_sub(object_prim);
    stage.set_up_axis(Axis::Z);
    stage.set_scale(1.0);
    stage.set_default_prim("Cube".to_string());

    File::create("cube.usda")
        .unwrap()
        .write_all(stage.serialize_to_text().as_bytes())
        .unwrap();
}
