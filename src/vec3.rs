use crate::{Vec2, VecX};
use std::fmt;
use std::ops::{self, Index};

/*
    Definition
*/
#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }
}

impl VecX for Vec3 {
    /*
        Maths
    */
    fn dot_product(&self, other: &Self) -> f64 {
        return self.0 * other.0 + self.1 * other.1 + self.2 * other.2;
    }

    fn normalized(&self) -> Vec3 {
        let m = self.magnitude();
        if m == 0.0 {
            panic!("{}: Can't be normalized because its magnitude is 0", self);
        }
        Vec3(self.x() / m, self.y() / m, self.z() / m)
    }

    fn magnitude(&self) -> f64 {
        let x = self.x();
        let y = self.y();
        let z = self.z();

        f64::sqrt(x * x + y * y + z * z)
    }

    /*
        Accessors
    */
    fn comp(&self, component: char) -> f64 {
        match component {
            'x' => self.0,
            'y' => self.1,
            'z' => self.2,
            'r' => self.0,
            'g' => self.1,
            'b' => self.2,
            _ => {
                eprintln!(
                    "Warning: Trying to access invalid component '{}' on Vec3",
                    component
                );
                0.0
            }
        }
    }
    fn at(&self, idx: usize) -> f64 {
        self[idx]
    }

    /*
        Type functions
    */
    fn size() -> usize {
        3
    }
}

/*
    Ops
*/
/*
    Neg
*/
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

/*
    Add
*/
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.0;
        self.2 -= rhs.2;
    }
}

/*
    Sub
*/
impl ops::Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

/*
    Mul
*/
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

/*
    Div
*/
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        if (self.0 == 0.0 || self.1 == 0.0 || self.2 == 0.0) && rhs == 0.0 {
            panic!("Division of 0.0 by 0.0: {} / {}", self, rhs);
        }

        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        if self.0 == 0.0 && rhs.0 == 0.0
            || self.1 == 0.0 && rhs.1 == 0.0
            || self.2 == 0.0 && rhs.2 == 0.0
        {
            panic!("Division of 0.0 by 0.0: {} / {}", self, rhs);
        }

        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        if (self.0 == 0.0 || self.1 == 0.0 || self.2 == 0.0) && rhs == 0.0 {
            panic!("Division of 0.0 by 0.0: {} / {}", self, rhs);
        }

        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        if self.0 == 0.0 && rhs.0 == 0.0
            || self.1 == 0.0 && rhs.1 == 0.0
            || self.2 == 0.0 && rhs.2 == 0.0
        {
            panic!("Division of 0.0 by 0.0: {} / {}", self, rhs);
        }

        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

/*
    Rem
*/
impl ops::Rem<f64> for Vec3 {
    type Output = Vec3;

    fn rem(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!(
                "{:?} % {:?}: Attempt to find remainder with a divisor of 0",
                self, rhs
            );
        }
        Vec3(self.0 % rhs, self.1 % rhs, self.2 % rhs)
    }
}

impl ops::Rem<Vec3> for Vec3 {
    type Output = Vec3;

    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0.0 || rhs.1 == 0.0 || rhs.2 == 0.0 {
            panic!(
                "{:?} % {:?}: Attempt to find remainder with a divisor of 0",
                self, rhs
            );
        }
        Vec3(self.0 % rhs.0, self.1 % rhs.1, self.2 % rhs.2)
    }
}

impl ops::RemAssign<f64> for Vec3 {
    fn rem_assign(&mut self, rhs: f64) {
        if rhs == 0.0 {
            panic!(
                "{:?} % {:?}: Attempt to find remainder with a divisor of 0",
                self, rhs
            );
        }

        self.0 %= rhs;
        self.1 %= rhs;
        self.2 %= rhs;
    }
}

impl ops::RemAssign<Vec3> for Vec3 {
    fn rem_assign(&mut self, rhs: Vec3) {
        if rhs.0 == 0.0 || rhs.1 == 0.0 || rhs.2 == 0.0 {
            panic!(
                "{:?} % {:?}: Attempt to find remainder with a divisor of 0",
                self, rhs
            );
        }

        self.0 %= rhs.0;
        self.1 %= rhs.1;
        self.2 %= rhs.2;
    }
}

/*
    Equality
*/
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Eq for Vec3 {}

/*
    Display
*/
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

/*
    From
*/
impl From<Vec<f64>> for Vec3 {
    fn from(values: Vec<f64>) -> Self {
        Vec3(
            *values.get(0).unwrap_or(&0.0),
            *values.get(1).unwrap_or(&0.0),
            *values.get(2).unwrap_or(&0.0),
        )
    }
}

impl From<f64> for Vec3 {
    fn from(f: f64) -> Self {
        Vec3(f, f, f)
    }
}

impl From<Vec2> for Vec3 {
    fn from(v: Vec2) -> Self {
        Vec3(v.x(), v.y(), 0.0)
    }
}

impl From<(Vec2, f64)> for Vec3 {
    fn from(t: (Vec2, f64)) -> Self {
        Vec3(t.0.x(), t.0.y(), t.1)
    }
}

impl From<(f64, Vec2)> for Vec3 {
    fn from(t: (f64, Vec2)) -> Self {
        Vec3(t.0, t.1.x(), t.1.y())
    }
}

/*
    Index
*/
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!(
                "Warning: accessing vector {} by invalid index {}",
                idx, self
            ),
        }
    }
}
