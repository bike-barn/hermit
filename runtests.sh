#!/usr/bin/env bash

export TESTDIR=".test-place"

printf '\n\033[0;33m%s\033[0m\n' "Running tests in test"
/tmp/urchin test/
