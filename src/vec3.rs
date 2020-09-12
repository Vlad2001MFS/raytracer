use std::{
    ops::{
        Add, AddAssign,
        Sub, SubAssign,
        Mul, MulAssign,
        Div, DivAssign,
        Neg,
    }
};

use rand::{
    Rng,
    distributions::{
        Uniform,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0*rhs.0, self.1*rhs.1, self.2*rhs.2)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0*rhs, self.1*rhs, self.2*rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self*rhs.0, self*rhs.1, self*rhs.2)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(self / rhs.0, self / rhs.1, self / rhs.2)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.1)
    }
}

impl Vec3 {
    pub fn length_sq(&self) -> f64 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }

    pub fn distance_sq(&self, target: Vec3) -> f64 {
        (target - *self).length_sq()
    }

    pub fn distance(&self, target: Vec3) -> f64 {
        self.distance_sq(target).sqrt()
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.0*rhs.0 + self.1*rhs.1 + self.2*rhs.2
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.1*rhs.2 - self.2*rhs.1,
            self.2*rhs.0 - self.0*rhs.2,
            self.0*rhs.1 - self.1*rhs.0,
        )
    }
    
    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn random() -> Vec3 {
        let mut rand_gen = rand::thread_rng();
        Vec3(rand_gen.gen(), rand_gen.gen(), rand_gen.gen())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rand_gen = rand::thread_rng();
        let rand_distrib = Uniform::new_inclusive(min, max);
        Vec3(rand_gen.sample(rand_distrib), rand_gen.sample(rand_distrib), rand_gen.sample(rand_distrib))
    }

    pub fn random_unit_sphere() -> Vec3 {
        loop {
            let point = Self::random_range(-1.0, 1.0);
            if point.length_sq() < 1.0 {
                return point
            }
        }
    }
}
