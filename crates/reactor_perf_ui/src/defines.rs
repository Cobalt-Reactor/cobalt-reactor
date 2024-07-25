pub mod font_defines {
    use bevy::prelude::*;

    /// Handle for the bold font
    pub const BOLD: Handle<Font> = Handle::weak_from_u128(142557032797694866313246460271653377843);

    /// Handle for the std font
    pub const STD: Handle<Font> = Handle::weak_from_u128(87594384175368175745280848615741477491);

    /// Handle for the std font
    pub const NARROW: Handle<Font> = Handle::weak_from_u128(89553236650606954176758320963299136106);

    /// Handle for the std font
    pub const SQUARE: Handle<Font> =
        Handle::weak_from_u128(106265013825184720123862027595927598287);
}
