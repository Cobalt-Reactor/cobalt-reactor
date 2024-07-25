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
            .floating_window(perf_panel_ui_config())
            .row(|row| {
                row.style()
                    .width(Val::Percent(100.0))
                    .min_width(Val::Percent(100.0));

                if config.fps {
                    PerfUiEntryFps::spawn(row);
                }

                if config.ecs {
                    PerfUiEntryEcs::spawn(row);
                }

                if config.window {
                    PerfUiEntryWindow::spawn(row);
                }

                if config.system {
                    PerfUiEntrySystem::spawn(row);
                }
            });
    }
}
