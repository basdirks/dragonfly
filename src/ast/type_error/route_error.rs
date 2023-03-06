use std::fmt::{
    self,
    Display,
    Formatter,
};

/// Errors that can occur when type checking a route.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RouteError {
    /// The name of a route must be unique within the application. This route
    /// has the same name as another route.
    Duplicate,
    /// The root of a route must refer to an existing component. This route
    /// refers to an undefined component.
    UndefinedComponent(String),
}

impl Display for RouteError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Duplicate => write!(f, "duplicate route"),
            Self::UndefinedComponent(name) => {
                write!(f, "component `{name}` is undefined")
            }
        }
    }
}
