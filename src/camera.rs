use glam::{Mat4, Vec3};

pub struct Camera {
    pub yaw: f64,
    pub pitch: f64,
    pub position: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub speed: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

impl Camera {
    pub fn new(fov: f32, speed: f32) -> Camera {
        let camera = Camera {
            yaw: 0.0,
            pitch: 0.0,
            position: Vec3::splat(0.0),
            up: Vec3::new(0.0,1.0,0.0),
            fov,
            speed,
            near_plane: 1.0,
            far_plane: 100000.0
        };
        camera
    }
    pub fn front(&self) -> Vec3 {
        Vec3 {
            x: (self.yaw.to_radians().cos() * self.pitch.to_radians().cos()) as f32,
            y: self.pitch.to_radians().sin() as f32,
            z: (self.yaw.to_radians().sin() * self.pitch.to_radians().cos()) as f32,
        }.normalize()
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::look_at_lh(self.position, self.position + self.front(), self.up)
    }
}