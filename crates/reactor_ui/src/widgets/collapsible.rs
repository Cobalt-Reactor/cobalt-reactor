use crate::{picking::prelude::*, prelude::*, sickle::prelude::*};
use bevy::{prelude::*, ui::FocusPolicy};

/// A collapsible item (mostly for use in lists)
#[derive(Component, Debug)]
pub struct ReactorCollapsible {
    open: bool,
    collapsed_icon: Entity,
    expanded_icon: Entity,
    container: Entity,
}

/// Fast way to create a list item
pub trait UiReactorCollapsibleExt<'w, 's> {
    /// Creates a list item.
    fn collapsible(
        &mut self,
        config: ReactorCollapsibleConfig,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl<'w, 's> UiReactorCollapsibleExt<'w, 's> for UiBuilder<'_, Entity> {
    /// Creates a list item
    /// Returns an `UiBuilder` for further customization.
    fn collapsible(
        &mut self,
        config: ReactorCollapsibleConfig,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        let open = config.open;
        let name = config.label.label.text.clone();
        let mut collapsed_icon = None;
        let mut expanded_icon = None;

        // Fills width, has an icon, can be clicked, when clicked flips visibility of children
        let button = self
            .container(
                (
                    Name::new(format!("Foldable [{}] - Button", name)),
                    ButtonBundle {
                        focus_policy: FocusPolicy::Pass,
                        ..default()
                    },
                ),
                |button| {
                    let mut collapsed = button.icon(config.expand_icon);
                    collapsed.style().hide();
                    if !config.open {
                        collapsed.style().show();
                    }
                    collapsed_icon = Some(collapsed.id());

                    let mut expanded = button.icon(config.collapse_icon);
                    expanded.style().hide();
                    if config.open {
                        expanded.style().show();
                    }
                    expanded_icon = Some(expanded.id());

                    button
                        .label(config.label.label.into())
                        .style()
                        .margin(UiRect::left(Val::Px(10.0)))
                        .entity_commands()
                        .with_font(config.label.font.as_ref().unwrap().clone());
                },
            )
            .style()
            .with_size(&config.size)
            .entity_commands()
            .pickable(true, false)
            .on_click_event::<CollapsibleClicked>()
            .id();

        // Make a child container
        let mut container = self.panel("".to_string(), spawn_children);
        let container_id = container.id();
        container
            .style()
            //.margin(UiRect::left(Val::Px(20.0)))
            .height(Val::Auto)
            .justify_self(JustifySelf::Stretch)
            .justify_content(JustifyContent::Stretch)
            .justify_items(JustifyItems::Stretch);
        if !config.open {
            self.commands().style(container_id).hide();
        }

        let collapsible = ReactorCollapsible {
            open,
            collapsed_icon: collapsed_icon.unwrap(),
            expanded_icon: expanded_icon.unwrap(),
            container: container_id,
        };

        self.commands().entity(button).insert(collapsible);
        self.commands().ui_builder(button)
    }
}

/// Configuration for a list item widget.
#[derive(Debug, Clone)]
pub struct ReactorCollapsibleConfig {
    /// Whether the container starts open or not
    pub open: bool,
    /// The background of the list item.
    pub background: ReactorBackground,
    /// The size of the list item
    pub size: ReactorSize,
    /// The text label for the collapsible
    pub label: ReactorTextLabelConfig,
    /// The icon to use for the collapsible, when collapsed
    pub expand_icon: String,
    /// The icon to use for the collapsible, when expanded
    pub collapse_icon: String,
}

#[derive(Event, Debug)]
pub(crate) struct CollapsibleClicked {
    entity: Entity,
    button: PointerButton,
}

impl From<ListenerInput<Pointer<Click>>> for CollapsibleClicked {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        CollapsibleClicked {
            entity: event.listener(),
            button: event.event.button,
        }
    }
}

pub(crate) fn handle_collapsible_clicked(
    mut commands: Commands,
    mut query: Query<&mut ReactorCollapsible>,
    mut events: EventReader<CollapsibleClicked>,
) {
    for event in events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }

        if let Ok(mut rc) = query.get_mut(event.entity) {
            rc.open = !rc.open;
            match rc.open {
                true => {
                    commands.style(rc.container).show();
                    commands.style(rc.collapsed_icon).hide();
                    commands.style(rc.expanded_icon).show();
                }
                false => {
                    commands.style(rc.container).hide();
                    commands.style(rc.collapsed_icon).show();
                    commands.style(rc.expanded_icon).hide();
                }
            };
        }
    }
}
