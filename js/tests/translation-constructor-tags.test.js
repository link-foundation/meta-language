import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';

const tags = JSON.parse(readFileSync(new URL('../../parity/fixtures/translation-constructor-tags.json', import.meta.url), 'utf8')).tags;
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
