use std::{fmt::Display, ops::Sub};

use crate::{Vec2, Vec3};

pub trait VecX:
    Copy + Sub<Self, Output = Self> + From<f64> + From<Vec2> + From<Vec3> + From<Vec<f64>> + Display
{
    fn size() -> usize
    where
        Self: Sized;

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

    fn as_values_of<T: VecX>(&self) -> Vec<f64> {
        let mut values = Vec::<f64>::new();
        for i in 0..T::size() {
            values.push(self.at(i))
        }
        return values;
    }

    /*
        Swizzling
    */
    fn s2(&self, s: &str) -> Vec2 {
        self.s::<Vec2>(s)
    }

    fn s3(&self, s: &str) -> Vec3 {
        self.s::<Vec3>(s)
    }

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
