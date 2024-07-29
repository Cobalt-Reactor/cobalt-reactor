use crate::{
    fonts,
    prelude::*,
    sickle::{prelude::*, widgets::layout::label::SetLabelTextExt},
};
use bevy::prelude::*;

#[derive(Component)]
struct PanelEntryText;

/// Fast way to create a list item
pub trait UiPanelEntryTextExt<'w, 's, T: Bundle, C: Bundle> {
    /// Creates a list item.
    fn list_item_two_text(&mut self, config: ListItemTwoTextConfig<T, C>) -> UiBuilder<Entity>;
}

impl<'w, 's, T, C> UiPanelEntryTextExt<'w, 's, T, C> for UiBuilder<'_, Entity>
where
    T: Bundle,
    C: Bundle,
{
    /// Creates a list item
    /// Returns an `UiBuilder` for further customization.
    fn list_item_two_text(&mut self, config: ListItemTwoTextConfig<T, C>) -> UiBuilder<Entity> {
        let internal_config = internal_config();

        self.list_item(internal_config.clone(), |item| {
            item.with_background(&internal_config.background)
                .style()
                .entity_commands()
                .insert(PanelEntryText);

            item.column(|col| {
                col.style()
                    .with_size(&internal_config.size)
                    .width(Val::Percent(50.0))
                    .min_width(Val::Percent(50.0))
                    .border_color(Color::Srgba(tailwind::GRAY_800))
                    .border(UiRect::left(Val::Px(2.0)));

                col.label(LabelConfig::default())
                    .style()
                    .width(Val::Percent(90.0))
                    .with_size(&internal_config.size)
                    .margin(UiRect::new(
                        Val::Px(10.0),
                        Val::Px(0.0),
                        Val::Px(2.0),
                        Val::Px(2.0),
                    ))
                    .entity_commands()
                    .set_label_text(config.title_text)
                    .with_font(ReactorFontConfig {
                        font: fonts::SQUARE.into(),
                        size: 16.0,
                        color: Color::Srgba(tailwind::GRAY_100),
                    })
                    .insert(config.title_component);
            });

            item.column(|col| {
                col.style()
                    .with_size(&internal_config.size)
                    .width(Val::Percent(50.0))
                    .min_width(Val::Percent(50.0))
                    .border(UiRect {
                        left: Val::Px(2.0),
                        right: Val::Px(1.0),
                        top: Val::Px(1.0),
                        bottom: Val::Px(1.0),
                    })
                    .border_color(Color::Srgba(tailwind::GRAY_800));

                col.label(LabelConfig::default())
                    .style()
                    .width(Val::Percent(90.0))
                    .with_size(&internal_config.size)
                    .margin(UiRect::new(
                        Val::Px(10.0),
                        Val::Px(0.0),
                        Val::Px(2.0),
                        Val::Px(2.0),
                    ))
                    .entity_commands()
                    .set_label_text(config.content_text)
                    .with_font(ReactorFontConfig {
                        font: fonts::SQUARE.into(),
                        size: 16.0,
                        color: Color::Srgba(tailwind::GRAY_100),
                    })
                    .insert(config.content_component);
            });
        })
    }
}

/// Configuration for a list item widget.
#[derive(Default, Debug, Clone)]
pub struct ListItemTwoTextConfig<T, C>
where
    T: Bundle,
    C: Bundle,
{
    /// The name of the list item, to appear in the left hand column
    pub title_text: String,
    /// The marker component of the list item title
    pub title_component: T,
    /// The content of the list item, to appear in the right hand column
    pub content_text: String,
    /// The marker component of the list item content
    pub content_component: C,
}

fn internal_config() -> ReactorListItemConfig {
    ReactorListItemConfig {
        background: ReactorBackground::Flat(ReactorFlatBackground {
            border_config: Some(ReactorBorder {
                border_color: Color::Srgba(tailwind::GRAY_900),
                border_width: UiRect::bottom(Val::Px(2.0)),
            }),
            ..default()
        }),
        size: ReactorSize {
            height: Val::Px(20.0).into(),
            ..default()
        },
    }
}
