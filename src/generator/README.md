# Generators

Generators produce code from Dragonfly ASTs.

* `generator::graphql` prints GQL, converts Dragonfly => GraphQL queries.
* `generator::typescript` prints TS, converts Dragonfly => TS types.
* `generator::prisma` prints PSL, converts Dragonfly => PSL schema.
* `generator::printer` contains pretty printing utilities.

## GraphQL

* GraphQL AST implementation: `generator::graphql`.
* GraphQL types, printing, and conversion from Dragonfly types: `generator::graphql::type` 
* GraphQL queries, printing, and conversion from Dragonfly queries: `generator::graphql::Query`

## TypeScript

* TypeScript AST implementation: `generator::typescript`.
* TypeScript types, printing, and conversion from Dragonfly: `generator::typescript::type`
prints them.

`generator::typescript::Interface` generates TypeScript interfaces from Dragonfly models and prints them.

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

`generator::typescript::StringEnum` generates TypeScript enums from Dragonfly enumerated types and prints them.

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

## Prisma

`generator::prisma` implements the Prisma Schema Language AST.

`generator::prisma::Model` generates Prisma models and enums from Dragonfly models and enumerated types and prints them.

```dfly
model User {
  name: String
  profile: @Profile
}

model Profile {
  birthday: DateTime
  title: String
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
  createdAt DateTime  @default(now())
  id        Int       @id @default(autoincrement())
  name      String
  profile   Profile?
}

model Profile {
  birthday  DateTime
  createdAt DateTime @default(now())
  id        Int      @id @default(autoincrement())
  user      User     @relation("profileOnUser", fields: [userId], references: [id])
  userId    Int      @unique
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

`dragonfly::prisma::data_source` creates and prints data sources.
`dragonfly::prisma::generator` creates and prints generators.