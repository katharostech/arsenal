# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    # Copy python plugin to build dir
    cp -R arsenal-blender $stage

    # Compile Python extension
    cross +nightly rustc --target $TARGET --release -- -C lto
    cp target/$TARGET/release/libarsenal.so $stage/arsenal-blender/arsenal.so
    mkdir -p $stage/arsenal-blender/bin/
    cd arsenal-runtime
    cross +stable rustc --target $TARGET --release -- -C lto
    cp target/$TARGET/release/arsenal-runtime $stage/arsenal-blender/bin/arsenal-runtime

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
