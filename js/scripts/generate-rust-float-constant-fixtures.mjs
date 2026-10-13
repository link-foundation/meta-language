// IEEE infinity bits and NaN classification are independent of the translator.
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { createTypedProgramLinks } from '../src/typed-program-links.js';

const constants = [
  ['not-a-number', 'NAN', 'NaN', 'NaN', true],
  ['positive-infinity', 'INFINITY', 'Infinity', '7ff0000000000000', false],
  ['negative-infinity', 'NEG_INFINITY', '-Infinity', 'fff0000000000000', false],
];
const refused = [
  ...['NAN', 'INFINITY', 'NEG_INFINITY'].map(name => [`f32-${name}`, `fn apply()->f64{f32::${name}}`]),
  ['custom-owner', 'enum Value{NAN} fn apply()->f64{Value::NAN}'],
  ['enum-shadow', 'enum f64{NAN} fn apply()->f64{f64::NAN}'],
  ['module-shadow', 'mod f64{} fn apply()->f64{f64::INFINITY}'],
  ['import-shadow', 'enum Value{NAN} use crate::Value as f64; fn apply()->f64{f64::NAN}'],
  ['qualified-import', 'use std::f64 as f64; fn apply()->f64{f64::NEG_INFINITY}'],
  ['qualified-owner', 'fn apply()->f64{std::f64::INFINITY}'],
  ['separate-finite-constant', 'fn apply()->f64{f64::MAX}'],
  ['associated-call', 'fn apply()->f64{f64::NAN()}'],
  ['shadowed-suffix-chain', 'mod f64{} fn apply()->bool{f64::INFINITY.is_finite()}'],
];
export function rustFloatConstantFixture() {
  return {schemaVersion: 1, id: 'rust-float-constants',
    premises: ['exact f64 associated owner spelling resolves to the unshadowed primitive owner', 'constants retain actual binary64 source evidence for method chains', 'infinity signs are observed by exact IEEE bits; NaN only by class, without a payload or sign promise'],
    cases: constants.map(([id, name, javascriptValue, expectedBits, expectedNaN]) => {
      const spelling = name === 'INFINITY' ? 'f64 :: INFINITY' : `f64::${name}`;
      const source = `// π 😀\nfn apply()->f64{${spelling}} fn classify()->bool{f64::${name}.is_nan()} fn absolute()->f64{f64::${name}.abs()} fn main(){}`;
      const originalJavaScript = `function apply(){return ${javascriptValue};}function classify(){return Number.isNaN(${javascriptValue});}function absolute(){return Math.abs(${javascriptValue});}`;
      const surface = parseRust(source), program = checkProgram(surface);
      const original = checkProgram(parseJavaScript(originalJavaScript));
      const rustDefinitions = emitRust(program).definitions.join('\n\n');
      const reverseDefinitions = emitRust(original).definitions.join('\n\n');
      const replay = checkProgram(parseRust(rustDefinitions + '\nfn main(){}'));
      const reverseReplay = checkProgram(parseRust(reverseDefinitions + '\nfn main(){}'));
      const archive = createTypedProgramLinks(source, 'Rust');
      return {id, name, spelling, source, originalJavaScript, expectedBits, expectedNaN,
        expectedAbsoluteBits: name === 'NAN' ? 'NaN' : '7ff0000000000000',
        surface: JSON.parse(JSON.stringify(surface)), javascript: emitJavaScript(program).text,
        javascriptDefinitions: emitJavaScript(program).definitions.join('\n\n'), rust: emitRust(program).text,
        rustDefinitions, reverseDefinitions, replayJavaScript: emitJavaScript(replay).text,
        reverseReplayJavaScript: emitJavaScript(reverseReplay).text,
        archive: archive.network.toLino(), root: archive.root.asU64()};
    }),
    invalid: refused.map(([id, body]) => {
      const source = body + ' fn main(){}';
      try { checkProgram(parseRust(source)); } catch (error) {
        return {id, source, kind: error.kind, reason: error.reason, span: error.span ?? null};
      }
      throw new Error('unexpected primitive constant admission: ' + id);
    }),
    edit: {case: 'positive-infinity', from: 'Infinity', to: '-Infinity', expectedBits: 'fff0000000000000'},
  };
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const path = new URL('../../parity/fixtures/rust-float-constants.lino', import.meta.url);
  const value = rustFloatConstantFixture();
  if (process.argv.includes('--check')) {
    if (!checkProjectData(path, value)) throw new Error('Rust float constant fixtures are stale');
  } else writeProjectData(path, value);
  console.log(`Rust float constants: ${value.cases.length} checked pairs; ${value.invalid.length} exact refusals`);
}
