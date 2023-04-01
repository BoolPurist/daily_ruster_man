# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased] 

### Added 

- Templates can be given date components for day, month and year via builtin variables

## [0.5.4] - 2023.02.05

### Added 

- CLI argument show-only: For only printing out the content of journal to terminal without using editor.

## [0.5.3] - 2023.02.04

### Added 

- Editor for opening journals can be adjusted via CLI option, environment variable or option in configuration file.
- Confirmation prompt before deletion can skipped via CLI argument or environment variable.
- Deletion of a journal is only done before confirming prompt.
- Yearly, monthly and daily journals can be deleted via corresponding delete commands.
- Added example template file for documentation.
- Added example configuration file for documentation.
- Added usage file which describes configuration, templates and placeholders for documentation.

## [0.5.1] - 2023.02.02

### Added
- paths in configuration can be given relative to configuration folder path via "+" at the front of path.
- paths in configuration can be given relative to home folder via "~" at the front of path.
- paths in configuration can be given environment  variables which will be expanded.
- path to data folder can given via CLI, environment variable or in configuration file.
- path to configuration folder can given via CLI and environment   variable.

## [0.5.0] - 2023.02.01

[unreleased]: https://github.com/BoolPurist/daily_ruster_man/compare/v0.5.4...HEAD
[0.5.4]: https://github.com/BoolPurist/daily_ruster_man/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/BoolPurist/daily_ruster_man/compare/v0.5.1...v0.5.3
[0.5.1]: https://github.com/BoolPurist/daily_ruster_man/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/BoolPurist/daily_ruster_man/releases/tag/v0.5.0

