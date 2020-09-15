#![allow(dead_code)]

use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::{perspective, vec3, Deg, EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};
use gl::types::*;
use glfw::Context;

use crate::c_str;
use crate::shared::{load_texture, process_events, process_input, Camera, Shader};

// settings
const SCR_WIDTH: u32 = 480;
const SCR_HEIGHT: u32 = 320;

pub fn main_2_5_1() {
    let mut camera = Camera { position: Point3::new(0.0, 0.0, 3.0), ..Camera::default() };

    let mut first_mouse = true;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.0;

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.0;

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

    let (light_shader, vbo, cube_vao, light_vao, diffuse_map, specular_map, cube_pos) = unsafe {
        // configure global opengl state
        gl::Enable(gl::DEPTH_TEST);

        // build and compile our shader program
        // ------------------------------------
        let light_shader = Shader::new(
            "src/tutorial/_2_lighting/shaders/5.1.light_casters.vsh",
            "src/tutorial/_2_lighting/shaders/5.1.light_casters.fsh",
        );

        // setup vertex data
        // -----------------
        let vertices: [f32; 288] = [
            // positions       // normals        // texture coords
            -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0, 0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 0.0, 0.5, 0.5, -0.5, 0.0,
            0.0, -1.0, 1.0, 1.0, 0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0, //
            -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0, 0.0,
            1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0, -0.5, -0.5,
            0.5, 0.0, 0.0, 1.0, 0.0, 0.0, //
            -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0, -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, 1.0, 1.0, -0.5, -0.5, -0.5,
            -1.0, 0.0, 0.0, 0.0, 1.0, -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0, -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, 0.0,
            0.0, -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0, //
            0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0, 0.5, -0.5, -0.5, 1.0, 0.0,
            0.0, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5,
            0.5, 1.0, 0.0, 0.0, 1.0, 0.0, //
            -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 1.0, 1.0, 0.5, -0.5, 0.5, 0.0,
            -1.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 0.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0, //
            -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 1.0, 0.5, 0.5, 0.5, 0.0, 1.0,
            0.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0, -0.5, 0.5,
            -0.5, 0.0, 1.0, 0.0, 0.0, 1.0,
        ];

        let cube_pos: [Vector3<f32>; 10] = [
            vec3(0.0, 0.0, 0.0),
            vec3(2.0, 5.0, -15.0),
            vec3(-1.5, -2.2, -2.5),
            vec3(-3.8, -2.0, -12.3),
            vec3(2.4, -0.4, -3.5),
            vec3(-1.7, 3.0, -7.5),
            vec3(1.3, -2.0, -2.5),
            vec3(1.5, 2.0, -2.5),
            vec3(1.5, 0.2, -1.5),
            vec3(-1.3, 1.0, -1.5),
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

        let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // normal attribute
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);
        // texture coord
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(2);

        let mut light_vao = 0;
        gl::GenVertexArrays(1, &mut light_vao);
        gl::BindVertexArray(light_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        // load textures
        // -------------
        let diffuse_map = load_texture("resources/textures/container2.png");
        let specular_map = load_texture("resources/textures/container2_specular.png");

        // shader configuration
        // --------------------
        light_shader.use_program();
        light_shader.set_int(c_str!("material.diffuse"), 0);
        light_shader.set_int(c_str!("material.specular"), 1);

        (light_shader, vbo, cube_vao, light_vao, diffuse_map, specular_map, cube_pos)
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
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            light_shader.use_program();
            light_shader.set_vec3(c_str!("light.direction"), -0.2, -1.0, -0.3);
            light_shader.set_vector3(c_str!("viewPos"), &camera.position.to_vec());

            // light properties
            light_shader.set_vec3(c_str!("light.ambient"), 0.2, 0.2, 0.2);
            light_shader.set_vec3(c_str!("light.diffuse"), 0.5, 0.5, 0.5);
            light_shader.set_vec3(c_str!("light.specular"), 1.0, 1.0, 1.0);

            // material properties
            light_shader.set_float(c_str!("material.shininess"), 32.0);

            // view/projection transformations
            let projection: Matrix4<f32> =
                perspective(Deg(camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.get_view_matrix();
            light_shader.set_mat4(c_str!("projection"), &projection);
            light_shader.set_mat4(c_str!("view"), &view);

            // world transformation
            light_shader.set_mat4(c_str!("model"), &Matrix4::<f32>::identity());

            // bind diffuse map
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, diffuse_map);
            // bind specular map
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, specular_map);

            // render
            gl::BindVertexArray(cube_vao);
            for (i, position) in cube_pos.iter().enumerate() {
                let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
                let angle = 20.0 * i as f32;

                model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
                light_shader.set_mat4(c_str!("model"), &model);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            // no lamp here
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
