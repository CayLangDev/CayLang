# USYD-09-DataEinops

CayLang is a powerful directory description and manipulation language and toolkit.

## Installation
### Compile from source
CayLang is built in Rust, so in order to build it you'll need to install the Cargo toolchain. Afterwards, you can build the binaries with the following command:
```bash
cargo build --bins --release
```
This will create a `target/release` directory containing the binaries. To run the executable, you can execute:
```bash
./target/release/cay <command>
```
Calling `./target/release/cay help` will display a list of commands.