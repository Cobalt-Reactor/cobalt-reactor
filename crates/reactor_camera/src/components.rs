use bevy::prelude::*;
use reactor_spatial::prelude::*;

/// Marker component for the main camera
#[derive(Component, Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub struct MainCamera;

/// Marker component for the main camera target
#[derive(Component, Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub struct MainCameraShouldTarget;

/// Component that specifies a given cameras target
#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug, Reflect)]
pub struct CameraTarget(pub Option<Entity>);

/// Component that specifies the cameras movement style
#[derive(Component, Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub enum CameraStyle {
    /// Follows the target with a deadzone
    DeadZone(DeadZone),
    /// Follows the target with a screen by screen movement
    ScreenByScreen,
    #[default]
    /// Follows the target exactly
    Exact,
}

/// Sub-component that specifies the cameras deadzone for `CameraStyle::DeadZone`
#[derive(Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub struct DeadZone {
    /// The deadzone's width
    pub width: f32,
    /// The deadzone's height
    pub height: f32,
}

impl DeadZone {
    /// Creates a new deadzone
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Returns the deadzone's half width
    pub fn half_width(&self) -> f32 {
        self.width / 2.
    }

    /// Returns the deadzone's half height
    pub fn half_height(&self) -> f32 {
        self.height / 2.
    }
}

/// Component that specifies the cameras lerp factor
#[derive(Component, Clone, Copy, PartialEq, Debug, Reflect)]
pub struct CameraLerp {
    /// The lerp factor, higher is faster
    pub factor: Vec2,
}

impl CameraLerp {
    /// Creates a new lerp
    pub fn new(factor: f32) -> Self {
        Self {
            factor: Vec2::new(factor, factor),
        }
    }
}

impl Default for CameraLerp {
    fn default() -> Self {
        Self { factor: Vec2::ONE }
    }
}

/// Component that specifies the cameras lead amount
#[derive(Component, Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub struct CameraLead {
    /// The amount to lead the cameras target by
    pub lead_amount: Vec2,
    /// The last position of the camera's target, used to calculate the lead
    pub last_target_position: Option<Position2D>,
}

impl CameraLead {
    /// Creates a new lead
    pub fn new(lead_amount: Vec2) -> Self {
        Self {
            lead_amount,
            last_target_position: None,
        }
    }
}

/// Bundles together the necessary components for a 2D camera
#[derive(Bundle, Default)]
pub struct CameraBundle2D {
    /// 2D spatial bundle
    pub spatial: SpatialBundle2DRaw,
    /// 2D camera bundle from Bevy
    pub camera_bundle: Camera2dBundle,
    /// The camera's style
    pub style: CameraStyle,
    /// The camera's lerp factor
    pub lerp: CameraLerp,
    /// The camera's lead amount
    pub lead: CameraLead,
    /// The camera's target
    pub target: CameraTarget,
}
