#!/usr/bin/env bash

URCHIN=$(which urchin)
URCHIN=${URCHIN:-/tmp/urchin}

export TESTDIR=".test-place"

printf '\n\033[0;33m%s\033[0m\n' "Running tests in test"
$URCHIN test/
