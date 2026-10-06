// Translates a JavaScript program to a target and prints the target text:
//   node experiments/issue-195-float/translate.mjs <target> <file|->
// JavaScript → JavaScript is not a public translation pair, so that target
// runs the portable core (frontend, checker, emitter) directly.
import { readFileSync } from 'node:fs';
import { translateProgram } from '../../js/src/program-translation.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { emitJavaScript } from '../../js/src/translation/emit-javascript.js';
import { parseJavaScript } from '../../js/src/translation/javascript.js';

const [target = 'rust', file = '-'] = process.argv.slice(2);
const source = readFileSync(file === '-' ? 0 : file, 'utf8');
if (target === 'javascript') {
  process.stdout.write(emitJavaScript(checkProgram(parseJavaScript(source))).text);
} else {
  const result = translateProgram(source, 'javascript', target);
  console.error(result.contract?.support, result.diagnostic?.message ?? '');
  process.stdout.write(result.code);
}
