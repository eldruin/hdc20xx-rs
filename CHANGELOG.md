# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Replaced usage of `nb::block!` macro in examples with explicit loop including
  a delay between repetitions. Without this the repetitions can be too quick and
  some devices do not respond correctly. Thanks to @bernardoaraujor for noticing this.

## [0.1.0] - 2020-03-01

Initial release to crates.io.

[Unreleased]: https://github.com/eldruin/hdc20xx-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/eldruin/hdc20xx-rs/releases/tag/v0.1.0
