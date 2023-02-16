/// Types that can be pretty-printed.
pub trait Print {
    /// Pretty-print the type with the given indentation.
    ///
    /// # Arguments
    ///
    /// * `indentation` - The indentation level.
    fn print(
        &self,
        indentation: usize,
    ) -> String;
}
