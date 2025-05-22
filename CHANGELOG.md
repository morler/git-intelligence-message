# Changelog

## [1.3.0] - 2025-05-22

### Added
- New `prompt` subcommand to manage AI prompt templates
  - View both diff and subject prompt templates
  - Edit prompt files with `--edit` flag
  - Support for custom editors with `--editor` option
  - Cross-platform file management (macOS, Windows, Linux)
  - Short aliases for prompt types (d/diff/diff_prompt, s/subject/subject_prompt)

### Changed
- Improved error handling and user feedback for prompt operations
- More descriptive command-line help text for prompt management

## [1.2.2] - 2025-05-21

### Added
- Added support for new AI model endpoints:
  - GLM (ChatGLM)
  - DeepSeek
  - Qianfan (Baidu)
- Updated HTTP client to handle the new model endpoints

## [1.2.1] - 2025-05-21

### Restored
- It’s a sad story. All the code I’d been working on for days was suddenly gone. Luckily, I happened to try the IDE’s REDO function, and it turned out to be incredibly helpful. In the end, I was able to get back almost everything.
- Automatic update check on application start (unimplemented in previous version)
- Add url constant for Duobao model

## [1.2.0] - 2025-05-20

### Added
- New `update` command to check for and install updates
  - Added `--force` flag to force reinstall even when on the latest version
- Automatic update check on application start

### Changed
- Updated dependencies
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