use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

#[derive(Component)]
struct ReactorList;

/// Fast way to create a list
pub trait UiReactorListExt<'w, 's> {
    /// Creates a list.
    fn list(
        &mut self,
        config: ReactorListConfig,
        content: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl<'w, 's> UiReactorListExt<'w, 's> for UiBuilder<'_, Entity> {
    /// Creates a list
    /// Returns an `UiBuilder` for further customization.
    fn list(
        &mut self,
        config: ReactorListConfig,
        content_builder: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        self.scroll_view(Some(ScrollAxis::Vertical), |scroll| {
            scroll.with_background(&config.background);

            scroll
                .style()
                .width(Val::Percent(100.0))
                .min_width(Val::Percent(100.0))
                .justify_self(JustifySelf::Start)
                .align_self(AlignSelf::Start)
                .justify_items(JustifyItems::Start)
                .align_items(AlignItems::Start)
                .entity_commands()
                .insert(ReactorList);

            content_builder(scroll);
        })
    }
}

/// Configuration for a list widget.
#[derive(Default, Debug, Clone)]
pub struct ReactorListConfig {
    /// The background of the list.
    pub background: ReactorBackground,
}
