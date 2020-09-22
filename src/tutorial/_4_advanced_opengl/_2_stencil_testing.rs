use crate::shared::{load_texture, process_events, process_input, Camera, Shader};
use cgmath::{perspective, vec3, Deg, Matrix4, Point3, SquareMatrix};
use glfw::Context;
use std::ffi::CStr;

use crate::c_str;
use crate::tutorial::internal::TutorialGeometry;

// settings
const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

pub fn main_4_2() {
    let mut camera = Camera { position: Point3::new(0.0, 0.0, 3.0), ..Camera::default() };

    let mut first_mouse = true;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.0;

    // timing
    let mut delta_time: f32; // time between current frame and last frame
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
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    // tell GLFW to capture our mouse
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    // -------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader, shader_single_color, cube, plane, cube_texture, floor_texture) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Enable(gl::STENCIL_TEST);
        gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
        gl::StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);

        // build and compile our shader program
        // ------------------------------------
        let shader = Shader::new(
            "src/tutorial/_4_advanced_opengl/shaders/2.stencil_testing.vsh",
            "src/tutorial/_4_advanced_opengl/shaders/2.stencil_testing.fsh",
        );
        let shader_single_color = Shader::new(
            "src/tutorial/_4_advanced_opengl/shaders/2.stencil_testing.vsh",
            "src/tutorial/_4_advanced_opengl/shaders/2.stencil_single_color.fsh",
        );

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let cube = TutorialGeometry::new_xyzuv(vec![
            // positions       // texture Coords
            -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0,
            -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, //
            -0.5, -0.5, 0.5, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 1.0,
            -0.5, 0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, //
            -0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, -0.5, 1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0,
            1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, //
            0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0,
            0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, //
            -0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0,
            -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, //
            -0.5, 0.5, -0.5, 0.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, 0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0,
        ]);

        let plane = TutorialGeometry::new_xyzuv(vec![
            // positions       // texture Coords (note we set these higher than 1 (together with GL_REPEAT as texture wrapping mode). this will cause the floor texture to repeat)
            5.0, -0.5, 5.0, 2.0, 0.0, -5.0, -0.5, 5.0, 0.0, 0.0, -5.0, -0.5, -5.0, 0.0, 2.0, //
            5.0, -0.5, 5.0, 2.0, 0.0, -5.0, -0.5, -5.0, 0.0, 2.0, 5.0, -0.5, -5.0, 2.0, 2.0,
        ]);

        // load textures
        // -------------
        let cube_texture = load_texture("resources/textures/marble.jpg");
        let floor_texture = load_texture("resources/textures/metal.png");

        // shader configuration
        // --------------------
        shader.use_program();
        shader.set_int(c_str!("texture1"), 0);

        (shader, shader_single_color, cube, plane, cube_texture, floor_texture)
    };

    // render loop
    // -----------
    while !window.should_close() {
        // per-frame time logic
        // --------------------
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // events
        // ------
        process_events(&events, &mut first_mouse, &mut last_x, &mut last_y, &mut camera);

        // input
        // -----
        process_input(&mut window, delta_time, &mut camera);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            // set uniforms
            shader_single_color.use_program();
            let mut model: Matrix4<f32>;
            let view = camera.get_view_matrix();
            let projection: Matrix4<f32> =
                perspective(Deg(camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            shader_single_color.set_mat4(c_str!("view"), &view);
            shader_single_color.set_mat4(c_str!("projection"), &projection);

            shader.use_program();
            shader.set_mat4(c_str!("view"), &view);
            shader.set_mat4(c_str!("projection"), &projection);

            // draw floor as normal, but don't write the floor to the stencil buffer, we only care about the containers. We set its mask to 0x00 to not write to the stencil buffer.
            gl::StencilMask(0x00);
            // floor
            plane.bind();
            gl::BindTexture(gl::TEXTURE_2D, floor_texture);
            shader.set_mat4(c_str!("model"), &Matrix4::identity());
            plane.draw();
            // 1st. render pass, draw objects as normal, writing to the stencil buffer
            // -----------------------------------------------------------------------
            gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
            gl::StencilMask(0xFF);
            // cubes
            cube.bind();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, cube_texture);
            model = Matrix4::from_translation(vec3(-1.0, 0.0, -1.0));
            shader.set_mat4(c_str!("model"), &model);
            cube.draw();
            model = Matrix4::from_translation(vec3(2.0, 0.0, 0.0));
            shader.set_mat4(c_str!("model"), &model);
            cube.draw();

            // 2nd. render pass: now draw slightly scaled versions of the objects. this time disabling stencil writing.
            // Because the stencil buffer is now filled with several 1s. The parts of the buffer that are 1 are not drawn, thus only drawing
            // the objects' size differences, making it look like borders.
            // -----------------------------------------------------------------------------------------------------------------------------
            gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
            gl::StencilMask(0x00);
            gl::Disable(gl::DEPTH_TEST);
            shader_single_color.use_program();
            let scale = 1.1;
            // cubes
            cube.bind();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, cube_texture);
            model = Matrix4::from_translation(vec3(-1.0, 0.0, -1.0));
            model = model * Matrix4::from_scale(scale);
            shader.set_mat4(c_str!("model"), &model);
            cube.draw();
            model = Matrix4::from_translation(vec3(2.0, 0.0, 0.0));
            model = model * Matrix4::from_scale(scale);
            shader.set_mat4(c_str!("model"), &model);
            cube.draw();

            gl::StencilMask(0xFF);
            gl::Enable(gl::DEPTH_TEST);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
}
