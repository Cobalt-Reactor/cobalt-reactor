use crate::prelude::*;
use reactor_random::prelude::*;

impl Random for Degrees {
    fn random() -> Degrees {
        Degrees::from(f32::random() * 360.0)
    }
}

impl Random for Radians {
    fn random() -> Radians {
        Radians::from(f32::random() * 2.0 * std::f32::consts::PI)
    }
}

impl Random for Compass {
    fn random() -> Compass {
        Compass::from(Degrees::from(f32::random() * 360.0))
    }
}

impl Random for CompassRose {
    fn random() -> CompassRose {
        CompassRose::from(Degrees::from(f32::random() * 360.0))
    }
}

impl Random for CompassHalfwinds {
    fn random() -> CompassHalfwinds {
        CompassHalfwinds::from(Degrees::from(f32::random() * 360.0))
    }
}

impl Random for Position2D {
    fn random() -> Position2D {
        let x = f32::random() * f32::MIN + f32::random() * f32::MAX;
        let y = f32::random() * f32::MIN + f32::random() * f32::MAX;

        Position2D::new(x, y)
    }
}

impl RandomRange for Degrees {
    fn random_range(min: Degrees, max: Degrees) -> Degrees {
        let diff = max - min;
        min + (diff * f32::random())
    }
}

impl RandomRange for Radians {
    fn random_range(min: Radians, max: Radians) -> Radians {
        let diff = max - min;
        min + (diff * f32::random())
    }
}

impl RandomRange for Position2D {
    fn random_range(min: Position2D, max: Position2D) -> Position2D {
        let diff = max - min;
        min + (diff * Position2D::new(f32::random(), f32::random()))
    }
}
