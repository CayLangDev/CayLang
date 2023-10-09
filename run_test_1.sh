log=/dev/null

./test/tests_clean.sh >> $log
python test/simple_tests.py >> $log
tree test/testbed/test_1
cargo run build -r -v samples/simple_test_1.cay &>> $log
cat samples/simple_test_1.cay
tree test/testbed/test_1
