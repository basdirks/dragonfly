# Usage

Run `dragonfly help` to see a list of commands and options.

```sh
$ dragonfly help
Usage: dragonfly [command] [command-args]

Commands:
  help                          Print this help message.
  help <command>                Print help message for a command.
  version                       Print the version number.
  check <source-file>           Check a source file for errors.
  build <flags> <source-file>   Generate code from a source file. (see `help build`).
```

Run `dragonfly help <command>` to see help for a specific command.

```sh
$ help check
Usage: dragonfly check <source-file>

$ help build
Usage: dragonfly build [flags] <source-file>

Flags:
  -o, --output <output-directory>   The output directory. (default: `./out`)
```

Run `dragonfly version` to see the version number.

```sh
$ dragonfly --version
0.1.0
```

Run `dragonfly check <source-file>` to check a source file for errors.

```sh
No errors found in `example.dfy`.
```

Run `dragonfly build <flags> <source-file>` to generate code from a source file.

```sh
$ dragonfly build example.dfy
Generated `out/typescript/Country.ts`
Generated `out/typescript/Image.ts`
Generated `out/typescript/DrivingSide.ts`
Generated `out/typescript/CountryName.ts`
Generated `out/typescript/Category.ts`

$ cat out/typescript/Country.ts
interface Country {
    domain: string;
    drivingSide: DrivingSide;
    flag: string;
    name: CountryName;
}% 
```