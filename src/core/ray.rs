use crate::core::vec3::{ Point3, Vec3 };

#[derive(Clone, Copy)]
pub struct Ray { // a + tb
    pub origin: Point3, // a
    pub direction: Vec3, // b
}

impl Ray {

    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray {
            origin: orig,
            direction: dir
        }
    }

    pub fn origin(&self) -> Point3 { return self.origin; }
    pub fn direction(&self) -> Vec3 { return self.direction; }

    pub fn at(&self, t: f32) -> Point3 {
        return self.origin() + self.direction() * t;
    }
}