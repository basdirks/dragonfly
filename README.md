# Dragonfly

Dragonfly is a toy DSL that explores ways to describe the structure of full-stack web applications. You should not use it in production.

For a production-ready solution, check out [Wasp](https://wasp-lang.dev/).

See also:

- [CLI](src/bin/README.md)
- [AST](src/ast/README.md)
- [Generators](src/generators/README.md)

# Roadmap

- [ ] Consolidate the type system
- [ ] Support additional useful types:
  - [ ] `Email`
  - [ ] `URL`
  - [ ] `Currency`
- [ ] Generate full GraphQL queries
- [ ] CLI
- [ ] Aggregate queries :)

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
  Yemen
  Zambia
  Zimbabwe
}

enum Category {
  Architecture
  Bollard
  Chevron
  TrafficLight
  TrafficSign
  UtilityPole
}
```

# Development

## Rust version

Rust nightly (1.69.0 or higher) is required.

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
