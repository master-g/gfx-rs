#![allow(dead_code)]

use std::ffi::CStr;

use cgmath::{perspective, vec3, Deg, Matrix4, Point3, SquareMatrix};
use glfw::Context;

use crate::c_str;
use crate::shared::{process_events, process_input, Camera, Shader};
use crate::tutorial::TutorialGeometry;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_2_1() {
    let mut camera = Camera { position: Point3::new(0.0, 0.0, 3.0), ..Camera::default() };

    let mut first_mouse = true;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.0;

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.0;

    // lighting
    let light_pos = vec3(1.2, 1.0, 2.0);

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
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    // tell GLFW to capture our mouse
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (light_shader, lamp_shader, cube, light) = unsafe {
        // configure global opengl state
        gl::Enable(gl::DEPTH_TEST);

        // build and compile our shader program
        // ------------------------------------
        let light_shader = Shader::new(
            "src/tutorial/_2_lighting/shaders/1.colors.vsh",
            "src/tutorial/_2_lighting/shaders/1.colors.fsh",
        );
        let lamp_shader =
            Shader::new("src/tutorial/_2_lighting/shaders/1.lamp.vsh", "src/tutorial/_2_lighting/shaders/1.lamp.fsh");

        // setup vertex data
        // -----------------
        let vertices = vec![
            -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5,
            //
            -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5,
            //
            -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5,
            //
            0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5,
            //
            -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5,
            //
            -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5,
        ];
        let cube = TutorialGeometry::new_xyz(vertices.clone());
        let light = TutorialGeometry::new_xyz(vertices.clone());

        (light_shader, lamp_shader, cube, light)
    };

    // render loop
    // -----------
    while !window.should_close() {
        // pre-frame time logic
        // --------------------
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // events
        // -----
        process_events(&events, &mut first_mouse, &mut last_x, &mut last_y, &mut camera);

        // input
        // -----
        process_input(&mut window, delta_time, &mut camera);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            light_shader.use_program();
            light_shader.set_vec3(c_str!("objectColor"), 1.0, 0.5, 0.31);
            light_shader.set_vec3(c_str!("lightColor"), 1.0, 1.0, 1.0);

            // projection matrix
            let projection: Matrix4<f32> =
                perspective(Deg(camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.get_view_matrix();
            light_shader.set_mat4(c_str!("projection"), &projection);
            light_shader.set_mat4(c_str!("view"), &view);

            // world transformation
            let mut model = Matrix4::<f32>::identity();
            light_shader.set_mat4(c_str!("model"), &model);

            // render the cube
            cube.draw();

            // also draw the lamp object
            lamp_shader.use_program();
            lamp_shader.set_mat4(c_str!("projection"), &projection);
            lamp_shader.set_mat4(c_str!("view"), &view);
            model = Matrix4::from_translation(light_pos);
            model = model * Matrix4::from_scale(0.2);
            lamp_shader.set_mat4(c_str!("model"), &model);

            light.draw();
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}
