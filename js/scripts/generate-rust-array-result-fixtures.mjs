import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';

export const arrayResultModel = 'immutable fresh scalar vector results 1';
export const arrayResultPremises = [
  'standard vector owners are resolved and unshadowed',
  'result elements are direct scalar Copy values under the checked immutable value model',
  'every returning tail allocates a fresh vector; conditionals and scalar let bindings preserve this evidence',
  'results may be observed through immutable reads; local vector moves, by-value vector calls, mutation, borrowed results and named result forwarding remain unproved',
  'length observations explicitly select the binary64 Number model with as f64',
];
const sources = [
  ['fresh-token-fields', 'fn token(code:f64)->Vec<f64>{vec![1f64,code,0f64]}', 'token', [[0], [65], [9731]], [[1, 0, 0], [1, 65, 0], [1, 9731, 0]], 'fn main(){let value=token(65f64);assert_eq!(value.len() as f64,3f64);assert!(!value.is_empty());}'],
  ['fresh-empty-number-vector', 'fn values()->Vec<f64>{Vec::<f64>::new()}', 'values', [[]], [[]]],
  ['fresh-boolean-vector', 'fn flags(flag:bool)->Vec<bool>{vec![flag,!flag]}', 'flags', [[true], [false]], [[true, false], [false, true]]],
  ['fresh-fixed-scalar-vector', 'fn values(value:u32)->Vec<u32>{vec![value,1u32]}', 'values', [[0], [42], [4294967295]], [[0, 1], [42, 1], [4294967295, 1]], 'fn main(){let value=values(42u32);assert!(!value.is_empty());}'],
  ['fresh-unit-vector', 'fn values()->Vec<()>{vec![(),()]}', 'values', [[]], [[null, null]]],
  ['fresh-conditional-with-scalar-binding', 'fn token(code:f64)->Vec<f64>{if code==0f64{Vec::<f64>::new()}else{let next=code+1f64;vec![next]}}', 'token', [[0], [65], [-2]], [[], [66], [-1]]],
];
const refused = [
  ['borrowed-result', "fn values<'a>(values:&'a [f64])->&'a [f64]{values}"],
  ['owned-parameter-result', 'fn values(values:Vec<f64>)->Vec<f64>{values}'],
  ['owned-local-result', 'fn values()->Vec<f64>{let values=vec![1f64];values}'],
  ['named-result-forwarding', 'fn first()->Vec<f64>{vec![1f64]} fn values()->Vec<f64>{first()}'],
  ['conditional-parameter-result', 'fn values(values:Vec<f64>,flag:bool)->Vec<f64>{if flag{vec![1f64]}else{values}}'],
  ['owned-string-result', 'fn values()->Vec<String>{vec![String::from("value")]}'],
  ['array-constant', 'const VALUES:Vec<f64> = vec![1f64];'],
  ['owned-local-move', 'fn values()->Vec<f64>{vec![1f64]} fn main(){let first=values();let second=first;}'],
  ['result-clone', 'fn values()->Vec<f64>{vec![1f64]} fn main(){let first=values();let second=first.clone();}'],
  ['mutable-result-binding', 'fn values()->Vec<f64>{vec![1f64]} fn main(){let mut first=values();}'],
  ['by-value-result-call', 'fn values()->Vec<f64>{vec![1f64]} fn want(values:Vec<f64>)->bool{values.is_empty()} fn main(){assert!(want(values()));}'],
  ['bare-result-function-value', 'fn values()->Vec<f64>{vec![1f64]} fn main(){let first=values;}'],
];
export function rustArrayResultFixture() {
  const options = { arrayResultModel };
  return {
    schemaVersion: 1, id: 'rust-array-results', arrayResultModel, premises: arrayResultPremises,
    legacyFixture: 'rust-array-source', legacyRefusal: 'array-return',
    bidirectionalCases: ['fresh-token-fields', 'fresh-empty-number-vector', 'fresh-boolean-vector', 'fresh-conditional-with-scalar-binding'],
    reverseBoundaries: ['fixed integer JavaScript output uses runtime width guards outside the current reimport model', 'JavaScript null cannot recover the source unit element type without explicit source-free type provenance'],
    cases: sources.map(([id, body, name, inputs, expected, main = 'fn main(){}']) => {
      const source = body + ' ' + main;
      const surface = parseRust(source, options), program = checkProgram(surface);
      const rustDefinitions = emitRust(program).definitions.join('\n\n');
      const replay = checkProgram(parseRust(rustDefinitions + '\n' + main, options));
      const literal = (value, type) => type.kind === 'unit' ? '()' : type.kind === 'float' ? `${value}f64` : type.kind === 'fixed' ? `${value}u32` : String(value);
      const nativeCalls = inputs.map((input, index) => `assert_eq!(${name}(${input.map((value, i) => literal(value, surface.items[0].params[i].type)).join(',')}),vec![${expected[index].map(value => literal(value, surface.items[0].ret.element)).join(',')}]);`).join('');
      return { id, source, main, name, inputs, expected, nativeCalls, surface: JSON.parse(JSON.stringify(surface)), javascript: emitJavaScript(program).text, javascriptDefinitions: emitJavaScript(program).definitions.join('\n\n'), rustDefinitions, replayJavaScript: emitJavaScript(replay).text };
    }),
    invalid: refused.map(([id, body]) => {
      const source = body + (body.includes('fn main()') ? '' : ' fn main(){}');
      try { checkProgram(parseRust(source, options)); } catch (error) { return { id, source, kind: error.kind, reason: error.reason, span: error.span ?? null }; }
      throw new Error(`fresh array result admission unexpectedly accepted ${id}`);
    }),
  };
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const path = new URL('../../parity/fixtures/rust-array-results.lino', import.meta.url);
  const value = rustArrayResultFixture();
  if (process.argv.includes('--check')) {
    if (!checkProjectData(path, value)) throw new Error('Rust fresh array result fixtures are stale');
  } else writeProjectData(path, value);
}
