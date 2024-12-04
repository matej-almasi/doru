# Rudo

Rudo is a simple TODO list in your terminal.

## Features

### Add new TODOs

```sh
rudo add "Code something amazing"
```

### List all your TODOs

```sh
rudo list
[ ] Learn Rust           [Open] (ID: 1)
[ ] Build a project      [InProgress] (ID: 2)
[ ] Publish on GitHub    [Open] (ID: 3)
```

### Edit a TODO

```sh
rudo edit 1 "Learn Rust like a Pro"
```

### Update status of a TODO

```sh
rudo status 1 in-progress
```

### List TODOs with specific status

```sh
rudo list in-progress
[ ] Learn Rust like a Pro [InProgress] (ID: 1)
[ ] Build a project      [InProgress] (ID: 2)
```

### Delete a TODO

```sh
rudo delete 3
```

## Installation

### Using Cargo Install

You can use Cargo to install Rudo from [crates.io](https://crates.io/)

### Download the binary directly

TODO: add instructions to download artifact from GitHub, when ready

If you want to run Rudo like in the examples above, you will need to add Rudo
to your Path.

### Build from source

If you have [Rust](https://www.rust-lang.org/) toolchain installed on your
machine, you can clone directly from GitHub and build locally. No additional
dependencies are required:

```sh
git clone https://github.com/yourusername/rudo.git
cd rudo
cargo build --release
```

## Configuration

Rudo stores TODO items in a JSON file. By default, this file is located at
`~/.rudo/TODOs.json`. You can change the location of the storage file by
setting the `RUDO_PATH` environment variable or by using the `--path`
option with the `rudo` command.

## Documentation

TODO: add docs.rs link

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
