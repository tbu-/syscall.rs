# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.1.5] - 2017-02-03

### Fixed

- The syscall number for GETRANDOM on x86

## [v0.1.4] - 2017-02-02

### Added

- GETRANDOM and NEWFSTATAT syscalls

## [v0.1.3] - 2017-01-21

### Fixed

- Error when using MIPS' syscall5/syscall6 with optimizations enabled

## [v0.1.2] - 2017-01-21

### Added

- syscall5 and syscall6 for mips

## [v0.1.1] - 2017-01-14

### Fixed

- syscall2 for linux-aarch64. There was a type in the register constraints.

## v0.1.0 - 2017-01-13

Initial release. Forked from [syscall] v0.2.1.

[syscall]: https://crates.io/crates/syscall

- Support for ARM, Aarch64, MIPS, MIPS64, PowerPC, PowerPC64, SPARC64, x86 and
  x86_64 Linux

- Support for x86_64 macOS

- Support for x86_64 FreeBSD

[Unreleased]: https://github.com/japaric/syscall.rs/compare/v0.1.5...HEAD
[v0.1.5]: https://github.com/japaric/syscall.rs/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/japaric/syscall.rs/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/japaric/syscall.rs/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/japaric/syscall.rs/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/syscall.rs/compare/v0.1.0...v0.1.1
