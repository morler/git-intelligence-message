# Changelog

## [1.2.0] - 2025-05-19

### Added
- New `update` command to check for and install updates
  - Added `--force` flag to force reinstall even when on the latest version
- Automatic update check on application start

### Changed
- Updated dependencies
- Removed `self-update` dependency as we're now using Homebrew for updates
- Improved error handling for update process

## [1.1.0] - 2025-05-19

### Added
- New `gim-config` crate for better configuration management
- Support for modular configuration handling

### Changed
- **Breaking Change**: Moved configuration logic to external `gim-config` crate
- Updated project structure for better code organization
- Improved error handling for configuration management

### Dependencies
- Added `gim-config` as a local path dependency
- Updated project version to 1.1.0

### Migration
- Update imports from `crate::config` to `gim_config::config`
- Ensure `gim-config` crate is available at the specified path

### Internal
- Refactored configuration-related code into a separate crate
- Updated test cases to use the new configuration module
- Improved code maintainability through better separation of concerns