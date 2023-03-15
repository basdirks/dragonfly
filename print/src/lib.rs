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

//! A trait for pretty-printing types.
//!
//! # Examples
//!
//! ```rust
//! use {
//!     print::Print,
//!     std::io,
//! };
//!
//! #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
//! enum Foo {
//!     Bar(String, Box<Foo>),
//!     Baz(String),
//! }
//!
//! impl Print for Foo {
//!     const TAB_SIZE: usize = 3;
//!
//!     fn print(
//!         &self,
//!         level: usize,
//!         f: &mut dyn io::Write,
//!     ) -> io::Result<()> {
//!         let indent = "   ".repeat(level);
//!
//!         match self {
//!             Foo::Bar(a, b) => {
//!                 writeln!(f, "{indent}{a} {{")?;
//!                 b.print(level + 1, f)?;
//!                 writeln!(f, "{indent}}}")
//!             }
//!             Foo::Baz(a) => writeln!(f, "{indent}{a}"),
//!         }
//!     }
//! }
//!
//! let mut f = Vec::new();
//!
//! Foo::Bar("foo".to_string(), Box::new(Foo::Baz("bar".to_string())))
//!     .print(0, &mut f)
//!     .unwrap();
//!
//! assert_eq!(
//!     String::from_utf8(f).unwrap(),
//!     "foo {
//!    bar
//! }
//! "
//! );
//! ```

use std::io;

/// Types that can be pretty-printed but not indented.
pub trait PrintInline {
    /// Pretty print the type.
    ///
    /// # Arguments
    ///
    /// * `f` - The output stream.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the output stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     print::PrintInline,
    ///     std::io,
    /// };
    ///
    /// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Foo {
    ///     Bar(String, Box<Foo>),
    ///     Baz(String),
    /// }
    ///
    /// impl PrintInline for Foo {
    ///     fn print(
    ///         &self,
    ///         f: &mut dyn io::Write,
    ///     ) -> io::Result<()> {
    ///         match self {
    ///             Foo::Bar(a, b) => {
    ///                 write!(f, "{a} => ")?;
    ///                 b.print(f)
    ///             }
    ///             Foo::Baz(a) => write!(f, "{a}"),
    ///         }
    ///     }
    /// }
    ///
    /// let mut f = Vec::new();
    ///
    /// Foo::Bar("foo".to_string(), Box::new(Foo::Baz("bar".to_string())))
    ///     .print(&mut f)
    ///     .unwrap();
    ///
    /// assert_eq!(String::from_utf8(f).unwrap(), "foo => bar");
    /// ```
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()>;

    /// Print the type, separated by the given separator.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator over the items to print.
    /// * `f` - The output stream.
    /// * `separator` - The separator to use between items.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the output stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     print::PrintInline,
    ///     std::io,
    /// };
    ///
    /// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Foo {
    ///     Bar(String, Box<Foo>),
    ///     Baz(String),
    /// }
    ///
    /// impl PrintInline for Foo {
    ///     fn print(
    ///         &self,
    ///         f: &mut dyn io::Write,
    ///     ) -> io::Result<()> {
    ///         match self {
    ///             Foo::Bar(a, b) => {
    ///                 write!(f, "{a}(")?;
    ///                 b.print(f)?;
    ///                 write!(f, ")")
    ///             }
    ///             Foo::Baz(a) => write!(f, "<{a}>"),
    ///         }
    ///     }
    /// }
    ///
    /// let mut f = Vec::new();
    ///
    /// Foo::intercalate(
    ///     vec![
    ///         Foo::Bar(
    ///             "foo".to_string(),
    ///             Box::new(Foo::Bar(
    ///                 "bar".to_string(),
    ///                 Box::new(Foo::Baz("baz".to_string())),
    ///             )),
    ///         ),
    ///         Foo::Baz("qux".to_string()),
    ///         Foo::Baz("quux".to_string()),
    ///     ],
    ///     &mut f,
    ///     ", ",
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(
    ///     String::from_utf8(f).unwrap(),
    ///     "foo(bar(<baz>)), <qux>, <quux>"
    /// );
    /// ```
    fn intercalate<I>(
        iter: I,
        f: &mut dyn io::Write,
        separator: &str,
    ) -> io::Result<()>
    where
        Self: Sized,
        I: IntoIterator<Item = Self>,
    {
        let mut iter = iter.into_iter();

        if let Some(first) = iter.next() {
            first.print(f)?;

            for item in iter {
                write!(f, "{separator}")?;
                item.print(f)?;
            }
        }

        Ok(())
    }
}

/// Types that can be printed.
pub trait Print {
    /// The number of characters to indent per level.
    /// The default is 2.
    const TAB_SIZE: usize;

    /// Indentation whitespace.
    ///
    /// # Arguments
    ///
    /// * `level` - The indentation level.
    #[must_use]
    fn indent(level: usize) -> String {
        " ".repeat(Self::TAB_SIZE * level)
    }

    /// Print the type with the given indentation.
    ///
    /// # Arguments
    ///
    /// * `level` - The indentation level.
    /// * `f` - The output stream.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the output stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     print::Print,
    ///     std::io,
    /// };
    ///
    /// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Foo {
    ///     Bar(String, Box<Foo>),
    ///     Baz(String),
    /// }
    ///
    /// impl Print for Foo {
    ///     const TAB_SIZE: usize = 3;
    ///
    ///     fn print(
    ///         &self,
    ///         level: usize,
    ///         f: &mut dyn io::Write,
    ///     ) -> io::Result<()> {
    ///         match self {
    ///             Foo::Bar(a, b) => {
    ///                 writeln!(f, "{}{a} {{", Self::indent(level))?;
    ///                 b.print(level + 1, f)?;
    ///                 writeln!(f, "{}}}", Self::indent(level))
    ///             }
    ///             Foo::Baz(a) => writeln!(f, "{}{a}", Self::indent(level)),
    ///         }
    ///     }
    /// }
    ///
    /// let mut f = Vec::new();
    ///
    /// let foo = Foo::Bar(
    ///     "Foo".to_owned(),
    ///     Box::new(Foo::Bar(
    ///         "Bar".to_owned(),
    ///         Box::new(Foo::Baz("Baz".to_owned())),
    ///     )),
    /// );
    ///
    /// foo.print(2, &mut f).unwrap();
    ///
    /// assert_eq!(
    ///     String::from_utf8(f).unwrap(),
    ///     "      Foo {
    ///          Bar {
    ///             Baz
    ///          }
    ///       }
    /// "
    /// );
    /// ```
    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()>;
}
