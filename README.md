# README

This CLI tool allows you to create/manage daily, monthly and yearly journals with your editor of 
choice. You also can list your created journals by this app.

This application is still considered an alpha ! Things can break for sure.
This application is developed on a Linux system. So it is only tested on a Linux system !

## Installation

### Prerequisite

You need to have rust and cargo installed on your system to build this application and install it.
It is recommended that you install rust/cargo via rustup. Follow instrucions [here][rustup]

### Steps
1. Clone this respo.
2. Build and install this app via cargo through the following command
```text
cargo install --path '.' --force
```

### Installation path

This installs the app at ~/.cargo/bin/ by default. 
You could choose another location via the option --root.
Make sure the path ~/.cargo/bin/ or your chosen destination is in your $PATH environment variable
to make this app accessible everywhere in your terminal.

## Usage

See [usage] for more details for how to use this application.

## Examples 

Examples can be found [here][examples].

## Development

### File system access during development

If this app is complied in debug mode then a folder called ".dev_data" at the project root is used
for accessing/saving configuration/journals.

This allows testing/development the app without affecting the actual user journals/configuration.

- .dev_data/conf for configuration path
- .dev_data/share for path to saved journals

All dev data folders are not checked in. 
The folder .dev_data and its subfolders mentioned above, are created if not 
present when the app needs them.


### Bumping version

If a significant version bump happens or a release is approaching then do the following things

- Bump the version in the [Cargo File]
- Adjust changelog file [Changelog]
- Bump version in the [Rust Cli File] 

### CI

This respo runs github workflow ci for validation of formatting, linting and unit test 
on pushes and pull requests.
The workflow currently uses a bash [script](./ci_check.sh).
You can see if the ci on github works via running this script. It is recommended that you start this
script before a push.

## License
Licensed is under either of Apache License, Version 2.0 or MIT license at your option. 

[Cargo File]:Cargo.toml
[Changelog]:CHANGELOG.md
[Rust Cli File]:/src/cli/app_args.rs
[usage]:/doc/usage.md
[examples]:/doc/examples.md
[rustup]:https://rustup.rs/
