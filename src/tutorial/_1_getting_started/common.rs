use std::{mem, ptr};
use std::ffi::CString;
use std::os::raw::c_void;
use std::sync::mpsc::Receiver;

use gl::types::*;
use glfw::{Action, Key};

use crate::shared::check_compile_errors;

// NOTE: not the same version as in common.rs!
pub fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}

pub struct TutorialTriangle {
    vao: u32,
}

impl TutorialTriangle {
    pub unsafe fn new(vertices: [f32; 9]) -> TutorialTriangle {
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        // 1. bind the Vertex Array Object
        gl::BindVertexArray(vao);
        // 2. bind and set vertex buffer(s)
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        // 3. configure vertex attributes(s).
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other vao calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        TutorialTriangle {
            vao,
        }
    }

    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}

pub(crate) struct TutorialShader {
    prog: u32,
}

impl TutorialShader {
    pub unsafe fn new(vert: &str, frag: &str) -> TutorialShader {
        let vs = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(vert.as_bytes()).unwrap();
        gl::ShaderSource(vs, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vs);
        check_compile_errors(vs, "VERTEX");

        let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(frag.as_bytes()).unwrap();
        gl::ShaderSource(fs, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fs);
        check_compile_errors(fs, "FRAGMENT");

        let prog = gl::CreateProgram();
        gl::AttachShader(prog, vs);
        gl::AttachShader(prog, fs);
        gl::LinkProgram(prog);
        check_compile_errors(prog, "PROGRAM");
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);

        TutorialShader {
            prog,
        }
    }

    pub unsafe fn use_prog(&self) {
        gl::UseProgram(self.prog);
    }
}
