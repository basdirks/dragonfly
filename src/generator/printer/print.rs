/// Types that can be pretty-printed.
pub trait Print {
    /// Pretty-print the type with the given indentation.
    ///
    /// # Arguments
    ///
    /// * `level` - The indentation level.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::printer::{
    ///     indent,
    ///     print::Print,
    /// };
    ///
    /// enum Foo {
    ///     Bar(String, Box<Foo>),
    ///     Baz(String),
    /// }
    ///
    /// impl Print for Foo {
    ///     fn print(
    ///         &self,
    ///         level: usize,
    ///     ) -> String {
    ///         let indent = "   ".repeat(level);
    ///
    ///         match self {
    ///             Foo::Bar(a, b) => {
    ///                 let b = b.print(level + 1);
    ///
    ///                 format!("{indent}{a} {{\n{b}\n{indent}}}")
    ///             }
    ///             Foo::Baz(a) => format!("{indent}{a}"),
    ///         }
    ///     }
    /// }
    ///
    /// let foo = Foo::Bar(
    ///     "Foo".to_string(),
    ///     Box::new(Foo::Bar(
    ///         "Bar".to_string(),
    ///         Box::new(Foo::Baz("Baz".to_string())),
    ///     )),
    /// );
    ///
    /// assert_eq!(
    ///     foo.print(2),
    ///     "      Foo {
    ///          Bar {
    ///             Baz
    ///          }
    ///       }"
    /// );
    /// ```
    fn print(
        &self,
        level: usize,
    ) -> String;
}
