use {
    super::{
        Field,
        QueryArgument,
        QueryCondition,
        QueryReturnType,
    },
    component_error::ComponentError,
    enum_error::EnumError,
    model_error::ModelError,
    query_error::QueryError,
    route_error::RouteError,
    std::fmt::Display,
};

/// Component errors.
pub mod component_error;
/// Enum errors.
pub mod enum_error;
/// Model errors.
pub mod model_error;
/// Query errors.
pub mod query_error;
/// Route errors.
pub mod route_error;

/// Type checking errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TypeError {
    /// A component error.
    ComponentError {
        /// The name of the component.
        component_name: String,
        /// The error.
        error: ComponentError,
    },
    /// An enum error.
    EnumError {
        /// The name of the enum.
        enum_name: String,
        /// The error.
        error: EnumError,
    },
    /// A model error.
    ModelError {
        /// The name of the model.
        model_name: String,
        /// The error.
        error: ModelError,
    },
    /// A query error.
    QueryError {
        /// The name of the query.
        query_name: String,
        /// The error.
        error: QueryError,
    },
    /// A route error.
    RouteError {
        /// The name of the route.
        route_name: String,
        /// The error.
        error: RouteError,
    },
}

impl TypeError {
    /// Create a duplicate component error.
    ///
    /// # Arguments
    ///
    /// * `component_name` - The name of the component.
    #[must_use]
    pub fn duplicate_component(component_name: &str) -> Self {
        Self::ComponentError {
            component_name: component_name.to_owned(),
            error: ComponentError::Duplicate,
        }
    }

    /// Create a duplicate enum error.
    ///
    /// # Arguments
    ///
    /// * `enum_name` - The name of the enum.
    #[must_use]
    pub fn duplicate_enum(enum_name: &str) -> Self {
        Self::EnumError {
            enum_name: enum_name.to_owned(),
            error: EnumError::Duplicate,
        }
    }

    /// Create a duplicate enum variant error.
    ///
    /// # Arguments
    ///
    /// * `enum_name` - The name of the enum.
    /// * `variant_name` - The name of the variant.
    #[must_use]
    pub fn duplicate_enum_variant(
        enum_name: &str,
        variant_name: &str,
    ) -> Self {
        Self::EnumError {
            enum_name: enum_name.to_owned(),
            error: EnumError::DuplicateVariant(variant_name.to_owned()),
        }
    }

    /// Create an empty enum error.
    ///
    /// # Arguments
    ///
    /// * `enum_name` - The name of the enum.
    #[must_use]
    pub fn empty_enum(enum_name: &str) -> Self {
        Self::EnumError {
            enum_name: enum_name.to_owned(),
            error: EnumError::Empty,
        }
    }

    /// Create a duplicate model error.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    #[must_use]
    pub fn duplicate_model(model_name: &str) -> Self {
        Self::ModelError {
            model_name: model_name.to_owned(),
            error: ModelError::Duplicate,
        }
    }

    /// Create a duplicate model field error.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    /// * `field_name` - The name of the field.
    #[must_use]
    pub fn duplicate_model_field(
        model_name: &str,
        field_name: &str,
    ) -> Self {
        Self::ModelError {
            model_name: model_name.to_owned(),
            error: ModelError::DuplicateField(field_name.to_owned()),
        }
    }

    /// Create an empty model error.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    #[must_use]
    pub fn empty_model(model_name: &str) -> Self {
        Self::ModelError {
            model_name: model_name.to_owned(),
            error: ModelError::Empty,
        }
    }

    /// Create an unknown model field error.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    /// * `field` - The field.
    #[must_use]
    pub fn unknown_model_field_type(
        model_name: &str,
        field: &Field,
    ) -> Self {
        Self::ModelError {
            model_name: model_name.to_owned(),
            error: ModelError::UnknownFieldType(field.clone()),
        }
    }

    /// Create a duplicate query error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    #[must_use]
    pub fn duplicate_query(query_name: &str) -> Self {
        Self::QueryError {
            query_name: query_name.to_owned(),
            error: QueryError::Duplicate,
        }
    }

    /// Create an empty query schema error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    #[must_use]
    pub fn empty_query_schema(query_name: &str) -> Self {
        Self::QueryError {
            query_name: query_name.to_owned(),
            error: QueryError::EmptySchema,
        }
    }

    /// Create an invalid query argument type error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `argument` - The argument.
    #[must_use]
    pub fn invalid_query_argument_type(
        query_name: &str,
        argument: &QueryArgument,
    ) -> Self {
        Self::QueryError {
            query_name: query_name.to_owned(),
            error: QueryError::InvalidArgumentType(argument.clone()),
        }
    }

    /// Create an invalid query condition error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `condition` - The condition.
    #[must_use]
    pub fn invalid_query_condition(
        query_name: &str,
        condition: &QueryCondition,
    ) -> Self {
        Self::QueryError {
            query_name: query_name.to_owned(),
            error: QueryError::InvalidCondition(condition.clone()),
        }
    }

    /// Create an invalid query where name error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `schema_name` - The name of the root node of schema.
    /// * `where_name` - The name of the root node of where clause.
    #[must_use]
    pub fn invalid_query_where(
        query_name: &str,
        schema_name: &str,
        where_name: &str,
    ) -> Self {
        Self::QueryError {
            query_name: query_name.to_owned(),
            error: QueryError::InvalidWhereName {
                schema_name: schema_name.to_owned(),
                where_name: where_name.to_owned(),
            },
        }
    }

    /// Create an undefined query argument error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `argument_name` - The name of the argument.
    #[must_use]
    pub fn undefined_query_argument(
        query_name: &str,
        argument_name: &str,
    ) -> Self {
        Self::QueryError {
            query_name: query_name.to_owned(),
            error: QueryError::UndefinedArgument(argument_name.to_owned()),
        }
    }

    /// Create an undefined query field name error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `field_name` - The name of the field.
    #[must_use]
    pub fn undefined_query_field(
        query_name: &str,
        field_name: &str,
    ) -> Self {
        Self::QueryError {
            query_name: query_name.to_owned(),
            error: QueryError::UndefinedField(field_name.to_owned()),
        }
    }

    /// Create an undefined return type error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `return_type` - The return type.
    #[must_use]
    pub fn undefined_query_return_type(
        query_name: &str,
        return_type: &QueryReturnType,
    ) -> Self {
        Self::QueryError {
            query_name: query_name.to_owned(),
            error: QueryError::UndefinedReturnType(return_type.clone()),
        }
    }

    /// Create an unused query argument error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `argument_name` - The name of the argument.
    #[must_use]
    pub fn unused_query_argument(
        query_name: &str,
        argument_name: &str,
    ) -> Self {
        Self::QueryError {
            query_name: query_name.to_owned(),
            error: QueryError::UnusedArgument(argument_name.to_owned()),
        }
    }

    /// Create a duplicate route error.
    ///
    /// # Arguments
    ///
    /// * `route_name` - The name of the route.
    #[must_use]
    pub fn duplicate_route(route_name: &str) -> Self {
        Self::RouteError {
            route_name: route_name.to_owned(),
            error: RouteError::Duplicate,
        }
    }

    /// Create an undefined component error.
    ///
    /// # Arguments
    ///
    /// * `route_name` - The name of the route.
    /// * `component_name` - The name of the component.
    #[must_use]
    pub fn undefined_route_component(
        route_name: &str,
        component_name: &str,
    ) -> Self {
        Self::RouteError {
            route_name: route_name.to_owned(),
            error: RouteError::UndefinedComponent(component_name.to_owned()),
        }
    }
}

impl Display for TypeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::ComponentError {
                component_name,
                error,
            } => {
                write!(f, "Component error in \"{component_name}\": {error}.")
            }
            Self::EnumError { enum_name, error } => {
                write!(f, "Enum error in \"{enum_name}\": {error}.")
            }
            Self::ModelError { model_name, error } => {
                write!(f, "Model error in \"{model_name}\": {error}.")
            }
            Self::QueryError { query_name, error } => {
                write!(f, "Query error in \"{query_name}\": {error}.")
            }
            Self::RouteError { route_name, error } => {
                write!(f, "Route error in \"{route_name}\": {error}.")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::ast::{
            QueryOperator,
            QueryPath,
        },
    };

    #[test]
    fn test_duplicate_component() {
        assert_eq!(
            TypeError::duplicate_component("foo").to_string(),
            "Component error in \"foo\": duplicate component."
        );
    }

    #[test]
    fn test_duplicate_enum() {
        assert_eq!(
            TypeError::duplicate_enum("foo").to_string(),
            "Enum error in \"foo\": duplicate enum."
        );
    }

    #[test]
    fn test_duplicate_enum_variant() {
        assert_eq!(
            TypeError::duplicate_enum_variant("foo", "bar").to_string(),
            "Enum error in \"foo\": duplicate variant \"bar\"."
        );
    }

    #[test]
    fn test_empty_enum() {
        assert_eq!(
            TypeError::empty_enum("foo").to_string(),
            "Enum error in \"foo\": empty enum."
        );
    }

    #[test]
    fn test_duplicate_model() {
        assert_eq!(
            TypeError::duplicate_model("foo").to_string(),
            "Model error in \"foo\": duplicate model."
        );
    }

    #[test]
    fn test_duplicate_model_field() {
        assert_eq!(
            TypeError::duplicate_model_field("foo", "bar").to_string(),
            "Model error in \"foo\": duplicate field \"bar\"."
        );
    }

    #[test]
    fn test_empty_model() {
        assert_eq!(
            TypeError::empty_model("foo").to_string(),
            "Model error in \"foo\": empty model."
        );
    }

    #[test]
    fn test_unknown_model_field_type() {
        assert_eq!(
            TypeError::unknown_model_field_type("foo", &Field::booleans("bar"))
                .to_string(),
            "Model error in \"foo\": field \"bar\" has unknown type \
             \"[Boolean]\"."
        );
    }

    #[test]
    fn test_duplicate_query() {
        assert_eq!(
            TypeError::duplicate_query("foo").to_string(),
            "Query error in \"foo\": duplicate query."
        );
    }

    #[test]
    fn test_empty_query_schema() {
        assert_eq!(
            TypeError::empty_query_schema("foo").to_string(),
            "Query error in \"foo\": empty schema."
        );
    }

    #[test]
    fn test_invalid_query_argument_type() {
        assert_eq!(
            TypeError::invalid_query_argument_type(
                "foo",
                &QueryArgument::float("bar"),
            )
            .to_string(),
            "Query error in \"foo\": argument \"bar\" has invalid type \
             \"Float\"."
        );
    }

    #[test]
    fn test_invalid_query_condition() {
        assert_eq!(
            TypeError::invalid_query_condition(
                "foo",
                &QueryCondition {
                    path: QueryPath::new(&["foo", "bar", "baz"]),
                    operator: QueryOperator::Equals,
                    argument_name: "baz".to_owned(),
                }
            )
            .to_string(),
            "Query error in \"foo\": operator \"equals\" is not compatible \
             with the types of field \"foo { bar { baz } }\" and argument \
             \"$baz\"."
        );
    }

    #[test]
    fn test_invalid_query_where() {
        assert_eq!(
            TypeError::invalid_query_where("user", "post", "posts").to_string(),
            "Query error in \"user\": name of where root \"posts\" does not \
             match name of schema root \"post\"."
        );
    }

    #[test]
    fn test_undefined_query_argument() {
        assert_eq!(
            TypeError::undefined_query_argument("foo", "bar").to_string(),
            "Query error in \"foo\": argument \"$bar\" is undefined."
        );
    }

    #[test]
    fn test_undefined_query_field() {
        assert_eq!(
            TypeError::undefined_query_field("foo", "bar").to_string(),
            "Query error in \"foo\": schema field \"bar\" is undefined."
        );
    }

    #[test]
    fn test_undefined_query_return_type() {
        assert_eq!(
            TypeError::undefined_query_return_type(
                "foo",
                &QueryReturnType::model("bar")
            )
            .to_string(),
            "Query error in \"foo\": return type refers to undefined model \
             \"bar\"."
        );

        assert_eq!(
            TypeError::undefined_query_return_type(
                "foo",
                &QueryReturnType::array("bar")
            )
            .to_string(),
            "Query error in \"foo\": return type refers to undefined model \
             \"bar\"."
        );
    }

    #[test]
    fn test_unused_query_argument() {
        assert_eq!(
            TypeError::unused_query_argument("foo", "bar").to_string(),
            "Query error in \"foo\": argument \"bar\" is unused."
        );
    }

    #[test]
    fn test_duplicate_route() {
        assert_eq!(
            TypeError::duplicate_route("foo").to_string(),
            "Route error in \"foo\": duplicate route."
        );
    }

    #[test]
    fn test_undefined_route_component() {
        assert_eq!(
            TypeError::undefined_route_component("foo", "bar").to_string(),
            "Route error in \"foo\": component \"bar\" is undefined."
        );
    }
}
