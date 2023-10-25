# USYD-09-DataEinops

CayLang is a powerful directory description and manipulation language and toolkit.

To learn about cay, its implementation and goals you can read through the markdown files in spec.

For interacting with this iteration of Cay the FinalImplementationGuide documents all existing features with a simple example.

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

To complete the installation run
```bash
./install.sh
```
To move the binary to `/usr/bin`, or use any method of your choice to add the binary to your path.

## Unit Tests

To run all unit tests you must specify all caylang packages in this workspace with a glob specifier.
```bash
cargo test -p caylang-*
```

To check the coverage of the unit tests install [tarpaulin](https://github.com/xd009642/tarpaulin) and run.
```bash
cargo tarpaulin -p 'caylang-*' -o html
```
Then check the `taurplin-report.html` file generated in the root directory.

## System Tests
To run all the system tests you must specify the cay package for testing.
```bash
cargo test -p cay
```

## Samples

Samples are available to run in the `samples` folder, to build one and inspect the AST execute:
```bash
cay build -v samples/<program>
```
To build one and execute the code in the interpreter, run:
```bash
cay build -r samples/<program>
```

Or equivalent, i.e. `cargo run build <flags> <file_path>` or `./target/release/cay build <flags> <file_path>` if cay is not installed.

All the samples in the `parse_tests` and `final_demo` directories can be actually executed by the interpreter.

These samples operate on datasets generated in the "testbed" folder by running `./demo_setup.sh`. Ensure to run that command atleast once before interacting with the samples.

It is recommended users install and utilise the program `tree` to inspect `test/testbed` after executing the samples.

Run
```bash
tree test/testbed
```
To see the state of all test datasets, or
```bash
tree test/testbed/test_<i>
```
to see the state of test dataset i.
