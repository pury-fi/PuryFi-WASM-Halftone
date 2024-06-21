#!/bin/bash

wasm-pack build --release --target no-modules
cp pkg/halftone_processor_bg.wasm ../../components/halftone_processor_bg.wasm
cp pkg/halftone_processor.js ../../components/halftone_processor.js