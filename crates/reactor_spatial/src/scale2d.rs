use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Component, Reflect)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A 2D scale component
pub struct Scale2D {
    /// Scale on the X axis
    pub x: f32,
    /// Scale on the Y axis
    pub y: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Component, Reflect)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A 2D scale propagation mode
pub enum ScalePropagation {
    #[default]
    /// Scale is relative to the parent
    Relative,
    /// Scale is absolute (does not change scale as the parent scales)
    Absolute,
}

impl Scale2D {
    /// Create a new `Scale2D`
    pub fn new(x: f32, y: f32) -> Self {
        Self::from_f32(x, y)
    }

    fn from_f32(x: f32, y: f32) -> Self {
        assert!(x != 0.0 && y != 0.0);
        Self { x, y }
    }

    /// Create a new `Scale2D` from a `Vec2`
    pub fn from_vec(vec: Vec2) -> Self {
        assert!(vec.x != 0.0 && vec.y != 0.0);
        Self { x: vec.x, y: vec.y }
    }

    /// Returns a new `Scale2D` with the scale flipped horizontally (x scale becomes -x),
    /// useful for sprite flipping
    pub fn flip_horizontal(&self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }

    /// Returns a new `Scale2D` with the scale flipped vertically (y scale becomes -y),
    /// useful for sprite flipping
    pub fn flip_vertical(&self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
        }
    }

    /// Returns a new `Scale2D` with the scale flipped horizontally and vertically,
    /// useful for sprite flipping
    pub fn flip(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    /// Flip the scale horizontally for this `Scale2D`
    pub fn flip_horizontal_mut(&mut self) {
        self.x = -self.x;
    }

    /// Flip the scale vertically for this `Scale2D`
    pub fn flip_vertical_mut(&mut self) {
        self.y = -self.y;
    }

    /// Flip the scale horizontally and vertically for this `Scale2D`
    pub fn flip_mut(&mut self) {
        self.flip_horizontal_mut();
        self.flip_vertical_mut();
    }
}

mod default {
    use super::*;
    impl Default for Scale2D {
        fn default() -> Self {
            Self { x: 1.0, y: 1.0 }
        }
    }
}

mod conversions {
    use super::Scale2D;
    use bevy::prelude::*;

    impl From<Vec2> for Scale2D {
        fn from(value: Vec2) -> Self {
            Self::from_vec(value)
        }
    }

    impl From<&Vec2> for Scale2D {
        fn from(value: &Vec2) -> Self {
            Self::from_vec(*value)
        }
    }

    impl From<Scale2D> for Vec2 {
        fn from(value: Scale2D) -> Self {
            Vec2::new(value.x, value.y)
        }
    }

    impl From<&Scale2D> for Vec2 {
        fn from(value: &Scale2D) -> Self {
            Vec2::new(value.x, value.y)
        }
    }

    impl From<f32> for Scale2D {
        fn from(value: f32) -> Self {
            Self::from_f32(value, value)
        }
    }

    impl From<&f32> for Scale2D {
        fn from(value: &f32) -> Self {
            Self::from_f32(*value, *value)
        }
    }
}

mod operators {
    mod add {
        use crate::prelude::Scale2D;
        use bevy::prelude::*;
        use std::ops::{Add, AddAssign};

        impl Add<Scale2D> for Scale2D {
            type Output = Scale2D;
            fn add(self, rhs: Self) -> Self::Output {
                Scale2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<&Scale2D> for Scale2D {
            type Output = Scale2D;
            fn add(self, rhs: &Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<Scale2D> for &Scale2D {
            type Output = Scale2D;
            fn add(self, rhs: Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<&Scale2D> for &Scale2D {
            type Output = Scale2D;
            fn add(self, rhs: &Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<f32> for Scale2D {
            type Output = Scale2D;
            fn add(self, rhs: f32) -> Self::Output {
                Scale2D::from_f32(self.x + rhs, self.y + rhs)
            }
        }

        impl Add<f32> for &Scale2D {
            type Output = Scale2D;
            fn add(self, rhs: f32) -> Self::Output {
                Scale2D::from_f32(self.x + rhs, self.y + rhs)
            }
        }

        impl Add<Vec2> for Scale2D {
            type Output = Scale2D;
            fn add(self, rhs: Vec2) -> Self::Output {
                Scale2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<Vec2> for &Scale2D {
            type Output = Scale2D;
            fn add(self, rhs: Vec2) -> Self::Output {
                Scale2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<&Vec2> for Scale2D {
            type Output = Scale2D;
            fn add(self, rhs: &Vec2) -> Self::Output {
                Scale2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<&Vec2> for &Scale2D {
            type Output = Scale2D;
            fn add(self, rhs: &Vec2) -> Self::Output {
                Scale2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl AddAssign<Scale2D> for Scale2D {
            fn add_assign(&mut self, rhs: Scale2D) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<&Scale2D> for Scale2D {
            fn add_assign(&mut self, rhs: &Scale2D) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<f32> for Scale2D {
            fn add_assign(&mut self, rhs: f32) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<Vec2> for Scale2D {
            fn add_assign(&mut self, rhs: Vec2) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<&Vec2> for Scale2D {
            fn add_assign(&mut self, rhs: &Vec2) {
                *self = *self + rhs;
            }
        }
    }
    mod sub {
        use crate::prelude::Scale2D;
        use bevy::prelude::*;
        use std::ops::{Sub, SubAssign};

        impl Sub<Scale2D> for Scale2D {
            type Output = Scale2D;
            fn sub(self, rhs: Self) -> Self::Output {
                Scale2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<&Scale2D> for Scale2D {
            type Output = Scale2D;
            fn sub(self, rhs: &Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<Scale2D> for &Scale2D {
            type Output = Scale2D;
            fn sub(self, rhs: Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<&Scale2D> for &Scale2D {
            type Output = Scale2D;
            fn sub(self, rhs: &Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<f32> for Scale2D {
            type Output = Scale2D;
            fn sub(self, rhs: f32) -> Self::Output {
                Scale2D::from_f32(self.x - rhs, self.y - rhs)
            }
        }

        impl Sub<f32> for &Scale2D {
            type Output = Scale2D;
            fn sub(self, rhs: f32) -> Self::Output {
                Scale2D::from_f32(self.x - rhs, self.y - rhs)
            }
        }

        impl Sub<Vec2> for Scale2D {
            type Output = Scale2D;
            fn sub(self, rhs: Vec2) -> Self::Output {
                Scale2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<Vec2> for &Scale2D {
            type Output = Scale2D;
            fn sub(self, rhs: Vec2) -> Self::Output {
                Scale2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<&Vec2> for Scale2D {
            type Output = Scale2D;
            fn sub(self, rhs: &Vec2) -> Self::Output {
                Scale2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<&Vec2> for &Scale2D {
            type Output = Scale2D;
            fn sub(self, rhs: &Vec2) -> Self::Output {
                Scale2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl SubAssign<Scale2D> for Scale2D {
            fn sub_assign(&mut self, rhs: Scale2D) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<&Scale2D> for Scale2D {
            fn sub_assign(&mut self, rhs: &Scale2D) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<f32> for Scale2D {
            fn sub_assign(&mut self, rhs: f32) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<Vec2> for Scale2D {
            fn sub_assign(&mut self, rhs: Vec2) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<&Vec2> for Scale2D {
            fn sub_assign(&mut self, rhs: &Vec2) {
                *self = *self - rhs;
            }
        }
    }
    mod mul {
        use crate::prelude::Scale2D;
        use bevy::prelude::*;
        use std::ops::{Mul, MulAssign};

        impl Mul<Scale2D> for Scale2D {
            type Output = Scale2D;
            fn mul(self, rhs: Self) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<&Scale2D> for Scale2D {
            type Output = Scale2D;
            fn mul(self, rhs: &Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<Scale2D> for &Scale2D {
            type Output = Scale2D;
            fn mul(self, rhs: Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<&Scale2D> for &Scale2D {
            type Output = Scale2D;
            fn mul(self, rhs: &Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<f32> for Scale2D {
            type Output = Scale2D;
            fn mul(self, rhs: f32) -> Self::Output {
                Scale2D::from_f32(self.x * rhs, self.y * rhs)
            }
        }

        impl Mul<f32> for &Scale2D {
            type Output = Scale2D;
            fn mul(self, rhs: f32) -> Self::Output {
                Scale2D::from_f32(self.x * rhs, self.y * rhs)
            }
        }

        impl Mul<Vec2> for Scale2D {
            type Output = Scale2D;
            fn mul(self, rhs: Vec2) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<Vec2> for &Scale2D {
            type Output = Scale2D;
            fn mul(self, rhs: Vec2) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<&Vec2> for Scale2D {
            type Output = Scale2D;
            fn mul(self, rhs: &Vec2) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<&Vec2> for &Scale2D {
            type Output = Scale2D;
            fn mul(self, rhs: &Vec2) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl MulAssign<Scale2D> for Scale2D {
            fn mul_assign(&mut self, rhs: Scale2D) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<&Scale2D> for Scale2D {
            fn mul_assign(&mut self, rhs: &Scale2D) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<f32> for Scale2D {
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<Vec2> for Scale2D {
            fn mul_assign(&mut self, rhs: Vec2) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<&Vec2> for Scale2D {
            fn mul_assign(&mut self, rhs: &Vec2) {
                *self = *self * rhs;
            }
        }
    }
    mod div {
        use crate::prelude::Scale2D;
        use bevy::prelude::*;
        use std::ops::{Div, DivAssign};

        impl Div<Scale2D> for Scale2D {
            type Output = Scale2D;
            fn div(self, rhs: Self) -> Self::Output {
                Scale2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<&Scale2D> for Scale2D {
            type Output = Scale2D;
            fn div(self, rhs: &Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<Scale2D> for &Scale2D {
            type Output = Scale2D;
            fn div(self, rhs: Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<&Scale2D> for &Scale2D {
            type Output = Scale2D;
            fn div(self, rhs: &Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<f32> for Scale2D {
            type Output = Scale2D;
            fn div(self, rhs: f32) -> Self::Output {
                Scale2D::from_f32(self.x / rhs, self.y / rhs)
            }
        }

        impl Div<f32> for &Scale2D {
            type Output = Scale2D;
            fn div(self, rhs: f32) -> Self::Output {
                Scale2D::from_f32(self.x / rhs, self.y / rhs)
            }
        }

        impl Div<Vec2> for Scale2D {
            type Output = Scale2D;
            fn div(self, rhs: Vec2) -> Self::Output {
                Scale2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<Vec2> for &Scale2D {
            type Output = Scale2D;
            fn div(self, rhs: Vec2) -> Self::Output {
                Scale2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<&Vec2> for Scale2D {
            type Output = Scale2D;
            fn div(self, rhs: &Vec2) -> Self::Output {
                Scale2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<&Vec2> for &Scale2D {
            type Output = Scale2D;
            fn div(self, rhs: &Vec2) -> Self::Output {
                Scale2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl DivAssign<Scale2D> for Scale2D {
            fn div_assign(&mut self, rhs: Scale2D) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<&Scale2D> for Scale2D {
            fn div_assign(&mut self, rhs: &Scale2D) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<f32> for Scale2D {
            fn div_assign(&mut self, rhs: f32) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<Vec2> for Scale2D {
            fn div_assign(&mut self, rhs: Vec2) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<&Vec2> for Scale2D {
            fn div_assign(&mut self, rhs: &Vec2) {
                *self = *self / rhs;
            }
        }
    }
    mod rem {
        use crate::prelude::Scale2D;
        use bevy::prelude::*;
        use std::ops::{Rem, RemAssign};

        impl Rem<Scale2D> for Scale2D {
            type Output = Scale2D;
            fn rem(self, rhs: Self) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Rem<&Scale2D> for Scale2D {
            type Output = Scale2D;
            fn rem(self, rhs: &Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Rem<Scale2D> for &Scale2D {
            type Output = Scale2D;
            fn rem(self, rhs: Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Rem<&Scale2D> for &Scale2D {
            type Output = Scale2D;
            fn rem(self, rhs: &Scale2D) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Rem<f32> for Scale2D {
            type Output = Scale2D;
            fn rem(self, rhs: f32) -> Self::Output {
                Scale2D::from_f32(self.x * rhs, self.y * rhs)
            }
        }

        impl Rem<f32> for &Scale2D {
            type Output = Scale2D;
            fn rem(self, rhs: f32) -> Self::Output {
                Scale2D::from_f32(self.x * rhs, self.y * rhs)
            }
        }

        impl Rem<Vec2> for Scale2D {
            type Output = Scale2D;
            fn rem(self, rhs: Vec2) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Rem<Vec2> for &Scale2D {
            type Output = Scale2D;
            fn rem(self, rhs: Vec2) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Rem<&Vec2> for Scale2D {
            type Output = Scale2D;
            fn rem(self, rhs: &Vec2) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Rem<&Vec2> for &Scale2D {
            type Output = Scale2D;
            fn rem(self, rhs: &Vec2) -> Self::Output {
                Scale2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl RemAssign<Scale2D> for Scale2D {
            fn rem_assign(&mut self, rhs: Scale2D) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<&Scale2D> for Scale2D {
            fn rem_assign(&mut self, rhs: &Scale2D) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<f32> for Scale2D {
            fn rem_assign(&mut self, rhs: f32) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<Vec2> for Scale2D {
            fn rem_assign(&mut self, rhs: Vec2) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<&Vec2> for Scale2D {
            fn rem_assign(&mut self, rhs: &Vec2) {
                *self = *self % rhs;
            }
        }
    }
    mod neg {
        use crate::prelude::Scale2D;
        use std::ops::Neg;

        impl Neg for Scale2D {
            type Output = Scale2D;
            fn neg(self) -> Self::Output {
                self.flip()
            }
        }

        impl Neg for &Scale2D {
            type Output = Scale2D;
            fn neg(self) -> Self::Output {
                self.flip()
            }
        }
    }
}
