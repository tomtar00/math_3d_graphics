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

pub fn projection_matrix(width: i32, height: i32) -> Mat4 {
    Mat4::perspective_rh_gl(45.0, width as f32 / height as f32, 0.1, 100.0)
}
pub fn view_matrix(position: Vec3, center: Vec3) -> Mat4 {
    glam::Mat4::look_at_rh(
        position,
        center,
        vec3(0.0, 1.0, 0.0),
    )
}
pub fn model_matrix(transform: &Transform) -> Mat4 {
    // use glam
    let translation = Mat4::from_translation(transform.position);
    let rotation = Mat4::from_rotation_x(transform.rotation.x.to_radians())
        * Mat4::from_rotation_y(transform.rotation.y.to_radians())
        * Mat4::from_rotation_z(transform.rotation.z.to_radians());
    let scale = Mat4::from_scale(transform.scale);
    translation * rotation * scale
}