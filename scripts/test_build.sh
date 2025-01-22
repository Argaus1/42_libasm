#!/bin/bash

cd test_libasm

echo -e "which tests do you wish to execute?\n
    all\t\t->\trun all tests\n
    mandatory\t->\trun mandatory tests\n
    bonus\t->\trun bonus tests\n
    other:
    strlen
    strcpy
    strcmp
    write
    read
    strdup"

read testchoice

case $testchoice in
    "all") cargo test;;
    "strlen") cargo test strlen;;
    "strcpy") cargo test strcpy;;
    "strcmp") cargo test strcmp;;
    "write") cargo test write;;
    "read") cargo test read;;
    *) echo "This test is not available";;
esac
