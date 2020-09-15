#![allow(dead_code)]

use cgmath::prelude::*;
use cgmath::vec3;

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

#[derive(PartialEq, Clone, Copy)]
pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub struct Camera {
    // Camera Attributes
    pub position: Point3,
    pub front: Vector3,
    pub up: Vector3,
    pub right: Vector3,
    pub world_up: Vector3,
    // Euler Angles
    pub yaw: f32,
    pub pitch: f32,
    // Camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Camera {
            position: Point3::new(0.0, 0.0, 0.0),
            front: vec3(0.0, 0.0, -1.0),
            up: Vector3::zero(),
            right: Vector3::zero(),
            world_up: Vector3::unit_y(),
            yaw: YAW,
            pitch: PITCH,
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVITY,
            zoom: ZOOM,
        };
        camera.update_camera_vectors();
        camera
    }
}

impl Camera {
    /// Returns the view matrix calculated using Euler Angles and the LookAt Matrix
    pub fn get_view_matrix(&self) -> Matrix4 {
        Matrix4::look_at(self.position, self.position + self.front, self.up)
    }

    /// Processes input received from any keyboard-like input system. Accepts input parameter in the form of camera defined ENUM (to abstract it from windowing systems)
    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        self.position += match direction {
            CameraMovement::FORWARD => self.front * velocity,
            CameraMovement::BACKWARD => -(self.front * velocity),
            CameraMovement::LEFT => -(self.right * velocity),
            CameraMovement::RIGHT => self.right * velocity,
        }
    }

    /// Processes input received from a mouse input system. Expects the offset value in both the x and y direction.
    pub fn process_mouse_movement(&mut self, mut xoffset: f32, mut yoffset: f32, constrain_pitch: bool) {
        xoffset *= self.mouse_sensitivity;
        yoffset *= self.mouse_sensitivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        // Make sure that when pitch is out of bounds, screen doesn't get flipped
        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        // Update Front, Right and Up Vectors using the updated Euler angles
        self.update_camera_vectors();
    }

    // Processes input received from a mouse scroll-wheel event. Only requires input on the vertical wheel-axis
    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= yoffset;
        }
        if self.zoom < 1.0 {
            self.zoom = 1.0
        } else if self.zoom > 45.0 {
            self.zoom = 45.0
        }
    }

    /// Calculates the front vector from the Camera's (updated) Eular Angles
    fn update_camera_vectors(&mut self) {
        // Calculate the new Front vector
        let front = Vector3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        };
        self.front = front.normalize();
        // Also re-calculate the Right and Up vector
        self.right = self.front.cross(self.world_up).normalize(); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
        self.up = self.right.cross(self.front).normalize();
    }
}
