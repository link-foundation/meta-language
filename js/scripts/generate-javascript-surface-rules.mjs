// Translate the complete JavaScript source-decision module without a native implementation copy.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { readPortableSourceParameterStyles, readPortableSourceLintStyles } from './portable-source-library-style.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';

const input = new URL('../src/translation/javascript-surface-rules.js', import.meta.url);
const output = new URL('../../rust/src/translation/javascript/surface_rules.rs', import.meta.url);
const translated = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8'), { referenceSemantics: false })), { arrayConsumption: true });
if (translated.preludes.length !== 0) throw new Error('source decisions must remain closed scalar operations');
const header = translated.text.split('\n').find(line => line.startsWith('#![allow('));
if (!header) throw new Error('the checked emitter omitted its generated module header');
const definitions = translated.definitions.join('\n\n').replace(/^pub fn /gmu, '#[must_use]\npub fn ');
const raw = header + '\n\n' + definitions;
const styleFile = new URL('../../parity/self-translation/javascript-surface-rules-decorators.lino', import.meta.url);
const args = process.argv.slice(2);
const groups = [
  {id: 'javascript-terminal-tag-guard-library-style', order: 100,
    names: ['accept_java_script_terminal_tag_guard', 'accept_java_script_tag_guard_sequence'], flag: '--update-terminal-tag-guard-styles'},
  {id: 'javascript-equality-inference-library-style', order: 110,
    names: ['read_java_script_equality_inference_step'], flag: '--update-equality-inference-styles'},
];
const interfaceId = 'javascript-terminal-tag-guard-interface-style';
const interfaceFlag = '--update-terminal-tag-guard-interface-styles';
assert.ok(args.length === 0 || args.length === 1 && ['--check', interfaceFlag, ...groups.map(group => group.flag)].includes(args[0]),
  'use no arguments, --check, --update-terminal-tag-guard-styles, --update-equality-inference-styles, or --update-terminal-tag-guard-interface-styles');
const document = readFileSync(styleFile, 'utf8');
const lines = document.match(/[^\n]*\n|[^\n]+$/gu) ?? [];
assert.equal(Buffer.byteLength(lines[0]), 106060, 'the entire historical JavaScript surface style row retains its bytes');
assert.equal(createHash('sha256').update(lines[0]).digest('hex'), 'ac9cf3ef2a0a4a4b16e59b9c2d33a2619fd8cbe44bf4042913f52419cc275e6a');
assert.equal(Buffer.byteLength(lines[1]), 1287, 'the reviewed terminal guard pair remains byte exact');
assert.equal(createHash('sha256').update(lines[1]).digest('hex'), '61cbbe05766356d057df68a3d53fcf20bfb257f42b9ea3a45430d9d0a0112650');
const baseDocument = lines.filter(line => !line.startsWith('(decorator ' + interfaceId + ' ')
  && !groups.some(group => line.startsWith('(decorator ' + group.id + ' '))).join('');
const base = DecoratorSet.fromLino(baseDocument).decorate('emitter', { format: 'Rust', source: raw }).source;
let reviewed = base;
let expectedDocument = baseDocument;
for (const group of groups) {
  const actions = [];
  for (const name of group.names) {
    const from = '#[must_use]\n' + readSourceTextRustBlock(base, 'fn', name);
    assert.equal(base.split(from).length, 2, 'the source-owned function has one complete attribute and declaration context');
    const loans = readPortableSourceParameterStyles(from);
    const to = readPortableSourceLintStyles(loans.source).source;
    if (from !== to) {
      actions.push({op: 'replace', field: 'source', from, to});
      reviewed = reviewed.replace(from, to);
    }
  }
  const additional = new DecoratorSet([{id: group.id, level: 'emitter', order: group.order,
    when: [['format', 'Rust']], actions}]).toLino();
  const actual = lines.filter(line => line.startsWith('(decorator ' + group.id + ' '));
  assert.ok(actual.length <= 1, 'each scoped source style has one exact row');
  if (actual[0] !== additional) assert.equal(args[0], group.flag, 'scoped JavaScript source styles are stale; run ' + group.flag);
  expectedDocument += (expectedDocument.endsWith('\n') ? '' : '\n') + additional;
  assert.equal(DecoratorSet.fromLino(additional).decorate('emitter', {format: 'Rust', source: reviewed}).source,
    reviewed, 'the additional source styles are idempotent');
}
// Preserve the five separately named source facts instead of changing their
// public ABI to satisfy a design heuristic. The pinned complete historical
// function owns this expectation; unrelated or changed functions cannot gain it.
const interfaceFrom = '#[must_use]\npub const ' + readSourceTextRustBlock(reviewed, 'fn', 'accept_java_script_terminal_tag_guard');
const originalInterface = DecoratorSet.fromLino(lines[1]).decorators[0].actions[0];
assert.equal(originalInterface.op, 'replace');
assert.equal(originalInterface.field, 'source');
assert.equal(interfaceFrom, originalInterface.to, 'the exact historical five-Boolean ABI and complete body remain unchanged');
const interfaceAttribute = '#[expect(clippy::fn_params_excessive_bools, reason = "the checked source interface retains five independent ordered Boolean facts")]\n';
const interfaceTo = interfaceFrom.replace('#[must_use]\n', '#[must_use]\n' + interfaceAttribute);
assert.equal(interfaceTo.replace(interfaceAttribute, ''), interfaceFrom, 'only the one function-specific expectation is added');
const interfaceAdditional = new DecoratorSet([{id: interfaceId, level: 'emitter', order: 120,
  when: [['format', 'Rust']], actions: [{op: 'replace', field: 'source', from: interfaceFrom, to: interfaceTo}]}]).toLino();
const actualInterface = lines.filter(line => line.startsWith('(decorator ' + interfaceId + ' '));
assert.ok(actualInterface.length <= 1, 'the exact Boolean interface has one recorded style');
if (actualInterface[0] !== interfaceAdditional) assert.equal(args[0], interfaceFlag, 'scoped Boolean interface styles are stale; run ' + interfaceFlag);
expectedDocument += (expectedDocument.endsWith('\n') ? '' : '\n') + interfaceAdditional;
assert.equal(reviewed.split(interfaceFrom).length, 2, 'the complete source function has one style owner');
reviewed = reviewed.replace(interfaceFrom, interfaceTo);
assert.equal(DecoratorSet.fromLino(interfaceAdditional).decorate('emitter', {format: 'Rust', source: reviewed}).source,
  reviewed, 'the exact interface expectation is idempotent');
if (args[0] === interfaceFlag || groups.some(group => group.flag === args[0])) writeFileSync(styleFile, expectedDocument);
else assert.equal(document, expectedDocument, 'all recorded JavaScript source style rows retain their exact order and bytes');
const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
assert.equal(decorated, reviewed, 'recorded source styles reproduce only the actual scoped declarations');
const text = '// Generated from js/src/translation/javascript-surface-rules.js by\n'
  + '// js/scripts/generate-javascript-surface-rules.mjs. Do not edit by hand.\n'
  + formatProductRuleSource(decorated);
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== text) throw new Error('native JavaScript source decisions are stale');
} else writeFileSync(output, text);
