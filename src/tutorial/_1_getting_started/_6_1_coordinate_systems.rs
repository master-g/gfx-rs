#![allow(dead_code)]

use std::ffi::CStr;

use cgmath::{Deg, Matrix, Matrix4, perspective, vec3};
use glfw::Context;

use crate::c_str;
use crate::shared::Shader;
use crate::tutorial::{process_events, TutorialTexture, TutorialGeometry};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_1_6_1() {
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

    let (shader, triangle, texture1, texture2, loc_model, loc_view, loc_proj) = unsafe {
        let shader = Shader::new(
            "src/tutorial/_1_getting_started/shaders/6.1.coordinate_systems.vsh",
            "src/tutorial/_1_getting_started/shaders/6.1.coordinate_systems.fsh",
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

        let loc_model = gl::GetUniformLocation(shader.id, c_str!("model").as_ptr());
        let loc_view = gl::GetUniformLocation(shader.id, c_str!("view").as_ptr());
        let loc_proj = gl::GetUniformLocation(shader.id, c_str!("projection").as_ptr());

        (shader, triangle, texture1, texture2, loc_model, loc_view, loc_proj)
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

            let model: Matrix4<f32> = Matrix4::from_angle_x(Deg(-55.));
            let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -3.));
            let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);

            shader.use_program();
            gl::UniformMatrix4fv(loc_model, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(loc_view, 1, gl::FALSE, &view[0][0]);
            gl::UniformMatrix4fv(loc_proj, 1, gl::FALSE, projection.as_ptr());

            triangle.draw();
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}
