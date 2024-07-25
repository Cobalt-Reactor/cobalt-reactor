use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

/// Fast handling of `ReactorBackground`.
pub trait StyleWithBaseConfigExt {
    /// Calls `callback` when the widget is clicked.
    fn with_base_config(&mut self, background: &ReactorBaseConfig) -> &mut Self;
}

impl StyleWithBaseConfigExt for UiBuilder<'_, Entity> {
    fn with_base_config(&mut self, config: &ReactorBaseConfig) -> &mut Self {
        self.style()
            .with_alignment(&config.alignment)
            .with_size(&config.size)
            .with_position(&config.position);

        if let Some(picking) = &config.picking {
            self.entity_commands()
                .pickable(picking.block_lower, picking.hoverable);
        }

        self
    }
}
