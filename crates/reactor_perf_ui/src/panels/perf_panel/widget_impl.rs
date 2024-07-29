use crate::{entries::*, prelude::*};
use bevy::prelude::*;
use reactor_ui::{prelude::*, sickle::prelude::*};

impl ReactorPerfUiPanel for ReactorPerfPanel {
    type Config = PerfPanelConfig;

    fn setup(app: &mut App, config: Self::Config) {
        if config.fps {
            PerfUiEntryFps::setup(app);
        }

        if config.time {
            PerfUiEntryTime::setup(app);
        }

        if config.window {
            PerfUiEntryWindow::setup(app);
        }

        #[cfg(feature = "sysinfo")]
        if config.system {
            PerfUiEntrySystem::setup(app);
        }

        if config.ecs {
            PerfUiEntryEcs::setup(app);
        }
    }

    fn spawn(mut commands: Commands, config: Res<Self::Config>) {
        commands
            .ui_builder(UiRoot)
            .floating_window(internal_config(), |window| {
                window.growable_list(ReactorGrowableListConfig::default(), |list| {
                    if config.fps {
                        PerfUiEntryFps::spawn(list);
                    }

                    if config.time {
                        PerfUiEntryTime::spawn(list);
                    }

                    if config.window {
                        PerfUiEntryWindow::spawn(list);
                    }

                    #[cfg(feature = "sysinfo")]
                    if config.system {
                        PerfUiEntrySystem::spawn(list);
                    }

                    if config.ecs {
                        PerfUiEntryEcs::spawn(list);
                    }
                });
            })
            .entity_commands()
            .insert(ReactorPerfPanel);
    }
}

fn internal_config() -> ReactorFloatingWindowConfig {
    ReactorFloatingWindowConfig {
        draggable: true,
        size: ReactorSize {
            width: Val::Px(290.0).into(),
            height: Val::Auto.into(),
        },
        position: ReactorPosition::Absolute(ReactorPositionAbsolute { x: 32.0, y: 32.0 }),
        background: ReactorBackground::Flat(ReactorFlatBackground {
            background_color: Some(Color::Srgba(tailwind::GRAY_600)),
            corner_radius: Some(ReactorCornerRadius::from(10.0)),
            border_config: Some(ReactorBorder {
                border_color: Color::Srgba(tailwind::GRAY_900),
                border_width: UiRect::all(Val::Px(2.0)),
            }),
        }),
        header_config: Some(ReactorHeaderConfig {
            label: "Performance".into(),
            font: Some(ReactorFontConfig {
                size: 16.0,
                color: Color::Srgba(tailwind::GRAY_50),
                font: fonts::STD.into(),
            }),
        }),
    }
}
