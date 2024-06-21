# PuryFi-WASM-Halftone
Rust source code for PuryFi's halftone pixelation feature.

## Prerequisites

Before building, ensure you have the following tools installed:

1. **Rust Programming Language**: Install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. **wasm-pack**: A tool for building Rust-generated WebAssembly packages. Install wasm-pack from [https://rustwasm.github.io/wasm-pack/installer/](https://rustwasm.github.io/wasm-pack/installer/).

Cargo, Rust's package manager, will be installed automatically with Rust.

## Building
Just execute the following command at the root of this repository:
```
wasm-pack build --release --target no-modules
```
Which will output the `halftone_processor_bg.wasm` and `halftone_processor_bg.js` files to `pkg/`.

Additionally, the simple [`build.sh`](/build.sh) script can be used to both execute the previous command and then copy the outputted files of our interest over to the destination the extension expects them to be: `components/`. This script assumes the root directory of the extension to be found at `../../` from the root of this repository.