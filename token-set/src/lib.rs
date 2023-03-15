//! # Token Set
//!
//! A set of `String` values that preserves insertion order.
#![feature(rustdoc_missing_doc_code_examples)]
#![feature(const_btree_len)]
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

//! An insertion-ordered set of `String`s.
//!
//! # Examples
//!
//! ```rust
//! use token_set::TokenSet;
//!
//! let mut set = TokenSet::new();
//!
//! assert!(set.insert("foo"));
//! assert!(set.insert("bar"));
//! assert!(set.insert("baz"));
//!
//! let mut iter = set.iter();
//!
//! assert_eq!(iter.next(), Some("foo"));
//! assert_eq!(iter.next(), Some("bar"));
//! assert_eq!(iter.next(), Some("baz"));
//! assert_eq!(iter.next(), None);
//!
//! assert!(!set.insert("foo"));
//! ```

use std::{
    collections::{
        vec_deque,
        BTreeSet,
        VecDeque,
    },
    fmt::Debug,
};

/// An insertion-ordered set of `String` values.
///
/// # Examples
///
/// ```rust
/// use token_set::TokenSet;
///
/// let mut set = TokenSet::new();
///
/// assert!(set.insert("foo"));
/// assert!(set.insert("bar"));
/// assert!(set.insert("baz"));
///
/// let mut iter = set.iter();
///
/// assert_eq!(iter.next(), Some("foo"));
/// assert_eq!(iter.next(), Some("bar"));
/// assert_eq!(iter.next(), Some("baz"));
/// assert_eq!(iter.next(), None);
///
/// assert!(!set.insert("foo"));
/// ```
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TokenSet {
    /// The underlying set.
    set: BTreeSet<String>,
    /// The insertion order.
    order: VecDeque<String>,
}

impl TokenSet {
    /// Create an empty token set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            set: BTreeSet::new(),
            order: VecDeque::new(),
        }
    }

    /// Insert a token into the set.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to insert.
    ///
    /// # Returns
    ///
    /// `true` if the token was inserted, `false` if it was already in the set.
    pub fn insert<S>(
        &mut self,
        token: S,
    ) -> bool
    where
        S: Into<String> + Clone,
    {
        if self.set.insert(token.clone().into()) {
            self.order.push_back(token.into());

            true
        } else {
            false
        }
    }

    /// Check whether the set contains a token.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to check for.
    ///
    /// # Returns
    ///
    /// `true` if the set contains the token, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use token_set::TokenSet;
    ///
    /// let mut set = TokenSet::new();
    ///
    /// assert!(set.insert("foo"));
    /// assert!(set.insert("bar"));
    /// assert!(set.insert("baz"));
    ///
    /// assert!(set.contains("foo"));
    /// assert!(set.contains("bar"));
    /// assert!(set.contains("baz"));
    /// assert!(!set.contains("qux"));
    /// ```
    #[must_use]
    pub fn contains<S>(
        &self,
        token: S,
    ) -> bool
    where
        S: AsRef<str>,
    {
        self.set.contains(token.as_ref())
    }

    /// An iterator visiting all elements in insertion order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use token_set::TokenSet;
    ///
    /// let set = TokenSet::from_iter(["foo", "bar", "baz"]);
    /// let mut iter = set.iter();
    ///
    /// assert_eq!(iter.next(), Some("foo"));
    /// assert_eq!(iter.next(), Some("bar"));
    /// assert_eq!(iter.next(), Some("baz"));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            iter: self.order.iter(),
        }
    }

    /// Check whether the set is empty.
    ///
    /// # Returns
    ///
    /// `true` if the set is empty, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use token_set::TokenSet;
    ///
    /// let mut set = TokenSet::new();
    ///
    /// assert!(set.is_empty());
    ///
    /// set.insert("foo");
    ///
    /// assert!(!set.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}

impl<S, T> From<T> for TokenSet
where
    S: Into<String> + Clone,
    T: IntoIterator<Item = S>,
{
    fn from(elements: T) -> Self {
        let mut set = Self::new();

        for element in elements {
            let _: bool = set.insert(element);
        }

        set
    }
}

impl<S> FromIterator<S> for TokenSet
where
    S: Into<String> + Clone,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = S>,
    {
        Self::from(iter)
    }
}

/// An iterator over the elements of a `TokenSet`.
///
/// This `struct` is created by the [`iter`] method on [`TokenSet`]. See its
/// documentation for more.
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    /// The underlying iterator.
    iter: vec_deque::Iter<'a, String>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut set = TokenSet::new();

        assert!(set.insert("foo"));
        assert!(set.insert("bar"));
        assert!(set.insert("baz"));
        assert!(!set.insert("foo"));
    }

    #[test]
    fn test_iter() {
        let set = TokenSet::from(["foo", "bar", "baz"]);
        let mut iter = set.iter();

        assert_eq!(iter.next(), Some("foo"));
        assert_eq!(iter.next(), Some("bar"));
        assert_eq!(iter.next(), Some("baz"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_is_empty() {
        let mut set = TokenSet::new();

        assert!(set.is_empty());

        let _: bool = set.insert("foo");

        assert!(!set.is_empty());
    }

    #[test]
    fn test_from() {
        let set = TokenSet::from(["foo", "bar", "baz"]);
        let mut iter = set.iter();

        assert_eq!(iter.next(), Some("foo"));
        assert_eq!(iter.next(), Some("bar"));
        assert_eq!(iter.next(), Some("baz"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_from_iterator() {
        let set = ["foo", "bar", "baz"].iter().copied().collect::<TokenSet>();
        let mut iter = set.iter();

        assert_eq!(iter.next(), Some("foo"));
        assert_eq!(iter.next(), Some("bar"));
        assert_eq!(iter.next(), Some("baz"));
        assert_eq!(iter.next(), None);
    }
}
