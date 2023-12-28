use glow::*;
use image::*;

pub fn create_texture(gl: &glow::Context, path: &str) -> NativeTexture {
    let img = image::open(path).expect("Failed to load texture");
    let img = img.flipv();
    let (width, height) = img.dimensions();
    let img = img.into_rgba8();
    let img = img.into_raw();
    unsafe {
        let texture = gl.create_texture().expect("Failed to create texture");
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            width as i32,
            height as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(&img),
        );
        gl.generate_mipmap(glow::TEXTURE_2D);
        texture
    }
}