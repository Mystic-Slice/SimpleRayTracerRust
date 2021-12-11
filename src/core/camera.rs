use crate::core::{
    vec3::{
        Vec3,
        Point3
    },
    ray::Ray
};

use super::{utils::degrees_to_radians, vec3};
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(lookfrom-lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = focus_dist*viewport_width * u;
        let vertical = focus_dist*viewport_height * v; 

        Self {
            origin: lookfrom,
            horizontal: viewport_width*u,
            vertical: viewport_height*v,            
            lower_left_corner : origin - horizontal/2f32 - vertical/2f32 - focus_dist*w,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture/2.0
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


    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        return Ray::new(self.origin() + offset, self.lower_left_corner() + s*self.horizontal() + t*self.vertical() - self.origin() - offset);
    }
}