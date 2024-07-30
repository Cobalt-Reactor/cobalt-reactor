use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

#[derive(Component)]
struct ReactorGrowableList;

/// Fast way to create a list
pub trait UiReactorGrowableListExt<'w, 's> {
    /// Creates a list.
    fn growable_list(
        &mut self,
        config: ReactorGrowableListConfig,
        content: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl<'w, 's> UiReactorGrowableListExt<'w, 's> for UiBuilder<'_, Entity> {
    /// Creates a list
    /// Returns an `UiBuilder` for further customization.
    fn growable_list(
        &mut self,
        config: ReactorGrowableListConfig,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        let mut view = self.container(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            |list| {
                list.with_background(&config.background);

                list.style()
                    .width(Val::Percent(100.0))
                    .min_width(Val::Percent(100.0))
                    .height(Val::Auto)
                    .min_height(Val::Auto)
                    .justify_self(JustifySelf::Start)
                    .align_self(AlignSelf::Start)
                    .justify_items(JustifyItems::Start)
                    .align_items(AlignItems::Start)
                    .justify_content(JustifyContent::Start)
                    .align_content(AlignContent::Start)
                    .entity_commands()
                    .insert(Name::new("Growable List"))
                    .insert(ReactorGrowableList)
                    .insert(ReactorList);

                spawn_children(list);
            },
        );

        view.style().height(Val::Auto).min_height(Val::Auto);

        view
    }
}

/// Configuration for a list widget.
#[derive(Debug, Clone, Default)]
pub struct ReactorGrowableListConfig {
    /// The background of the list.
    pub background: ReactorBackground,
}
