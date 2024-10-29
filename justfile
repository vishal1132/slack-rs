alias m:=move

build:
    @cargo build

build_warnings:
    @cargo rustc -- -Awarnings

move: build_release
    @mv target/release/slack-rs ~/.local/bin/slack

build_release:
    @cargo build --release