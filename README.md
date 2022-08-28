# dnd-rs
[![ci](https://github.com/mbaeum/dnd-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/mbaeum/dnd-rs/actions/workflows/ci.yml) [![release](https://github.com/mbaeum/dnd-rs/actions/workflows/release.yml/badge.svg)](https://github.com/mbaeum/dnd-rs/actions/workflows/release.yml)

A rust lib to get D&amp;D data, inlcuding a clid

## Usage (Web App)
Run
```bash
trunk serve
```
and go to [localhost](localhost:8080)
### Prerequisites
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli
```

## Usage (CLI)
```bash
USAGE:
    dnd <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    dice     Enter Dice API
    help     Print this message or the help of the given subcommand(s)
    spell    Enter Spells API
```

```bash
USAGE:
    dnd spell [OPTIONS]

OPTIONS:
    -c, --classes <CLASSES>    Comma-separated list of classes
    -e, --exact-level          Get spells for exact <LEVEL>
    -h, --help                 Print help information
    -l, --level <LEVEL>        Level of spell (by default this is maximum level, get exact with -e)
    -n, --name <NAME>          Get spell by name
    -r, --random               Get random spell
```

```bash
USAGE:
    dnd dice [DICE_SETS]...

ARGS:
    <DICE_SETS>...    Space-separated list of dice (e.g. 1d20+2 1d3)

OPTIONS:
    -h, --help    Print help information
```

### Usage Examples (local only at the moment)
1. Get any random spell
```bash
cargo run -p cli -- spell -r
```
2. Get random spell for class(es)

```bash
cargo run -p cli -- spell -r -c bard,wizard
```
3. Get random spell for for level (upper limit)

```bash
cargo run -p cli -- spell -r -l 2
```
4. Get random spell for for level (exact)

```bash
cargo run -p cli -- spell -r -l 2 -e
```
5. Roll a list of dice
```bash
cargo run -p cli -- dice 2d20 1d6+3
```

## Contributing

### Setting up pre-commit hook
```bash 
cp scripts/pre-commit .git/hooks/ && chmod +x ./.git/hooks/pre-commit && git init
```

### Running benchmarks
```bash
cargo +nightly bench --features unstable
```