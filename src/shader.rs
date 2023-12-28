use glow::*;

pub unsafe fn create_shader(gl: &glow::Context, vertex_source: &str, fragment_source: &str) -> NativeProgram {
    let vertex_shader = gl
        .create_shader(glow::VERTEX_SHADER)
        .expect("Failed to create vertex shader");
    gl.shader_source(vertex_shader, vertex_source);
    gl.compile_shader(vertex_shader);
    if !gl.get_shader_compile_status(vertex_shader) {
        panic!("{}", gl.get_shader_info_log(vertex_shader));
    }

    let fragment_shader = gl
        .create_shader(glow::FRAGMENT_SHADER)
        .expect("Failed to create fragment shader");
    gl.shader_source(fragment_shader, fragment_source);
    gl.compile_shader(fragment_shader);
    if !gl.get_shader_compile_status(fragment_shader) {
        panic!("{}", gl.get_shader_info_log(fragment_shader));
    }

    let shader_program = gl
        .create_program()
        .expect("Failed to create shader program");
    gl.attach_shader(shader_program, vertex_shader);
    gl.attach_shader(shader_program, fragment_shader);
    gl.link_program(shader_program);
    if !gl.get_program_link_status(shader_program) {
        panic!("{}", gl.get_program_info_log(shader_program));
    }

    gl.delete_shader(vertex_shader);
    gl.delete_shader(fragment_shader);
    shader_program
}
