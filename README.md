# PuryFi-WASM-Halftone
Rust source code for PuryFi's halftone pixelation feature.

## Building
Just execute the following command at the root directory of this repository:
```
wasm-pack build --release --target no-modules
```
Which will output the `halftone_processor_bg.wasm` and `halftone_processor_bg.js` files to `pkg/`.

Additionally, the `build.sh` script can be used to both execute the previous command and then copy the outputted files of our interest over to the destination the extension expects them to be in: `components/`. This scripts assumes the root directory of the extension to be found at `../../` from the root directory of this repository.