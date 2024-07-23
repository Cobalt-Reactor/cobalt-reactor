use super::defines::*;
use crate::components::*;
use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
        texture::GpuImage,
    },
    sprite::Material2d,
};

/// Material used for tracked grids.
/// It will clip beyond a certain distance from the camera, creating the illusion of an infinite
/// grid.
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
#[uniform(0, ClippedLineMaterialUniform)]
pub struct ClippedLineMaterial {
    pub color: Color,
    pub alignment: GridAlignment,
    pub radius: f32,
    pub offset: f32,
    pub x_axis_color: Color,
    pub y_axis_color: Color,
}

impl ClippedLineMaterial {
    pub fn new(
        color: Color,
        alignment: GridAlignment,
        radius: f32,
        offset: f32,
        axis: Option<&GridAxis>,
    ) -> Self {
        let x_axis_color = axis.and_then(|axis| axis.x).unwrap_or(color);
        let y_axis_color = axis.and_then(|axis| axis.y).unwrap_or(color);
        Self {
            color,
            alignment,
            radius,
            offset,
            x_axis_color,
            y_axis_color,
        }
    }
}

/// Uniform for the `ClippedLineMaterial`
#[derive(Clone, Default, ShaderType)]
pub struct ClippedLineMaterialUniform {
    pub color: LinearRgba,
    pub alignment: Vec3,
    pub radius: f32,
    pub offset: f32,
    pub x_axis_color: LinearRgba,
    pub y_axis_color: LinearRgba,
}

impl AsBindGroupShaderType<ClippedLineMaterialUniform> for ClippedLineMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &RenderAssets<GpuImage>,
    ) -> ClippedLineMaterialUniform {
        ClippedLineMaterialUniform {
            color: self.color.into(),
            alignment: self.alignment.into(),
            radius: self.radius,
            offset: self.offset,
            x_axis_color: self.x_axis_color.into(),
            y_axis_color: self.y_axis_color.into(),
        }
    }
}

impl Material2d for ClippedLineMaterial {
    fn fragment_shader() -> ShaderRef {
        CLIPPED_LINE_SHADER_HANDLE.into()
    }
}

/// Simple line material with no functionality beyond assigning a color
#[derive(Default, Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct SimpleLineMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl SimpleLineMaterial {
    /// Construct a `SimpleLineMaterial` from a `LinearRgba` and an `AlphaMode`
    pub const fn from_linear_rgba(color: LinearRgba, _: AlphaMode) -> Self {
        Self { color }
    }

    /// Construct a `SimpleLineMaterial` from a `Color` and an `AlphaMode`
    pub fn from_color(color: Color, _: AlphaMode) -> Self {
        Self {
            color: color.into(),
        }
    }

    /// Set the color using a `Color` instead of an `LinearRgba`
    pub fn set_color(&mut self, color: Color) {
        self.color = color.into();
    }
}

impl Material2d for SimpleLineMaterial {
    fn fragment_shader() -> ShaderRef {
        SIMPLE_LINE_SHADER_HANDLE.into()
    }
}
