#!/bin/bash

DISABLE_INSTRUMENTATION=1 ../x --stage 2 build std

cd ../daikon_tests

RUSTC=/checkout/obj/build/host/stage0/bin/rustc /checkout/obj/build/host/stage0/bin/cargo run
