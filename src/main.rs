use std::{sync::Arc};

use ray_tracer::{
    core::{ 
        vec3::{ 
            Vec3, 
            Point3, 
            Color 
        }, 
        ray::Ray, 
        hittable_list::HittableList, 
        hittable::{
            Hittable, HitRecord
        }, 
        camera::Camera,
        material::{
            Material,
            scatter, self
        }, utils::PI
    },
    shapes::sphere::Sphere
};

fn ray_color(ray: Ray, world: &HittableList, depth: i64) -> Color {

    if depth <= 0 {
        return Color::new(0f32, 0f32, 0f32);
    }

    let mut rec: HitRecord = HitRecord::new();
    if world.hit(&ray, 0.001, f32::MAX, &mut rec) {
        let mut scattered: Ray = Ray::new(Point3::default(), Vec3::default());
        let mut attenuation: Color = Color::new(0f32, 0f32, 0f32);
        if scatter(&rec.material, &ray, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth-1);
        }
        return Color::new(0f32, 0f32, 0f32);
    }

    let unit_vector: Vec3 = Vec3::unit_vector(ray.direction());
    let t = 0.5*(unit_vector.y() + 1.0);
    return Color::new(1f32, 1f32, 1f32)*(1f32-t) + Color::new(0.5, 0.7, 1.0)*t;
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16f32/9f32;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = ((IMAGE_WIDTH as f32)/ASPECT_RATIO) as i64; 
    const SAMPLES_PER_PIXEL: i64 = 100;
    const MAX_DEPTH: i64 = 50;

    // World
    let mut world: HittableList = HittableList::new();

    let materialGround = Material::Lambertian { albedo: Color::new(0.8, 0.8, 0.0) };
    let materialCenter = Material::Lambertian { albedo: Color::new(0.1, 0.2, 0.5) };
    let materialLeft = Material::Dielectric { ref_idx: 1.5 };
    let materialRight = Material::Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.0 };
    

    world.add(Arc::new(Sphere::new(Point3::new(0f32,-100.5f32,-1f32), 100.0, materialGround)));
    world.add(Arc::new(Sphere::new(Point3::new(0f32,0f32,-1f32), 0.5, materialCenter)));
    world.add(Arc::new(Sphere::new(Point3::new(-1f32,0f32,-1f32), 0.5, materialLeft)));
    world.add(Arc::new(Sphere::new(Point3::new(-1f32,0f32,-1f32), -0.45, materialLeft)));
    world.add(Arc::new(Sphere::new(Point3::new(1f32,0f32,-1f32), 0.5, materialRight)));

    // Camera
    let lookfrom = Point3::new(0.0, 0.0, 0.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2f32;
    let cam = Camera::new(lookfrom, lookat, vup, 90f32, ASPECT_RATIO, aperture, dist_to_focus);

    print!("P3\n {} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in (0..IMAGE_HEIGHT).rev() {
        eprint!("{}", format!("\rScanlines remaining: {:03}", i));
        for j in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0f32, 0f32, 0f32);

            for _sample in 0..SAMPLES_PER_PIXEL {
                let u: f32 = (j as f32)/((IMAGE_WIDTH - 1) as f32);
                let v: f32 = (i as f32)/((IMAGE_HEIGHT -1) as f32);
                let ray: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(ray, &world, MAX_DEPTH);
            }
            pixel_color.print_color(SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone");
}
