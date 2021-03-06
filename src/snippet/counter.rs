use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Index, IndexMut},
};

/// Counter, alias of HashMap<T, usize>
///
/// ```
/// use counter::Counter;
///
/// let mut c = Counter::new();
///
/// assert_eq!(c["foo"], 0);
/// c["foo"] += 1;
/// assert_eq!(c["foo"], 1);
/// c["foo"] += 1;
/// assert_eq!(c["foo"], 2);
/// ```
#[derive(Debug, Clone)]
pub struct Counter<T>(pub HashMap<T, usize>);

impl<T> Counter<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<T, usize> {
        self.0.iter()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
}

impl<T> Default for Counter<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq + Hash> Index<T> for Counter<T> {
    type Output = usize;

    fn index(&self, index: T) -> &Self::Output {
        self.0.get(&index).unwrap_or(&0)
    }
}

impl<T: Eq + Hash> IndexMut<T> for Counter<T> {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        self.0.entry(index).or_insert(0)
    }
}

impl<T> IntoIterator for Counter<T> {
    type Item = (T, usize);

    type IntoIter = std::collections::hash_map::IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
