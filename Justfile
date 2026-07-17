all: build
    diff small/Cargo.toml small_no_lto/Cargo.toml || true
    diff big/Cargo.toml small/Cargo.toml || true
    ls -lahS target/release/*.dylib

@build:
    # rm -rf target
    echo "Building small"
    cargo build --quiet --release --manifest-path small/Cargo.toml --target-dir ./target/
    echo "Building small no lto"
    cargo build --quiet --release --manifest-path small_no_lto/Cargo.toml --target-dir ./target/
    echo "Building big"
    cargo build --quiet --release --manifest-path big/Cargo.toml --target-dir ./target/
