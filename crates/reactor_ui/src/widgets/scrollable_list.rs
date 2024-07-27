use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

#[derive(Component)]
struct ReactorScrollableList;

/// Fast way to create a list
pub trait UiReactorScrollableListExt<'w, 's> {
    /// Creates a list.
    fn scrollable_list(
        &mut self,
        config: ReactorScrollableListConfig,
        content: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl<'w, 's> UiReactorScrollableListExt<'w, 's> for UiBuilder<'_, Entity> {
    /// Creates a list
    /// Returns an `UiBuilder` for further customization.
    fn scrollable_list(
        &mut self,
        config: ReactorScrollableListConfig,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        let mut view = self.scroll_view(Some(ScrollAxis::Vertical), |scroll| {
            scroll.with_background(&config.background);

            scroll
                .style()
                .with_size(&config.size)
                .width(Val::Percent(100.0))
                .min_width(Val::Percent(100.0))
                .justify_self(JustifySelf::Start)
                .align_self(AlignSelf::Start)
                .justify_items(JustifyItems::Start)
                .align_items(AlignItems::Start)
                .entity_commands()
                .insert(ReactorScrollableList);

            spawn_children(scroll);
        });

        view.style().height(Val::Auto).min_height(Val::Auto);

        view
    }
}

/// Configuration for a list widget.
#[derive(Debug, Clone, Default)]
pub struct ReactorScrollableListConfig {
    /// The background of the list.
    pub background: ReactorBackground,
    /// The size of the list (width is overridden to 100%)
    pub size: ReactorSize,
}
