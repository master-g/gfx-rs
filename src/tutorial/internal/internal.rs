#![allow(dead_code)]

use std::{mem, ptr};
use std::ffi::CString;
use std::os::raw::c_void;
use std::path::Path;
use std::sync::mpsc::Receiver;

use gl::types::*;
use glfw::{Action, Key};
use image::GenericImageView;

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

pub struct TutorialGeometry {
    vao: u32,
    vbo: u32,
    ebo: u32,
    elements: i32,
    primitive: u32,
}

impl TutorialGeometry {
    pub unsafe fn new_xyz(vertices: Vec<f32>) -> Self {
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

        Self {
            vao,
            vbo,
            ebo: 0,
            elements: (vertices.len() / 3) as i32,
            primitive: gl::TRIANGLES,
        }
    }

    pub unsafe fn new_xyzrgb(vertices: Vec<f32>) -> Self {
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
        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
        // position
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // color / normal
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);

        // note that this is allowed, the call to gl::VertexAttribPointer registered vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other vao calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        Self {
            vao,
            vbo,
            ebo: 0,
            elements: (vertices.len() / 6) as i32,
            primitive: gl::TRIANGLES,
        }
    }

    pub unsafe fn new_xyzuv(vertices: Vec<f32>) -> Self {
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
        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
        // position
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // texture coordinates
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);

        // note that this is allowed, the call to gl::VertexAttribPointer registered vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other vao calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        Self {
            vao,
            vbo,
            ebo: 0,
            elements: (vertices.len() / 5) as i32,
            primitive: gl::TRIANGLES,
        }
    }

    pub unsafe fn new_xyzrgbuv_indices(vertices: Vec<f32>, indices: Vec<i32>) -> Self {
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;
        let mut ebo: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        // 1. bind the Vertex Array Object
        gl::BindVertexArray(vao);
        // 2. bind and set vertex buffer(s)
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        // 3. indices
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &indices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        // 4. configure vertex attributes(s).
        let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
        // position
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // color
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);
        // texture coord
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(2);

        // note that this is allowed, the call to gl::VertexAttribPointer registered vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other vao calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        Self {
            vao,
            vbo,
            ebo,
            elements: indices.len() as i32,
            primitive: gl::TRIANGLES,
        }
    }

    pub unsafe fn set_primitive(&mut self, primitive: u32) {
        self.primitive = primitive;
    }

    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        if self.ebo != 0 {
            gl::DrawElements(self.primitive, self.elements, gl::UNSIGNED_INT, ptr::null());
        } else {
            gl::DrawArrays(self.primitive, 0, self.elements);
        }
    }
}

impl Drop for TutorialGeometry {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
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

    pub unsafe fn get_location(&self, name: &str) -> i32 {
        let c_str_name = CString::new(name.as_bytes()).unwrap();
        let location = gl::GetUniformLocation(self.prog, c_str_name.as_ptr());
        location
    }

    pub unsafe fn uniform4f(&self, location: i32, v0: f32, v1: f32, v2: f32, v3: f32) {
        gl::Uniform4f(location, v0, v1, v2, v3);
    }
}

impl Drop for TutorialShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.prog);
        }
    }
}

pub(crate) struct TutorialTexture {
    tex: u32,
    unit: u32,
}

impl TutorialTexture {
    pub unsafe fn new(path: &str, unit: u32, has_alpha: bool, fliph: bool, flipv: bool) -> Self {
        let mut tex = 0;
        gl::GenTextures(1, &mut tex);
        gl::BindTexture(gl::TEXTURE_2D, tex);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let img = image::open(&Path::new(path)).expect("Failed to load texture");
        if fliph {
            img.fliph();
        }
        if flipv {
            img.flipv();
        }
        let data = img.to_bytes();
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGB as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0,
                       if has_alpha { gl::RGBA } else { gl::RGB },
                       gl::UNSIGNED_BYTE,
                       &data[0] as *const u8 as *const c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        Self {
            tex,
            unit,
        }
    }

    pub unsafe fn bind(&self) {
        gl::ActiveTexture(gl::TEXTURE0 + self.unit);
        gl::BindTexture(gl::TEXTURE_2D, self.tex);
    }
}

impl Drop for TutorialTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.tex)
        }
    }
}
