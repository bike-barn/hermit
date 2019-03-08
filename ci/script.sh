# This script takes care of testing your crate

set -ex

main() {
    # Only run clippy on the basic linux target
    if [[ "$TARGET" -eq x86_64-unknown-linux-gnu ]]; then
        cargo clippy --version || ( rustup component add clippy )
        cargo clippy
    fi

    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    ./runtests.sh

    cross test --target $TARGET
    cross test --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
