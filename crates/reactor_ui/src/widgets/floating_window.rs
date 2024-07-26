use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

#[derive(Component)]
struct ReactorFloatingWindow;

/// Fast way to create a floating window
pub trait UiReactorFloatingWindowExt<'w, 's> {
    /// Creates a floating window.
    fn floating_window(
        &mut self,
        config: ReactorFloatingWindowConfig,
        content: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl<'w, 's> UiReactorFloatingWindowExt<'w, 's> for UiBuilder<'_, UiRoot> {
    /// Creates a floating window
    /// Returns an `UiBuilder` for further customization.
    fn floating_window(
        &mut self,
        config: ReactorFloatingWindowConfig,
        content_builder: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        let base = ReactorBaseConfig {
            position: config.position.clone(),
            size: config.size.clone(),
            alignment: ReactorAlignment {
                child_alignment: Some(ReactorChildAlignment {
                    horizontal: Some(JustifyItems::Start),
                    vertical: Some(AlignItems::Start),
                    horizontal_distribution: Some(JustifyContent::SpaceEvenly),
                    vertical_distribution: Some(AlignContent::Start),
                }),
                ..default()
            },
            picking: None,
        };

        self.container(ReactorFloatingWindow, |window| {
            window
                .with_background(&config.background)
                .with_base_config(&base);

            if let Some(header) = config.header_config {
                window.column(|column| {
                    column
                        .style()
                        .width(Val::Percent(100.0))
                        .min_width(Val::Percent(100.0));

                    column.row(|row| {
                        row.style()
                            .width(Val::Percent(100.0))
                            .min_width(Val::Percent(100.0))
                            .background_color(Color::srgba(0.0, 0.0, 0.0, 0.7))
                            .justify_content(JustifyContent::Center);

                        if let ReactorBackground::Flat(flat) = config.background {
                            if let Some(corner_radius) = flat.corner_radius {
                                row.style().border_tl_radius(Val::Px(f32::max(
                                    corner_radius.tl - 2.0,
                                    0.0,
                                )));
                                row.style().border_tr_radius(Val::Px(f32::max(
                                    corner_radius.tr - 2.0,
                                    0.0,
                                )));
                            }
                        }

                        let mut label = row.label(header.label.into());

                        if let Some(font) = header.font {
                            label.entity_commands().with_font(font);
                        }
                    });
                    content_builder(column);
                });
            }
        })
    }
}

/// Configuration for a floating window widget.
#[derive(Default, Debug, Clone)]
pub struct ReactorFloatingWindowConfig {
    /// The background of the floating window.
    pub background: ReactorBackground,
    /// The position of the floating window.
    pub position: ReactorPosition,
    /// The size of the floating window.
    pub size: ReactorSize,
    /// Config for the header, if provided a header will be added to the window, displaying
    /// the name.
    pub header_config: Option<ReactorHeaderConfig>,
}

/// Configuration for a button widget.
#[derive(Default, Debug, Clone)]
pub struct ReactorHeaderConfig {
    /// The text label to display on the button.
    pub label: ReactorLabelConfig,
    /// The font config for the button's label.
    pub font: Option<ReactorFontConfig>,
}
