# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) and this 
project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.2.0]

### Added
- `libui::build!` macro for easier UI description

### Changed
- No API changes.

### Deprecated
- No API deprecations.

### Removed
- No API removals.

### Fixed
- Windows: `Combobox` inside `TabGroup` now render properly.
- Windows: Reduced flicker during resize.
- `MenuItem::on_clicked()` no longer requires an UI handle.

### Security
- No security changes.

## [0.1.0]

This is the initial version of the library, based on [`iui`](https://github.com/rust-native-ui/libui-rs/tree/3496903ae9c4fd68731587dc11739da90a4f0e12). The following changes refer to this version.

### Added
- Added the `SearchEntry` control
- Added the `ColorButton` control
- Added the `DateTimePicker` control
- Added the `FontButton` control
- Added the `Table` control
- Added the `Form` control
- Added the `EditableCombobox` control
- New features for `Combobox`
- Readonly property for the `MultilineEntry`
- Folder dialogs

### Changed
- Removed the `UI` token from most functions to simplify the API, at the cost of loosing the static library initialization check and lifetime guarantee. Make sure the `UI` library handle is kept till the end manually instead.

### Deprecated
- No API deprecations.

### Removed
- Removed the dependency on the `failure` crate, using the `std::error::Error` trait instead.

### Fixed
- Fixed a memory leak in the file dialog functions.

### Security
- No security changes.