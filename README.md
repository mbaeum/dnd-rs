# random-spells-cli
[![Rust](https://github.com/mbaeum/random-spells-cli/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/mbaeum/random-spells-cli/actions/workflows/test.yml) 

A rust cli to get random D&amp;D spells


## Usage
```bash
USAGE:
    random-spells-cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help            Print this message or the help of the given subcommand(s)
    random-spell    Enter Spells API
```

```bash
USAGE:
    random-spells-cli random-spell [OPTIONS]

OPTIONS:
    -c, --classes <CLASSES>    Comma-separated list of classes
    -e, --exact-level          Get spells for exact <LEVEL>
    -h, --help                 Print help information
    -l, --level <LEVEL>        Level of spell (by default this is maximum level, get exact with -e,
                               minimum with -m)
```

## Usage Examples (local only at the moment)
1. Get any random spell
```bash
cargo run -- random-spell
```
2. Get random spell for class(es)

```bash
cargo run -- random-spell -c bard,wizard
```
3. Get random spell for for level (upper limit)

```bash
cargo run -- random-spell -l 2
```
4. Get random spell for for level (exact)

```bash
cargo run -- random-spell -l 2 -e
```