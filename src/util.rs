#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash, ops::AddAssign};

use num_traits::{One, Zero};
use rustc_hash::FxBuildHasher;

pub struct Counter<K, V>(HashMap<K, V, FxBuildHasher>);

impl<K> Counter<K, u64>
where
    K: Hash + Eq,
{
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::with_hasher(FxBuildHasher))
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Counter::custom(capacity)
    }
}

impl<K, V> Counter<K, V>
where
    K: Hash + Eq,
    V: Zero + One + AddAssign<V> + Copy,
{
    #[inline]
    pub fn custom(capacity: usize) -> Self {
        Self(HashMap::with_capacity_and_hasher(capacity, FxBuildHasher))
    }

    #[inline]
    pub fn add(&mut self, k: K) {
        self.0
            .entry(k)
            .and_modify(|e| *e += V::one())
            .or_insert(V::one());
    }

    #[inline]
    pub fn get(&self, k: &K) -> V {
        self.0.get(k).copied().unwrap_or(V::zero())
    }
}
