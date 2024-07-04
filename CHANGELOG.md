# Changelog

## [0.2.0]

### Added

- Command-line argument parsing using the `clap` crate.
- Option to fix PE headers (`--fix-headers`).
- Option to extract resources from the PE file (`--extract-resources`).
- Option to analyze PE metadata (`--analyze-metadata`).
- Progress bar display during file saving using the `indicatif` crate.
- Parallel processing of file chunks during saving using the `rayon` crate.

### Changed

- Refactored the code into separate functions for better readability and maintainability.
- Improved error handling and error reporting.
- Replaced direct file I/O with memory-mapped file access using the `memmap2` crate.
- Changed the output file naming convention to include the original file extension.

### Removed

- Removed the logic for finding the end of the file by searching for null bytes (0x30, 0x80...)

## [0.1.0] - Initial Release

### Added

- Basic functionality to remove the "pumped" data from a PE file.
- Parsing of PE files using the `goblin` crate.
- Command-line argument handling for input file path.
- Output file generation with "-trimmed" appended to the original filename.

[Unreleased]: https://github.com/your-repo/file-unpumper/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/your-repo/file-unpumper/releases/tag/v0.1.0
