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
            .floating_window(perf_panel_ui_config(), |window| {
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
            .draggable()
            .pickable(false, true)
            .insert(ReactorPerfPanel);
    }
}
