// Extract the entire original portable function into a shared callable template.
// Both runtime checkers instantiate its real syntax and check ordinary core
// operations; neither adapter supplies a native character-reading algorithm.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { readCallableCapturedBindings } from '../src/translation/callable-source.js';
import { STRING_PRELUDE } from '../src/translation/string-runtime.js';
import { checkProjectData, readProjectData, writeProjectData } from './project-data.mjs';

const sourcePath = new URL('../src/translation/string-code-unit-rules.js', import.meta.url);
const javascriptPath = new URL('../src/translation/data/string-code-unit-template.lino', import.meta.url);
const nativePath = new URL('../../rust/src/data/string-code-unit-template.lino', import.meta.url);
const digest = value => createHash('sha256').update(value).digest('hex');

export function readStringCodeUnitTemplateGeneration() {
  const source = readFileSync(sourcePath, 'utf8');
  const surface = parseJavaScript(source, { referenceSemantics: false });
  const checked = checkProgram(surface);
  assert.equal(surface.items.length, 1, 'the whole kernel contains one actual original declaration');
  assert.equal(checked.items.length, surface.items.length, 'checking retains every source declaration');
  assert.equal(checked.declarations.size, 1, 'the complete kernel has no hidden helper dependency');
  const originalDeclaration = surface.items[0];
  const checkedDeclaration = checked.items[0];
  assert.equal(originalDeclaration.k, 'fn');
  assert.equal(checkedDeclaration.k, 'fn');
  assert.equal(originalDeclaration.name, 'readStringCodeUnit');
  assert.equal(originalDeclaration.params.length, 2);
  assert.deepEqual(originalDeclaration.params.map(parameter => parameter.type.kind), ['string', 'float']);
  assert.equal(originalDeclaration.ret.kind, 'float');
  assert.equal(readCallableCapturedBindings(checkedDeclaration.body, checkedDeclaration.params.map(parameter => parameter.name)).size, 0,
    'the actual checked kernel is closed over precisely its two formal parameters');
  const template = { k: 'lambda', params: structuredClone(originalDeclaration.params),
    body: structuredClone(originalDeclaration.body), ret: structuredClone(originalDeclaration.ret), span: originalDeclaration.span };
  return {
    schemaVersion: 1,
    model: 'portable UTF16 code unit lambda 1',
    sourcePath: 'js/src/translation/string-code-unit-rules.js',
    source,
    sourceSha256: digest(source),
    inventory: surface.items.map(declaration => ({ kind: declaration.k, name: declaration.name, span: declaration.span })),
    originalDeclaration: structuredClone(originalDeclaration),
    checkedDeclaration: structuredClone(checkedDeclaration),
    template,
    basePreludeSha256: digest(STRING_PRELUDE),
    inputDomain: 'A primitive string and evaluated Number index; method checking supplies omitted zero and represented nullable numeric conversion. Boolean, String and object conversion require a separate ToNumber interpretation.',
    evaluation: 'Check this closed lambda under only its two formal binders, then apply it to the separately checked receiver and index once in original order. No actual operand is substituted into the template body.',
    reverseBoundary: 'The resulting checked graph uses existing Lambda, Apply, Let, Math, Slice and CodePointAt operations. Generated Rust runtime source still requires its own source interpretation and is never admitted by a helper name.',
    character: readStringCharacterTemplateGeneration(),
  };
}

import {readStringCharacterTemplateGeneration} from './string-character-template-generation.mjs';

export function generateStringCodeUnitTemplate(check = false) {
  const policy = readStringCodeUnitTemplateGeneration();
  for (const path of [javascriptPath, nativePath]) {
    if (check) checkProjectData(path, policy);
    else writeProjectData(path, policy);
    assert.deepEqual(readProjectData(path), policy, 'the complete generated callable template rereads exactly');
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const policy = generateStringCodeUnitTemplate(process.argv.includes('--check'));
  console.log('UTF-16 code unit template: ' + policy.inventory.length + ' complete checked original declaration; two ordered operands');
}
