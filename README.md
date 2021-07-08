<div align="center">

![Logo](public/logo-192.png)

# SamsonR

A simple command line client for [Samson](https://github.com/zendesk/samson), written in Rust.

[![Made with Rust](https://shields.io/badge/Made_with-Rust-f05869?style=flat)](https://www.rust-lang.org/)
[![Probably works](https://shields.io/badge/Probably-works-f6a34d?style=flat)]()
[![License MIT](https://shields.io/badge/License-MIT-e0dd52?style=flat)](#license)

[Key Features](#key-features) · [Installation](#installation) · [How to Use](#how-to-use) · [Contributing](#contributing) · [License](#license)

</div>

## Installation

Clone this repository, and run:

```shell
# Clone the repository
$ cargo run
```

## How to Use

For usage information execute `samsonr --help`. To start with, create a new configuration file in `$XDG_CONFIG_HOME/samsonr/config.toml`:

```toml
url = "<url-to-your-samson-instance>"
token = "<authorization-token>" # Optional
project_id = 2 # Optional
```

The following commands are available:

- `projects`: List available projects on this instance
- `stages`: List stages for a specific project
- `deploy`: Deploy the specified reference to a stage. If no reference is specified, falls back to the current branch.

## Contributing

Merge requests and bug reports are welcome. This is my first time working with Rust, so be kind :heart:

## License

[MIT](LICENSE)
