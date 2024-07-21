#![doc = include_str!("../README.md")]
mod random_impl;
mod random_range_impl;
mod random_traits;
mod shuffle_impl;
mod shuffle_trait;
mod weighted_table;

/// Seed the random number generator
pub fn seed(seed: u64) {
    fastrand::seed(seed);
}

/// Package contents
pub mod prelude {
    pub use crate::{
        random_traits::{Random, RandomContainer, RandomRange, RandomWeightedContainer},
        shuffle_trait::Shuffle,
        weighted_table::WeightedTable,
    };
}
