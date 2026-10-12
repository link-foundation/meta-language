import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { readProjectData } from '../scripts/project-data.mjs';
import { productUnitFixture } from '../scripts/generate-product-unit-fixture.mjs';

const fixture = readProjectData(new URL('../../parity/fixtures/translation-product-unit.lino', import.meta.url));

test('source-free immutable Unit fields retain the established source representation', () => {
  assert.deepEqual(productUnitFixture(), fixture);
  for (const entry of fixture.cases) {
    const emitted = emitJavaScript(checkProgram(entry.surface)).text;
    assert.equal(emitted, entry.javascript);
    const value = runInNewContext(`${emitted}\nmake()`);
    assert.equal(value[0], entry.language === 'JavaScript' ? undefined : null, entry.language);
    assert.equal(typeof value[0], entry.unitType, entry.language);
    assert.equal(value[1], true);
    assert.equal(Object.isFrozen(value), true);
  }
});
