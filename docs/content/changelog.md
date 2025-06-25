# Changelog

## [1.4.0] - 2025-06-26

- Added mkdocs docs
- Added custom param `lines_limit` to halt app running when too many changes
- Added support for set 'update' block params: `gim update --max <M> --interval <V>`

## [1.3.2] - 2025-06-17

- add command to show 'ai' block

## [1.3.1] - 2025-05-22

### Changed
- Docs rule

## [1.3.0] - 2025-05-22

### Added
- New `prompt` subcommand to manage AI prompt templates
  - View both diff and subject prompt templates
  - Edit prompt files with `--edit` flag
  - Support for custom editors with `--editor` option
  - Short aliases for prompt types (d/diff/diff_prompt, s/subject/subject_prompt)