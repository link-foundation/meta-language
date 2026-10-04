#!/usr/bin/env bash
# Reproduces why tree-sitter-swift is held at 0.7.3: the 0.7.4 scanner uses
# fprintf/stderr/exit, which `tree-sitter build --wasm` (CLI 0.27.0) rejects.
# Needs the crates in ~/.cargo/registry (cargo fetch) and the tree-sitter CLI.
set -u
for version in 0.7.3 0.7.4; do
  src=$(ls -d ~/.cargo/registry/src/*/tree-sitter-swift-$version 2>/dev/null | head -1)
  [ -n "$src" ] || { echo "$version: not fetched"; continue; }
  work=$(mktemp -d); cp -r "$src"/. "$work"
  echo "== $version"; grep -n 'fprintf\|exit(' "$work/src/scanner.c"
  (cd "$work" && tree-sitter build --wasm -o out.wasm . 2>&1 | grep -v '^    ' | head -3)
  rm -rf "$work"
done
