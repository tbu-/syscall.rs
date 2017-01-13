set -ex

main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
    cross test --target $TARGET --release

    cross run --target $TARGET --example hello
    cross run --target $TARGET --release --example hello
}

if [ -z $TRAVIS_TAG ]; then
    main
fi
