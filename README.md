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

Samples are available to run in the `samples` folder, to run one execute:
```bash
./target/release/cay build samples/<program>
```

To demonstrate the current progress of the interpreter, run from the project root
```
python test/simple_tests.py
cargo run build -r -v samples/simple_test_1.cay
```

you may need to first run 
```
./test/tests_clean.sh
```
if you've already ran `python test/simple_tests.py` previously.

and then 
```
cargo run build -r -v samples/simple_test_2.cay
```
(this might not work)
