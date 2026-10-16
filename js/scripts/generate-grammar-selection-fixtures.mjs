import { fileURLToPath } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { grammarSelectionFixture } from '../tests/support/grammar-selection-cases.js';
import { readProjectData, writeProjectData } from './project-data.mjs';

export function generateGrammarSelectionFixtures({ check = false } = {}) {
  const file = new URL('../../parity/fixtures/grammar-recursive-selection.lino', import.meta.url);
  const fixture = grammarSelectionFixture();
  if (check) {
    if (!isDeepStrictEqual(readProjectData(file), fixture)) throw new Error('authored recursive grammar selection fixtures are stale');
  } else writeProjectData(file, fixture);
  return fixture;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  generateGrammarSelectionFixtures({ check: process.argv.includes('--check') });
  console.log('authored recursive grammar selection fixture matches its source definitions');
}
