use std::hash::Hash;

use dashmap::{DashMap, DashSet};

use crate::{GetSize, GetSizeTracker};

impl<K, V> GetSize for DashMap<K, V>
where
    K: PartialEq + Eq + Hash + GetSize,
    V: GetSize,
{
    fn get_heap_size<Tracker: GetSizeTracker>(&self, tracker: &mut Tracker) -> usize {
        let mut total = 0;
        for kv in self.iter() {
            total += GetSize::get_size(kv.key(), tracker);
            total += GetSize::get_size(kv.value(), tracker);
        }
        let additional: usize = self.capacity() - self.len();
        total += additional * K::get_stack_size();
        total += additional * V::get_stack_size();
        total
    }
}

impl<T> GetSize for DashSet<T>
where
    T: PartialEq + Eq + Hash + GetSize,
{
    fn get_heap_size<Tracker: GetSizeTracker>(&self, tracker: &mut Tracker) -> usize {
        let mut total = 0;
        for v in self.iter() {
            total += GetSize::get_size(&*v, tracker);
        }
        let additional: usize = self.capacity() - self.len();
        total += additional * T::get_stack_size();
        total
    }
}
