import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';

const sources = [
  ['slice-number-length', 'fn bounded(values:&[f64])->bool { values.len() as f64 < 3f64 }', 'bounded', [[], [1], [1, 2], [1, 2, 3]], [true, true, true, false]],
  ['vector-number-length', 'fn bounded(values:Vec<f64>)->bool { values.len() as f64 >= 2f64 }', 'bounded', [[], [1], [1, 2]], [false, false, true]],
  ['slice-emptiness', 'fn empty(values:&[bool])->bool { values.is_empty() }', 'empty', [[], [false], [true, false]], [true, false, false]],
  ['qualified-vector-owner', 'fn empty(values:&std::vec::Vec<u32>)->bool { values.is_empty() }', 'empty', [[], [1, 2]], [true, false]],
  ['module-vector-alias', 'use std::vec::Vec as Values; fn empty(values:&Values<String>)->bool { values.is_empty() }', 'empty', [[], ['空', 'text']], [true, false]],
  ['scoped-slice-lifetime', "fn empty<'a>(values:&'a [f64])->bool { values.is_empty() }", 'empty', [[], [1]], [true, false]],
];
const refused = [
  ['shadowed-vector', 'enum Vec{Value} fn empty(values:Vec<f64>)->bool{values.is_empty()}'],
  ['shadowed-standard-module', 'mod std{} fn empty(values:std::vec::Vec<f64>)->bool{values.is_empty()}'],
  ['shadowed-element', 'enum String{Text} fn empty(values:Vec<String>)->bool{values.is_empty()}'],
  ['nested-vectors', 'fn empty(values:&[Vec<f64>])->bool{values.is_empty()}'],
  ['pointer-elements', 'fn empty(values:Vec<&u32>)->bool{values.is_empty()}'],
  ['mutable-slice', 'fn empty(values:&mut [f64])->bool{true}'],
  ['array-return', 'fn values()->Vec<f64>{vec![1f64]}'],
  ['array-constant', 'const VALUES:Vec<f64> = vec![1f64];'],
  ['slice-return', "fn values<'a>(values:&'a [f64])->&'a [f64]{values}"],
  ['unconverted-source-length', 'fn count(values:&[f64])->usize{values.len()}'],
  ['other-length-conversion', 'fn count(values:&[f64])->u64{values.len() as u64}'],
  ['source-indexing', 'fn first(values:&[f64])->f64{values[0]}'],
  ['slice-to-vector-reference', 'fn want(values:&Vec<f64>)->bool{values.is_empty()} fn wrong(values:&[f64])->bool{want(values)}'],
  ['by-value-vector-call', 'fn want(values:Vec<f64>)->bool{values.is_empty()} fn wrong(values:Vec<f64>)->bool{want(values)}'],
  ['owned-local-vector-move', 'fn empty(values:Vec<f64>)->bool{let other=values;other.is_empty()}'],
  ['conditional-vector-move', 'fn empty(values:Vec<f64>,flag:bool)->bool{let other=if flag{values}else{values};other.is_empty()}'],
  ['conditional-vector-with-literal', 'fn empty(values:Vec<f64>,flag:bool)->bool{let other=if flag{values}else{vec![1f64]};other.is_empty()}'],
  ['last-slice-dereference', 'fn empty(values:&[f64])->bool{let other=*values;other.is_empty()}'],
  ['array-clone', 'fn empty(values:&[f64])->bool{let other=values.clone();other.is_empty()}'],
  ['array-is-not-a-string', 'fn empty(values:&[f64])->bool{let text=values.as_str();true}'],
  ['borrowed-string-elements', 'fn main(){let values=vec!["borrowed"]; }'],
];
export function rustArraySourceFixture() {
  return {
    schemaVersion: 1, id: 'rust-array-source', sourceModel: 'immutable Rust value projection',
    premises: ['standard vector owners are resolved and unshadowed', 'elements are direct immutable scalar or owned string values', 'length observations explicitly select the binary64 Number model with as f64', 'source vector moves, array results and mutable borrowing require separate proofs'],
    cases: sources.map(([id, body, name, inputs, expected]) => {
      const source = body + ' fn main(){}';
      const surface = parseRust(source), program = checkProgram(surface);
      const definitions = emitRust(program).definitions.join('\n\n');
      const replay = checkProgram(parseRust(definitions + '\nfn main(){}'));
      return { id, source, name, inputs, expected, surface: JSON.parse(JSON.stringify(surface)), javascript: emitJavaScript(program).text, rustDefinitions: definitions, replayJavaScript: emitJavaScript(replay).text };
    }),
    invalid: refused.map(([id, body]) => {
      const source = body + (body.includes('fn main()') ? '' : ' fn main(){}');
      try { checkProgram(parseRust(source)); } catch (error) { return { id, source, kind: error.kind, reason: error.reason, span: error.span ?? null }; }
      throw new Error(`source array admission unexpectedly accepted ${id}`);
    }),
  };
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const path = new URL('../../parity/fixtures/rust-array-source.lino', import.meta.url);
  const value = rustArraySourceFixture();
  if (process.argv.includes('--check')) {
    if (!checkProjectData(path, value)) throw new Error('Rust array source fixtures are stale');
  } else writeProjectData(path, value);
}
