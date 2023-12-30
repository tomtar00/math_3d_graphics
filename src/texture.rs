use glow::*;
use image::*;

pub struct TextureData {
    width: u32,
    height: u32,
    img: Vec<u8>,
}

pub fn load_texture(path: &str) -> TextureData {
    let img = image::open(path).expect("Failed to load texture");
    let img = img.flipv();
    let (width, height) = img.dimensions();
    let img = img.into_rgba8();
    let img = img.into_raw();
    TextureData{width, height, img}
}
pub unsafe fn create_texture(gl: &glow::Context, texture_data: &TextureData) -> NativeTexture {
    let texture = gl.create_texture().expect("Failed to create texture");
    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
    gl.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        glow::RGBA as i32,
        texture_data.width as i32,
        texture_data.height as i32,
        0,
        glow::RGBA,
        glow::UNSIGNED_BYTE,
        Some(&texture_data.img),
    );
    gl.generate_mipmap(glow::TEXTURE_2D);
    texture
}
