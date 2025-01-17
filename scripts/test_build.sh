#!/bin/bash

cd test_libasm

cargo build --release

echo building test executable...

sleep 3

cp target/release/test_libasm ../test
