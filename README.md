# README

## Roadmap

### Features

- Provide build in placeholders for data the app knows like date of the created journal 
or the template content.
- Add option to only show content of created journal 
  or content open first opening journal aka templates
- Option to create journal with skipping inserting template
- Option to reset journal with empty content or with template 

### Documentation

#### Usage
- editing/opening of journals
- listing journals

#### Examples
- examples for editing/opening of journals
- examples for listing journals

## Usage

See [usage] for more details for how to use this application.

## Examples 

Examples can be found [here][examples].

## Development

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
