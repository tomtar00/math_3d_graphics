use glow::*;
use imgui_glfw_rs::imgui as ImGui;
use imgui_glfw_rs::ImguiGLFW;

mod buffers;
mod math;
mod shader;
mod texture;
mod window;

fn main() {
    // create window

    // vertex and index buffer for 3d cube
    let vertices: [f32; 64] = [
        // positions     // colors      // texture coords
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, // top left
        0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 1.0, // top right
        0.5, -0.5, 0.5, 1.0, 0.0, 1.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 1.0, // top left
        0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, // top right
    ];
    let indices: [u32; 36] = [
        // back face
        0, 1, 2, 2, 3, 0, 
        // front face
        4, 5, 6, 6, 7, 4, 
        // left face
        5, 1, 2, 2, 6, 5, 
        // right face
        4, 0, 3, 3, 7, 4, 
        // top face
        7, 6, 2, 2, 3, 7, 
        // bottom face
        4, 5, 1, 1, 0, 4,
    ];

    let vertex_source = r#"#version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aColor;
        layout (location = 2) in vec2 aTexCoord;

        out vec3 Color;
        out vec2 TexCoord;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        void main()
        {
            gl_Position = projection * view * model * vec4(aPos, 1.0);
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

    let (mut width, mut height): (i32, i32) = (800, 600);
    let mut window = window::Window::new(width as u32, height as u32, "3D Transformations");

    unsafe {
        let gl =
            glow::Context::from_loader_function(|s| window.ptr.get_proc_address(s) as *const _);
        let mut imgui = ImGui::Context::create();
        let mut imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window.ptr);

        let vertex_buffer = buffers::create_vertex_buffer(&gl, &vertices);
        let index_buffer = buffers::create_index_buffer(&gl, &indices);
        let buffer_layout = buffers::BufferLayout::new(&[
            buffers::BufferAttribute {
                name: "aPos",
                size: 3,
                offset: 0,
            },
            buffers::BufferAttribute {
                name: "aColor",
                size: 3,
                offset: 3,
            },
            buffers::BufferAttribute {
                name: "aTexCoord",
                size: 2,
                offset: 6,
            },
        ]);
        let vertex_array =
            buffers::create_vertex_array(&gl, &vertex_buffer, &index_buffer, &buffer_layout);
        let shader_program = shader::create_shader(&gl, vertex_source, fragment_source);
        let texture = texture::create_texture(&gl, "res/crate.jpeg");

        let mut transform = math::Transform::new();
        let view = math::view_matrix(glam::vec3(0.0, 2.0, 5.0), glam::vec3(0.0, 0.0, 0.0));

        gl.enable(glow::DEPTH_TEST);

        gl.clear_color(0.2, 0.3, 0.3, 1.0);
        while !window.ptr.should_close() {
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            let ui = imgui_glfw.frame(&mut window.ptr, &mut imgui);

            ui.window(ImGui::im_str!("Transform"))
                .size([300.0, 100.0], ImGui::Condition::FirstUseEver)
                .build(|| {
                    ui.text(format!("Window: {}x{}", width, height));

                    let mut position = transform.position.to_array();
                    if ui
                        .drag_float3(ImGui::im_str!("Position"), &mut position)
                        .build()
                    {
                        transform.position = glam::Vec3::from(position);
                    }

                    let mut rotation = transform.rotation.to_array();
                    if ui
                        .drag_float3(ImGui::im_str!("Rotation"), &mut rotation)
                        .build()
                    {
                        transform.rotation = glam::Vec3::from(rotation);
                    }

                    let mut scale = transform.scale.to_array();
                    if ui.drag_float3(ImGui::im_str!("Scale"), &mut scale).build() {
                        transform.scale = glam::Vec3::from(scale);
                    }
                });

            gl.use_program(Some(shader_program));

            let location = gl.get_uniform_location(shader_program, "model");
            gl.uniform_matrix_4_f32_slice(
                location.as_ref(),
                false,
                &math::model_matrix(&transform).to_cols_array(),
            );
            let location = gl.get_uniform_location(shader_program, "view");
            gl.uniform_matrix_4_f32_slice(location.as_ref(), false, &view.to_cols_array());
            let projection = math::projection_matrix(width, height);
            let location = gl.get_uniform_location(shader_program, "projection");
            gl.uniform_matrix_4_f32_slice(location.as_ref(), false, &projection.to_cols_array());

            gl.bind_vertex_array(Some(vertex_array));
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.draw_elements(glow::TRIANGLES, 36, glow::UNSIGNED_INT, 0);

            imgui_glfw.draw(ui, &mut window.ptr);

            let error = gl.get_error();
            if error != glow::NO_ERROR {
                panic!("GL error: {}", error);
            }

            window.swap_buffers();

            window.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&window.events) {
                imgui_glfw.handle_event(&mut imgui, &event);
            }
            (width, height) = window.ptr.get_framebuffer_size();
        }
    }
}
