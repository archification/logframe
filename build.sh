#!/bin/bash

# linux
#RUSTFLAGS="-Zlocation-detail=none" cargo build --release
RUSTFLAGS="-Zlocation-detail=none" cross build --target x86_64-unknown-linux-musl --release
# windows
#RUSTFLAGS="-Zlocation-detail=none" cargo build --target x86_64-pc-windows-gnu --release --verbose
#RUSTFLAGS="-Zlocation-detail=none" cross build --target x86_64-pc-windows-gnu --release --verbose

#upx --best --lzma target/release/logframe
upx --best --lzma target/x86_64-unknown-linux-musl/release/logframe
#upx --best --lzma target/x86_64-pc-windows-gnu/release/logframe.exe
