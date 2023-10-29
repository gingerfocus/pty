default:
    @just -l

build:
    cargo build

publish:
    cargo fmt
    cargo clippy -q -- -D warnings
    cargo test -q
    cargo build --release

loc:
    find src/ -name "*.rs" | xargs cat | wc -l
