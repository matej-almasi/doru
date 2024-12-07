# Doru

A simple TODO list in your terminal. ✅  
Use as a CLI or use the lib in your own Rust project! 🚀

## Features

### Add new TODOs

```sh
doru add "Code something amazing"
```

### List all your TODOs

```sh
doru list
[ ] Learn Rust           [Open] (ID: 1)
[ ] Build a project      [InProgress] (ID: 2)
[ ] Publish on GitHub    [Open] (ID: 3)
```

### Edit a TODO

```sh
doru edit 1 "Learn Rust like a Pro"
```

### Update status of a TODO

```sh
doru status 1 in-progress
```

### List TODOs with specific status

```sh
doru list in-progress
[ ] Learn Rust like a Pro [InProgress] (ID: 1)
[ ] Build a project      [InProgress] (ID: 2)
```

### Delete a TODO

```sh
doru delete 3
```

## Installation

### Using Cargo Install

You can use Cargo to install Doru from [crates.io](https://crates.io/)

### Download the binary directly

You can also download Doru [directly](https://github.com/matej-almasi/doru/releases) as a standalone binary.

If you want to run Doru like in the examples above, you will need to add Doru
to your Path.

### Build from source

If you have [Rust](https://www.rust-lang.org/) toolchain installed on your
machine, you can clone directly from GitHub and build locally. No additional
dependencies are required:

```sh
git clone https://github.com/yourusername/doru.git
cd doru
cargo build --release
```

## Configuration

Doru stores TODO items in a JSON file. By default, this file is located at
`~/.doru/TODOs.json`. You can change the location of the storage file by
setting the `DORU_PATH` environment variable or by using the `--path`
option with the `doru` command.

## Documentation

Run doru with -h or --help option to learn about available commands and how to
use them:

```sh
doru --help
```

For the library documentation, check out the
[docs](https://docs.rs/doru/latest/doru/).

If you cloned the repository, you can run

```sh
cargo doc --no-deps --open
```

to generate the documentation and open it in your browser.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
