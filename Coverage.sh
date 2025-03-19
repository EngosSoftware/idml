#!/usr/bin/env bash

WORKING_DIRECTORY=$(pwd)

# Run all tests with code coverage.
cargo llvm-cov --html
echo ""

cargo llvm-cov report

# Display links to the coverage reports.
echo ""
echo -e "\e[1;32mOpen HTML report\x3A\e[0m file://$WORKING_DIRECTORY/target/llvm-cov/html/index.html"
echo ""
