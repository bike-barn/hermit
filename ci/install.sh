set -ex

main() {
    local target=
    if [ $TRAVIS_OS_NAME = linux ]; then
        target=x86_64-unknown-linux-musl
        sort=sort
    else
        target=x86_64-apple-darwin
        sort=gsort  # for `sort --sort-version`, from brew's coreutils.
    fi

    # Builds for iOS are done on OSX, but require the specific target to be
    # installed.
    case $TARGET in
        aarch64-apple-ios)
            rustup target install aarch64-apple-ios
            ;;
        armv7-apple-ios)
            rustup target install armv7-apple-ios
            ;;
        armv7s-apple-ios)
            rustup target install armv7s-apple-ios
            ;;
        i386-apple-ios)
            rustup target install i386-apple-ios
            ;;
        x86_64-apple-ios)
            rustup target install x86_64-apple-ios
            ;;
    esac

    # This fetches latest stable release
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/cross \
                       | cut -d/ -f3 \
                       | grep -E '^v[0.1.0-9.]+$' \
                       | $sort --version-sort \
                       | tail -n1)
    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git japaric/cross \
           --tag $tag \
           --target $target

    cargo clippy --version || ( rustup component add clippy )
    curl -LSfs https://github.com/xd009642/tarpaulin/releases/download/0.7.0/cargo-tarpaulin-0.7.0-travis.tar.gz | tar -C ~/.cargo/bin xz
    curl -o /tmp/urchin https://raw.githubusercontent.com/tlevine/urchin/v0.0.6/urchin && chmod +x /tmp/urchin
    git fetch --unshallow
    git config remote.$(git remote | head -n1).fetch "+refs/heads/*:refs/remotes/$(git remote | head -n1)/*"
    git fetch $(git remote | head -n1) master
    git checkout master
    git checkout -
}

main
