use crate::prelude::RandomContainer;

/// Trait for shuffling a container
pub trait Shuffle<T>: RandomContainer<T> + FromIterator<T>
where
    T: Clone,
{
    /// Shuffles the container
    fn shuffle(&mut self) {
        let mut vec = self.clone().into_iter().collect::<Vec<_>>();
        vec.shuffle();
        *self = Self::from_iter(vec);
    }

    /// Returns a shuffled copy of the container
    fn shuffled(&self) -> Self {
        let mut vec = self.clone().into_iter().collect::<Vec<_>>();
        vec.shuffle();
        Self::from_iter(vec)
    }
}
