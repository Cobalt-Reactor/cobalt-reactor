use crate::{fonts, icons};
use bevy::prelude::*;
use reactor_ui::{prelude::*, sickle::prelude::*};

#[derive(Component)]
struct PanelEntryCollapsible;

/// Fast way to create a list item
pub trait UiPanelEntryCollapsibleExt<'w, 's> {
    /// Creates a list item.
    fn panel_entry_collapsible(
        &mut self,
        config: PanelEntryCollapsibleConfig,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl<'w, 's> UiPanelEntryCollapsibleExt<'w, 's> for UiBuilder<'_, Entity> {
    /// Creates a list item
    /// Returns an `UiBuilder` for further customization.
    fn panel_entry_collapsible(
        &mut self,
        config: PanelEntryCollapsibleConfig,
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
            .insert(PanelEntryCollapsible);

        collapsible
    }
}

/// Configuration for a list item widget.
#[derive(Default, Debug, Clone)]
pub struct PanelEntryCollapsibleConfig {
    pub label: ReactorLabelConfig,
}

fn internal_config(config: &PanelEntryCollapsibleConfig) -> ReactorCollapsibleConfig {
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
