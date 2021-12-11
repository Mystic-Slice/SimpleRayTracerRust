use crate::core::{
    vec3::{
        Vec3,
        Color,
    },
    ray::Ray,
    hittable::HitRecord
};

use super::utils::random_double;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { ref_idx: f32 },
}

pub fn scatter(material: &Material, ray_in: &Ray, rec: &HitRecord, attentuation: &mut Vec3, scattered: &mut Ray) -> bool {
    match material {
        &Material::Lambertian { albedo } => {
            let mut target = rec.impact + rec.normal + Vec3::random_in_unit_sphere();

            if target.near_zero() {
                target = rec.normal;
            }
            *scattered = Ray::new(rec.impact, target - rec.impact);
            *attentuation = albedo;
            return true;
        }
        &Material::Metal { albedo, fuzz} => {
            let mut f = 1.0;
            if fuzz < 1.0 {
                f = fuzz;
            }
            let reflected = Vec3::reflect(Vec3::unit_vector(ray_in.direction()), rec.normal);
            *scattered = Ray::new(rec.impact, reflected + f*Vec3::random_in_unit_sphere());
            *attentuation = albedo;
            return Vec3::dot(scattered.direction(), rec.normal) > 0.0;
        }
        &Material::Dielectric { ref_idx } => {
            let mut outward_normal = Vec3::default();
            let reflected = Vec3::reflect(ray_in.direction(), rec.normal);
            let mut ni_over_nt = 0.0;
            *attentuation = Vec3::new(1.0, 1.0, 1.0);

            let mut refracted = Vec3::default();

            let mut reflect_prob = 0.0;
            let mut cosine = 0.0;

            if Vec3::dot(ray_in.direction(), rec.normal) > 0.0 {
                outward_normal = -rec.normal;
                ni_over_nt = ref_idx;
                cosine = ref_idx * Vec3::dot(ray_in.direction(), rec.normal)
                    / ray_in.direction().length();
            } else {
                outward_normal = rec.normal;
                ni_over_nt = 1.0 / ref_idx;
                cosine = -Vec3::dot(ray_in.direction(), rec.normal) / ray_in.direction().length();
            }

            if Vec3::refract( &ray_in.direction(), &outward_normal, ni_over_nt, &mut refracted) {
                reflect_prob = schlick_approx(cosine, ref_idx);
            } else {
                reflect_prob = 1.0;
            }

            if random_double() < reflect_prob {
                *scattered = Ray::new(rec.impact, reflected);
            } else {
                *scattered = Ray::new(rec.impact, refracted);
            }

            return true;
        }
    }
}

fn schlick_approx(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}