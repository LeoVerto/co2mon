# Changelog

## [1.0.0] - 2019-05-17

### Changed

- Simplified timeout logic

## [1.0.0-rc.3] - 2019-05-16

### Added

- Fixed docs.rs build

## [1.0.0-rc.2] - 2019-05-16

### Added

- Improved documentation

## [1.0.0-rc.1] - 2019-05-16

### Added

- Added optional `serde` support
- Added `Sensor::read` to read both values at once
- Improved documentation

### Changed

- `OpenOptions` is no longer consumed when used
- Implemented common traits for `SingleReading` and `OpenOptions`
- Renamed `OpenOptions::with_serial` to `with_serial_number`
- Renamed `Sensor::read` to `read_one`

## [0.1.0-alpha.4] - 2019-02-02

### Fixed

- Added report id to fix the Windows support

## [0.1.0-alpha.3] - 2019-02-02

### Added

- Added read timeout support
- Added features to control `hidapi` linking

### Changed

- Set default read timeout to 5 seconds

### Fixed

- Short USB reads are now reported as errors

## [0.1.0-alpha.2] - 2019-02-01

### Changed

- Bumped `zo-co2` dependency

## [0.1.0-alpha.1] - 2019-02-01

### Added

- Initial release
