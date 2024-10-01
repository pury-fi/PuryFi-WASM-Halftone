@echo off

rmdir /s /q npm
wasm-pack build --out-dir npm/browser --out-name index --dev --target web
wasm-pack build --out-dir npm/node --out-name index --dev --target nodejs