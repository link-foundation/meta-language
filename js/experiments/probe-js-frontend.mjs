// Parses, checks and emits a JavaScript program, printing the first stage that rejects it.
import { readFileSync } from 'node:fs';

import { checkProgram } from '../src/translation/check.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const source = readFileSync(process.argv[2], 'utf8');
try {
  const checked = checkProgram(parseJavaScript(source));
  for (const [name, emit] of [['Rust', emitRust], ['Lean', emitLean], ['Rocq', emitRocq]]) {
    try {
      emit(checked);
      console.log(name, 'emits');
    } catch (error) {
      console.log(name, 'rejects:', error.message);
    }
  }
} catch (error) {
  console.log('frontend rejects:', error.message);
}
