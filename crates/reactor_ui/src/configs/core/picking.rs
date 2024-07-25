/// Config for pickability of a widget.
#[derive(Default, Debug, Clone)]
pub struct ReactorPicking {
    /// Whether the widget blocks events from widgets lower in the hierarchy.
    pub block_lower: bool,
    /// Whether the widget is hoverable.
    pub hoverable: bool,
}
