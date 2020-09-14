use crate::shared::{Camera, process_input, process_events, Shader, Model};
use cgmath::{Point3, Matrix4, perspective, Deg, vec3};
use glfw::Context;
use std::ffi::CStr;

use crate::c_str;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_3_1() {
    let mut camera = Camera {
        position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    let mut first_mouse = true;
    let mut last_x = SCR_WIDTH as f32 / 2.0;
    let mut last_y = SCR_HEIGHT as f32 / 2.0;

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

    let (shader, our_model) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // build and compile shaders
        // -------------------------
        let shader = Shader::new(
            "src/tutorial/_3_model_loading/shaders/1.model_loading.vsh",
            "src/tutorial/_3_model_loading/shaders/1.model_loading.fsh",
        );

        // load models
        // -----------
        let our_model = Model::new("resources/objects/nanosuit/nanosuit.obj");

        // draw in wireframe
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader, our_model)
    };

    // render loop
    // -----------
    while !window.should_close() {
        // per-frame tie logic
        // -------------------
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
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // don't forget to enable shader before setting uniforms
            shader.use_program();

            // view/projection transformations
            let projection: Matrix4<f32> =
                perspective(Deg(camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.get_view_matrix();
            shader.set_mat4(c_str!("projection"), &projection);
            shader.set_mat4(c_str!("view"), &view);

            // render the loaded model
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -1.75, 0.0)); // translate it down so it's at the center of the scene
            model = model * Matrix4::from_scale(0.2);
            shader.set_mat4(c_str!("model"), &model);
            our_model.draw(&shader);
        }

        // glfw:: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // --------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}
