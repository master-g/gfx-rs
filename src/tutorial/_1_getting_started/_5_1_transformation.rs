#![allow(dead_code)]

use std::ffi::CStr;

use cgmath::{vec3, Matrix, Matrix4, Rad, SquareMatrix};
use glfw::Context;

use crate::c_str;
use crate::shared::Shader;
use crate::tutorial::{process_events, TutorialGeometry, TutorialTexture};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_1_5_1() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw
        .create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader, triangle, texture1, texture2, transform_location) = unsafe {
        let shader = Shader::new(
            "src/tutorial/_1_getting_started/shaders/5.1.transform.vsh",
            "src/tutorial/_1_getting_started/shaders/5.1.transform.fsh",
        );

        let triangle = TutorialGeometry::new_xyzrgbuv_indices(
            vec![
                // positions       // colors        // texture coords
                0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
                0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
                -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
                -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
            ],
            vec![
                0, 1, 3, // first Triangle
                1, 2, 3, // second Triangle
            ],
        );

        // texture
        let texture1 = TutorialTexture::new("resources/textures/container.jpg", 0, false, false, false);
        let texture2 = TutorialTexture::new("resources/textures/awesomeface.png", 1, true, false, true);

        shader.use_program();
        shader.set_int(c_str!("texture1"), 0);
        shader.set_int(c_str!("texture2"), 1);
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        let transform_location = gl::GetUniformLocation(shader.id, c_str!("transform").as_ptr());

        (shader, triangle, texture1, texture2, transform_location)
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

            texture1.bind();
            texture2.bind();

            let transform = Matrix4::<f32>::identity()
                * Matrix4::<f32>::from_translation(vec3(0.5, -0.5, 0.0))
                * Matrix4::<f32>::from_angle_z(Rad(glfw.get_time() as f32));

            shader.use_program();
            gl::UniformMatrix4fv(transform_location, 1, gl::FALSE, transform.as_ptr());
            triangle.draw();
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}
