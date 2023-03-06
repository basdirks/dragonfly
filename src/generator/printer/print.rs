use std::io;

/// Types that can be pretty-printed.
pub trait Print {
    /// Pretty-print the type with the given indentation.
    ///
    /// # Arguments
    ///
    /// * `level` - The indentation level.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the output stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     dragonfly::generator::printer::{
    ///         indent,
    ///         print::Print,
    ///     },
    ///     std::io,
    /// };
    ///
    /// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    /// enum Foo {
    ///     Bar(String, Box<Foo>),
    ///     Baz(String),
    /// }
    ///
    /// impl Print for Foo {
    ///     fn print(
    ///         &self,
    ///         level: usize,
    ///         f: &mut dyn io::Write,
    ///     ) -> io::Result<()> {
    ///         let indent = "   ".repeat(level);
    ///
    ///         match self {
    ///             Foo::Bar(a, b) => {
    ///                 writeln!(f, "{indent}{a} {{")?;
    ///                 b.print(level + 1, f)?;
    ///                 writeln!(f, "{indent}}}")
    ///             }
    ///             Foo::Baz(a) => writeln!(f, "{indent}{a}"),
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
