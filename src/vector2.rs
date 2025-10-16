#![allow(unused)]
use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { 
            x: x, 
            y: y 
        }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }

    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0
        }
    }

    pub fn from_angle(angle: f64) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin()
        }
    }

    pub fn from_angle_and_magnitude(angle: f64, mangitude: f64) -> Self {
        Self {
            x: angle.cos() * mangitude,
            y: angle.sin() * mangitude
        }
    }

    pub fn recp(self) -> Self {
        Self {
            x: 1.0 / self.x,
            y: 1.0 / self.y
        }
    }

    pub fn recp_mul(self, mul: f64) -> Self {
        Self {
            x: mul / self.x,
            y: mul / self.y
        }
    }

    pub fn length(self) -> f64 {
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }

    pub fn angle(self) -> f64 {
        self.x.atan2(self.y)
    }

    pub fn normalized(self) -> Self {
        self / self.length()
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        Self { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y 
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        };
    }
}

impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Self) -> Self::Output {
        Self { 
            x: self.x - rhs.x, 
            y: self.y - rhs.y 
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Self) -> Self::Output {
        Self { 
            x: self.x * rhs.x, 
            y: self.y * rhs.y 
        }
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }    
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f64) -> Self::Output {
        Self { 
            x: self.x * rhs, 
            y: self.y * rhs 
        }
    }
}

impl MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Div for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Self) -> Self::Output {
        Self { 
            x: self.x / rhs.x, 
            y: self.y / rhs.y 
        }
    }
}

impl DivAssign for Vector2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x / rhs.y,
            y: self.y / rhs.y
        }
    }
}

impl Div<f64> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f64) -> Self::Output {
        Self { 
            x: self.x / rhs, 
            y: self.y / rhs 
        }
    }
}

impl DivAssign<f64> for Vector2 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self::Output {
        Self { 
            x: -self.x,
            y: -self.y
        }
    }
}
