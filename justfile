# Aliases
alias b := build-release
alias n := build-nightly
alias u := upx
alias c := copy

# Variables
builddir := "target/release/utpm"
buildndir := "target/debug/utpm"
bindir := "~/.cargo/bin/utpm"
tjust := "target/just"

prepare:
    mkdir -p {{tjust}}

# Build UTPM release
build-release: format prepare
    cargo build --release --bin utpm
    cp {{builddir}} {{tjust}}/utpm

# Build UTPM nightly
build-nightly: format prepare
    cargo build --features nightly --bin utpm
    cp {{builddir}} {{tjust}}/utpm

# Copy utpm if exists.
copy:
    cp {{tjust}}/utpm {{bindir}}

# 
upx:
    upx --best --lzma {{tjust}}/utpm

# Format with cargo
format: 
    cargo fmt
