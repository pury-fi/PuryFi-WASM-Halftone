# PuryFi's WASM Halftone

PuryFi's halftone feature as a WASM module.

## Building Prerequisites

Before building, ensure you have the following tools installed:

1. **Rust Programming Language**: Install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. **wasm-pack**: A tool for building Rust-generated WebAssembly packages. Install wasm-pack from [https://rustwasm.github.io/wasm-pack/installer/](https://rustwasm.github.io/wasm-pack/installer/).

Cargo, Rust's package manager, will be installed automatically with Rust.

## Building

To build the project, follow these steps:

1. Clone the repository and enter it if you haven't already:

   ```
   git clone https://github.com/pury-fi/PuryFi-WASM-Halftone.git
   cd PuryFi-Wasm-Halftone
   ```

2. Run the appropriate build script based on your operating system:

   -  For Windows, run:

      ```
      ./build.bat
      ```

   -  For Unix-based systems (Linux, macOS), run:
      ```
      ./build.sh
      ```

The resulting npm modules for the browser and node will then be found under `npmbrowser` and `npmnode` respectively.
