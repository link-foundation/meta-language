// Close the actual required-column import; retain the complete six-function
// source and every reference decision without native algorithm substitutions.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';

const boundary = "import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';";

export function readSurfaceReferenceSourceModule(source = readFileSync(new URL('../src/translation/surface-reference-analysis-rules.js', import.meta.url), 'utf8')) {
  assert.equal(source.split(boundary).length, 2, 'the reference module retains exactly its recorded actual fact-column import');
  const accessors = readFileSync(new URL('../src/translation/fact-column-rules.js', import.meta.url), 'utf8');
  return accessors + '\n' + source.replace(boundary, '');
}
