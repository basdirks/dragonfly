# Entities

An application consists of:

- types: sets of values,
- enums: sets of predefined strings,
- models: how data is stored,
- queries: retrieving stored data,
- routes: tying components to endpoints,
- components: the user interface.

## Models

A model describes entities in an application by giving names to groups of fields. A field has a name and a type. A type can be an array or a scalar type.

![Types](https://user-images.githubusercontent.com/578048/221204820-72a139b4-3005-43a5-8be6-0a281f9f8b96.png)

### Types

Scalar types are:

- `Boolean`: `true` or `false`,
- `DateTime`: date and time,
- `Float`: a 64-bit floating point number,
- `Int`: a 64-bit integer,
- `Reference`: a reference to an enumerated type or another model,
- `Owned`: a reference to a model that is owned by the current model,
- `String`: a sequence of UTF-8 characters.

### Relationships

A reference points to an enumerated type or a model. A reference to a model implies a relationship between two models. Three types of relationships exist: one-to-one, one-to-many, and many-to-many. Similarly, an owned reference points to a model and implies that the referenced model is owned by the current model.

#### One-to-one

```
A { B }
B { A }
```

An example of a one-to-one relationship is between a user and their profile. A user has one profile and a profile belongs to one user.

```dfly
model User {
  profile: Profile
}

model Profile {
  user: User
}
```

#### One-to-many

An example of a one-to-many relationship is between a user and their posts. A user has many posts and a post belongs to one user.

```
A { [B] }
B { A }
```

```dfly
model User {
  posts: [Post]
}

model Post {
  author: User
}
```

#### Many-to-many

An example of a many-to-many relationship is between users and groups. A user belongs to many groups and a group has many users.

```
A { [B] }
B { [A] }
```

```dfly
model User {
  groups: [Group]
}

model Group {
  users: [User]
}
```

### Validation

- A model must have at least one field.
- The name of a model must be unique.
- The name of a field must be unique within a model.
- Arrays may not be nested.
- Applications must define all types referenced by models.

### EBNF

```ebnf
(* Model *)

model          = "model" model_name "{" field+ "}";
field          = field_name ":" type;

(* Types *)

type           = "[" basic_type "]" | basic_type;
basic_type     = primitive_type | reference;
primitive_type = "Boolean" | "DateTime" | "Float" | "Int" | "String";
reference      = model_name | enum_name;

(* Names *)

model_name     = pascal_case;
field_name     = camel_case;

(* see Enums

enum_name = ...

*)
```

### Example

```dfly
model Image {
  id: Int
  title: String
  country: Country
  category: [Category]
}
```

## Enums

An enumerated type is a predefined list of one or more string values.

### Validation

- The enum must have at least one variant.
- The name of an enumerated type must be unique.
- The name of an enumerated type variant must be unique within the enum.

### EBNF

```ebnf
(* Enum *)

enum         = "enum" enum_name "{" enum_variant+ "}";

(* Names *)

enum_name    = pascal_case;
enum_variant = pascal_case;
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

A query describes what data can be retrieved from the database.

- a name,
- a return type,
- a schema of the data to return,
- optional arguments to filter the data,
- an optional where clause to use the arguments as constraints.

### Validation

- The name of a query must be unique.
- The return type must be a known model or an array of such a type.
- The root node of the schema must contain at least one field.
- The schema fields must exist in the return type model or in the model referenced by the fields.
- The content of the where clause must be a subset of the schema, except for the  conditions.
- The name of the root node of the schema must match the name of the root node  of the content of the where clause.
- The types of operands of a condition must be compatible with the condition and one another.
- The right-hand side of a condition must refer to an existing argument.
- The name of each argument must be unique.
- The type of each argument must be a primitive scalar type, a reference to an enum, or an array of such a type. An argument may not reference a model.
- Some condition must make use of each argument.

### EBNF

```ebnf
(* Query *)

query         = "query" query_name [ "(" argument+ ")" ] ":" return_type
                schema [ where_clause ];
argument      = argument_name ":" type;
return_type   = model_name | "[" model_name "]";

(* Schema *)

schema        = root_name "{" schema_node+ "}";
schema_node   = node_name [ "{" schema_node+ "}" ];

(* Where *)

where_clause  = "where" "{" root_name "{" where_node+ "}" "}";
where_node    = node_name "{" where_node+ | condition+ "}";

(* Conditions *)

condition      = contains | equals;
contains      = "contains" ":" argument_name;
equals        = "equals" ":" argument_name;

(* Names *)

query_name    = camel_case;
node_name     = camel_case;
argument_name = "$" camel_case;

(* see Models

type = ...
model_name = ...

*)

(* see Enums

enum_name = ...

*)
```

## Routes

A route connects a URL to a component. It consists of:

- a path,
- a title,
- a root component.

### Validation

- The path must be unique.
- The path must consist of one or more segments starting with a forward slash.
- The component must be defined.

### EBNF

```ebnf
(* Route *)

route        = "route" path "{" "root" ":" component_name "title" ":" string "}";
path         = "/" | path_segment+;
path_segment = "/" kebab_case;

(* see Components

component_name = ...

*)
```

### Example

```dfly
route / {
  root: Home
  title: Home
}
```

## Components

A component is a Javascript function that renders a part of the user interface.

### Validation

- The name of a component must be unique.

### EBNF

```ebnf
(* Component *)

component      = "component" component_name "{" "path" ":" path "}";
path           = path_segment* file_name;
path_segment   = "/" kebab_case;

(* Names *)

component_name = pascal_case;
file_name      = pascal_case;
```

### Example

```dfly
component Home {
  path: Home
}
```

# Development

## Parsing

Parsers do not concern themselves with their surrounding whitespace. Their parent parsers are responsible for consuming whitespace.

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

High-level parsers do not concern themselves with EOF. Parsers like `char` and `literal` already handle EOF.

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
