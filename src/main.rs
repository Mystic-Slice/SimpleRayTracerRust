use ray_tracer::core::{ vec3::{ Vec3, Point3, Color }, ray::Ray };

fn hit_sphere(center: Point3, radius: f32, ray: Ray) -> f32 {
    let oc: Vec3 = ray.origin() - center;
    let a: f32 = Vec3::dot(ray.direction(), ray.direction());
    let b: f32 = 2f32 * Vec3::dot(oc, ray.direction());
    let c: f32 = Vec3::dot(oc, oc) - radius*radius;
    let discriminant: f32 = b*b - 4f32*a*c;

    if discriminant < 0.0 {
        return -1f32;
    } else {
        return (-b - discriminant.sqrt()) /(2f32*a);
    }
}
fn ray_color(ray: Ray) -> Color {
    let t: f32 =  hit_sphere(Point3::new(0f32,0f32,-1f32), 0.5, ray); 
    if t > 0.0 {
        let n: Vec3 = Vec3::unit_vector(ray.at(t) - Vec3::new(0f32,0f32,-1f32));
        return 0.5 * Color::new(n.x()+1f32,n.y()+1f32,n.z()+1f32);
    }
    let unit_vector: Vec3 = Vec3::unit_vector(ray.direction());
    let t = 0.5*(unit_vector.y() + 1.0);
    return Color::new(1f32, 1f32, 1f32)*(1f32-t) + Color::new(0.5, 0.7, 1.0)*t;
}

fn main() {
    // Image
    const ASPECT_RATIO : f32 = 16f32/9f32;
    const IMAGE_WIDTH : i64 = 400;
    const IMAGE_HEIGHT : i64 = ((IMAGE_WIDTH as f32)/ASPECT_RATIO) as i64; 

    // Camera
    let viewport_height: f32 = 2f32;
    let viewport_width: f32 = ASPECT_RATIO*viewport_height;
    let focal_length: f32 = 1f32;

    let origin: Point3 = Point3::new(0f32, 0f32, 0f32);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0f32, 0f32);
    let vertical: Vec3 = Vec3::new(0f32, viewport_height, 0f32);
    let lower_left_corner: Vec3 = origin - horizontal/2f32 - vertical/2f32 - Vec3::new(0f32, 0f32, focal_length);

    // Render
    print!("P3\n {} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in (0..IMAGE_HEIGHT).rev() {
        eprint!("{}", format!("\rScanlines remaining: {:03}", i));
        for j in 0..IMAGE_WIDTH {
            let u: f32 = (j as f32)/((IMAGE_WIDTH - 1) as f32);
            let v: f32 = (i as f32)/((IMAGE_HEIGHT -1) as f32);
            let ray: Ray = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);
            let pixel_color: Color = ray_color(ray);
            (pixel_color*255.999f32).print_vector();
        }
    }

    eprintln!("\nDone");
}