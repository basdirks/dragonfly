# Dragonfly

This is a toy DSL for describing full-stack web applications.

# Entities

An application is described as:

- Models: the structure of data
- Enums: predefined string values
- Queries: subsets of data
- Routes: the logical structure
- Components: the user interface

Unimplemented:

- Mutations
- Authentication and authorization
- Scheduled tasks

## Models

A model describes an entity. It has a name and one or more fields. A field has a name and a type. A type can be an array, a primitive type, or a reference to an enum or a model.

Primitive types are:

- `String`: UTF-8 string
- `Int`: 64-bit integer
- `Float`: 64-bit floating point number
- `Boolean`: boolean value

### Validation

- The model must have at least one field.
- The name of a model must be unique.
- The name of a field must be unique within a model.
- Arrays may not be nested.
- Non-primitive types must be defined elsewhere as a model or enum.

### EBNF

```ebnf
model          = "model" model_name "{" field+ "}"
field          = field_name ":" type
type           = "[" type "]" | single_type
single_type    = primitive_type | enum_name | model_name
primitive_type = "String" | "Int" | "Float" | "Boolean"

model_name     = upper [ alpha ]*
field_name     = lower [ alpha ]*
enum_name      = upper [ alpha ]*
```

### Example

```dfly
model Image {
  id: ID
  title: String
  country: Country
  category: [Category]
}
```

## Enums

An enum is a predefined list of one or more string values.

### Validation

- The enum must have at least one variant.
- The name of an enum must be unique.
- The name of an enum variant must be unique within an enum.

### EBNF

```ebnf
enum         = "enum" enum_name "{" enum_variant+ "}"
enum_name    = upper [ alpha ]*
enum_variant = upper [ alpha ]*
```

### Example

```dfly
enum Category {
  Architecture
  Bollard
  Chevron
  TrafficLight
  TrafficSign
  UtilityPole
}
```

## Queries

A query is a subset of data. It consists of:

- a name,
- a return type,
- a schema of the data to be returned,
- optional arguments to filter the data,
- an optional where-clause to use the arguments as constraints.

### Rules

- The name of a query must be unique.
- The return type must be a known model or an array of such a model.
- The root node of the schema must contain at least one field.
- The schema must be a subset of the return type, or in the case of an array, a subset of the array's item type.
- The content of the where-clause must be a subset of the schema, except for the selectors.
- The name of the root node of the schema must match the name of the root node of the content of the where-clause.
- The type of the argument must match the type of the field to which the selector is applied.
- Each selector must refer to an existing argument.
- The type of the selector (inferred by the argument) must be compatible with the type of the field to which it is applied.
- The name of each argument must be unique.
- The type of each argument must be known.
- Each argument must be used at least once in the where-clause.
- An argument may not be an array or a model.

### EBNF

```ebnf
query         = "query" query_name [ "(" argument+ ")" ] ":" return_type schema [ where_clause ]
argument      = argument_name ":" type
return_type   = model_name | "[" model_name "]"

schema        = root_name "{" schema_node+ "}"
schema_node   = node_name [ "{" schema_node+ "}" ]

where_clause  = "where" "{" root_name "{" where_node+ "}" "}"
where_node    = node_name "{" where_node+ | selector+ "}"

selector      = contains | equals
contains      = "contains" ":" argument_name
equals        = "equals" ":" argument_name

query_name    = lower [ alpha ]*
argument_name = "$" lower [ alpha ]*
node_name     = lower [ alpha ]*
```

## Routes

A route connects a URL to a component. It consists of:

- a path,
- a title,
- a root component.

### Rules

- The path must be unique.
- The path must consist of one or more segments, each starting with a forward slash.
- The component must be defined.

### EBNF

```ebnf
route          = "route" path "{" "root" ":" component_name "title" ":" string "}"
path           = path_segment+
path_segment   = "/" [ alphanumeric ]+
component_name = upper [ alpha ]*
```

### Example

```dfly
route / {
  root: Home
  title: Home
}
```

## Components

A component is a Javascript function that renders a user interface.

### Rules

- The name of a component must be unique.

### EBNF

```ebnf
component = "component" component_name "{" "path" ":" path "}"
```

### Example

```dfly
component Home {
  path: Home
}
```

# Example application

```dfly
route / {
  root: Home
  title: Home
}

component Home {
  path: Home
}

model Image {
  id: ID
  title: String
  country: Country
  category: [Category]
}

query images: [Image] {
  image {
    title
    country {
      name
    }
    category
  }
}

query imagesByCountryName($name: CountryName): [Image] {
  image {
    title
    category
  }
  where {
    image {
      country {
        name {
          equals: $name
        }
      }
    }
  }
}

enum DrivingSide {
  Left
  Right
}

model Country {
  id: ID
  domain: String
  drivingSide: DrivingSide
  flag: String
  name: CountryName
}

enum CountryName {
  Albania
  Andorra
  Austria
  ...
  Yemen
  Zambia
  Zimbabwe
}

enum Category {
  Architecture
  Bollard
  Chevron
  ...
  TrafficLight
  TrafficSign
  UtilityPole
}
```

# Technical overview

Dragonfly syntax is parsed into an AST. The AST is then type-checked and compiled into TypeScript code.

## Parsing

Parsing turns a string into an AST. This step fails if syntax is invalid or if an entity is defined multiple times. The AST type is defined in `dragonfly::ast::Ast`, and the parser is defined in `dragonfly::ast::Ast::parse`.

### TODO

- [ ] Replace `path` in `dragonfly::ast::component::Component` with proper path type.
- [ ] Replace `root` in `dragonfly::ast::route::Route` with proper path type.
- [ ] Implement variant of `choice` that counts and restricts parser usage.
- [ ] Use new variant of `choice` inside `dragonfly::ast::route::Route::parse`.

## Type-checking

Type-checking checks the AST for correctness, see the Rules sections above. Some checks could be done during parsing, but are done separately for simplicity. The type-checker is defined in `dragonfly::ast::Ast::check`.

## Generation

Generation turns the AST into TypeScript code.

### TODO

- [ ] Replace `Display for dragonfly::generator::typescript::enum::Enum` with proper pretty printer.
- [ ] Replace `Display for dragonfly::generator::typescript::interface::Interface` with proper pretty printer.
- [ ] Replace `Display for dragonfly::generator::typescript::interface::Property` with proper pretty printer.
- [ ] Replace `Display for dragonfly::generator::typescript::type::Type` with proper pretty printer.
- [ ] Support extended parameters in `dragonfly::generator::typescript::ast::Interface`.

# Development

Parsers do not concern themselves with their surrounding whitespace. This is handled inside their parent parser.

```rust
// Not this:
fn parse_a<T>(input: &str) -> ParseResult<T> {
    let (b, input) = parse_b(input)?;
    // ...
    Ok((foo, input))
}

fn parse_b<T>(input: &str) -> ParseResult<T> {
    let (_, input) = spaces(input)?;
    let (b, input) = do_parse_b(input)?;
    let (_, input) = spaces(input)?;
    // ...
    Ok((parsed, input))
}

// But this:
fn parse_a<T>(input: &str) -> ParseResult<T> {
    let (_, input) = spaces(input)?;
    let (b, input) = parse_b(input)?;
    let (_, input) = spaces(input)?;
    // ...
    Ok((foo, input))
}

fn parse_b<T>(input: &str) -> ParseResult<T> {
    let (b, input) = do_parse_b(input)?;
    // ...
    Ok((b, input))
}
```

High-level parsers do not concern themselves with EOF. This is handled in parsers like `char` and `literal`.

```rust
// Not this:
fn parse<T>(input: &str) -> ParseResult<T> {
    if input.is_empty() {
        return Err(ParseError::UnexpectedEof);
    }

    let (b, input) = char(input, 'a')?;
}

// But this:
fn parse<T>(input: &str) -> ParseResult<T> {
    let (b, input) = char(input, 'a')?;
}
```
