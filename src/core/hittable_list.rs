use std::sync::Arc;
use crate::core::{ hittable::{ Hittable, HitRecord }, ray::Ray };
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(self: &mut HittableList, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(self: &mut HittableList) {
        while self.objects.len() > 0 {
            self.objects.pop();
        }
    }
}

impl Hittable for HittableList {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f32 = t_max;

        for object in &self.objects {
            let is_hit: bool = object.hit(ray, t_min, closest_so_far, &mut temp_rec);

            if is_hit {
                hit_anything = true;
                closest_so_far = temp_rec.hit_t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}