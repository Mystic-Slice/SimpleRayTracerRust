use crate::core::{ hittable::{ HitRecord, Hittable }, vec3::{ Vec3, Point3 }, ray::Ray };

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(cen: Point3, r: f32) -> Sphere {
        Sphere {
            center: cen,
            radius: r
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool{
        let oc: Vec3 = ray.origin() - self.center;
        let a: f32 = ray.direction().length_squared();
        let b: f32 = 2f32 * Vec3::dot(oc, ray.direction());
        let c: f32 = oc.length_squared() - self.radius*self.radius;

        let discriminant: f32 = b*b - 4f32*a*c;
        if discriminant < 0.0 { return false; }
        let sqrtd: f32 = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let root: f32 = (-2f32*b - sqrtd)/(2f32*a);
        if root < t_min || root > t_max {
            let root = (-2f32*b + sqrtd)/(2f32*a);
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.hit_t = root;
        rec.impact = ray.at(rec.hit_t);
        let outward_normal: Vec3 = (rec.impact - self.center)/self.radius;
        rec.set_face_normal(ray, outward_normal);
        return true;
    }
}