use crate::core::{
    vec3::{
        Vec3,
        Point3
    },
    ray::Ray
};
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0/9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0f32, 0f32, 0f32);
        let horizontal = Vec3::new(viewport_width as f32, 0f32, 0f32);
        let vertical = Vec3::new(0f32, viewport_height, 0f32);

        Self {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,            
            lower_left_corner : origin - horizontal/2f32 - vertical/2f32 - Vec3::new(0f32, 0f32, focal_length)
        }
    }

    pub fn origin(&self) -> Point3 {
        return self.origin;
    }

    pub fn lower_left_corner(&self) -> Vec3 {
        return self.lower_left_corner;
    }

    pub fn horizontal(&self) -> Vec3 {
        return self.horizontal;
    }

    pub fn vertical(&self) -> Vec3 {
        return self.vertical;
    }


    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray::new(self.origin(), self.lower_left_corner() + u*self.horizontal() + v*self.vertical() - self.origin());
    }
}