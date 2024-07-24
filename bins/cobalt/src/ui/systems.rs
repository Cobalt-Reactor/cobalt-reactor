use bevy::prelude::*;
use reactor_core::ui::*;

pub fn spawn_simple_widget(mut commands: Commands) {
    let config = ReactorButtonConfig {
        label: Some("Click me!".to_string()),
        image: Some("ui/button_background.png".into()),
    };

    commands
        .ui_builder(UiRoot)
        .button(config)
        .style()
        .position_type(PositionType::Absolute)
        .right(Val::Px(0.0))
        .top(Val::Px(0.0))
        .entity_commands()
        .pickable(false, true)
        .on_click(|| info!("Button clicked!"));
}
