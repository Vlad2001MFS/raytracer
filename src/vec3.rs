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
    distributions::Uniform,
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
        Vec3(-self.0, -self.1, -self.2)
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

    pub fn reflected(self, normal: Vec3) -> Vec3 {
        self - 2.0*self.dot(normal)*normal
    }

    pub fn refracted(self, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = self.neg().dot(normal);
        let ray_out_perp = etai_over_etat*(self + cos_theta*normal);
        let ray_out_parallel = (1.0 - ray_out_perp.length_sq()).abs().sqrt().neg()*normal;
        ray_out_perp + ray_out_parallel
    }

    pub fn random() -> Vec3 {
        Vec3(rand::random(), rand::random(), rand::random())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3(rand::thread_rng().gen_range(min, max), rand::thread_rng().gen_range(min, max), rand::thread_rng().gen_range(min, max))
    }

    pub fn random_unit_vector() -> Vec3 {
        let a = rand::thread_rng().gen_range(0.0, 2.0*std::f64::consts::PI);
        let z = rand::thread_rng().gen_range(-1.0, 1.0);
        let r = (1.0_f64 - z*z).sqrt();
        let sin_cos = a.sin_cos();
        Vec3(r*sin_cos.1, r*sin_cos.0, z)
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rand_gen = rand::thread_rng();
        let rand_dist = Uniform::new(-1.0, 1.0);
        loop {
            let p = Vec3(rand_gen.sample(rand_dist), rand_gen.sample(rand_dist), 0.0);
            if p.length_sq() < 1.0 {
                return p;
            }
        }
    }
}
