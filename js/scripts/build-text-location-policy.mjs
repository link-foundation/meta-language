import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { selfTranslate } from '../src/self-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';

const source = readFileSync(new URL('../src/text-location-policy.js', import.meta.url), 'utf8');
const output = new URL('../../rust/src/link_network/text_location/policy.rs', import.meta.url);
const result = selfTranslate(source, 'JavaScript', 'Rust');
if (result.items.some(({ status }) => status !== 'translated')
  || selfTranslate(result.code, 'Rust', 'JavaScript').code !== source) {
  throw new Error('The text location policy did not translate losslessly');
}
const definitions = emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
const decorators = DecoratorSet.fromLino(readFileSync(new URL(
  '../../parity/self-translation/text-location-decorators.lino', import.meta.url,
), 'utf8'));
const { source: styled } = decorators.decorate('emitter', { format: 'Rust', source: definitions });
const generated = '// Generated from the JavaScript text location policy.\n' + styled + '\n';
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== generated) throw new Error('The Rust text location policy is stale');
} else {
  writeFileSync(output, generated);
}
