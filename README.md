# Dragonfly

Dragonfly is a toy DSL that explores ways to describe the structure of full-stack web applications. You should not use it in production.

For a production-ready solution, check out [Wasp](https://wasp-lang.dev/).

See also:

- [CLI](src/bin/README.md)
- [AST](src/ast/README.md)
- [Generators](src/generator/README.md)

![CLI](https://user-images.githubusercontent.com/578048/221036308-091fd3c5-684b-4445-bce2-81f8da1d5b6a.png)

# Roadmap

- [ ] Align columns in PSL output.
- [ ] Generate foreign keys in PSL.
- [ ] CLI: generate GQL.
- [ ] Support additional useful types:
  - [ ] `Email`
  - [ ] `URL`
  - [ ] `Currency`
- [ ] Generate full GraphQL queries.
- [ ] Support aggregate queries :)
- [ ] Create `Path` values as soon as possible.
- [ ] Wrap `ParseError` in `SyntaxError`.

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
