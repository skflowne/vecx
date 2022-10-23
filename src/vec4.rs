use crate::{Vec2, Vec3, VecX};
use std::fmt;
use std::ops::{self, Index};

/*
    Definition
*/
#[derive(Debug, Clone, Copy)]
pub struct Vec4(pub f64, pub f64, pub f64, pub f64);

impl Default for Vec4 {
    fn default() -> Self {
        Vec4(0.0, 0.0, 0.0, 0.0)
    }
}

impl Vec4 {
    // xyzw
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn w(&self) -> f64 {
        self.3
    }

    // rgba
    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }

    pub fn a(&self) -> f64 {
        self.3
    }

    pub fn rot(&self, rot: &Vec4) -> Vec4 {
        let mut rotated = self.rot_x(rot.x());
        rotated = rotated.rot_y(rot.y());
        rotated = rotated.rot_z(rot.z());
        rotated
    }

    pub fn rot_x(&self, angle: f64) -> Self {
        Vec4(
            self.x(),
            self.y() * f64::cos(angle) + self.z() * f64::sin(angle),
            self.y() * f64::sin(angle) - self.z() * f64::cos(angle),
            self.w(),
        )
    }

    pub fn rot_y(&self, angle: f64) -> Self {
        Vec4(
            self.x() * f64::cos(angle) - self.z() * f64::sin(angle),
            self.y(),
            self.x() * f64::sin(angle) + self.z() * f64::cos(angle),
            self.w(),
        )
    }

    pub fn rot_z(&self, angle: f64) -> Self {
        Vec4(
            self.x() * f64::cos(angle) - self.y() * f64::sin(angle),
            self.x() * f64::sin(angle) + self.y() * f64::cos(angle),
            self.z(),
            self.w(),
        )
    }
}

impl FromIterator<f64> for Vec4 {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        let values: Vec<f64> = iter.into_iter().collect();
        Vec4::from(values)
    }
}

impl VecX for Vec4 {
    /*
        Maths
    */

    fn dot_product(&self, other: &Self) -> f64 {
        return self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3;
    }

    fn normalized(&self) -> Vec4 {
        let m = self.magnitude();
        if m == 0.0 {
            panic!("{}: Can't be normalized because its magnitude is 0", self);
        }
        Vec4(self.x() / m, self.y() / m, self.z() / m, self.w())
    }

    fn magnitude(&self) -> f64 {
        let x = self.x();
        let y = self.y();
        let z = self.z();
        let w = self.w();

        f64::sqrt(x * x + y * y + z * z + w * w)
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
                panic!(
                    "Attempt to access invalid component '{}' of {:?}",
                    component, self
                )
            }
        }
    }

    fn at(&self, idx: usize) -> f64 {
        if idx < 3 {
            self[idx]
        } else {
            0.0
        }
    }

    /*
        Type functions
    */
    fn size() -> usize {
        4
    }
}

/*
    Ops
*/
/*
    Neg
*/
impl ops::Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec4(-self.0, -self.1, -self.2, -self.3)
    }
}

/*
    Add
*/
impl ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Vec4 {
        Vec4(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl ops::AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: Vec4) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
    }
}

/*
    Sub
*/
impl ops::Sub<Self> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Vec4) -> Self::Output {
        Vec4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl ops::SubAssign<Vec4> for Vec4 {
    fn sub_assign(&mut self, rhs: Vec4) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self.3 -= rhs.3;
    }
}

/*
    Mul
*/
impl ops::Mul<f64> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec4(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl ops::Mul<Vec4> for f64 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4(self * rhs.0, self * rhs.1, self * rhs.2, self * rhs.3)
    }
}

impl ops::Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
            self.3 * rhs.3,
        )
    }
}

impl ops::MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
        self.3 *= rhs;
    }
}

impl ops::MulAssign<Vec4> for Vec4 {
    fn mul_assign(&mut self, rhs: Vec4) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
        self.3 *= rhs.3;
    }
}

/*
    Div
*/
impl ops::Div<f64> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f64) -> Self::Output {
        if (self.0 == 0.0 || self.1 == 0.0 || self.2 == 0.0 || self.3 == 0.0) && rhs == 0.0 {
            panic!("Division of 0.0 by 0.0: {} / {}", self, rhs);
        }

        Vec4(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

impl ops::Div<Vec4> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: Vec4) -> Self::Output {
        if self.0 == 0.0 && rhs.0 == 0.0
            || self.1 == 0.0 && rhs.1 == 0.0
            || self.2 == 0.0 && rhs.2 == 0.0
            || self.3 == 0.0 && rhs.3 == 0.0
        {
            panic!("Division of 0.0 by 0.0: {} / {}", self, rhs);
        }

        Vec4(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2,
            self.3 / rhs.3,
        )
    }
}

impl ops::DivAssign<f64> for Vec4 {
    fn div_assign(&mut self, rhs: f64) {
        if (self.0 == 0.0 || self.1 == 0.0 || self.2 == 0.0 || self.3 == 0.0) && rhs == 0.0 {
            panic!("Division of 0.0 by 0.0: {} / {}", self, rhs);
        }

        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
        self.3 /= rhs;
    }
}

impl ops::DivAssign<Vec4> for Vec4 {
    fn div_assign(&mut self, rhs: Vec4) {
        if self.0 == 0.0 && rhs.0 == 0.0
            || self.1 == 0.0 && rhs.1 == 0.0
            || self.2 == 0.0 && rhs.2 == 0.0
            || self.3 == 0.0 && rhs.3 == 0.0
        {
            panic!("Division of 0.0 by 0.0: {} / {}", self, rhs);
        }

        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
        self.3 /= rhs.3;
    }
}

/*
    Rem
*/
impl ops::Rem<f64> for Vec4 {
    type Output = Vec4;

    fn rem(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!(
                "{:?} % {:?}: Attempt to find remainder with a divisor of 0",
                self, rhs
            );
        }
        Vec4(self.0 % rhs, self.1 % rhs, self.2 % rhs, self.3 % rhs)
    }
}

impl ops::Rem<Vec4> for Vec4 {
    type Output = Vec4;

    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0.0 || rhs.1 == 0.0 || rhs.2 == 0.0 || rhs.3 == 0.0 {
            panic!(
                "{:?} % {:?}: Attempt to find remainder with a divisor of 0",
                self, rhs
            );
        }
        Vec4(
            self.0 % rhs.0,
            self.1 % rhs.1,
            self.2 % rhs.2,
            self.3 % rhs.3,
        )
    }
}

impl ops::RemAssign<f64> for Vec4 {
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
        self.3 %= rhs;
    }
}

impl ops::RemAssign<Vec4> for Vec4 {
    fn rem_assign(&mut self, rhs: Vec4) {
        if rhs.0 == 0.0 || rhs.1 == 0.0 || rhs.2 == 0.0 || rhs.3 == 0.0 {
            panic!(
                "{:?} % {:?}: Attempt to find remainder with a divisor of 0",
                self, rhs
            );
        }

        self.0 %= rhs.0;
        self.1 %= rhs.1;
        self.2 %= rhs.2;
        self.3 %= rhs.3;
    }
}

/*
    Equality
*/
impl PartialEq for Vec4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Eq for Vec4 {}

/*
    Display
*/
impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

/*
    From
*/
impl From<Vec<f64>> for Vec4 {
    fn from(values: Vec<f64>) -> Self {
        Vec4(
            *values.get(0).unwrap_or(&0.0),
            *values.get(1).unwrap_or(&0.0),
            *values.get(2).unwrap_or(&0.0),
            *values.get(3).unwrap_or(&0.0),
        )
    }
}

impl From<f64> for Vec4 {
    fn from(f: f64) -> Self {
        Vec4(f, f, f, f)
    }
}

impl From<Vec2> for Vec4 {
    fn from(v: Vec2) -> Self {
        Vec4(v.x(), v.y(), 0.0, 0.0)
    }
}

impl From<Vec3> for Vec4 {
    fn from(v: Vec3) -> Self {
        Vec4(v.0, v.1, v.2, 0.0)
    }
}

impl From<(Vec3, f64)> for Vec4 {
    fn from(t: (Vec3, f64)) -> Self {
        Vec4(t.0.x(), t.0.y(), t.0.z(), t.1)
    }
}

impl From<(f64, Vec3)> for Vec4 {
    fn from(t: (f64, Vec3)) -> Self {
        Vec4(t.0, t.1.x(), t.1.y(), t.1.z())
    }
}

/*
    Index
*/
impl Index<usize> for Vec4 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!(
                "Warning: accessing vector {} by invalid index {}",
                idx, self
            ),
        }
    }
}
