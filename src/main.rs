use glow::*;
use imgui_glfw_rs::imgui as ImGui;
use imgui_glfw_rs::ImguiGLFW;

mod buffers;
mod math;
mod mesh;
mod shader;
mod texture;
mod window;

fn main() {
    let (vertices, indices) = mesh::load_mesh("res/viking.obj");
    let texture_data = texture::load_texture("res/viking.png");

    let vertex_source = r#"#version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec2 aTexCoord;

        out vec2 TexCoord;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        void main()
        {
            gl_Position = projection * view * model * vec4(aPos, 1.0);
            TexCoord = aTexCoord;
        }"#;

    let fragment_source = r#"#version 330 core
        in vec3 Color;
        in vec2 TexCoord;

        uniform sampler2D ourTexture;
        out vec4 FragColor;

        void main()
        {
            FragColor = texture(ourTexture, TexCoord);
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
                name: "aTexCoord",
                size: 2,
                offset: 3,
            },
        ]);
        let vertex_array =
            buffers::create_vertex_array(&gl, &vertex_buffer, &index_buffer, &buffer_layout);
        let shader_program = shader::create_shader(&gl, vertex_source, fragment_source);
        let texture = texture::create_texture(&gl, &texture_data);

        let mut transform = math::Transform::new();
        transform.position = glam::vec3(0.0, 0.5, 2.5);
        transform.rotation = glam::vec3(-90.0, 0.0, 270.0);
        let mut fov = 60.0;

        let view = math::view_matrix(glam::vec3(0.0, 2.0, 5.0), glam::vec3(0.0, 0.0, 0.0));

        gl.enable(glow::DEPTH_TEST);
        gl.clear_color(0.2, 0.3, 0.3, 1.0);

        while !window.ptr.should_close() {
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            gl.viewport(0, 0, width, height);

            let fps = imgui.io().framerate;
            let ui = imgui_glfw.frame(&mut window.ptr, &mut imgui);

            ui.window(ImGui::im_str!("Transform"))
                .size([600.0, 180.0], ImGui::Condition::FirstUseEver)
                .build(|| {
                    ui.text(format!("Window: {}x{}", width, height));
                    ui.text(format!("FPS: {:.1}", fps));

                    let mut position = transform.position.to_array();
                    if ui
                        .drag_float3(ImGui::im_str!("Position"), &mut position)
                        .speed(0.1)
                        .build()
                    {
                        transform.position = glam::Vec3::from(position);
                    }

                    let mut rotation = transform.rotation.to_array();
                    if ui
                        .drag_float3(ImGui::im_str!("Rotation"), &mut rotation)
                        .speed(0.5)
                        .build()
                    {
                        transform.rotation = glam::Vec3::from(rotation);
                    }

                    let mut scale = transform.scale.to_array();
                    if ui
                        .drag_float3(ImGui::im_str!("Scale"), &mut scale)
                        .speed(0.1)
                        .build()
                    {
                        transform.scale = glam::Vec3::from(scale);
                    }

                    ui.separator();
                    ui.slider_float(ImGui::im_str!("FOV"), &mut fov, 10.0, 180.0)
                        .build();
                });

            gl.use_program(Some(shader_program));

            let location = gl.get_uniform_location(shader_program, "model");
            let model = math::model_matrix(&transform);
            gl.uniform_matrix_4_f32_slice(location.as_ref(), false, &model.to_cols_array());
            let location = gl.get_uniform_location(shader_program, "view");
            gl.uniform_matrix_4_f32_slice(location.as_ref(), false, &view.to_cols_array());
            let location = gl.get_uniform_location(shader_program, "projection");
            let projection = math::projection_matrix(fov, width, height);
            gl.uniform_matrix_4_f32_slice(location.as_ref(), false, &projection.to_cols_array());

            gl.bind_vertex_array(Some(vertex_array));
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            gl.draw_elements(glow::TRIANGLES, indices.len() as i32, glow::UNSIGNED_INT, 0);
            imgui_glfw.draw(ui, &mut window.ptr);

            let error = gl.get_error();
            if error != glow::NO_ERROR {
                println!("GL error: {}", error);
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
