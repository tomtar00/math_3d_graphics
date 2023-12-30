use std::f32;
use glam::*;

pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
        }
    }
}

pub fn projection_matrix(fov: f32, width: i32, height: i32) -> Mat4 {
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/opengl-perspective-projection-matrix.html
    let near = 0.1;
    let far = 100.0;
    let aspect_ratio = width as f32 / height as f32;
    let inv_length = 1.0 / (near - far);
    let f = 1.0 / f32::tan(0.5 * fov.to_radians());
    let a = f / aspect_ratio;
    let b = (near + far) * inv_length;
    let c = (2.0 * near * far) * inv_length;
    Mat4::from_cols(
        Vec4::new(a, 0.0, 0.0, 0.0),
        Vec4::new(0.0, f, 0.0, 0.0),
        Vec4::new(0.0, 0.0, b, -1.0),
        Vec4::new(0.0, 0.0, c, 0.0)
    )
}
pub fn view_matrix(position: Vec3, center: Vec3) -> Mat4 {
    // https://www.3dgep.com/understanding-the-view-matrix/
    let dir = (center - position).normalize();
    let up = vec3(0.0, 1.0, 0.0);
    let f = dir.normalize();
    let s = f.cross(up).normalize();
    let u = s.cross(f);

    Mat4::from_cols(
        Vec4::new(s.x, u.x, -f.x, 0.0),
        Vec4::new(s.y, u.y, -f.y, 0.0),
        Vec4::new(s.z, u.z, -f.z, 0.0),
        Vec4::new(-position.dot(s), -position.dot(u), position.dot(f), 1.0)
    )
}
pub fn model_matrix(transform: &Transform) -> Mat4 {
    let translation = Mat4::from_cols(
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
        Vec4::new(transform.position.x, transform.position.y, transform.position.z, 1.0)
    );

    let cosx = transform.rotation.x.to_radians().cos();
    let sinx = transform.rotation.x.to_radians().sin();
    let rotation_x = Mat4::from_cols(
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, cosx, sinx, 0.0),
        Vec4::new(0.0, -sinx, cosx, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0)
    );
    let cosy = transform.rotation.y.to_radians().cos();
    let siny = transform.rotation.y.to_radians().sin();
    let rotation_y = Mat4::from_cols(
        Vec4::new(cosy, 0.0, -siny, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(siny, 0.0, cosy, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0)
    );
    let cosz = transform.rotation.z.to_radians().cos();
    let sinz = transform.rotation.z.to_radians().sin();
    let rotation_z = Mat4::from_cols(
        Vec4::new(cosz, sinz, 0.0, 0.0),
        Vec4::new(-sinz, cosz, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0)
    );
    let rotation = rotation_x * rotation_y * rotation_z;

    let scale = Mat4::from_cols(
        Vec4::new(transform.scale.x, 0.0, 0.0, 0.0),
        Vec4::new(0.0, transform.scale.y, 0.0, 0.0),
        Vec4::new(0.0, 0.0, transform.scale.z, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0)
    );

    translation * rotation * scale
}