@echo off

rmdir /s /q npm
wasm-pack build --out-dir npm/browser --out-name index --release --target web
wasm-pack build --out-dir npm/node --out-name index --release --target nodejs