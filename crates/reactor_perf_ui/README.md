# `reactor_perf_ui`

Performance windows using Sickle UI via Reactor UI

## Usage

Add the `ReactorPerfUiPlugin` to your app. Configure it with the built-in functions.

If you find the windows don't appear correctly you may have to use `configure_sets` to control setup, spawning, and updating. For example:

```rust
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Running,
    Exiting,
}

pub fn configure_sets(&self, app: &mut App) {
    app.configure_sets(
        Update,
        ReactorPerfUiSchedule::Setup.run_if(in_state(GameState::Loading)),
    ).configure_sets(
        Update,
        ReactorPerfUiSchedule::Spawn.run_if(in_state(GameState::Loading)),
    ).configure_sets(
        Update,
        ReactorPerfUiSchedule::Update.run_if(in_state(GameState::Running)),
    );
}
```
