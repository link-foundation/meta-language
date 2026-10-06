#!/usr/bin/env bash
# Shows why tree-sitter-swift 0.7.4 needs a Wasm-only scanner patch: its
# scanner uses fprintf/stderr/exit, which `tree-sitter build --wasm` (CLI
# 0.27.0) rejects; js/scripts/grammar-patches/tree-sitter-swift-wasm.patch
# makes the Wasm build trap instead. Needs the crate in ~/.cargo/registry
# (cargo fetch) and the tree-sitter CLI.
set -u
src=$(ls -d ~/.cargo/registry/src/*/tree-sitter-swift-0.7.4 2>/dev/null | head -1)
[ -n "$src" ] || { echo "0.7.4: not fetched"; exit 1; }
patch=$(cd "$(dirname "$0")/.." && pwd)/js/scripts/grammar-patches/tree-sitter-swift-wasm.patch
for variant in upstream patched; do
  work=$(mktemp -d); cp -r "$src"/. "$work"
  [ "$variant" = patched ] && (cd "$work" && git apply "$patch")
  echo "== $variant"; grep -n 'fprintf\|exit(\|__builtin_trap' "$work/src/scanner.c"
  (cd "$work" && tree-sitter build --wasm -o out.wasm . 2>&1 | grep -v '^    ' | head -3; ls out.wasm 2>/dev/null)
  rm -rf "$work"
done
