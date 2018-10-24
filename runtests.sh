#!/usr/bin/env bash

function ignore_tests() {
    for file in ${IGNORE_SUBCOMMAND_LIST[*]}
    do
        mv test/{,.}${file}
    done
}

function restore_tests() {
    for file in ${IGNORE_SUBCOMMAND_LIST[*]}
    do
        mv test/{.,}${file}
    done
}

function run_tests() {
    export TEST_PROFILE_NAME=$1
    printf '\n\033[0;33m%s\033[0m\n' "Running tests in test with profile name $TEST_PROFILE_NAME"
    $URCHIN test/
}

URCHIN=$(which urchin)
URCHIN=${URCHIN:-/tmp/urchin}

set -e

cargo build --verbose
cargo test  --verbose

export TESTDIR=".test-place"

# Integration test shell hermit

export PROFILE_DIR_NAME=profiles
export USE_COMMAND=use

RANDOM_PROFILE_NAME=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 8 | head -n 1)

run_tests default

run_tests ${RANDOM_PROFILE_NAME}


# Integration test Rust hermit

export USE_HERMIT_RS=true
export PROFILE_DIR_NAME=shells
export USE_COMMAND=inhabit

IGNORE_SUBCOMMAND_LIST=(add clone commands completions doctor git use link unlink utilities)

ignore_tests

trap restore_tests EXIT

run_tests default

run_tests ${RANDOM_PROFILE_NAME}
