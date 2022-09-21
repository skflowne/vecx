use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

use crate::{Vec2, Vec3};

pub trait VecX: 
    Copy 
    + Display
    // operations
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Mul<f64, Output = Self>
    + Div<Self, Output = Self>
    + Div<f64, Output = Self>
    + Rem<Self, Output = Self>
    + Rem<f64, Output = Self>
    // assignments
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + MulAssign<f64>
    + DivAssign<Self>
    + DivAssign<f64>
    + RemAssign<Self>
    + RemAssign<f64>
    // from
    + From<f64>
    + From<Vec2>
    + From<Vec3>
    + From<Vec<f64>>
    + FromIterator<f64>
{
    fn size() -> usize where Self: Sized;

    /*
        Maths
    */
    fn dist(&self, other: &Self) -> f64 {
        self.distance(other)
    }
    fn distance(&self, other: &Self) -> f64 {
        (*other - *self).magnitude()
    }

    fn mag(&self) -> f64 {
        self.magnitude()
    }
    fn magnitude(&self) -> f64;

    fn angle(&self, other: &Self) -> f64 {
        return f64::acos(self.dot(other));
    }

    /// Returns the dot product of both **normalized** vectors.
    /// 
    /// Use `dot_product` if you need the dot product of the actual vectors
    /// 
    /// The result will be between -1.0 and 1.0
    /// 
    /// -1.0: Vectors are opposites
    /// 
    /// 0.0: Vectors are orthogonal
    /// 
    /// 1.0: Vectors are codirectional
    fn dot(&self, other: &Self) -> f64 {
        return self.normalized().dot_product(&other.normalized());
    }


    fn dot_product(&self, other: &Self) -> f64;
    fn normalized(&self) -> Self;

    /*
        Accessors
    */
    fn comp(&self, component: char) -> f64;
    fn at(&self, idx: usize) -> f64;

    /// Returns the current vector's values as an array of `f64`
    /// # Examples
    /// ```
    /// use vecx::{Vec2, Vec3, VecX};
    /// 
    /// let v = Vec2(1.0, 2.0);
    /// assert_eq!(v.as_values_of::<Vec3>(), vec![1.0_f64, 2.0_f64, 0.0_f64])
    /// ```
    fn as_values_of<T: VecX>(&self) -> Vec<f64> {
        let mut values = Vec::<f64>::new();
        for i in 0..T::size() {
            values.push(self.at(i))
        }
        return values;
    }

    /// Returns a new Vec2 made of the components of the calling vector
    /// Alias for s::<Vec2>(xy)
    /// # Examples
    /// ```
    /// use vecx::{Vec2, Vec3, VecX};
    /// 
    /// let v2 = Vec2(0.0, 1.0);
    /// let v3 = Vec3(0.0, 1.0, 2.0);
    ///
    /// assert_eq!(v2.s2("xx"), Vec2(0.0, 0.0));
    /// assert_eq!(v3.s2("zz"), Vec2(2.0, 2.0));
    ///
    /// assert_eq!(v3.s2("zx"), Vec2(2.0, 0.0));
    /// assert_eq!(v3.s2("rg"), Vec2(0.0, 1.0));
    /// ```
    fn s2(&self, xy: &str) -> Vec2 {
        self.s::<Vec2>(xy)
    }

    /// Returns a new Vec3 made of the components of the calling vector
    /// Alias for s::<Vec3>(xyz)
    /// # Examples
    /// ```
    /// use vecx::{Vec2, Vec3, VecX};
    /// 
    /// let v2 = Vec2(0.0, 1.0);
    /// let v3 = Vec3(0.0, 1.0, 2.0);
    ///
    /// assert_eq!(v2.s3("xxx"), Vec3(0.0, 0.0, 0.0));
    /// assert_eq!(v2.s3("xyx"), Vec3(0.0, 1.0, 0.0));
    ///
    /// assert_eq!(v3.s3("zyx"), Vec3(2.0, 1.0, 0.0));
    /// assert_eq!(v3.s3("rgb"), Vec3(0.0, 1.0, 2.0));
    /// ```
    fn s3(&self, xyz: &str) -> Vec3 {
        self.s::<Vec3>(xyz)
    }

    /// Returns a new Vec `T` made of the components of the calling vector
    /// # Examples
    /// ```
    /// use vecx::{Vec2, Vec3, VecX};
    /// 
    /// let v2 = Vec2(0.0, 1.0);
    /// let v3: Vec3 = v2.s("xxx");
    ///
    /// assert_eq!(v3, Vec3(0.0, 0.0, 0.0));
    /// ```
    fn s<T: VecX>(&self, swizzle: &str) -> T {
        if !self.is_valid_swizzle::<T>(swizzle) {
            panic!(
                "Invalid swizzle size: expected length {} vs swizzle {} length {}",
                T::size(),
                swizzle,
                swizzle.len(),
            )
        }

        let values = self.swizzle_as_vec64(swizzle);

        return T::from(values);
    }

    fn is_valid_swizzle<T: VecX>(&self, swizzle: &str) -> bool {
        swizzle.len() == T::size()
    }

    fn swizzle_as_vec64(&self, swizzle: &str) -> Vec<f64> {
        let values: Vec<f64> = swizzle
            .chars()
            .map(|c: char| -> f64 {
                return self.comp(c);
            })
            .collect();

        return values;
    }
}
