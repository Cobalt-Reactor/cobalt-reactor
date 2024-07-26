use super::ui_config::perf_panel_ui_config;
use crate::{entries::*, prelude::*};
use bevy::prelude::*;
use reactor_ui::{prelude::*, sickle::prelude::*};

impl ReactorPerfUiWidget for ReactorPerfPanel {
    type Config = PerfPanelConfig;

    fn setup(mut commands: Commands, config: Res<Self::Config>) {
        if config.fps {
            PerfUiEntryFps::setup(commands.reborrow());
        }

        if config.ecs {
            PerfUiEntryEcs::setup(commands.reborrow());
        }

        if config.window {
            PerfUiEntryWindow::setup(commands.reborrow());
        }

        if config.system {
            PerfUiEntrySystem::setup(commands.reborrow());
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
                        PerfUiEntryEcs::spawn(list);
                    }

                    if config.window {
                        PerfUiEntryWindow::spawn(list);
                    }

                    if config.system {
                        PerfUiEntrySystem::spawn(list);
                    }
                });
            });
    }
}
