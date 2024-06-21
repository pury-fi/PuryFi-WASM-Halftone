@echo off

wasm-pack build --release --target no-modules
copy pkg\halftone_processor_bg.wasm ..\..\components\
copy pkg\halftone_processor.js ..\..\components\