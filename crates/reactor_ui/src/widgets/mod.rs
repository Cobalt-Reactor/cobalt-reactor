#[allow(missing_docs)]
mod button;
pub use button::*;
mod text_label;
pub use text_label::*;

// #[derive(Component)]
// struct TemplateWidget;

// /// Fast way to create a button
// pub trait UiTemplateWidgetExt<'w, 's> {
//     /// Creates a template widget.
//     fn button(&mut self) -> UiBuilder<Entity>;
// }

// impl<'w, 's> UiTemplateWidgetExt<'w, 's> for UiBuilder<'_, UiRoot> {
//     /// Creates a template widget.
//     /// Returns an `UiBuilder` for further customization.
//     fn button(&mut self) -> UiBuilder<Entity> {
//         self.spawn((NodeBundle::default(), TemplateWidget))
//     }
// }
