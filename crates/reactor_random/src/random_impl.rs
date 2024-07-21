use crate::prelude::{Random, RandomContainer};

impl Random for bool {
    fn random() -> bool {
        fastrand::bool()
    }
}

impl Random for f32 {
    fn random() -> f32 {
        fastrand::f32()
    }
}

impl Random for f64 {
    fn random() -> f64 {
        fastrand::f64()
    }
}

impl Random for i8 {
    fn random() -> i8 {
        fastrand::i8(..)
    }
}

impl Random for i16 {
    fn random() -> i16 {
        fastrand::i16(..)
    }
}

impl Random for i32 {
    fn random() -> i32 {
        fastrand::i32(..)
    }
}

impl Random for i64 {
    fn random() -> i64 {
        fastrand::i64(..)
    }
}

impl Random for i128 {
    fn random() -> i128 {
        fastrand::i128(..)
    }
}

impl Random for isize {
    fn random() -> isize {
        fastrand::isize(..)
    }
}

impl Random for u8 {
    fn random() -> u8 {
        fastrand::u8(..)
    }
}

impl Random for u16 {
    fn random() -> u16 {
        fastrand::u16(..)
    }
}

impl Random for u32 {
    fn random() -> u32 {
        fastrand::u32(..)
    }
}

impl Random for u64 {
    fn random() -> u64 {
        fastrand::u64(..)
    }
}

impl Random for u128 {
    fn random() -> u128 {
        fastrand::u128(..)
    }
}

impl Random for usize {
    fn random() -> usize {
        fastrand::usize(..)
    }
}

impl<T, U> RandomContainer<T> for U
where
    U: Clone + IntoIterator<Item = T>,
    T: Clone,
{
}

#[cfg(feature = "bevy")]
mod bevy {
    use crate::prelude::Random;
    use bevy::prelude::*;

    impl Random for Color {
        fn random() -> Color {
            Color::srgba(f32::random(), f32::random(), f32::random(), 1.0)
        }
    }

    impl Random for Vec2 {
        fn random() -> Vec2 {
            let x = fastrand::f32() * f32::MIN + fastrand::f32() * f32::MAX;
            let y = fastrand::f32() * f32::MIN + fastrand::f32() * f32::MAX;
            Vec2::new(x, y)
        }
    }

    impl Random for Vec3 {
        fn random() -> Vec3 {
            let x = fastrand::f32() * f32::MIN + fastrand::f32() * f32::MAX;
            let y = fastrand::f32() * f32::MIN + fastrand::f32() * f32::MAX;
            let z = fastrand::f32() * f32::MIN + fastrand::f32() * f32::MAX;
            Vec3::new(x, y, z)
        }
    }

    impl Random for Vec4 {
        fn random() -> Vec4 {
            let x = fastrand::f32() * f32::MIN + fastrand::f32() * f32::MAX;
            let y = fastrand::f32() * f32::MIN + fastrand::f32() * f32::MAX;
            let z = fastrand::f32() * f32::MIN + fastrand::f32() * f32::MAX;
            let w = fastrand::f32() * f32::MIN + fastrand::f32() * f32::MAX;
            Vec4::new(x, y, z, w)
        }
    }

    impl Random for IVec2 {
        fn random() -> IVec2 {
            let x = fastrand::i32(i32::MIN..i32::MAX);
            let y = fastrand::i32(i32::MIN..i32::MAX);
            IVec2::new(x, y)
        }
    }

    impl Random for IVec3 {
        fn random() -> IVec3 {
            let x = fastrand::i32(i32::MIN..i32::MAX);
            let y = fastrand::i32(i32::MIN..i32::MAX);
            let z = fastrand::i32(i32::MIN..i32::MAX);
            IVec3::new(x, y, z)
        }
    }

    impl Random for IVec4 {
        fn random() -> IVec4 {
            let x = fastrand::i32(i32::MIN..i32::MAX);
            let y = fastrand::i32(i32::MIN..i32::MAX);
            let z = fastrand::i32(i32::MIN..i32::MAX);
            let w = fastrand::i32(i32::MIN..i32::MAX);
            IVec4::new(x, y, z, w)
        }
    }

    impl Random for UVec2 {
        fn random() -> UVec2 {
            let x = fastrand::u32(u32::MIN..u32::MAX);
            let y = fastrand::u32(u32::MIN..u32::MAX);
            UVec2::new(x, y)
        }
    }

    impl Random for UVec3 {
        fn random() -> UVec3 {
            let x = fastrand::u32(u32::MIN..u32::MAX);
            let y = fastrand::u32(u32::MIN..u32::MAX);
            let z = fastrand::u32(u32::MIN..u32::MAX);
            UVec3::new(x, y, z)
        }
    }

    impl Random for UVec4 {
        fn random() -> UVec4 {
            let x = fastrand::u32(u32::MIN..u32::MAX);
            let y = fastrand::u32(u32::MIN..u32::MAX);
            let z = fastrand::u32(u32::MIN..u32::MAX);
            let w = fastrand::u32(u32::MIN..u32::MAX);
            UVec4::new(x, y, z, w)
        }
    }
}
