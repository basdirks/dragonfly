//! # Ordered String Map
//!
//! An insertion-ordered map with unique `String` keys.
//!
//! This crate provides the [`OrdStrMap`] type, which is a map with unique
//! `String` keys that preserves insertion order. Implemented as a combination
//! of [`BTreeMap`] and [`VecDeque`], it does double bookkeeping to ensure that
//! keys are unique and that insertion order is preserved.
#![feature(rustdoc_missing_doc_code_examples)]
#![deny(
    clippy::all,
    clippy::format_push_string,
    clippy::if_then_some_else_none,
    clippy::missing_docs_in_private_items,
    clippy::mixed_read_write_in_expression,
    clippy::nursery,
    clippy::pedantic,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rustdoc::missing_doc_code_examples,
    rustdoc::missing_crate_level_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

use std::{
    collections::{
        vec_deque,
        BTreeMap,
        VecDeque,
    },
    fmt::Debug,
};

/// An insertion-ordered map with unique `String` keys.
///
/// # Examples
///
/// ```rust
/// use ord_str_map::OrdStrMap;
///
/// let mut map = OrdStrMap::new();
///
/// assert!(map.insert("foo", 1).is_none());
/// assert!(map.insert("bar", 2).is_none());
/// assert!(map.insert("baz", 3).is_none());
///
/// assert_eq!(map.get("foo"), Some(&1));
/// assert_eq!(map.get("qux"), None);
///
/// let mut iter = map.iter();
///
/// assert_eq!(iter.next(), Some(("foo", 1)));
/// assert_eq!(iter.next(), Some(("bar", 2)));
/// assert_eq!(iter.next(), Some(("baz", 3)));
/// assert_eq!(iter.next(), None);
/// ```
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OrdStrMap<V> {
    /// The underlying map.
    map: BTreeMap<String, V>,
    /// The insertion order.
    order: VecDeque<String>,
}

impl<K> OrdStrMap<K> {
    /// Create an empty map.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            order: VecDeque::new(),
        }
    }
}

impl<V> OrdStrMap<V>
where
    V: Clone + Debug + Ord,
{
    /// Insert an element into the map.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the element to insert.
    /// * `value` - The value to insert.
    ///
    /// # Returns
    ///
    /// `true` if the element was inserted, `false` if it was already present.
    pub fn insert<K>(
        &mut self,
        key: K,
        value: V,
    ) -> Option<V>
    where
        K: Into<String> + Clone,
    {
        if let Some(old) = self.map.insert(key.clone().into(), value) {
            Some(old)
        } else {
            self.order.push_back(key.into());

            None
        }
    }

    /// An iterator visiting all key-value pairs in insertion order.
    #[must_use]
    pub fn iter(&self) -> Iter<'_, V> {
        Iter {
            iter: self.order.iter(),
            map: self.map.clone(),
        }
    }

    /// An iterator visiting all values in insertion order.
    #[must_use]
    pub fn values(&self) -> Values<'_, V> {
        Values {
            iter: self.order.iter(),
            map: &self.map,
        }
    }

    /// Get an iterator over owned values.
    #[must_use]
    pub fn into_values(self) -> IntoValues<V> {
        IntoValues {
            iter: self.order.into_iter(),
            map: self.map,
        }
    }

    /// Get an element by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the element to get.
    #[must_use]
    pub fn get<S>(
        &self,
        key: S,
    ) -> Option<&V>
    where
        S: AsRef<str>,
    {
        self.map.get(key.as_ref())
    }

    /// Get a mutable reference to an element by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the element to get.
    #[must_use]
    pub fn get_mut<S>(
        &mut self,
        key: S,
    ) -> Option<&mut V>
    where
        S: AsRef<str>,
    {
        self.map.get_mut(key.as_ref())
    }

    /// Check whether the map contains an element with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the element to check.
    #[must_use]
    pub fn contains_key<S>(
        &self,
        key: S,
    ) -> bool
    where
        S: AsRef<str>,
    {
        self.map.contains_key(key.as_ref())
    }

    /// Return the number of elements in the map.
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check whether the map is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }
}

impl<T> Default for OrdStrMap<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<V> IntoIterator for OrdStrMap<V>
where
    V: Clone + Debug + Ord,
{
    type IntoIter = IntoIter<V>;
    type Item = (String, V);

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.order.into_iter(),
            map: self.map,
        }
    }
}

impl<K, V> FromIterator<(K, V)> for OrdStrMap<V>
where
    K: Into<String>,
    V: Clone + Debug + Ord,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        let mut map = Self::new();

        for (key, value) in iter {
            let _: Option<V> = map.insert(key.into(), value);
        }

        map
    }
}

/// An iterator over owned key-value pairs of an `OrdStrMap` in insertion order.
#[derive(Debug, Clone)]
pub struct IntoIter<V> {
    /// The underlying iterator.
    iter: vec_deque::IntoIter<String>,
    /// The map.
    map: BTreeMap<String, V>,
}

impl<V> Iterator for IntoIter<V>
where
    V: Clone + Debug + Ord,
{
    type Item = (String, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|key| (key.clone(), self.map.remove(&key).unwrap()))
    }
}

/// An iterator over the key-value pairs of an `OrdStrMap` in insertion order.
#[derive(Debug, Clone)]
pub struct Iter<'a, V> {
    /// The underlying iterator.
    iter: vec_deque::Iter<'a, String>,
    /// The map.
    map: BTreeMap<String, V>,
}

impl<'a, V> Iterator for Iter<'a, V>
where
    V: Clone + Debug + Ord,
{
    type Item = (&'a str, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|key| (key.as_str(), self.map.get(key).unwrap().clone()))
    }
}

/// An iterator over the values of an `OrdStrMap` in insertion order.
#[derive(Debug, Clone)]
pub struct Values<'a, V> {
    /// The underlying iterator.
    iter: vec_deque::Iter<'a, String>,
    /// The map.
    map: &'a BTreeMap<String, V>,
}

impl<'a, V: 'a> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.iter.next()?;

        self.map.get(key)
    }
}

/// An iterator over the owned values of an `OrdStrMap` in insertion order.
#[derive(Debug, Clone)]
pub struct IntoValues<V> {
    /// The underlying iterator.
    iter: vec_deque::IntoIter<String>,
    /// The map.
    map: BTreeMap<String, V>,
}

impl<V> Iterator for IntoValues<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.iter.next()?;

        self.map.remove(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion() {
        let mut map: OrdStrMap<i32> = OrdStrMap::new();

        assert!(map.insert("3", 1).is_none());
        assert!(map.insert("2", 2).is_none());
        assert!(map.insert("1", 2).is_none());
        assert!(map.insert("3", 3).is_some());
    }

    #[test]
    fn test_iter() {
        let mut map = OrdStrMap::new();

        assert!(map.insert("a", "alpha").is_none());
        assert!(map.insert("b", "beta").is_none());
        assert!(map.insert("g", "gamma").is_none());

        let mut iter = map.iter();

        assert_eq!(iter.next(), Some(("a", "alpha")));
        assert_eq!(iter.next(), Some(("b", "beta")));
        assert_eq!(iter.next(), Some(("g", "gamma")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_values() {
        let mut map = OrdStrMap::new();

        assert!(map.insert("a", "alpha").is_none());
        assert!(map.insert("b", "beta").is_none());
        assert!(map.insert("g", "gamma").is_none());

        let mut iter = map.values();

        assert_eq!(iter.next(), Some(&"alpha"));
        assert_eq!(iter.next(), Some(&"beta"));
        assert_eq!(iter.next(), Some(&"gamma"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_get() {
        let mut map = OrdStrMap::new();

        assert!(map.insert("a", "alpha").is_none());
        assert!(map.insert("b", "beta").is_none());
        assert!(map.insert("g", "gamma").is_none());

        assert_eq!(map.get("a"), Some(&"alpha"));
        assert_eq!(map.get("b"), Some(&"beta"));
        assert_eq!(map.get("g"), Some(&"gamma"));
        assert_eq!(map.get("c"), None);
    }

    #[test]
    fn test_get_mut() {
        let mut map = OrdStrMap::new();

        assert!(map.insert("a", "alpha".to_owned()).is_none());

        let a = map.get_mut("a").unwrap();

        a.push_str("beta");

        assert_eq!(map.get("a").unwrap().as_str(), "alphabeta");
    }

    #[test]
    fn test_contains_key() {
        let mut map = OrdStrMap::new();

        assert!(map.insert("a", "alpha").is_none());
        assert!(map.insert("b", "beta").is_none());
        assert!(map.insert("g", "gamma").is_none());

        assert!(map.contains_key("a"));
        assert!(map.contains_key("b"));
        assert!(map.contains_key("g"));
        assert!(!map.contains_key("c"));
    }

    #[test]
    fn test_len() {
        let mut map = OrdStrMap::new();

        assert_eq!(map.len(), 0);

        let _: Option<&str> = map.insert("a", "alpha");
        assert_eq!(map.len(), 1);

        let _: Option<&str> = map.insert("b", "beta");
        assert_eq!(map.len(), 2);

        let _: Option<&str> = map.insert("g", "gamma");
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_is_empty() {
        let mut map = OrdStrMap::new();

        assert!(map.is_empty());

        assert!(map.insert("a", "alpha").is_none());
        assert!(!map.is_empty());
    }

    #[test]
    fn test_default() {
        let set: OrdStrMap<i32> = OrdStrMap::default();

        assert!(set.is_empty());
    }

    #[test]
    fn test_into_iter() {
        let mut set = OrdStrMap::new();

        assert!(set.insert("a", "alpha").is_none());
        assert!(set.insert("b", "beta").is_none());
        assert!(set.insert("g", "gamma").is_none());

        let mut iter = set.into_iter();

        assert_eq!(iter.next(), Some(("a".to_owned(), "alpha")));
        assert_eq!(iter.next(), Some(("b".to_owned(), "beta")));
        assert_eq!(iter.next(), Some(("g".to_owned(), "gamma")));
        assert_eq!(iter.next(), None);
    }
}
