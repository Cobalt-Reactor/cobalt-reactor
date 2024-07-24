use crate::{prelude::*, sickle::prelude::*};

/// Configuration for a button widget.
#[derive(Default, Debug)]
pub struct ReactorButtonConfig {
    /// The base config of the button (size, position, alignment, etc).
    pub base_config: ReactorBaseConfig,
    /// The text label to display on the button.
    pub label: Option<ReactorTextLabelConfig>,
    /// The image to display as the button's background.
    pub image: Option<ImageSource>,
}
