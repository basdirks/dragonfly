# Dragonfly

Dragonfly is a toy DSL that explores ways to describe the structure of full-stack web applications. You should not use it in production.

For a production-ready solution, check out [Wasp](https://wasp-lang.dev/).

See also:

- [CLI](src/bin/README.md)
- [IR](src/ir/README.md)
- [AST](src/ast/README.md)
- [Generators](src/generator/README.md)

![CLI](https://user-images.githubusercontent.com/578048/221036308-091fd3c5-684b-4445-bce2-81f8da1d5b6a.png)

# Roadmap

- [ ] Check argument docs.
- [ ] Check test-only functions.
- [ ] Review public API. (Any functions that should be private?)
- [ ] Replace all unwraps with ?
- [ ] Give types their own file.
- [ ] Implement FromInterator and Extend for collections.
- [ ] Generate foreign keys in PSL.
- [ ] Generate GraphQL types.
- [ ] Generate full GraphQL queries.
  - [ ] CLI: generate GQL.
- [ ] Support aggregate queries :)

# Example application

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
