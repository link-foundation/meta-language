#!/usr/bin/env node
// Selected source-free products exercise the existing source Unit ABI;
// admitting new JavaScript source forms is outside this fixture's scope.
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';

export function productUnitFixture() {
  return { schemaVersion: 1, cases: ['JavaScript', 'Rust'].map(language => {
    const surface = { language, items: [{ k: 'fn', name: 'make', params: [],
      ret: { kind: 'tuple', elements: [{ kind: 'unit' }, { kind: 'bool' }] },
      body: { k: 'tuple', frozen: true, productModel: 'immutable value product', items: [{ k: 'unit' }, { k: 'bool', value: true }] },
    }], main: null };
    return { language, surface, javascript: emitJavaScript(checkProgram(surface)).text,
      unitExpression: language === 'JavaScript' ? 'undefined' : 'null',
      unitType: language === 'JavaScript' ? 'undefined' : 'object' };
  }) };
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const file = new URL('../../parity/fixtures/translation-product-unit.lino', import.meta.url);
  const fixture = productUnitFixture();
  if (process.argv.includes('--check')) checkProjectData(file, fixture);
  else writeProjectData(file, fixture);
}
