use tobj::*;

pub fn load_mesh(path: &str) -> (Vec<f32>, Vec<u32>) {
    let (models, _) = load_obj(path, &tobj::LoadOptions::default()).unwrap();
    let mut vertices: Vec<f32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    for m in models {
        let mesh = m.mesh;
        for i in 0..mesh.positions.len() / 3 {
            vertices.push(mesh.positions[i * 3]);
            vertices.push(mesh.positions[i * 3 + 1]);
            vertices.push(mesh.positions[i * 3 + 2]);
            vertices.push(mesh.texcoords[i * 2]); 
            vertices.push(mesh.texcoords[i * 2 + 1]);
        }
        for i in 0..mesh.indices.len() {
            indices.push(mesh.indices[i]);
        }
    }
    (vertices, indices)
}