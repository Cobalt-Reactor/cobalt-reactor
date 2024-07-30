use crate::{
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
        let internal_config = config.list_item_config;

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
                    .border(UiRect::left(Val::Px(2.0)))
                    .entity_commands()
                    .insert(Name::new(format!("{} Title", config.title_text.clone())));

                let mut title = col.label(LabelConfig::default());
                title
                    .style()
                    .width(Val::Percent(90.0))
                    .with_size(&internal_config.size)
                    .margin(UiRect::new(
                        Val::Px(10.0),
                        Val::Px(0.0),
                        Val::Px(2.0),
                        Val::Px(2.0),
                    ))
                    .align_self(AlignSelf::Center)
                    .entity_commands()
                    .set_label_text(config.title_text.clone())
                    .insert(config.title_component);

                if let Some(font) = config.title_font {
                    title.entity_commands().with_font(font);
                }
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
                    .border_color(Color::Srgba(tailwind::GRAY_800))
                    .entity_commands()
                    .insert(Name::new(format!("{} Content", &config.title_text.clone())));

                let mut content = col.label(LabelConfig::default());
                content
                    .style()
                    .width(Val::Percent(90.0))
                    .with_size(&internal_config.size)
                    .margin(UiRect::new(
                        Val::Px(10.0),
                        Val::Px(0.0),
                        Val::Px(2.0),
                        Val::Px(2.0),
                    ))
                    .align_self(AlignSelf::Center)
                    .entity_commands()
                    .set_label_text(config.content_text)
                    .insert(config.content_component);

                if let Some(font) = config.content_font {
                    content.entity_commands().with_font(font);
                }
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
    /// Config for the underlying list item
    pub list_item_config: ReactorListItemConfig,
    /// Font config for the title
    pub title_font: Option<ReactorFontConfig>,
    /// Font config for the content
    pub content_font: Option<ReactorFontConfig>,
}
