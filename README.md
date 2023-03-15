# Dragonfly

Dragonfly is a toy DSL that explores ways to describe the structure of full-stack web applications. You should not use it in production.

For a production-ready solution, check out [Wasp](https://wasp-lang.dev/).

## Crates

### Dragonfly

* **ast** — The initial Abstract Syntax Tree that Dragonfly is parsed into.
* **ir** — An intermediate representation that is more suitable for code generation.
* **cli** — A command-line interface to generate files from the DSL.

### Data structures

* **token-set** — An insertion-ordered set of String values.
* **ord-str-map** — An insertion-ordered map with keys of type String.

### Generators

* **prisma** — Generate and print a Prisma schema.
* **graphql** — Generate and print a GraphQL schema.
* **typescript** — Generate and print TypeScript types.

### Utilities

* **parser** — Combinators for parsing the DSL.
* **print** — A trait for printing types.

## Example application

```dfly
model Image {
  title: String
  country: Country
  category: [Category]
  dimensions: @Dimensions
}

model Dimensions {
  width: Int
  height: Int
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

## Roadmap

- [ ] Automate test coverage collection.
- [ ] Show test coverage in README.
- [ ] Output .d.ts instead of .ts.
- [ ] Implement FromInterator and Extend for collections.
- [ ] Fix ManyToOne foreign key field.
- [ ] Generate foreign keys in PSL.
- [ ] Generate GraphQL types.
- [ ] Generate full GraphQL queries.
  - [ ] CLI: generate GQL.
- [ ] Support aggregate queries :)
