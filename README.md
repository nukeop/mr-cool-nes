# mr-cool-nes ![Travis (.org)](https://img.shields.io/travis/nukeop/mr-cool-nes.svg?style=for-the-badge) [![Codecov](https://img.shields.io/codecov/c/github/codecov/example-python.svg?style=for-the-badge)](https://codecov.io/gh/nukeop/mr-cool-nes)
NES emulator (in development)

## Getting started

### Prerequisites
You need Rust and libsdl2-dev. To install it on a Debian-derivative:

```bash
$ sudo apt update
$ #Optional dependency that you might or might not have to install
$ sudo apt install libegl1-mesa-dev libgles2-mesa-dev
$ sudo apt install libsdl2-dev
```

### Installing
To build the dev version:

```bash
$ cargo build
```

To run:

```bash
$ RUST_LOG=nes=info,mr_cool_nes=info cargo run
```

To build the release version:

```bash
$ cargo build --release --target-x86_64-unknown-linux-gnu
```

Replace the target with your platform if you're not on Linux.

## Running the tests

Rust makes this very easy.

```bash
$ cargo test
```

## Contributing

All contributions are welcome. I'm not very good at Rust so if there's something I should be doing better, let me know.


## About
### License
Copyright © 2018, [nukeop](https://github.com/nukeop).
Released under [Affero GPL License](LICENSE).
