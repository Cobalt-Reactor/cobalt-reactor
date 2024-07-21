use crate::prelude::*;

/// The outcome of a `LoadRequest`
#[derive(Debug, Clone)]
pub enum LoadOutcome {
    /// The load was successful
    Success,
    /// The load failed
    Failure(CerealError),
}

/// The outcome of a `SaveRequest`
#[derive(Debug, Clone)]
pub enum SaveOutcome {
    /// The save was successful
    Success,
    /// The save failed
    Failure(CerealError),
}
