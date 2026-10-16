// Close one recorded source import over its actual JavaScript implementation.
// This resolves a module edge; it never replaces an algorithm or native body.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { readTypeSyntaxSourceModule } from './generate-type-syntax-rules.mjs';

const boundary = "import { readRequiredNumberFact, readRequiredTextFact, readRequiredBooleanFact } from './fact-column-rules.js';";
const types = "import { readTypeSeparatorPositions } from './type-syntax-rules.js';";

export function readCallableSourceModule(source = readFileSync(new URL('../src/translation/callable-source-rules.js', import.meta.url), 'utf8')) {
  assert.equal(source.split(boundary).length, 2, 'the callable module has exactly its recorded fact-column source import');
  assert.equal(source.split(types).length, 2, 'the callable module has exactly its recorded type separator source import');
  return readTypeSyntaxSourceModule() + '\n' + source.replace(boundary, '').replace(types, '');
}
