# file-unpumper

`file-unpumper` is a powerful command-line utility designed to clean and analyze Portable Executable (PE) files. It provides a range of features to help developers and security professionals work with PE files more effectively.

## Features

- **PE Header Fixing**: `file-unpumper` can fix and align the PE headers of a given executable file. This is particularly useful for resolving issues caused by packers or obfuscators that modify the headers.

- **Resource Extraction**: The tool can extract embedded resources from a PE file, such as icons, bitmaps, or other data resources. This can be helpful for reverse engineering or analyzing the contents of an executable.

- **Metadata Analysis**: `file-unpumper` provides a comprehensive analysis of the PE file's metadata, including information about the machine architecture, number of sections, timestamp, subsystem, image base, and section details.

- **File Cleaning**: The core functionality of `file-unpumper` is to remove any "pumped" or padded data from a PE file, resulting in a cleaned version of the executable. This can aid in malware analysis, reverse engineering, or simply reducing the file size.

- **Parallel Processing**: To ensure efficient performance, `file-unpumper` leverages the power of parallel processing using the `rayon` crate, allowing it to handle large files with ease.

- **Progress Tracking**: During the file cleaning process, a progress bar is displayed, providing a visual indication of the operation's progress and estimated time remaining.

## Installation

`file-unpumper` is written in Rust and can be easily installed using the Cargo package manager:

```bash
cargo install file-unpumper
```

## Usage

- `<INPUT>`: The path to the input PE file.

### Options

- `--fix-headers`: Fix and align the PE headers of the input file.
- `--extract-resources`: Extract embedded resources from the input file.
- `--analyze-metadata`: Analyze and display the PE file's metadata.
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

## Examples

1. Clean a PE file and remove any "pumped" data:

  ```bash
  file-unpumper path/to/input.exe
  ```

1. Fix the PE headers and analyze the metadata of a file:

  ```bash
  file-unpumper --fix-headers --analyze-metadata path/to/input.exe
  ```

1. Extract resources from a PE file:

  ```bash
  file-unpumper --extract-resources path/to/input.exe
  ```

1. Perform all available operations on a file:

  ```bash
  file-unpumper --fix-headers --extract-resources --analyze-metadata path/to/input.exe
  ```

## Contributing

Contributions to `file-unpumper` are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/0x11DFE/file-unpumper).

## Changelog

The latest changelogs can be found in [CHANGELOG.md](CHANGELOG.md)

## License

`file-unpumper` is released under the [MIT License](https://opensource.org/license/MIT).
