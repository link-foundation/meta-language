// Split the existing array annotation walker without changing its traversal.
// Run from the repository root; repeated runs leave the extracted module alone.
import assert from 'node:assert/strict';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';

const file = 'rust/src/translation/javascript/infer.rs';
const directory = 'rust/src/translation/javascript/infer';
const source = readFileSync(file, 'utf8');
const start = source.indexOf('/// Writes the inferred element types of the array literals');
if (start !== -1) {
  const end = source.indexOf('/// Writes the inferred types of the fields', start);
  assert.ok(end > start);
  const walker = source.slice(start, end).replace('fn fill_arrays(', 'pub(super) fn fill_arrays(');
  mkdirSync(directory, { recursive: true });
  writeFileSync(`${directory}/arrays.rs`, `//! Writes inferred array element types back into the parsed program.\n\nuse super::{HashMap, SEffect, SExpr, SItem, SNode, SProgram, SProp, SPropNode, Type};\n\n${walker.trimEnd()}\n`);
  const shortened = source.slice(0, start) + source.slice(end);
  writeFileSync(file, shortened.replace('use std::collections::{HashMap, HashSet};', 'mod arrays;\n\nuse arrays::fill_arrays;\nuse std::collections::{HashMap, HashSet};'));
  assert.equal(readFileSync(`${directory}/arrays.rs`, 'utf8').split('\n\n').slice(2).join('\n\n').replace('pub(super) fn fill_arrays(', 'fn fill_arrays(').trimEnd(), source.slice(start, end).trimEnd());
}
