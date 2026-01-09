#!/bin/bash

# Run a mimic of the tidy job

TIDY_PRINT_DIFF=1 ../x test \
src/tools/tidy tidyselftest --extra-checks=py,cpp,js,spellcheck
