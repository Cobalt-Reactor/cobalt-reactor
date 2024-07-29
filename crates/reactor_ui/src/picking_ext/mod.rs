/// All the extensions that are available for the UI elements.
///
/// The `on_` extensions run a callback every time the associated event listener is triggered. This
/// can be a closure or a function, as described by bevy’s documentation. The only notable
/// difference from Bevy systems is that the callback system can access a resource with event data,
/// `ListenerInput`. You can more easily access this with the system params `Listener` and
/// `ListenerMut`. These callback functions must have exclusive `World` access, and thus can't be
/// used in parallel with other systems. That means that if you are doing a lot of stuff, or doing
/// things every frame it's going to get expensive. Use with caution.
///
/// The `while_` extensions are different, because they run every frame an action is happening
/// rather than sporadically. They must be passed an `Event` that implements
/// `From<ListenerInput<EntityEvent>>`. This must then be handled elsewhere in another system.
///
/// For more information see `bevy_mod_picking`'s documentation.
mod pickable;
pub use pickable::*;
mod draggable;
pub use draggable::*;

mod on_click;
pub use on_click::*;
mod on_released;
pub use on_released::*;
mod on_pressed;
pub use on_pressed::*;

mod on_hover_start;
pub use on_hover_start::*;
mod on_hover_end;
pub use on_hover_end::*;
mod while_hovered;
pub use while_hovered::*;

mod on_drag_start;
pub use on_drag_start::*;
mod on_drag_end;
pub use on_drag_end::*;
mod while_dragged;
pub use while_dragged::*;

mod on_draggable_entered;
pub use on_draggable_entered::*;
mod on_draggable_exited;
pub use on_draggable_exited::*;
mod on_draggable_dropped;
pub use on_draggable_dropped::*;
mod while_dragged_over;
pub use while_dragged_over::*;

mod on_pointer_lost;
pub use on_pointer_lost::*;
