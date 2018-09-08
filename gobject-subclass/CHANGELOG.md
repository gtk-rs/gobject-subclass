# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html),
specifically the [variant used by Rust](http://doc.crates.io/manifest.html#the-version-field).

## [0.2.0] - 2018-09-08
### Changed
- Big refactoring and resulting simplification of how the traits work

### Added
- Support for directly subclassing GObject
- Unit test with a simple object subclass using properties, signals, etc.

### Removed
- Callback guards. Unwinding across FFI boundaries is not undefined behaviour
  anymore and cleanly panics directly.
- ObjectImpl::notify(). This is directly available via GLib now.

## [0.1.0] - 2018-05-02

- Initial release of gobject-subclass.

[Unreleased]: https://github.com/gtk-rs/gobject-subclass/compare/0.2.0...HEAD
[0.2.0]: https://github.com/gtk-rs/gobject-subclass/compare/0.1.0...0.2.0
