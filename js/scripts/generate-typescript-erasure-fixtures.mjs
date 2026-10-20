#!/usr/bin/env node
// Bounded native CST/runtime parity cases for the shared TypeScript eraser.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import { checkProjectData, writeProjectData, serializeProjectData } from './project-data.mjs';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import { eraseTypeScript } from '../src/typescript-erasure.js';
import { selfTranslate } from '../src/self-translation.js';
import { projectTypeScriptTyping } from '../src/typescript-portable-typing.js';

const inputs = [
  ['annotated-function', 'function add(a: number, b: number): number { return a + b; }\n'],
  ['annotated-string-function', 'function identity(value: string): string { return value; }\n'],
  ['annotated-boolean-function', 'function identity(value: boolean): boolean { return value; }\n'],
  ['exported-interface-and-alias', 'export interface Entry { value: string }\ntype Amount = number;\nconst amount: Amount = 3;\n'],
  ['generic-optional-and-Unicode', 'function identity<T>(value?: T): T { return value!; }\nconst 汉字: number = 3 as number;\nconst result = 汉字 > 0 ? identity<number>(汉字) : 0;\n'],
  ['comments-and-compile-time-shape', '// shape\nexport interface Entry { value: number }\n\n// runtime\nexport function add(a: number, b: number): number { return a + b; }\n'],
  ['carriage-returns-and-Unicode-type', 'type 类型 = number;\r\nfunction add(a: 类型, b: 类型): 类型 { return a + b; }\r\n'],
  ['type-only-import', 'import type { Entry } from "./entry.js";\nconst amount: number = 3;\n'],
  ['ordinary-JavaScript', 'function add(a, b) { return a + b; }\n'],
  ['enum-runtime', 'enum Entry { One }\n'],
  ['namespace-runtime', 'namespace Entry { export const value = 1; }\n'],
  ['parameter-property-runtime', 'class Entry { constructor(private value: number) {} }\n'],
  ['mixed-type-import', 'import { type Entry, value } from "./entry.js";\n'],
  ['mixed-type-import-last', 'import { value, type Entry } from "./entry.js";\nconsole.log(value);\n'],
  ['mixed-type-import-middle', 'import { value, type Entry, other } from "./entry.js";\nconsole.log(value + other);\n'],
  ['mixed-type-import-default', 'import entry, { type Entry, value } from "./entry.js";\nconsole.log(entry + value);\n'],
  ['mixed-type-import-all-types', 'import { type Entry, type Other, } from "./entry.js";\n'],
  ['mixed-type-import-comments', 'import { /* before */ type Entry /* middle */, /* retained */ value, /* after */ type Other } from "./entry.js";\nconsole.log(value);\n'],
  ['mixed-type-import-unicode-lines', 'import {\r\n type 类型 as Shape,\r\n value,\r\n type Other\r\n} from "./entry.js";\r\nconsole.log(value);\r\n'],
  ['runtime-import-named-type', 'import { type as value } from "./entry.js";\nconsole.log(value);\n'],
  ['ordinary-named-import', 'import { value, other } from "./entry.js";\nconsole.log(value + other);\n'],
];

export function typeScriptErasureFixture() {
  return { schemaVersion: 1, generatedBy: 'js/scripts/generate-typescript-erasure-fixtures.mjs',
    rules: 'js/src/data/typescript-erasure.lino',
    cases: inputs.map(([id, source]) => {
      let erased;
      let error;
      try { erased = eraseTypeScript(source); } catch (failure) { error = { reason: failure.reason, span: failure.span }; }
      return { id, source, ...(erased ? { erased, typing: projectTypeScriptTyping(source).code } : { error }),
        javascript: selfTranslate(source, 'TypeScript', 'JavaScript').code,
        ...(['annotated-function', 'annotated-string-function', 'annotated-boolean-function'].includes(id)
          ? { rust: selfTranslate(source, 'TypeScript', 'Rust').code } : {}) };
    }) };
}

const path = new URL('../../parity/fixtures/typescript-erasure.json', import.meta.url);
const fixture = typeScriptErasureFixture();
if (process.argv.includes('--check')) {
  checkProjectData(path, fixture);
} else writeProjectData(path, fixture);

// Keep the earlier unsupported case and every original input independently
// recoverable. Only the now-implemented mixed import changes its expectation.
const revision='15745ffcd322c48bdd537ccb48a5f7c9deb96917';
const fixturePath='parity/fixtures/typescript-erasure.lino';
const original=captureGitArtifactCertificate(revision,fixturePath);
const previous=readStructuredDataLino(original.blob.toString('utf8'));
assert.equal(previous.cases.length,13);
const corrected=previous.cases.find(row=>row.id==='mixed-type-import');
assert.equal(corrected.error.reason,'TypeScript import_specifier: mixed type imports need a linked separator reconstruction rule');
for(const old of previous.cases){
  const current=fixture.cases.find(row=>row.id===old.id);assert.ok(current);
  assert.equal(current.source,old.source);
  if(old.id!==corrected.id)assert.deepEqual(current,old);
  else {assert.ok(current.erased);assert.equal(current.error,undefined);}
}
const appended=fixture.cases.slice(previous.cases.length).map(row=>row.id);
assert.equal(appended.length,8);assert.equal(new Set(fixture.cases.map(row=>row.id)).size,fixture.cases.length);
const inverse={...fixture,cases:fixture.cases.slice(0,previous.cases.length).map(row=>row.id===corrected.id?corrected:row)};
assert.equal(serializeProjectData(inverse,fixturePath),original.blob.toString('utf8'));
const history={schemaVersion:1,revision,path:fixturePath,certificate:original.certificate,
 previousCase:corrected,originalCases:previous.cases.map(row=>({id:row.id,source:row.source})),appended,
 currentSha256:createHash('sha256').update(serializeProjectData(fixture,fixturePath)).digest('hex'),
 basis:'https://www.typescriptlang.org/tsconfig/verbatimModuleSyntax.html',
 oracle:{subject:'Node.js independent stripTypeScriptTypes API',ranges:['>=22.15.0 <23.0.0','>=24.0.0 <25.0.0']},
 boundary:'Expectation extension for the implemented explicit type-modifier rule. The sole prior unsupported expectation and every complete preceding fixture byte remain independently recoverable through its Git certificate. Independent runtime, token and provenance controls are required; this source packet supplies no test execution or native parity evidence.'};
const historyPath=new URL('../../parity/fixtures/typescript-named-import-expectation-history.lino',import.meta.url);
if(process.argv.includes('--check'))checkProjectData(historyPath,history);else writeProjectData(historyPath,history);
