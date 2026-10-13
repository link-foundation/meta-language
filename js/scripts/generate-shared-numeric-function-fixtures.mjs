#!/usr/bin/env node
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { Grammar, serializeGrammar } from '../src/grammar.js';
import { createGlobalConceptSpace } from '../src/global-concept-space.js';
import { LinkNetwork } from '../src/network.js';
import { TYPED_PROGRAM_LINKS_SCHEMA } from '../src/typed-program-links.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeSharedNumericFunctionDefinition } from '../src/shared-numeric-function-links.js';
import { encodeFormalFunctionDefinition } from '../src/formal-function-links.js';
import { proveFormalConceptEquivalence, readFormalConceptDefinition } from '../src/formal-concept-definition.js';

const descriptions = [
  ['add', '/** @param {number} a @param {number} b @returns {number} */ function add(a,b){let sum=a+b;return sum;}', 'fn add(a:f64,b:f64)->f64{let sum=a+b;sum} fn main(){}', [[5,2],[0,-0],[1e308,1e308]]],
  ['factorial', '/** @param {number} n @returns {number} */ function factorial(n){if(n<1)return 1;return n*factorial(n-1);}', 'fn factorial(n:f64)->f64{if n<1.0{1.0}else{n*factorial(n-1.0)}} fn main(){}', [[0],[1],[5]]],
  ['less', '/** @param {number} a @param {number} b @returns {boolean} */ function less(a,b){return a<b;}', 'fn less(a:f64,b:f64)->bool{a<b} fn main(){}', [[2,3],[3,2],[-0,0]]],
  ['rem', '/** @param {number} a @param {number} b @returns {number} */ function rem(a,b){return a%b;}', 'fn rem(a:f64,b:f64)->f64{a%b} fn main(){}', [[-5,2],[5,-2],[-0,3]]],
  ['divide', '/** @param {number} a @param {number} b @returns {number} */ function divide(a,b){return a/b;}', 'fn divide(a:f64,b:f64)->f64{a/b} fn main(){}', [[0,0],[1,-0],[1,0]]],
  ['flip', '/** @param {boolean} flag @returns {boolean} */ function flip(flag){return !flag;}', 'fn flip(flag:bool)->bool{!flag} fn main(){}', [[true],[false]]],
  ['negate', '/** @param {number} value @returns {number} */ function negate(value){return -value;}', 'fn negate(value:f64)->f64{-value} fn main(){}', [[0],[-0],[5]]],
  ['even', '/** @param {number} n @returns {boolean} */ function even(n){if(n<=0)return true;return odd(n-1);}/** @param {number} n @returns {boolean} */ function odd(n){if(n<=0)return false;return even(n-1);}', 'fn even(n:f64)->bool{if n<=0.0{true}else{odd(n-1.0)}} fn odd(n:f64)->bool{if n<=0.0{false}else{even(n-1.0)}} fn main(){}', [[0],[1],[6]]],
];
const cases = []; const equivalent = [];
for (const [declaration, javascript, rust, inputs] of descriptions) {
  const pair = [];
  for (const [language, source] of [['JavaScript', javascript], ['Rust', rust]]) {
    const input = createTypedProgramLinks(source, language);
    const provenance = { source: `${language} ${declaration}` };
    const definition = encodeSharedNumericFunctionDefinition(input, declaration, { provenance });
    const oldDefinition = encodeFormalFunctionDefinition(input, declaration);
    pair.push(cases.length);
    cases.push({ name: `${language} ${declaration}`, language, declaration, source, inputs, provenance,
      programRoot: input.root.asU64(), programLinks: input.network.toLino(),
      root: definition.root.asU64(), links: definition.network.toLino(), expected: readFormalConceptDefinition(definition),
      conservativeRoot: oldDefinition.root.asU64(), conservativeLinks: oldDefinition.network.toLino() });
  }
  if (!proveFormalConceptEquivalence(cases[pair[0]], cases[pair[1]])) throw new Error(`shared numeric premise failed for ${declaration}`);
  equivalent.push(pair);
}
const changed = createTypedProgramLinks(descriptions[0][1].replace('a+b', 'a-b'), 'JavaScript');
const definition = encodeSharedNumericFunctionDefinition(changed, 'add', { provenance: { source: 'changed add operation' } });
cases.push({ name: 'changed add operation', language: 'JavaScript', declaration: 'add', source: descriptions[0][1].replace('a+b', 'a-b'), provenance: { source: 'changed add operation' }, inputs: [[5,2]], programRoot: changed.root.asU64(), programLinks: changed.network.toLino(), root: definition.root.asU64(), links: definition.network.toLino(), expected: readFormalConceptDefinition(definition) });
const refusals = [
  { name: 'fixed width integer premise', language: 'Rust', declaration: 'add', source: 'fn add(a:i32,b:i32)->i32{a+b}fn main(){}' },
  { name: 'string interpretation', language: 'JavaScript', declaration: 'add', source: '/** @param {string} a @returns {string} */ function add(a){return a;}' },
  { name: 'host output effect', language: 'JavaScript', declaration: 'add', source: '/** @param {number} a @returns {number} */ function add(a){console.log(a);return a;}' },
  { name: 'parameter default', language: 'JavaScript', declaration: 'add', source: '/** @param {number} a @returns {number} */ function add(a=1){return a;}' },
  { name: 'undeclared numeric library primitive', language: 'JavaScript', declaration: 'add', source: '/** @param {number} a @returns {number} */ function add(a){return Math.abs(a);}' },
];
for (const refusal of refusals) {
  const program = createTypedProgramLinks(refusal.source, refusal.language);
  refusal.programRoot = program.root.asU64(); refusal.programLinks = program.network.toLino();
  let refused = false;
  try { encodeSharedNumericFunctionDefinition(program, refusal.declaration); } catch { refused = true; }
  if (!refused) throw new Error(`profile must refuse ${refusal.name}`);
}
const originRefusals = [];
for (const name of ['referenceSemantics', 'utf16Strings']) {
  const input = { network: LinkNetwork.fromLino(cases[0].programLinks), root: cases[0].programRoot };
  const sample = LinkNetwork.fromLino(cases[2].programLinks);
  const metadata = sample.links().find(link => link.metadata().term === TYPED_PROGRAM_LINKS_SCHEMA.scalars.boolean && link.metadata().definition === 'true').metadata();
  const flag = input.network.insertLink([], metadata.clone());
  input.network.insertFieldRelation(input.network.link(input.root).references()[0], TYPED_PROGRAM_LINKS_SCHEMA.fields[name] ?? name, flag);
  let refused = false;
  try { encodeSharedNumericFunctionDefinition(input, 'add'); } catch { refused = true; }
  if (!refused) throw new Error(`the source profile must retain the ${name} premise`);
  originRefusals.push({ name, declaration: 'add', programRoot: input.root, programLinks: input.network.toLino() });
}
const encodedOriginRefusals = [];
for (const name of ['referenceSemantics', 'utf16Strings']) {
  const input = name === 'utf16Strings' ? createTypedProgramLinks(descriptions[0][1], 'JavaScript', { utf16Strings: true }) : (() => {
    const value = createTypedProgramLinks(descriptions[0][1], 'JavaScript');
    const checked = decodeTypedProgramLinks(value.network, value.root); checked.referenceSemantics = true;
    return encodeTypedProgramLinks(checked);
  })();
  const parent = input.network.link(input.root).references()[0];
  const label = name === 'referenceSemantics' ? 'reference semantics' : TYPED_PROGRAM_LINKS_SCHEMA.fields.utf16Strings;
  if (!input.network.fieldRelations().some(field => field.parent.equals(parent) && field.label === label && input.network.link(field.child).metadata().definition === 'true')) throw new Error(`the actual ${name} factory did not encode its premise`);
  let refused = false; try { encodeSharedNumericFunctionDefinition(input, 'add'); } catch { refused = true; }
  if (!refused) throw new Error(`the actual ${name} context cannot acquire the numeric model`);
  encodedOriginRefusals.push({ name: `encoded ${name}`, declaration: 'add', programRoot: input.root.asU64(), programLinks: input.network.toLino(), contextLabel: label });
}
for (const encoded of [...encodedOriginRefusals]) for (const [term, text] of [[TYPED_PROGRAM_LINKS_SCHEMA.scalars.string, 'false'], [TYPED_PROGRAM_LINKS_SCHEMA.scalars.number, '0']]) {
  const input = { network: LinkNetwork.fromLino(encoded.programLinks), root: encoded.programRoot };
  const body = input.network.link(input.root).references()[0];
  const relation = input.network.fieldRelations().find(field => field.parent.equals(body) && field.label === encoded.contextLabel);
  const scalar = input.network.link(relation.child);
  const changed = input.network.insertLink([], scalar.metadata().withTerm(term).withDefinition(text));
  const references = relation.link.references(); references[2] = changed; relation.link.setReferences(references);
  let refused = false; try { encodeSharedNumericFunctionDefinition(input, 'add'); } catch { refused = true; }
  if (!refused) throw new Error(`a malformed ${encoded.contextLabel} scalar cannot become a false flag`);
  encodedOriginRefusals.push({ name: `${encoded.name} ${term} ${text}`, declaration: 'add', programRoot: input.root, programLinks: input.network.toLino(), contextLabel: encoded.contextLabel, malformedScalar: true });
}
const invalidDefinitions = [];
for (const name of ['unsupported source context', 'counterfeit shared premise']) {
  const network = LinkNetwork.fromLino(cases[0].links); const root = cases[0].root;
  let point;
  if (name === 'unsupported source context') {
    const body = network.link(network.link(root).references()[0]).references()[0];
    point = network.fieldRelations().find(field => field.parent.equals(body) && field.label === TYPED_PROGRAM_LINKS_SCHEMA.fields.sourceLanguage).child;
  } else {
    const model = network.link(network.link(root).references()[1]).references()[0];
    point = network.fieldRelations().find(field => field.parent.equals(model) && field.label === 'sourceLanguageRole').child;
  }
  network.link(point).setMetadata(network.link(point).metadata().withDefinition(name === 'unsupported source context' ? 'Lean' : 'ignore every language unconditionally'));
  const value = { name, root, links: network.toLino() };
  let refused = false; try { readFormalConceptDefinition(value); } catch { refused = true; }
  if (!refused) throw new Error(`a forged ${name} cannot ground equality`);
  invalidDefinitions.push(value);
}
const grammar = JSON.parse(serializeGrammar(new Grammar('function', [['function', { expression: { kind: 'literal', value: 'function' } }]])));
const globalSources = ['JavaScript', 'Rust'].map(language => ({ id: language.toLowerCase(), language, grammar }));
const definitions = { 'javascript:function': cases[0], 'rust:function': cases[1] };
const space = createGlobalConceptSpace(globalSources.map(source => ({ ...source, grammar: new Grammar('function', [['function', { expression: { kind: 'literal', value: 'function' } }]]) })), { records: [], formalDefinitions: definitions });
if (space.mappings['javascript:function'] !== space.mappings['rust:function']) throw new Error('the checked numeric definitions must ground shared global identity');
const fixture = { globalSources, schemaVersion: 1, id: 'shared-numeric-function-definitions', model: 'closed binary floating point function', cases, equivalent, distinct: [[0, cases.length - 1]], refusals, originRefusals, encodedOriginRefusals, invalidDefinitions };
const path = new URL('../../parity/fixtures/shared-numeric-function-definitions.json', import.meta.url);
if (process.argv.includes('--check')) checkProjectData(path, fixture);
else writeProjectData(path, fixture);
