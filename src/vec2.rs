use core::fmt;
use std::ops::{self, Index};

use crate::vec3::Vec3;
use crate::vecx::VecX;

/*
    Definition
*/
#[derive(Debug, Clone, Copy)]
pub struct Vec2(pub f64, pub f64);

impl Vec2 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }
}

impl VecX for Vec2 {
    /*
        Maths
    */
    fn dot_product(&self, other: &Self) -> f64 {
        return self.0 * other.0 + self.1 * other.1;
    }

    fn normalized(&self) -> Vec2 {
        let m = self.magnitude();
        if m == 0.0 {
            panic!("{}: Can't be normalized because its magnitude is 0", self);
        }
        *self / m
    }
    fn magnitude(&self) -> f64 {
        let x = self.x();
        let y = self.y();
        f64::sqrt(x * x + y * y)
    }

    /*
        Accessors
    */
    fn comp(&self, component: char) -> f64 {
        match component {
            'x' => self.0,
            'y' => self.1,
            _ => panic!(
                "Trying to access invalid component '{}' of {:?}",
                component, self
            ),
        }
    }

    fn at(&self, idx: usize) -> f64 {
        self[idx]
    }

    /*
        Type functions
    */
    fn size() -> usize {
        2
    }
}

/*
    OPS
*/
/*
    Neg
*/
impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2(-self.0, -self.1)
    }
}

/*
    Add
*/
impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

/*
    Sub
*/
impl ops::Sub<Self> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

/*
    Mul
*/
impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl ops::Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2(self * rhs.0, self * rhs.1)
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl ops::MulAssign<Vec2> for Vec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
    }
}

/*
    Div
*/
impl ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        if (self.0 == 0.0 || self.1 == 0.0) && rhs == 0.0 {
            panic!("Division of 0.0 by 0.0: {} / {}", self, rhs);
        }

        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl ops::Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        if self.0 == 0.0 && rhs.0 == 0.0 || self.1 == 0.0 && rhs.1 == 0.0 {
            panic!("Division of 0.0 by 0.0: {} / {}", self, rhs);
        }
        Vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl ops::DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        if (self.0 == 0.0 || self.1 == 0.0) && rhs == 0.0 {
            panic!("Division of 0.0 by 0.0: {} / {}", self, rhs);
        }

        self.0 /= rhs;
        self.1 /= rhs;
    }
}

impl ops::DivAssign<Vec2> for Vec2 {
    fn div_assign(&mut self, rhs: Vec2) {
        if self.0 == 0.0 && rhs.0 == 0.0 || self.1 == 0.0 && rhs.1 == 0.0 {
            panic!("Division of 0.0 by 0.0: {} / {}", self, rhs);
        }

        self.0 /= rhs.0;
        self.1 /= rhs.1;
    }
}

/*
    Rem
*/
impl ops::Rem<f64> for Vec2 {
    type Output = Vec2;

    fn rem(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!(
                "{:?} % {:?}: Attempt to find remainder with a divisor of 0",
                self, rhs
            );
        }
        Vec2(self.0 % rhs, self.1 % rhs)
    }
}

impl ops::Rem<Vec2> for Vec2 {
    type Output = Vec2;

    fn rem(self, rhs: Vec2) -> Self::Output {
        if rhs.0 == 0.0 || rhs.1 == 0.0 {
            panic!(
                "{:?} % {:?}: Attempt to find remainder with a divisor of 0",
                self, rhs
            );
        }
        Vec2(self.0 % rhs.0, self.1 % rhs.1)
    }
}

impl ops::RemAssign<f64> for Vec2 {
    fn rem_assign(&mut self, rhs: f64) {
        if rhs == 0.0 {
            panic!(
                "Attempt to assign remainder with zero divisor: {:?} % {:?}",
                &self, &rhs
            );
        }

        self.0 %= rhs;
        self.1 %= rhs;
    }
}

impl ops::RemAssign<Vec2> for Vec2 {
    fn rem_assign(&mut self, rhs: Vec2) {
        if rhs.0 == 0.0 || rhs.1 == 0.0 {
            panic!(
                "Attempt to assign remainder with zero divisor: {:?} % {:?}",
                &self, &rhs
            );
        }

        self.0 %= rhs.0;
        self.1 %= rhs.1;
    }
}

/*
    Equality
*/
impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Vec2 {}

/*
    Display
*/
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

/*
    Index
*/
impl Index<usize> for Vec2 {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        match idx {
            0 => &self.0,
            1 => &self.1,
            _ => panic!(
                "Warning: accessing vector {} by invalid index {}",
                idx, self
            ),
        }
    }
}

/*
    From
*/
impl From<f64> for Vec2 {
    fn from(f: f64) -> Self {
        Vec2(f, f)
    }
}

impl From<Vec3> for Vec2 {
    fn from(v: Vec3) -> Self {
        Vec2(v.0, v.1)
    }
}

impl From<Vec<f64>> for Vec2 {
    fn from(values: Vec<f64>) -> Self {
        Vec2(
            *values.get(0).unwrap_or(&0.0),
            *values.get(1).unwrap_or(&0.0),
        )
    }
}
