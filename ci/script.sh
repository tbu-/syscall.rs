set -ex

main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ -n $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
    cross test --target $TARGET --release
}

if [ -z $TRAVIS_TAG ]; then
    main
fi
