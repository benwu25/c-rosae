#!/bin/bash

BORS_HASH="$(git log --author=bors | head -1 | awk '{print $2;}')"

echo $BORS_HASH > .bors_hash
