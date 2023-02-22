# Usage

```sh
$ dragonfly --help
Usage: dragonfly [options] [file]

Options:
    -h, --help      Print this help message.
    -v, --version   Print the version number.
    -o, --output    Specify the output directory.

If no output directory is specified, the current directory is used.
```

```sh
$ dragonfly --version
0.1.0
```

```sh
$ dragonfly examples/color.dfly
Writing to examples/color.ts...

$ cat examples/color.ts
export enum Color {
  Red = 'Red',
  Green = 'Green',
  Blue = 'Blue',
}

export interface RGB {
  red: number;
  green: number;
  blue: number;
}
```