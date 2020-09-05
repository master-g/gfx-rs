#![allow(dead_code)]

use std::str;

use glfw::Context;

use crate::tutorial::{process_events, TutorialShader, TutorialTriangle};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE_1: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

const FRAGMENT_SHADER_SOURCE_2: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0f, 1.0f, 0.0f, 1.0f);
    }
"#;

pub fn main_1_2_5() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (s1, s2, t1, t2) = unsafe {
        let s1 = TutorialShader::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE_1);
        let s2 = TutorialShader::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE_2);

        let t1 = TutorialTriangle::new_xyz(vec![
            -1.0, -0.5, 0.0,
            0.0, -0.5, 0.0,
            -0.5, 0.5, 0.0,
        ]);
        let t2 = TutorialTriangle::new_xyz(vec![
            0.0, -0.5, 0.0,
            1.0, -0.5, 0.0,
            0.5, 0.5, 0.0,
        ]);

        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (s1, s2, t1, t2)
    };

    // render loop
    // -----------
    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw our triangles
            s1.use_prog();
            t1.draw();
            s2.use_prog();
            t2.draw();
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}
