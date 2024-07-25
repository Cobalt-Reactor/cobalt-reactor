use super::ui_config::perf_panel_ui_config;
use crate::{entries::*, prelude::*};
use bevy::prelude::*;
use reactor_ui::{prelude::*, sickle::prelude::*};

impl ReactorPerfUiWidget for ReactorPerfPanel {
    type Config = PerfPanelConfig;

    fn setup(&self, app: &mut App, config: Self::Config) {
        if config.fps {
            PerfUiEntryFps::setup(app);
        }

        if config.ecs {
            PerfUiEntryEcs::setup(app);
        }

        if config.window {
            PerfUiEntryWindow::setup(app);
        }

        if config.system {
            PerfUiEntrySystem::setup(app);
        }
    }

    fn spawn(mut commands: Commands, _: bevy::prelude::Res<Self::Config>) {
        commands
            .ui_builder(UiRoot)
            .floating_window(perf_panel_ui_config())
            .row(|row| {
                row.style()
                    .width(Val::Percent(100.0))
                    .min_width(Val::Percent(100.0))
                    .justify_content(JustifyContent::Center);
            });
    }

    // fn update(
    //     entity: bevy::prelude::Entity,
    //     param: &mut <Self::SystemParamUpdate as bevy::ecs::system::SystemParam>::Item<'_, '_>,
    // ) {
    //     todo!()
    // }
}
