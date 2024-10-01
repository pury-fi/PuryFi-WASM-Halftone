#!/bin/bash

rm -rf dist
wasm-pack build --out-dir dist/browser --out-name index --dev --target web
wasm-pack build --out-dir dist/node --out-name index --dev --target nodejs
rm dist/browser/.gitignore
rm dist/node/.gitignore
rm dist/browser/package.json
rm dist/node/package.json
rm dist/browser/README.md
rm dist/node/README.md