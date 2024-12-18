# anabit

`anabit` is a command-line tool for extracting meta-information from Xilinx FPGA `.bit` files (bitstream files). It parses embedded data such as the Xilinx FPGA device, compilation date/time, top design name, and additional options, then outputs them in CSV format.

## Features

- Extracts Xilinx FPGA device info, compile date/time, design name, option information, and more from a `.bit` file.
- Automatically detects the most recently modified `.bit` file in the current directory if no file is specified.
- Can optionally append the extracted information to a CSV file.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/) and Cargo installed.

You can install `anabit` directly from GitHub using:

```bash
cargo install --git https://github.com/popons/anabit.git
```

This will build and place the `anabit` binary in your Cargo bin directory (e.g., `~/.cargo/bin`).

## Usage

### Basic Usage

- Running `anabit` without arguments will automatically find the latest `.bit` file in the current directory and print the extracted information as CSV to standard output:

  ```bash
  anabit
  ```

### Specify a `.bit` File

- To explicitly specify which `.bit` file to parse:

  ```bash
  anabit path/to/file.bit
  ```

### Append to a CSV File

- To append the output to a CSV file, use the `--append-to` option. If the file does not exist, it will be created with a header row.

  ```bash
  anabit --append-to results.csv
  ```

- If you also specify the `.bit` file:

  ```bash
  anabit path/to/file.bit --append-to results.csv
  ```

## Output Format

The output is given in CSV format, either printed to standard output or appended to the specified CSV file. It will look like this:

```
path,compiled date,top,device,full file md5,body section md5,option,memo,
<bit_filename_or_empty>,<compile_datetime>,<top_design_name>,<device_name>,<full_file_md5>,<body_section_md5>,<options>, 
```

## License

Please refer to the `LICENSE` file for information about licensing.

## Development Notes

- This project uses Rust 2021 edition.
- Depends on crates such as `anyhow`, `clap`, `md5`, and `nom`.
- Code formatting is managed by `.rustfmt.toml`.

## Contributions and Issues

Please feel free to open an Issue or submit a Pull Request if you find bugs or have suggestions for improvements.