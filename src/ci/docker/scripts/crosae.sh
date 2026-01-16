#!/bin/bash

# Build a stage 2 c-rosae compiler and run the c-rosae tests with the
# stage2 build.

DISABLE_INSTRUMENTATION=1 ../x --stage 2 build std

# move the testing environment to /checkout/obj since /checkout is read-only
mkdir /checkout/obj/daikon_tests/

cp -r /checkout/daikon_tests/test /checkout/obj/daikon_tests/

cd /checkout/daikon_tests

# the tests will run in the new environment with the CI environment variable set
CROSAE_CI=1 RUSTC=/checkout/obj/build/host/stage0/bin/rustc \
/checkout/obj/build/host/stage0/bin/cargo run --target-dir  \
/checkout/obj/target_daikon_tests
