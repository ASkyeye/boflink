# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2025-06-28

### Fixed

- Link libraries from the `.drectve` section not being parsed properly (58f0f2d).

## [0.3.0] - 2025-06-27

### Added

- Additional ignored command line arguments for better GCC compatibility (f3d02e2).
- `--no-merge-groups` flag to not merge grouped sections (#19).
- Whole archive linking using `--whole-archive/--no-whole-archive` (#20).
- Debug logging header with the entire command line used to invoke boflink when the `-v/--verbose` flag is passed (e8e8d47).
- Support for including MinGW link library search paths using the `--mingw[64|32]/--ucrt[64|32]` flag (#23).

### Changed

- Relax requirements on undefined external symbols (#18).

## [0.2.0] - 2025-06-09

### Added

- GC sections (`--gc-sections`) support for discarding unreferenced sections (#10).

### Fixed

- Incorrect checksum value calculation for GCC `.rdata$zzz` metadata sections (#7).
- Entrypoint symbol from the command line not being validated (#8).
- Relocations not being applied correctly when compiling with  `-ffunction-sections` (#9).

## [0.1.1] - 2025-06-07

### Added

- Github dependabot.yml configuration file for tracking dependency updates [`d649273`](https://github.com/MEhrn00/boflink/commit/d6492734b6f8df84f0cffebf69ac1522632ce658).
- Enable dependency graph in Github https://github.com/MEhrn00/boflink/network/dependencies.

### Fixed

- Issues with COMMON symbols not being allocated properly (#6).

### Changed

- Update object crate to 0.37.0 (#4).

## [0.1.0] - 2025-05-30

- Initial release

[unreleased]: https://github.com/MEhrn00/boflink/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/MEhrn00/boflink/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/MEhrn00/boflink/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/MEhrn00/boflink/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/MEhrn00/boflink/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/MEhrn00/boflink/releases/tag/v0.1.0
