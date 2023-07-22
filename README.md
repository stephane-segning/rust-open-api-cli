# openapi_cli

openapi_cli is a Rust-based command-line tool for generating code from OpenAPI specifications. It provides a simple way to download the OpenAPI generator and then generate code in a specified language.

## Features
- Downloads the OpenAPI generator CLI if it's not already present
- Generates code in a specified language from an OpenAPI specification file
- Provides a command-line interface for easy usage

## Prerequisites
- Rust programming language (You can install Rust from [here](https://rustup.rs/))
- Java Development Kit (JDK)

## Installation

```bash
$ git clone https://github.com/stephane-segning/rust-open-api-cli.git
$ cd rust-open-api-cli
$ cargo build --release
```

The executable will be located in the **target/release** directory.

## Usage

To use the application, run the command below from the command line:

```bash
$ ./target/release/openapi_cli generate -v <version> -i <input> -o <output> -l <language>
```

Replace <version> with the version of OpenAPI generator CLI, `<input>` with the path to your OpenAPI specification file, <output> with the path to the directory where the generated code should be saved, and <language> with the target language for the generated code.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)