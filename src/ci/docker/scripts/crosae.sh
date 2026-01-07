#!/bin/bash

DISABLE_INSTRUMENTATION=1 ../x --stage 2 --keep-stage 0 --keep-stage-std 0 --keep-stage 1 --keep-stage-std 1 build std

cd ../daikon_tests

# cargo is in build/host/stage0/bin/
../build/host/stage0/bin/cargo run
