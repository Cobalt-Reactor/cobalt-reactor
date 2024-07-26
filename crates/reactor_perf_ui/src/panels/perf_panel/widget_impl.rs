use super::ui_config::perf_panel_ui_config;
use crate::{entries::*, prelude::*};
use bevy::prelude::*;
use reactor_ui::{prelude::*, sickle::prelude::*};

impl ReactorPerfUiPanel for ReactorPerfPanel {
    type Config = PerfPanelConfig;

    fn setup(app: &mut App, config: Self::Config) {
        if config.fps {
            PerfUiEntryFps::setup(app);
        }

        if config.ecs {
            PerfUiEntryEngine::setup(app);
        }

        if config.window {
            PerfUiEntryWindow::setup(app);
        }

        if config.system {
            PerfUiEntrySystem::setup(app);
        }
    }

    fn spawn(mut commands: Commands, config: Res<Self::Config>) {
        commands
            .ui_builder(UiRoot)
            .floating_window(perf_panel_ui_config(), |window| {
                window.list(ReactorListConfig::default(), |list| {
                    if config.fps {
                        PerfUiEntryFps::spawn(list);
                    }

                    if config.ecs {
                        PerfUiEntryEngine::spawn(list);
                    }

                    if config.window {
                        PerfUiEntryWindow::spawn(list);
                    }

                    if config.system {
                        PerfUiEntrySystem::spawn(list);
                    }
                });
            })
            .entity_commands()
            .insert(ReactorPerfPanel);
    }
}
