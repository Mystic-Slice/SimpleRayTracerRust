use std::ops;
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

    pub fn length(self) -> f32 {
        return (self.length_squared()).sqrt();
    }

    pub fn length_squared(self) -> f32 {
        return self.x*self.x + self.y*self.y + self.z*self.z;
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
}

impl Copy for Vec3 {}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        *self
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