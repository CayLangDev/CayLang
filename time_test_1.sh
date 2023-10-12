log=/dev/null
rlog=run_log
cargo build --bins --release >> $log
./test/tests_clean.sh >> $log
python test/simple_tests.py >> $log
tree test/testbed/test_1
time ./target/release/cay build -v -r samples/simple_test_1.cay
cat samples/simple_test_1.cay
tree test/testbed/test_1
