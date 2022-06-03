use std::ops::{Add, Mul, Sub};
use vecmath::traits::Sqrt;
use vecmath::{vec3_add, vec3_len, vec3_mul, vec3_sub, Vector3};

pub type Vector = VectorT<f64>;
pub type V3 = Vector3<f64>;

#[derive(Debug, Default, Copy, Clone)]
pub struct VectorT<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> From<VectorT<T>> for Vector3<T> {
    fn from(v: VectorT<T>) -> Self {
        [v.x, v.y, v.z]
    }
}

impl<T: Copy> From<Vector3<T>> for VectorT<T> {
    fn from(v: Vector3<T>) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

impl<T> VectorT<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn add(self, other: Self) -> Self
    where
        T: Copy + Add<T, Output = T>,
    {
        vec3_add(self.into(), other.into()).into()
    }

    pub fn sub(self, other: Self) -> Self
    where
        T: Copy + Sub<T, Output = T>,
    {
        vec3_sub(self.into(), other.into()).into()
    }

    pub fn mul(self, other: Self) -> Self
    where
        T: Copy + Mul<T, Output = T>,
    {
        vec3_mul(self.into(), other.into()).into()
    }

    pub fn mul_by(self, other: T) -> Self
    where
        T: Copy + Mul<T, Output = T>,
    {
        vec3_mul(self.into(), [other, other, other]).into()
    }

    pub fn len(self) -> T
    where
        T: Copy + Sqrt + Add<T, Output = T> + Mul<T, Output = T>,
    {
        vec3_len(self.into())
    }
}
