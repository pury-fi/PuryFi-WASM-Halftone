@echo off

rmdir /s /q dist
wasm-pack build --out-dir dist/browser --out-name halftone --dev --target web
wasm-pack build --out-dir dist/node --out-name halftone --dev --target nodejs
del dist\browser\.gitignore
del dist\node\.gitignore
del dist\browser\package.json
del dist\node\package.json
del dist\browser\README.md
del dist\node\README.md