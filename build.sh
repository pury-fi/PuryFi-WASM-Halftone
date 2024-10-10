#!/bin/bash

rm -rf dist
wasm-pack build --out-dir dist/browser --out-name halftone --release --target web
wasm-pack build --out-dir dist/node --out-name halftone --release --target nodejs
rm dist/browser/.gitignore
rm dist/node/.gitignore
rm dist/browser/package.json
rm dist/node/package.json
rm dist/browser/README.md
rm dist/node/README.md