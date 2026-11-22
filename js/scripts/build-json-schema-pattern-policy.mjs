import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { selfTranslate } from '../src/self-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';

const source = readFileSync(new URL('../src/grammar-importers/json-schema-pattern-policy.js', import.meta.url), 'utf8');
const translation = selfTranslate(source, 'JavaScript', 'Rust');
assert.ok(translation.items.every(item => ['comment', 'translated'].includes(item.status)));
assert.equal(selfTranslate(translation.code, 'Rust', 'JavaScript').code, source);
const definitions = emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
const decorators = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/json-schema-pattern-decorators.lino', import.meta.url), 'utf8'));
const { source: policy } = decorators.decorate('emitter', { format: 'Rust', source: definitions });
const output = new URL('../../rust/src/grammar/json_document/pattern_policy.rs', import.meta.url);
const generated = `// Generated from js/src/grammar-importers/json-schema-pattern-policy.js.\n${policy}\n`;
if (process.argv.includes('--check')) assert.equal(readFileSync(output, 'utf8'), generated);
else writeFileSync(output, generated);
