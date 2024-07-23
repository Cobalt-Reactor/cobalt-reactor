use super::events::CleanupComplete;
use bevy::prelude::*;

pub(super) fn signal_cleanup_complete(mut writer: EventWriter<CleanupComplete>) {
    writer.send(CleanupComplete);
}

pub(super) fn exit_app(mut writer: EventWriter<AppExit>) {
    writer.send(AppExit::Success);
}
