use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Makes a widget draggable.
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait UiDraggableExt<'a> {
    /// Creates a button.
    fn draggable(&mut self) -> &mut EntityCommands<'a>;
}

impl<'a> UiDraggableExt<'a> for EntityCommands<'a> {
    fn draggable(&mut self) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Drag>>::listener_component_mut::<Style>(
            |drag, style| {
                if drag.button != PointerButton::Primary {
                    info!("Only primary button can be dragged");
                    return;
                }

                if style.position_type != PositionType::Absolute {
                    error!("Only absolute position can be dragged");
                    return;
                }

                match style.left {
                    Val::Px(current) => {
                        style.left = Val::Px(current + drag.delta.x);
                    }
                    _ => {
                        error!(
                            "Only absolute left position can be dragged: {:?}",
                            style.left
                        );
                    }
                }

                match style.top {
                    Val::Px(current) => {
                        style.top = Val::Px(current + drag.delta.y);
                    }
                    _ => {
                        error!("Only absolute top position can be dragged: {:?}", style.top);
                    }
                }
            },
        ))
    }
}
