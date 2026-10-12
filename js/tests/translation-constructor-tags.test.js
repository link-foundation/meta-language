import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';

import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { translateProgram } from '../src/program-translation.js';
import { requireDataObjectFreeze } from '../src/translation/primitive-value-rules.js';

const fixture = readProjectData(new URL('../../parity/fixtures/translation-constructor-tags.json', import.meta.url));
const { tags } = fixture;
const source = "/** @typedef {{ $: 'one' } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return { $: 'one' }; }\n/** @param {Tag} value @returns {number} */ function read(value) { if (value.$ === 'one') return 1; return 2; }";

test('constructor tag data is safely escaped and keeps its value through construction and dispatch', () => {
  for (const tag of tags) {
    const program = checkProgram(parseJavaScript(source));
    const entry = program.declarations.get('Tag');
    entry.ctors[0].name = tag;
    const renameReference = value => {
      if (!value || typeof value !== 'object') return;
      if (value.ctor === 'one') value.ctor = tag;
      for (const child of Object.values(value)) {
        if (Array.isArray(child)) child.forEach(renameReference);
        else renameReference(child);
      }
    };
    for (const declaration of program.declarations.values()) renameReference(declaration);
    const emitted = emitJavaScript(program).text;
    assert.ok(emitted.includes(`$: ${JSON.stringify(tag)}`), tag);
    assert.ok(emitted.includes(`case ${JSON.stringify(tag)}:`), tag);
    const context = { injected: false };
    const actual = runInNewContext(`${emitted}\n[make().$, read(make()), read({ $: 'two' })]`, context);
    assert.equal(actual[0], tag);
    assert.equal(actual[1], 1);
    assert.equal(actual[2], 2);
    assert.equal(context.injected, false, tag);
  }
});

test('constructor fields keep arbitrary names as own data properties during construction and dispatch', () => {
  const source = "/** @typedef {{ $: 'one', payload: number } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return { $: 'one', payload: 7 }; }\n/** @param {Tag} value @returns {number} */ function read(value) { if (value.$ === 'one') return value.payload; return 2; }";
  for (const field of fixture.fields) {
    const program = checkProgram(parseJavaScript(source));
    program.declarations.get('Tag').ctors[0].fields[0].name = field;
    const emitted = emitJavaScript(program).text;
    assert.ok(emitted.includes(`[${JSON.stringify(field)}]: 7`), field);
    const context = { field };
    const actual = runInNewContext(`${emitted}\n(() => { const item = make(); const supplied = { $: 'one', [field]: 9 }; return [Object.hasOwn(item, field), item[field], read(item), read(supplied), Object.getPrototypeOf(item) === Object.prototype]; })()`, context);
    assert.deepEqual(Array.from(actual), [true, 7, 7, 9, true], field);
  }
});

test('constructor descriptors retain their declared source model and explicit immutability', () => {
  for (const { language, frozen: explicitFrozen, descriptors } of fixture.models) {
    const program = checkProgram(parseJavaScript(source));
    program.sourceLanguage = language;
    if (explicitFrozen) program.declarations.get('make').body.frozen = true;
    const frozen = requireDataObjectFreeze(language, explicitFrozen);
    assert.equal(frozen, explicitFrozen || language !== 'JavaScript');
    const emitted = emitJavaScript(program).text;
    const actual = runInNewContext(`${emitted}\n(() => { const item = make(); const descriptor = Object.getOwnPropertyDescriptor(item, '$'); return [Object.isFrozen(item), Object.isExtensible(item), descriptor.writable, descriptor.configurable, Reflect.set(item, '$', 'two')]; })()`);
    assert.deepEqual(Array.from(actual), descriptors, `${language}:${explicitFrozen}`);
  }
  const frozenSource = "/** @typedef {{ $: 'one' } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return Object.freeze({ $: 'one' }); }";
  assert.equal(checkProgram(parseJavaScript(frozenSource)).declarations.get('make').body.frozen, true);
  for (const target of ['Rust', 'Lean', 'Rocq']) assert.match(translateProgram(frozenSource, 'JavaScript', target).diagnostic?.message ?? '', /^explicit data object freeze:/u, target);
});
