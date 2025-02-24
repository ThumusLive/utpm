# Aliases
alias t := tapes
alias b := build
alias btb := build-to-bin
alias ctb := copy-to-bin
alias bl := build-lw
alias bltb := build-lw-to-bin 
alias bnltb := buildn-lw-to-bin 

# Variables
builddir := "target/release/utpm"
buildndir := "target/debug/utpm"
tpscmd := `ls tapes`
tpsdir := "tapes"
bindir := "~/.cargo/bin/utpm"

# for e in {{tpscmd}}; do vhs {{tpsdir}}/$e; done
# Make .gif for the readme (require vhs)
tapes:
    bash {{tpsdir}}/build.sh



# Build UTPM
build: format
    cargo build --release --bin utpm

# Build UTPM with nightly
buildn: format
    cargo build --features nightly --bin utpm

# Copy utpm if exists.
copy-to-bin:
    cp {{builddir}} {{bindir}}

# Copy utpm if exists.
copyn-to-bin:
    cp {{buildndir}} {{bindir}}

# Build and copy
build-to-bin: build && copy-to-bin

# Build lightweight utpm (require upx)
build-lw: build
    upx --best --lzma {{builddir}}

buildn-lw: buildn
    upx --best --lzma {{buildndir}}

# Build and copy lightweight
build-lw-to-bin: build-lw && copy-to-bin
buildn-lw-to-bin: buildn && copyn-to-bin

# Format with cargo
format: 
    cargo fmt
