use bevy::{color::palettes::tailwind, prelude::*};
use reactor_core::ui::*;

pub fn spawn_simple_widget(mut commands: Commands) {
    // Could probably turn this into a prototype, and load buttons from a config file
    let config = ReactorButtonConfig {
        label: Some(ReactorTextLabelConfig {
            label: LabelConfig {
                label: "Click me!".to_string(),
                ..default()
            },
            font: Some(ReactorFontConfig {
                path: "fonts/std.ttf".to_string(),
                size: 24.0,
                color: Color::Srgba(tailwind::GRAY_950),
            }),
            ..default()
        }),
        image: Some("ui/button_background.png".into()),
        base_config: ReactorBaseConfig {
            picking: Some(ReactorPicking {
                block_lower: false,
                hoverable: true,
            }),
            size: ReactorSize {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
            },
            alignment: ReactorAlignment {
                self_alignment: Some(ReactorSelfAlignment {
                    horizontal: Some(JustifySelf::Center),
                    vertical: Some(AlignSelf::Center),
                }),
                child_alignment: Some(ReactorChildAlignment {
                    horizontal: Some(JustifyItems::Center),
                    vertical: Some(AlignItems::Center),
                    horizontal_distribution: Some(JustifyContent::Center),
                    vertical_distribution: Some(AlignContent::FlexStart),
                }),
            },
            ..default()
        },
    };

    commands
        .ui_builder(UiRoot)
        .button(config)
        .entity_commands()
        .on_click(|| info!("Button clicked!"));
}
