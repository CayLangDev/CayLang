# log=/dev/null

./test/tests_clean.sh
mkdir -p test/testbed/test_1
python3 test/simple_tests.py
tree test/testbed/test_1
cargo run build -r -v samples/simple_test_1.cay
cat samples/simple_test_1.cay
tree test/testbed/test_1
