use crate::{fonts, icons, prelude::*, sickle::prelude::*};
use bevy::prelude::*;

#[derive(Component)]
struct ListItemCollapsible;

/// Fast way to create a list item
pub trait UiListItemCollapsibleExt<'w, 's> {
    /// Creates a list item.
    fn list_item_collapsible(
        &mut self,
        config: ListItemCollapsibleConfig,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl<'w, 's> UiListItemCollapsibleExt<'w, 's> for UiBuilder<'_, Entity> {
    /// Creates a list item
    /// Returns an `UiBuilder` for further customization.
    fn list_item_collapsible(
        &mut self,
        config: ListItemCollapsibleConfig,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        let config = internal_config(&config);

        let mut collapsible = self.collapsible(config, |collapse| {
            spawn_children(collapse);
        });

        collapsible
            .style()
            .justify_items(JustifyItems::Start)
            .align_items(AlignItems::Center)
            .background_color(Color::Srgba(tailwind::GRAY_700))
            .border(UiRect::vertical(Val::Px(2.0)))
            .border_color(Color::Srgba(tailwind::GRAY_900))
            .entity_commands()
            .insert(ListItemCollapsible);

        collapsible
    }
}

/// Configuration for a collapsible list entry.
#[derive(Default, Debug, Clone)]
pub struct ListItemCollapsibleConfig {
    /// The label of the list item.
    pub label: ReactorLabelConfig,
}

fn internal_config(config: &ListItemCollapsibleConfig) -> ReactorCollapsibleConfig {
    ReactorCollapsibleConfig {
        open: true,
        background: ReactorBackground::default(),
        size: ReactorSize {
            width: Val::Percent(100.0).into(),
            height: ReactorSizeType::Set(Val::Px(24.0).into()),
        },
        label: ReactorTextLabelConfig {
            label: config.label.clone(),
            font: Some(ReactorFontConfig {
                size: 18.0,
                color: Color::Srgba(tailwind::GRAY_100),
                font: fonts::STD.into(),
            }),
            ..default()
        },
        expand_icon: icons::EXPAND.into(),
        collapse_icon: icons::COLLAPSE.into(),
    }
}
