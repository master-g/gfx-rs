#![allow(dead_code)]

use std::ffi::CStr;
use std::sync::mpsc::Receiver;

use cgmath::{Deg, InnerSpace, Matrix, Matrix4, perspective, Point3, vec3, Vector3};
use glfw::{Action, Context, Key};

use crate::c_str;
use crate::shared::Shader;
use crate::tutorial::{TutorialTexture, TutorialGeometry};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const CAMERA_UP: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub fn main_1_7_3() {
    let mut camera_pos = Point3::new(0.0, 0.0, 3.0);
    let mut camera_front: Vector3<f32> = Vector3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    let mut first_mouse = true;
    let mut yaw: f32 = -90.0;
    let mut pitch: f32 = 0.0;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.0;
    let mut fov: f32 = 45.0;

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
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    // tell GLFW to capture our mouse
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader, geometry, cube_pos, texture1, texture2, loc_model, loc_view, loc_proj) = unsafe {
        gl::Enable(gl::DEPTH_TEST);

        let shader = Shader::new(
            "src/tutorial/_1_getting_started/shaders/7.3.camera.vsh",
            "src/tutorial/_1_getting_started/shaders/7.3.camera.fsh",
        );

        let geometry = TutorialGeometry::new_xyzuv(vec![
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

        (shader, geometry, cube_pos, texture1, texture2, loc_model, loc_view, loc_proj)
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
        _process_events(&events,
                        &mut first_mouse,
                        &mut last_x,
                        &mut last_y,
                        &mut yaw,
                        &mut pitch,
                        &mut camera_front,
                        &mut fov);

        // input
        // -----
        _process_input(&mut window, delta_time, &mut camera_pos, &mut camera_front);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            texture1.bind();
            texture2.bind();

            shader.use_program();

            // projection matrix
            let projection: Matrix4<f32> = perspective(Deg(fov), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            gl::UniformMatrix4fv(loc_proj, 1, gl::FALSE, projection.as_ptr());

            // camera/view transformation
            let view: Matrix4<f32> = Matrix4::look_at(camera_pos, camera_pos + camera_front, CAMERA_UP);
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

fn _process_events(events: &Receiver<(f64, glfw::WindowEvent)>,
                   first_mouse: &mut bool,
                   last_x: &mut f32,
                   last_y: &mut f32,
                   yaw: &mut f32,
                   pitch: &mut f32,
                   camera_front: &mut Vector3<f32>,
                   fov: &mut f32) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                let (xpos, ypos) = (xpos as f32, ypos as f32);
                if *first_mouse {
                    *last_x = xpos;
                    *last_y = ypos;
                    *first_mouse = false;
                }

                let mut xoffset = xpos - *last_x;
                let mut yoffset = *last_y - ypos;
                *last_x = xpos;
                *last_y = ypos;

                let sensitivity: f32 = 0.1;
                xoffset *= sensitivity;
                yoffset *= sensitivity;

                *yaw += xoffset;
                *pitch += yoffset;

                if *pitch > 89.0 {
                    *pitch = 89.0;
                }
                if *pitch < -89.0 {
                    *pitch = -89.0;
                }

                let front = Vector3 {
                    x: yaw.to_radians().cos() * pitch.to_radians().cos(),
                    y: pitch.to_radians().sin(),
                    z: yaw.to_radians().sin() * pitch.to_radians().cos(),
                };
                *camera_front = front.normalize();
            }
            glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                if *fov >= 1.0 && *fov <= 45.0 {
                    *fov -= yoffset as f32;
                }
                if *fov <= 1.0 {
                    *fov = 1.0;
                }
                if *fov >= 45.0 {
                    *fov = 45.0;
                }
            }
            _ => {}
        }
    }
}

fn _process_input(window: &mut glfw::Window, delta_time: f32, camera_pos: &mut Point3<f32>, camera_front: &mut Vector3<f32>) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }

    let camera_speed = 2.5 * delta_time;
    if window.get_key(Key::W) == Action::Press {
        *camera_pos += camera_speed * *camera_front;
    }
    if window.get_key(Key::S) == Action::Press {
        *camera_pos += -(camera_speed * *camera_front);
    }
    if window.get_key(Key::A) == Action::Press {
        *camera_pos += -(camera_front.cross(CAMERA_UP).normalize() * camera_speed);
    }
    if window.get_key(Key::D) == Action::Press {
        *camera_pos += camera_front.cross(CAMERA_UP).normalize() * camera_speed;
    }
}
