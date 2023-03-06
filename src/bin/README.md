# Usage

Run `dragonfly help` to see a list of commands and options.

```console
$ dragonfly help
USAGE: dragonfly [COMMAND] [ARGS]

COMMANDS:
    help                    Print this help message.
    help <COMMAND>          Print help message for a command.
    version                 Print the version number.
    check <FILE>            Check a source file for errors.
    build [FLAGS] <FILE>    Generate code from a source file. See `help build`.
```

Run `dragonfly help <command>` to see help for a specific command.

```console
$ dragonfly help check
USAGE: dragonfly check <FILE>

$ dragonfly help build
Usage: dragonfly build [flags] <source-file>

Flags:
  -o, --output <output-directory>   The output directory. Default: `./out`.
```

Run `dragonfly version` to see the version number.

```console
$ dragonfly --version
0.1.0
```

Run `dragonfly check <source-file>` to check a source file for errors.

```console
$ dragonfly check example.dfy
No errors found in `example.dfy`.
```

Run `dragonfly build <flags> <source-file>` to generate code from a source file.

```console
$ cat example.dfy
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
}

enum Category {
  Architecture
  Bollard
  Chevron
}%

$ dragonfly build example.dfy
Generated `out/typescript/Image.ts`
Generated `out/typescript/Country.ts`
Generated `out/typescript/DrivingSide.ts`
Generated `out/typescript/CountryName.ts`
Generated `out/typescript/Category.ts`
Generated `out/prisma/application.prisma`

$ cat out/typescript/Country.ts
interface Country {
    domain: string;
    drivingSide: DrivingSide;
    flag: string;
    name: CountryName;
} 

$ cat out/prisma/application.prisma
model Country {
  id Int @id @default(autoincrement())
  createdAt DateTime
  flag String
  domain String
  drivingSide DrivingSide
  name CountryName
}

model Image {
  id Int @id @default(autoincrement())
  createdAt DateTime
  title String
  country Country
  category Category[]
}

enum CountryName {
  Albania
  Andorra
  Austria
}

enum DrivingSide {
  Left
  Right
}

enum Category {
  Architecture
  Bollard
  Chevron
}
```