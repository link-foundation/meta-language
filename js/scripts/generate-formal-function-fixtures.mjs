#!/usr/bin/env node
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { createTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeFormalFunctionDefinition } from '../src/formal-function-links.js';
import { readFormalConceptDefinition } from '../src/formal-concept-definition.js';

const factorial = '/** @param {number} n @returns {number} */ function factorial(n){if(n<1)return 1;return n*factorial(n-1);}';
const mutual = '/** @param {number} n @returns {boolean} */ function even(n){if(n<=0)return true;return odd(n-1);}/** @param {number} n @returns {boolean} */ function odd(n){if(n<=0)return false;return even(n-1);}';
const descriptions = [
  { name: 'factorial first context', language: 'JavaScript', declaration: 'factorial', source: `${factorial}console.log("first");` },
  { name: 'factorial other context', language: 'JavaScript', declaration: 'factorial', source: `${factorial}/** @returns {number} */ function unused(){return 9;}console.log("second");` },
  { name: 'factorial changed operator', language: 'JavaScript', declaration: 'factorial', source: factorial.replace('n*factorial', 'n+factorial') },
  { name: 'factorial changed public name', language: 'JavaScript', declaration: 'other', source: factorial.replaceAll('factorial', 'other') },
  { name: 'mutual even', language: 'JavaScript', declaration: 'even', source: mutual },
  { name: 'mutual odd', language: 'JavaScript', declaration: 'odd', source: mutual },
  { name: 'mutual even other context', language: 'JavaScript', declaration: 'even', source: `${mutual}/** @returns {number} */ function unused(){return 9;}console.log("unrelated");` },
  { name: 'factorial Rust source context', language: 'Rust', declaration: 'factorial', source: 'fn factorial(n:f64)->f64{if n<1.0 {1.0}else{n*factorial(n-1.0)}} fn main(){}' },
];
const cases = descriptions.map(description => {
  const input = createTypedProgramLinks(description.source, description.language);
  const definition = encodeFormalFunctionDefinition(input, description.declaration, { provenance: { source: description.name, sourceLanguage: description.language } });
  return { ...description, programRoot: input.root.asU64(), programLinks: input.network.toLino(), root: definition.root.asU64(), links: definition.network.toLino(), expected: readFormalConceptDefinition(definition) };
});
const fixture = { schemaVersion: 1, id: 'formal-function-definitions', cases,
  equivalent: [[0, 1], [4, 6]], distinct: [[0, 2], [0, 3], [4, 5], [0, 7]] };
const path = new URL('../../parity/fixtures/formal-function-definitions.json', import.meta.url);
if (process.argv.includes('--check')) {
  checkProjectData(path, fixture);
  console.log('formal function fixture matches the source-free definition bridge');
} else writeProjectData(path, fixture);
