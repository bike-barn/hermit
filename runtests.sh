#!/usr/bin/env bash

URCHIN=$(which urchin)
URCHIN=${URCHIN:-/tmp/urchin}

set -e

cargo build --verbose
cargo test  --verbose

export TESTDIR=".test-place"
export TEST_PROFILE_NAME=default

printf '\n\033[0;33m%s\033[0m\n' "Running tests in test"
$URCHIN test/

export TEST_PROFILE_NAME=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 8 | head -n 1)
printf '\n\033[0;33m%s\033[0m\n' "Running tests in test with profile name $TEST_PROFILE_NAME"

$URCHIN test/
