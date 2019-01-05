#!/bin/bash

set -e

publish() {
    cargo publish --manifest-path render/Cargo.toml
    cargo publish --manifest-path input/Cargo.toml
    cargo publish --manifest-path prototty/Cargo.toml
    cargo publish --manifest-path grid/Cargo.toml
    cargo publish --manifest-path unix/Cargo.toml
    cargo publish --manifest-path wasm/Cargo.toml
    cargo publish --manifest-path glutin/Cargo.toml
    cargo publish --manifest-path common/Cargo.toml
    cargo publish --manifest-path file-storage/Cargo.toml
}

read -r -p "Are you sure? " response
case "$response" in
    [yY][eE][sS])
        publish
        ;;
    *)
        echo "ok then"
        ;;
esac
