#!/usr/bin/env node
// Runtime source is target data emitted by JavaScript, not a second host
// implementation. Keep its exact bytes so both adapters emit identical code.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { readRustArrayPrelude } from '../src/translation/emit-rust.js';

const directory = new URL('../../rust/src/translation/runtime/', import.meta.url);
const outputs = [
  [new URL('array-legacy.txt', directory), readRustArrayPrelude(false)],
  [new URL('array-standard.txt', directory), readRustArrayPrelude(true)],
];
if (process.argv.includes('--check')) {
  for (const [path, source] of outputs) {
    if (readFileSync(path, 'utf8') !== source) throw new Error('Rust array runtime data is stale; run node js/scripts/generate-rust-array-runtime.mjs');
  }
  console.log('generated Rust array runtime data matches JavaScript');
} else {
  mkdirSync(directory, { recursive: true });
  for (const [path, source] of outputs) writeFileSync(path, source);
}
