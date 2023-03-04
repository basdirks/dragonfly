use std::fmt::{
    self,
    Display,
    Formatter,
};

/// Errors that can occur when type checking a component.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ComponentError {
    /// The name of a component must be unique within the application. This
    /// component has the same name as another component.
    Duplicate,
}

impl Display for ComponentError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Duplicate => write!(f, "duplicate component"),
        }
    }
}
