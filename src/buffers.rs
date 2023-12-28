use glow::*;
use std::mem::size_of;

pub unsafe fn create_vertex_buffer(gl: &glow::Context, vertices: &[f32]) -> NativeBuffer {
    let vertex_buffer = gl.create_buffer().expect("Failed to create vertex buffer");
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
    gl.buffer_data_u8_slice(
        glow::ARRAY_BUFFER,
        &vertices.align_to::<u8>().1,
        glow::STATIC_DRAW,
    );
    vertex_buffer
}
pub unsafe fn create_index_buffer(gl: &glow::Context, indices: &[u32]) -> NativeBuffer {
    let index_buffer = gl.create_buffer().expect("Failed to create index buffer");
    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer));
    gl.buffer_data_u8_slice(
        glow::ELEMENT_ARRAY_BUFFER,
        &indices.align_to::<u8>().1,
        glow::STATIC_DRAW,
    );
    index_buffer
}

#[derive(Clone)]
pub struct BufferAttribute<'a> {
    pub name: &'a str,
    pub size: i32,
    pub offset: i32,
}
pub struct BufferLayout<'a> {
    pub stride: i32,
    pub attributes: Vec<BufferAttribute<'a>>,
}
impl<'a> BufferLayout<'a> {
    pub fn new(attributes: &[BufferAttribute<'a>]) -> BufferLayout<'a> {
        let mut stride = attributes.iter().fold(0, |acc, a| acc + a.size);
        stride *= size_of::<f32>() as i32;
        BufferLayout { stride, attributes: attributes.to_vec() }
    }
}
pub unsafe fn create_vertex_array(gl: &glow::Context, vertex_buffer: &NativeBuffer, index_buffer: &NativeBuffer, buffer_layout: &BufferLayout<'_>) -> NativeVertexArray {
    let vao = gl.create_vertex_array().expect("Failed to create VAO");
    gl.bind_vertex_array(Some(vao));
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(*vertex_buffer));
    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(*index_buffer));
    for (i, attribute) in buffer_layout.attributes.iter().enumerate() {
        gl.vertex_attrib_pointer_f32(
            i as u32,
            attribute.size as i32,
            glow::FLOAT,
            false,
            buffer_layout.stride as i32,
            attribute.offset * size_of::<f32>() as i32
        );
        gl.enable_vertex_attrib_array(i as u32);
    }
    vao
}
