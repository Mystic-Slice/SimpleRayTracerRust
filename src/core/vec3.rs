use std::ops;
use crate::core::utils::{
    clamp,
    random_double,
    random_double_range
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }

    // Coord
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }
    
    // Image
    pub fn r(&self) -> f32 { self.x }
    pub fn g(&self) -> f32 { self.y }
    pub fn b(&self) -> f32 { self.z }

    pub fn print_vector_verbose(&self) {
        print!("Vector: ");
        self.print_vector();
    }

    pub fn print_vector(&self) {
        println!("{} {} {}", self.x(), self.y(), self.z());
    }

    pub fn print_color(&self, samples_per_pixel: i64) -> () {
        let scale = 1.0/samples_per_pixel as f32;

        let r_final = (256.0*clamp((self.r()*scale).sqrt(), 0.0, 0.999)) as i64;
        let g_final = (256.0*clamp((self.g()*scale).sqrt(), 0.0, 0.999)) as i64;
        let b_final = (256.0*clamp((self.b()*scale).sqrt(), 0.0, 0.999)) as i64;

        println!("{} {} {}", r_final, g_final, b_final);
    }

    pub fn length(self) -> f32 {
        return (self.length_squared()).sqrt();
    }

    pub fn length_squared(self) -> f32 {
        return Vec3::dot(self, self);
    }

    pub fn dot(u: Vec3, v: Vec3) -> f32 {
        return u.x()*v.x() + u.y()*v.y() + u.z()*v.z();
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        return Vec3::new(u.y()*v.z() - u.z()*v.y(),
                         u.z()*v.x() - u.x()*v.z(),
                         u.x()*v.y() - u.y()*v.x());
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        return v/v.length();
    }

    pub fn random() -> Vec3 {
        return Vec3::new(random_double(), random_double(), random_double());
    }

    pub fn random_range(min: f32, max: f32) -> Vec3 {
        return Vec3::new(random_double_range(min, max), random_double_range(min, max), random_double_range(min, max));
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(in_unit_sphere, normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        return Vec3::unit_vector(Vec3::random_in_unit_sphere());
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return (f32::abs(self.x()) < s) && (f32::abs(self.y()) < s) && (f32::abs(self.z()) < s);
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - n*2.0*Vec3::dot(v,n);
    }

    pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
        let uv = Vec3::unit_vector(*v);
        let dt = Vec3::dot(uv, *n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

        if discriminant > 0.0 {
            *refracted = ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt();
            return true;
        } else {
            return false;
        }
    }
}

// Operators overloading

impl ops::Neg<> for Vec3 {
    type Output = Vec3;
    
    fn neg(self) -> Vec3 {
        return Vec3::new(-&self.x, -&self.y, -&self.z);
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        if i == 0 {
            return &self.x;
        } else if i == 1 {
            return &self.y;
        } else if i == 2 {
            return &self.z;
        } else {
            return &0f32;
        }
    }
}

impl ops::AddAssign<f32> for Vec3 {
    fn add_assign(self: &mut Vec3, i: f32) -> () {
        self.x += i;
        self.y += i;
        self.z += i;
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, i: Vec3) {
       self.x += i.x();
       self.y += i.y();
       self.z += i.z(); 
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, i: Vec3) -> Vec3 {
        return Vec3::new(self.x()+i.x(), self.y()+i.y(), self.z()+i.z());
    }
}

impl ops::SubAssign<f32> for Vec3 {
    fn sub_assign(self: &mut Vec3, i: f32) -> () {
        *self += -i;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, i: Vec3) -> Vec3 {
        return self + (-i);
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(self: &mut Vec3, i: f32) -> () {
        self.x *= i;
        self.y *= i;
        self.z *= i;
    }
}


impl ops::Mul<f32> for Vec3 {
    type Output = Vec3; 

    fn mul(self, i: f32) -> Vec3 {
        return Vec3::new(self.x()*i, self.y()*i, self.z()*i);
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, i: Vec3) -> Vec3 {
        return i*self;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Vec3 {
        return Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z());
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(self: &mut Vec3, i: f32) -> () {
        // Use the already defined operator
        *self *= 1f32/i;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, i: f32) -> Vec3 {
        return self*(1f32/i);
    }
}

pub use Vec3 as Point3;
pub use Vec3 as Color;