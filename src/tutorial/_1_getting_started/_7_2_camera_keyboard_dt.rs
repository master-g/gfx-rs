#![allow(dead_code)]

use std::ffi::CStr;
use std::sync::mpsc::Receiver;

use cgmath::{Deg, InnerSpace, Matrix, Matrix4, perspective, Point3, vec3, Vector3};
use glfw::{Action, Context, Key};

use crate::c_str;
use crate::shared::Shader;
use crate::tutorial::{TutorialTexture, TutorialTriangle};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

// camera
const CAMERA_FRONT: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 0.0,
    z: -1.0,
};

const CAMERA_UP: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub fn main_1_7_2() {
    let mut camera_pos = Point3::new(0.0, 0.0, 3.0);
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
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader, geometry, cube_pos, texture1, texture2, loc_model, loc_view) = unsafe {
        gl::Enable(gl::DEPTH_TEST);

        let shader = Shader::new(
            "src/tutorial/_1_getting_started/shaders/7.2.camera.vsh",
            "src/tutorial/_1_getting_started/shaders/7.2.camera.fsh",
        );

        let geometry = TutorialTriangle::new_xyzuv(vec![
            //
            -0.5, -0.5, -0.5, 0.0, 0.0,
            0.5, -0.5, -0.5, 1.0, 0.0,
            0.5, 0.5, -0.5, 1.0, 1.0,
            0.5, 0.5, -0.5, 1.0, 1.0,
            -0.5, 0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 0.0,

            //
            -0.5, -0.5, 0.5, 0.0, 0.0,
            0.5, -0.5, 0.5, 1.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 1.0,
            0.5, 0.5, 0.5, 1.0, 1.0,
            -0.5, 0.5, 0.5, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,

            //
            -0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, 0.5, -0.5, 1.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, 0.5, 0.5, 1.0, 0.0,

            //
            0.5, 0.5, 0.5, 1.0, 0.0,
            0.5, 0.5, -0.5, 1.0, 1.0,
            0.5, -0.5, -0.5, 0.0, 1.0,
            0.5, -0.5, -0.5, 0.0, 1.0,
            0.5, -0.5, 0.5, 0.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 0.0,

            //
            -0.5, -0.5, -0.5, 0.0, 1.0,
            0.5, -0.5, -0.5, 1.0, 1.0,
            0.5, -0.5, 0.5, 1.0, 0.0,
            0.5, -0.5, 0.5, 1.0, 0.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,

            //
            -0.5, 0.5, -0.5, 0.0, 1.0,
            0.5, 0.5, -0.5, 1.0, 1.0,
            0.5, 0.5, 0.5, 1.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, 0.5, 0.5, 0.0, 0.0,
            -0.5, 0.5, -0.5, 0.0, 1.0
        ]);

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
            vec3(-1.3, 1.0, -1.5)
        ];

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
        let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
        gl::UniformMatrix4fv(loc_proj, 1, gl::FALSE, projection.as_ptr());

        (shader, geometry, cube_pos, texture1, texture2, loc_model, loc_view)
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
        process_events_no_input(&events);

        // input
        // -----
        _process_input(&mut window, delta_time, &mut camera_pos);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            texture1.bind();
            texture2.bind();

            shader.use_program();

            // camera/view transformation
            let view: Matrix4<f32> = Matrix4::look_at(camera_pos, camera_pos + CAMERA_FRONT, CAMERA_UP);
            gl::UniformMatrix4fv(loc_view, 1, gl::FALSE, &view[0][0]);

            for (i, position) in cube_pos.iter().enumerate() {
                let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
                let angle = 20.0 * i as f32;
                model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
                gl::UniformMatrix4fv(loc_model, 1, gl::FALSE, model.as_ptr());

                geometry.draw();
            }
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events_no_input(events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            _ => {}
        }
    }
}

fn _process_input(window: &mut glfw::Window, delta_time: f32, camera_pos: &mut Point3<f32>) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }

    let camera_speed = 2.5 * delta_time;
    if window.get_key(Key::W) == Action::Press {
        *camera_pos += camera_speed * CAMERA_FRONT;
    }
    if window.get_key(Key::S) == Action::Press {
        *camera_pos += -(camera_speed * CAMERA_FRONT);
    }
    if window.get_key(Key::A) == Action::Press {
        *camera_pos += -(CAMERA_FRONT.cross(CAMERA_UP).normalize() * camera_speed);
    }
    if window.get_key(Key::D) == Action::Press {
        *camera_pos += CAMERA_FRONT.cross(CAMERA_UP).normalize() * camera_speed;
    }
}
