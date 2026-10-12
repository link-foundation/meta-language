import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { selfTranslate } from '../src/self-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/self-translation-bindings.json', import.meta.url), 'utf8')).cases.find(({ name }) => name === 'documented-optional-string');

test('optional sibling bindings keep their presence in the Rust ABI and every source observation', () => {
  const rust = selfTranslate(fixture.source, 'JavaScript', 'Rust');
  assert.ok(rust.items.every(({ status }) => status === 'translated'));
  assert.match(rust.code, /pub fn identity\(value: Option<String>\) -> Option<String>/u);
  assert.equal(selfTranslate(rust.code, 'Rust', 'JavaScript').code, fixture.source);
  const javascript = emitJavaScript(checkProgram(parseJavaScript(fixture.source))).text;
  for (const observation of [fixture, ...fixture.additionalCalls]) {
    const output = (source) => {
      const lines = [];
      runInNewContext(`${source}\nconsole.log(${observation.call});`, { console: { log: (value) => lines.push(String(value)) } });
      return lines.join('\n') + '\n';
    };
    assert.equal(output(fixture.source.replace('export function', 'function')), observation.expected, observation.name);
    assert.equal(output(javascript), observation.expected, observation.name);
  }
  assert.match(fixture.rustCall, /identity\(Some\(/u);
  assert.ok(fixture.additionalCalls.filter(({ name }) => ['omitted', 'explicit-undefined'].includes(name)).every(({ rustCall }) => rustCall.startsWith('identity(None)')));
});
