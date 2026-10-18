import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { grammarSelectionMappingFixture } from '../tests/support/grammar-selection-mapping-cases.js';
import { readProjectData, readProjectFileBytes, writeProjectData } from './project-data.mjs';
import { originalGrammarSelectionCapture } from './grammar-selection-capture-profile.mjs';

const root = new URL('../../', import.meta.url);
const fixtureFile = new URL('parity/fixtures/grammar-selection-json-string.lino', root);
const sourceFiles = [
  ['inventory', 'parity/language-grammar-inventory.lino'],
  ['originalConcreteOracle', 'parity/fixtures/default-cst-expected.lino'],
  ['nativeConcreteOracle', 'parity/fixtures/native-default-cst-expected.lino'],
  ['javascriptJsonGrammar', 'js/src/data/native-grammars/json.lino'],
  ['javascriptPythonGrammar', 'js/src/data/native-grammars/python.lino'],
  ['rustJsonGrammar', 'rust/src/data/native-grammars/json.lino'],
  ['rustPythonGrammar', 'rust/src/data/native-grammars/python.lino'],
  ['javascriptTests', 'js/tests/grammar-selection-json-string.test.js'],
  ['rustTests', 'rust/tests/unit/grammar_recursive_selection.rs'],
  ['rustTestTransport', 'rust/tests/unit/grammar_recursive_selection_mapping.rs'],
  ['fixtureAuthor', 'js/tests/support/grammar-selection-mapping-cases.js'],
  ['fixtureGenerator', 'js/scripts/generate-grammar-selection-mapping-fixtures.mjs'],
];

export function generateGrammarSelectionMappingFixtures({ check = false } = {}) {
  const bytes = new Map(sourceFiles.map(([, relative]) =>
    [relative, originalGrammarSelectionCapture(relative, readProjectFileBytes(new URL(relative, root)))]));
  for (const language of ['json', 'python']) assert.deepEqual(
    bytes.get(`js/src/data/native-grammars/${language}.lino`),
    bytes.get(`rust/src/data/native-grammars/${language}.lino`),
    'the two runtime controls must use the same actual shipped grammar bytes');
  const inventory = readProjectData(new URL(sourceFiles[0][1], root));
  const python = inventory.languages.find(language => language.name === 'Python');
  assert.ok(python, 'the actual original Python inventory entry is required');
  const originalOracle = readProjectData(new URL(sourceFiles[1][1], root));
  const nativeOracle = readProjectData(new URL(sourceFiles[2][1], root));
  const expected = nativeOracle.languages.Python ?? originalOracle.languages.Python;
  assert.ok(expected?.positive, 'the existing independent concrete rows are required');
  const fixture = grammarSelectionMappingFixture({
    captures: sourceFiles.map(([id, relative]) => ({ id, path: relative,
      bytes: bytes.get(relative).length,
      sha256: createHash('sha256').update(bytes.get(relative)).digest('hex') })),
    python: { source: python.source, recoverySource: python.recoverySource,
      positiveRows: expected.positive },
  });
  if (check) {
    assert.ok(isDeepStrictEqual(readProjectData(fixtureFile), fixture),
      'the authored mapped-selection source inputs and independent expectations are stale');
  } else writeProjectData(fixtureFile, fixture);
  return fixture;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  assert.ok(process.argv.slice(2).every(argument => argument === '--check'), 'only --check is admitted');
  generateGrammarSelectionMappingFixtures({ check: process.argv.includes('--check') });
  console.log('authored JSON string grammar selection fixture matches its exact source inputs');
}
