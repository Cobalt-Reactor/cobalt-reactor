/// Trait for random number generation
pub trait Random {
    /// Generate a random number
    fn random() -> Self;
}

/// Trait for random number generation with a range
pub trait RandomRange {
    /// Generate a random number in the range (low, high)
    fn random_range(low: Self, high: Self) -> Self;
}

/// Trait for random number generation with a container
pub trait RandomContainer<T>: Clone + IntoIterator<Item = T>
where
    T: Clone,
{
    /// Returns a random element
    fn random(&self) -> Option<Self::Item> {
        self.random_element()
    }

    /// Returns a random element and its index
    fn random_with_index(&self) -> Option<(usize, Self::Item)> {
        if let Some(idx) = self.random_index() {
            let item = self.clone().into_iter().nth(idx).unwrap();
            return Some((idx, item));
        }
        None
    }

    /// Returns a random index
    fn random_index(&self) -> Option<usize> {
        let size = self.clone().into_iter().count();
        if size == 0 {
            None
        } else {
            Some(fastrand::usize(1..=size) - 1)
        }
    }

    /// Returns a random element
    fn random_element(&self) -> Option<Self::Item> {
        self.random_with_index().map(|(_, item)| item)
    }
}

/// Trait for random number generation with a weighted container
pub trait RandomWeightedContainer<T>: Clone + IntoIterator<Item = T>
where
    T: Clone,
{
    /// Returns the maximum weight of all elements in the container
    fn max_weight(&self) -> u32;

    /// Returns the weights of all elements in the container as a vector
    fn weights(&self) -> &Vec<u32>;

    /// Returns the values of all elements in the container as a vector
    fn values(&self) -> &Vec<T>;

    /// Returns a random weight below the maximum weight or None if the container is empty
    fn random_weight(&self) -> Option<u32> {
        if self.max_weight() == 0 {
            return None;
        }
        Some(fastrand::u32(0..=self.max_weight()))
    }

    /// Returns a random element mapping to the specified weight
    fn weighted_random_with_weight(&self, weight: u32) -> Option<Self::Item> {
        if weight > self.max_weight()
            || self.max_weight() == 0
            || self.weights().is_empty()
            || self.values().is_empty()
        {
            return None;
        }

        let mut n = weight;
        for (i, weight) in self.weights().iter().enumerate() {
            if &n <= weight {
                return self.values().get(i).cloned();
            }
            n -= weight;
        }

        None
    }

    /// Returns a random element based on the weights
    fn weighted_random(&self) -> Option<Self::Item> {
        if let Some(weight) = self.random_weight() {
            return self.weighted_random_with_weight(weight);
        }

        None
    }
}
