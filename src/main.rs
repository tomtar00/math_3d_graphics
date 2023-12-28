use glow::*;

mod window;
mod buffers;
mod shader;
mod texture;

fn main() {
    // create window
    let mut window = window::Window::new(800, 600);

    // vertex and index buffer for 3d cube
    let vertices: [f32; 64] = [
        // positions     // colors      // texture coords
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0, // bottom right
        -0.5, -0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom left
        -0.5, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, // top left
        0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 0.0, 1.0, // top right
        0.5, -0.5, 0.5, 1.0, 0.0, 1.0, 1.0, 1.0, // bottom right
        -0.5, -0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 0.0, // bottom left
        -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 0.0, // top left
        0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 1.0, // top right
    ];
    let indices: [u32; 36] = [
        // back face
        0, 1, 2, 2, 3, 0, // front face
        4, 5, 6, 6, 7, 4, // left face
        5, 1, 2, 2, 6, 5, // right face
        4, 0, 3, 3, 7, 4, // top face
        7, 6, 2, 2, 3, 7, // bottom face
        4, 5, 1, 1, 0, 4,
    ];

    let vertex_source = r#"#version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aColor;
        layout (location = 2) in vec2 aTexCoord;

        out vec3 Color;
        out vec2 TexCoord;

        void main()
        {
            gl_Position = vec4(aPos, 1.0);
            Color = aColor;
            TexCoord = aTexCoord;
        }"#;

    let fragment_source = r#"#version 330 core
        in vec3 Color;
        in vec2 TexCoord;

        uniform sampler2D ourTexture;
        out vec4 FragColor;

        void main()
        {
            FragColor = texture(ourTexture, TexCoord) * vec4(Color, 1.0);
        }"#;

    unsafe {
        let gl = glow::Context::from_loader_function(|s| window.ptr.get_proc_address(s) as *const _);

        let vertex_buffer = buffers::create_vertex_buffer(&gl, &vertices);
        let index_buffer = buffers::create_index_buffer(&gl, &indices);
        let buffer_layout = buffers::BufferLayout::new(&[
            buffers::BufferAttribute { name: "aPos", size: 3, offset: 0 },
            buffers::BufferAttribute { name: "aColor", size: 3, offset: 3 },
            buffers::BufferAttribute { name: "aTexCoord", size: 2, offset: 6 },
        ]);
        let vertex_array = buffers::create_vertex_array(&gl, &vertex_buffer, &index_buffer, &buffer_layout);
        let shader_program = shader::create_shader(&gl, vertex_source, fragment_source);
        let texture = texture::create_texture(&gl, "res/crate.jpeg");

        while !window.ptr.should_close() {
            window.glfw.poll_events();

            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            gl.use_program(Some(shader_program));
            gl.bind_vertex_array(Some(vertex_array));
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.draw_elements(glow::TRIANGLES, 36, glow::UNSIGNED_INT, 0);

            let error = gl.get_error();
            if error != glow::NO_ERROR {
                panic!("GL error: {}", error);
            }
            window.swap_buffers();
        }
    }
}
