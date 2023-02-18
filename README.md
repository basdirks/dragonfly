# Dragonfly

Dragonfly is a toy DSL that explores ways to describe the structure of
full-stack web applications. You should not use it in production.

# Entities

An application consists of:

- types: sets of values,
- models: the structure of entities,
- enums: predefined string values,
- queries: subsets of data,
- routes: the logical structure,
- components: the user interface.

Unimplemented:

- mutations,
- authentication and authorization,
- scheduled tasks.

## Models

A model describes entities in your application by giving names to groups of
fields. A field has a name and a type. A type can be an array or a scalar type.

### Types

Scalar types are:

- `Boolean`: `true` or `false`,
- `DateTime`: a date and time,
- `Float`: a 64-bit floating point number,
- `Int`: a 64-bit integer,
- `Reference`: a reference to an enum or another model,
- `String`: a sequence of UTF-8 characters.

### Relationships

A reference points to an enum or a model. A reference to a model implies a
relationship between two models.

An example of a 1-1 relationship is a user and their profile. A user has one
profile and a profile belongs to one user.
`A { B }, B { A }`.

```dfly
model User {
  profile: Profile
}

model Profile {
  user: User
}
```

An example of a 1-n relationship is a user and their posts. A user has many
posts and a post belongs to one user.
`A { [B] }, B { A }`.

```dfly
model User {
  posts: [Post]
}

model Post {
  author: User
}
```

An example of a n-n relationship users and groups. A user belongs to many
groups and a group has many users.
`A { [B] }, B { [A] }`.

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
- Referenced types must be defined inside the application.

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

An enum is a predefined list of one or more string values.

### Validation

- The enum must have at least one variant.
- The name of an enum must be unique.
- The name of an enum variant must be unique within an enum.

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

A query is a subset of data. It consists of:

- a name,
- a return type,
- a schema of the data to return,
- optional arguments to filter the data,
- an optional where clause to use the arguments as constraints.

### Rules

- The name of a query must be unique.
- The return type must be a known model or an array of such a type.
- The root node of the schema must contain at least one field.
- The schema must be a subset of the return type, or in the case of an array, a  
subset of the array's item type.
- The content of the where clause must be a subset of the schema, except for the  
conditions.
- The name of the root node of the schema must match the name of the root node  
of the content of the where clause.
- The type of the argument must match the type of the field to which the  
condition is applied.
- Each condition must refer to an existing argument.
- The type of the condition (inferred by the argument) must be compatible with  
the type of the field to which it is applied.
- The name of each argument must be unique.
- The type of each argument must be a scalar type.
- If the return type is a reference, there must exist an enum or model with that  
name inside the application.
- Each argument must be used at least once in the where clause.
- An argument may not be an array or a model.

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

### Rules

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

A component is a Javascript function that renders a user interface.

### Rules

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

The parser transforms Dragonfly syntax into an AST. The type checker validates
the AST. The generator turns the AST into TypeScript code.

## Parser

Parsing transforms a string into an AST. This step fails if the syntax is
invalid or if a declaration does not have a unique name. `ast::Ast` defines the
AST type. The `parse` method defines the parser.

* `ast::Ast` contains the root AST type.
* `ast::Ast::parse` defines the AST parser.
* `parser` contains parser combinators and common parsers.

## Type checker

The type checker verifies the internal consistency of the AST. Some checks could
be done during parsing but are done separately for simplicity. The type checker
can be found in `ast::Ast::check`.

## Generation

Generation turns the AST into code:

* `generator::graphql` converts generates GraphQL queries.
* `generator::typescript` generates TypeScript code.
* `generator::prisma` generates Prisma schemas.
* `generator::printer` contains common code for pretty printing.

### GraphQL

GraphQL queries are generated from the AST.

### TypeScript

Models can be converted to TypeScript interfaces:

```dfly
model Country {
  languages: [Language]
  name: String
  population: Int
  formation: DateTime
  area: Float
  hasCoverage: Boolean
}
```

```typescript
interface Country {
    languages: Array<Language>;
    name: string;
    population: bigint;
    formation: Date;
    area: number;
    hasCoverage: boolean;
}
```

Enums can be converted to TypeScript enums:

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

```typescript
enum Category {
    Architecture = "Architecture",
    Bollard = "Bollard",
    Chevron = "Chevron",
    TrafficLight = "TrafficLight",
    TrafficSign = "TrafficSign",
    UtilityPole = "UtilityPole",
}
```

### Prisma

Prisma schemas can be generated from the AST.

```dfly
model User {
  posts: [Post]
  birthday: DateTime
  name: String
  admin: Boolean
}

model Post {
  title: String
  author: User
}

enum Country {
  Albania
  Andorra
  Austria
  Yemen
  Zambia
  Zimbabwe
}
```

```prisma
model User {
  id Int @id @default(autoincrement())
  birthday DateTime
  name String
  admin Boolean
  posts Post[]
}

model Post {
  id Int @id @default(autoincrement())
  title String
  author User @relation(fields: [authorId], references: [id])
  authorId Int
}

enum Country {
  Albania
  Andorra
  Austria
  Yemen
  Zambia
  Zimbabwe
}
```

# Development

## Rust version

Rust nightly (1.69.0 or higher) is required.

## Parsing

Parsers do not concern themselves with their surrounding whitespace. Whitespace
is handled inside their parent parsers.

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

High-level parsers do not concern themselves with EOF. This is handled in
parsers like `char` and `literal`.

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
