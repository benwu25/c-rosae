#!/bin/bash

# Since CI performs a shallow clone and we do not have automated bors
# commits available in CI in general, we currently save the latest commit
# by bors in a file and push the hash to allow our CI to run and download
# ci-artifacts resources. This script can be run as is or via a git alias
# before committing.

# FIXME: write a script to set up git aliases and document how this script
# should typically be executed before making a commit and running CI.

BORS_HASH="$(git log --author=bors | head -1 | awk '{print $2;}')"

echo $BORS_HASH > .bors_hash
