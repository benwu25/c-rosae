#!/bin/bash

DISABLE_INSTRUMENTATION=1 ../x --stage 2 build std

cd ../daikon_tests

# cargo is in build/host/stage0/bin/
../build/host/stage0/bin/cargo run
