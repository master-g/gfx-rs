#![allow(dead_code)]

use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::{Deg, Matrix4, perspective, Point3, SquareMatrix, vec3};
use gl::types::*;
use glfw::Context;

use crate::c_str;
use crate::shared::{Camera, process_events, process_input, Shader};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_2_2_1() {
    let mut camera = Camera {
        position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

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

    let (light_shader, lamp_shader, vbo, cube_vao, light_vao) = unsafe {
        // configure global opengl state
        gl::Enable(gl::DEPTH_TEST);

        // build and compile our shader program
        // ------------------------------------
        let light_shader = Shader::new(
            "src/tutorial/_2_lighting/shaders/2.1.basic_lighting.vsh",
            "src/tutorial/_2_lighting/shaders/2.1.basic_lighting.fsh",
        );
        let lamp_shader = Shader::new(
            "src/tutorial/_2_lighting/shaders/2.1.lamp.vsh",
            "src/tutorial/_2_lighting/shaders/2.1.lamp.fsh",
        );

        // setup vertex data
        // -----------------
        let vertices: [f32; 216] = [
            -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 0.5,
            0.5, -0.5, 0.0, 0.0, -1.0, -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, //
            -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.5, 0.5, 0.5,
            0.0, 0.0, 1.0, -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, //
            -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, -0.5,
            -0.5, -0.5, -1.0, 0.0, 0.0, -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, //
            0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.5, -0.5,
            -0.5, 1.0, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.0, //
            -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 0.5,
            -0.5, 0.5, 0.0, -1.0, 0.0, -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, //
            -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.5, 0.5, 0.5,
            0.0, 1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.0,
        ];

        let (mut vbo, mut cube_vao) = (0, 0);
        gl::GenVertexArrays(1, &mut cube_vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(cube_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // normal attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        let mut light_vao = 0;
        gl::GenVertexArrays(1, &mut light_vao);
        gl::BindVertexArray(light_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        (light_shader, lamp_shader, vbo, cube_vao, light_vao)
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
            light_shader.set_vector3(c_str!("lightPos"), &light_pos);

            // view/projection matrix
            let projection: Matrix4<f32> =
                perspective(Deg(camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.get_view_matrix();
            light_shader.set_mat4(c_str!("projection"), &projection);
            light_shader.set_mat4(c_str!("view"), &view);

            // world transformation
            let mut model = Matrix4::<f32>::identity();
            light_shader.set_mat4(c_str!("model"), &model);

            // render the cube
            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // also draw the lamp object
            lamp_shader.use_program();
            lamp_shader.set_mat4(c_str!("projection"), &projection);
            lamp_shader.set_mat4(c_str!("view"), &view);
            model = Matrix4::from_translation(light_pos);
            model = model * Matrix4::from_scale(0.2);
            lamp_shader.set_mat4(c_str!("model"), &model);

            // render the lame
            gl::BindVertexArray(light_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &cube_vao);
        gl::DeleteVertexArrays(1, &light_vao);
        gl::DeleteBuffers(1, &vbo);
    }
}
