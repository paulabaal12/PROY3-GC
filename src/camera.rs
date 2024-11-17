use nalgebra_glm::{Vec3, rotate_vec3};
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub has_changed: bool,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            movement_speed: 0.5,
            rotation_speed: 0.03,
            has_changed: true,
        }
    }

    pub fn get_view_direction(&self) -> Vec3 {
        (self.center - self.eye).normalize()
    }

    pub fn get_right(&self) -> Vec3 {
        self.get_view_direction().cross(&self.up).normalize()
    }

    pub fn move_forward(&mut self, amount: f32) {
        let direction = self.get_view_direction();
        self.eye += direction * amount * self.movement_speed;
        self.center += direction * amount * self.movement_speed;
        self.has_changed = true;
    }

    pub fn move_right(&mut self, amount: f32) {
        let right = self.get_right();
        self.eye += right * amount * self.movement_speed;
        self.center += right * amount * self.movement_speed;
        self.has_changed = true;
    }

    pub fn move_up(&mut self, amount: f32) {
        self.eye += self.up * amount * self.movement_speed;
        self.center += self.up * amount * self.movement_speed;
        self.has_changed = true;
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        self.yaw = (self.yaw + delta_yaw * self.rotation_speed) % (2.0 * PI);
        self.pitch = (self.pitch + delta_pitch * self.rotation_speed)
            .clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let new_eye = self.center + Vec3::new(
            radius * self.yaw.cos() * self.pitch.cos(),
            -radius * self.pitch.sin(),
            radius * self.yaw.sin() * self.pitch.cos()
        );

        self.eye = new_eye;
        self.has_changed = true;
    }

    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.center - self.eye).normalize();
        let new_eye = self.eye + direction * delta * self.movement_speed;
        let min_distance = 1.0;
        if (new_eye - self.center).magnitude() > min_distance {
            self.eye = new_eye;
            self.has_changed = true;
        }
    }

    pub fn rotate_around_point(&mut self, delta_yaw: f32, delta_pitch: f32, point: Vec3) {
        let radius_vector = self.eye - point;
        let radius = radius_vector.magnitude();

        self.yaw = (self.yaw + delta_yaw * self.rotation_speed) % (2.0 * PI);
        self.pitch = (self.pitch + delta_pitch * self.rotation_speed)
            .clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let new_eye = point + Vec3::new(
            radius * self.yaw.cos() * self.pitch.cos(),
            -radius * self.pitch.sin(),
            radius * self.yaw.sin() * self.pitch.cos()
        );

        self.eye = new_eye;
        self.center = point;
        self.has_changed = true;
    }

    pub fn set_movement_speed(&mut self, speed: f32) {
        self.movement_speed = speed;
    }

    pub fn set_rotation_speed(&mut self, speed: f32) {
        self.rotation_speed = speed;
    }
}