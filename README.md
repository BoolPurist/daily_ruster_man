# README

## Roadmap

- Editor should be adjustable via CLI, configuration file and environment variable.
- Create configuration file with defaults and comments
- Commands to delete created journals.
- Option to reset journal with empty content or with template 

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

