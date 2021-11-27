use crate::core::{ vec3::{ Vec3, Point3 }, ray::Ray };

pub struct HitRecord {
    pub impact: Point3,
    pub normal: Vec3,
    pub hit_t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(self: &mut HitRecord, ray: &Ray, outward_normal: Vec3) -> () {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0f32;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}