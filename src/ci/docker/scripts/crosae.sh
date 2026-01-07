#!/bin/bash

DISABLE_INSTRUMENTATION=1 ../x --stage 2 build std

rustup toolchain link daikon host/stage1

cd ../daikon_tests

cargo run
