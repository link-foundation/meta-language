#!/usr/bin/env node
// Compile bounded public consumers with an explicitly supplied existing compiler.
// This checker never downloads a compiler or installs consumer dependencies.
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { readProjectData } from './project-data.mjs';

const publicModule = fileURLToPath(new URL('../src/index.js', import.meta.url));

export function publicDeclarationConsumers(moduleSpecifier = publicModule) {
  const module = JSON.stringify(moduleSpecifier);
  const emptyOptionFixture = readProjectData(new URL('../../parity/fixtures/rust-empty-options.lino', import.meta.url));
  const typeNamespaceFixture = readProjectData(new URL('../../parity/fixtures/rust-type-namespace.lino', import.meta.url));
  const ownedTextFixture = readProjectData(new URL('../../experiments/rust-owned-local-state-fixture.lino', import.meta.url));
  const privateUnitFixture = readProjectData(new URL('../../parity/fixtures/rust-pure-unit-calls.lino', import.meta.url));
  const publicUnitFixture = readProjectData(new URL('../../parity/fixtures/rust-public-pure-unit-calls.lino', import.meta.url));
  const constFunctionFixture = readProjectData(new URL('../../experiments/rust-const-function-fixture.lino', import.meta.url));
  const sourceTextByteFixture = readProjectData(new URL('../../parity/fixtures/source-text-bytes.lino', import.meta.url));
  const runtimeShellFixture = readProjectData(new URL('../../parity/fixtures/rust-runtime-shell.lino', import.meta.url));
  const repeatedRuntime = runtimeShellFixture.cases.find(entry => entry.id === 'ordered-repeated');
  const ownedTupleCallFixture = readProjectData(new URL('../../experiments/owned-tuple-call-fixture.lino', import.meta.url));
  const retargetTupleCall = ownedTupleCallFixture.cases.find(entry => entry.name === 'source-free target');
  const tupleCallGraphPrelude = `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork, LinkId, SubstitutionRule } from ${module};
const packet = createTypedProgramLinks(${JSON.stringify(retargetTupleCall.source)},'Rust',
  {rustOwnedLocalStateModel:${JSON.stringify(ownedTupleCallFixture.model)}});
const network = LinkNetwork.fromLino(packet.network.toLino());
const originalProgram = decodeTypedProgramLinks(network,packet.root);
const relations = network.links().filter(link=>link.references().length===3);
const field = (parent:LinkId,label:string):LinkId => {
  const relation = relations.find(link=>link.references()[0].equals(parent)
    &&network.link(link.references()[1])?.metadata().term===label);
  if(!relation) throw new Error('missing actual owned tuple field '+label);
  return relation.references()[2];
};
const declaration = (name:string):LinkId => {
  const relation = relations.find(link=>network.link(link.references()[1])?.metadata().term==='full name'
    &&network.link(link.references()[2])?.metadata().definition===name);
  if(!relation) throw new Error('missing actual owned tuple declaration '+name);
  return relation.references()[0];
};
const seed = declaration('seed'),choose = declaration('choose'),alternate = declaration('alternate');
const reference = relations.find(link=>network.link(link.references()[1])?.metadata().term==='declaration reference'
  &&link.references()[2].equals(seed));
if(!reference) throw new Error('the actual owned initializer declaration reference is missing');
const initializerList = network.link(field(field(choose,'source local state'),'initializers'));
if(!initializerList) throw new Error('the actual owned initializer list is missing');
const initializer = initializerList.references()[0];
`;
  const imports = `import { importEbnf, encodeFormalConceptDefinition, createDiscoveredConceptLinks,
    readDiscoveredConceptLinks, importResourceDescriptionGraph, decodeResourceDescriptionGraph,
    mergeResourceDescriptionGraphs, verifyResourceDescriptionGraphWitnesses } from ${module};\n`;
  const semanticRecord = `id:'fixture.consumer-atom',phrase:'consumer atom',role:'concept',
    definition:'A source-owned consumer atom.',constraints:['The declared formal archive remains inspectable.'],
    sourceAliases:[{source:'consumer',name:'root'}],formerNames:[]`;
  const scalarLoopSource = JSON.stringify('fn advance(mut value:f64,limit:f64,mut at:f64)->f64{loop{return if at<limit{(value,at)=(value+1f64,at+1f64);continue;}else{value};}}fn main(){}');
  return [
    {
      name: 'valid-readonly-triples-and-formal-records',
      expectedDiagnostics: [],
      source: `${imports}
import type { ResourceDescriptionGraph, DiscoveredConceptGraph, FormalConceptArchive } from ${module};
const triples = Object.freeze([Object.freeze([{kind:'blank',value:'subject'}, {kind:'identifier',value:'urn:predicate'},
  {kind:'literal',value:'retained',language:'EN'}] as const)] as const);
const packet = importResourceDescriptionGraph(triples,{sourceIdentifier:'typed consumer'});
const graph: ResourceDescriptionGraph = decodeResourceDescriptionGraph(packet.network,packet.root,{nodeLimit:10000});
const subject = graph.triples[0][0];
const object = graph.triples[0][2];
let blankScope: string = '';
let literalDatatype: string = '';
let literalLanguage: string = '';
if(subject.kind === 'blank') blankScope = subject.sourceIdentifier;
if(object.kind === 'literal') { literalDatatype = object.datatype; literalLanguage = object.language; }
const predicateKind: 'identifier' = graph.triples[0][1].kind;
const merged = mergeResourceDescriptionGraphs([packet] as const);
const witnesses: boolean = verifyResourceDescriptionGraphWitnesses(merged.network,merged.root);
const position: number = graph.occurrences[0].position;
const archive: FormalConceptArchive = encodeFormalConceptDefinition(undefined,{model:'source text value'});
const candidates = [{id:'consumer',language:'custom',grammar:importEbnf('root = "x" ;')}] as const;
const created = createDiscoveredConceptLinks(candidates,{records:[{${semanticRecord},formalDefinition:archive}]});
const restored: DiscoveredConceptGraph = readDiscoveredConceptLinks(created.network,created.root);
const preserved: FormalConceptArchive = restored.formalDefinitions['fixture.consumer-atom'];
const portable = {links:archive.network.toLino(),root:archive.root.asU64()};
const copied = createDiscoveredConceptLinks(candidates,{records:[{${semanticRecord},formalDefinition:portable}]});
if(graph.triples[0][0].kind !== 'blank' || graph.triples[0][0].sourceIdentifier !== 'typed consumer') throw new Error('blank scope changed');
if(graph.triples[0][2].kind !== 'literal' || graph.triples[0][2].language !== 'en') throw new Error('literal normalization changed');
if(!witnesses || position !== 0 || !preserved.network.link(preserved.root)) throw new Error('a graph contract changed');
export const contractEvidence = {tripleCount:graph.triples.length,sourceCount:restored.sources.length,
  preservedDefinition:!!preserved.network.link(preserved.root),copiedDefinition:!!copied.network.link(copied.root),
  blankScope,literalDatatype,literalLanguage,predicateKind};
`,
    },
    {
      name: 'reject-object-statements', expectedDiagnostics: ['TS2353'],
      source: `${imports}importResourceDescriptionGraph([{subject:{kind:'identifier',value:'urn:subject'},
  predicate:{kind:'identifier',value:'urn:predicate'},object:{kind:'literal',value:'value'}}],{sourceIdentifier:'consumer'});\n`,
    },
    {
      name: 'reject-text-in-place-of-formal-archive', expectedDiagnostics: ['TS2322'],
      source: `${imports}createDiscoveredConceptLinks([{id:'consumer',language:'custom',grammar:importEbnf('root = "x" ;')}],
  {records:[{${semanticRecord},formalDefinition:'opaque text'}]});\n`,
    },
    {
      name: 'reject-decoded-blank-without-scope', expectedDiagnostics: ['TS2322'],
      source: `import type { DecodedResourceDescriptionTerm } from ${module};\nconst term:DecodedResourceDescriptionTerm={kind:'blank',value:'label'};\n`,
    },
    {
      name: 'reject-decoded-literal-without-metadata', expectedDiagnostics: ['TS2322'],
      source: `import type { DecodedResourceDescriptionTerm } from ${module};\nconst term:DecodedResourceDescriptionTerm={kind:'literal',value:'text'};\n`,
    },
    {
      name: 'valid-selected-vector-profile-and-formal-function', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, encodeFormalFunctionDefinition } from ${module};
import type { FormalConceptArchive } from ${module};
const packet = createTypedProgramLinks('fn values()->Vec<f64>{vec![1f64]} fn main(){}','Rust',
  {rustArrayResultModel:'immutable fresh scalar vector results 1',provenance:{consumer:'typed'}});
const definition:FormalConceptArchive = encodeFormalFunctionDefinition(packet,'values',{provenance:'selected'});
`,
    },
    {
      name: 'reject-unknown-vector-result-profile', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks } from ${module};\ncreateTypedProgramLinks('fn main(){}','Rust',{rustArrayResultModel:'unknown model'});\n`,
    },
    {
      name: 'valid-combined-guarded-completion-and-vector-profiles', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, encodeFormalFunctionDefinition } from ${module};
import type { FormalConceptArchive } from ${module};
const packet = createTypedProgramLinks('fn values(n:f64)->Vec<f64>{if n < 0f64{return vec![n];}vec![n.abs()]} fn main(){}','Rust',
  {rustCompletionModel:'closed guarded function returns 1',rustArrayResultModel:'immutable fresh scalar vector results 1'});
const definition:FormalConceptArchive = encodeFormalFunctionDefinition(packet,'values');
`,
    },
    {
      name: 'reject-unknown-guarded-completion-profile', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks } from ${module};\ncreateTypedProgramLinks('fn main(){}','Rust',{rustCompletionModel:'unknown model'});\n`,
    },
    {
      name: 'reject-source-text-in-place-of-formal-function-graph', expectedDiagnostics: ['TS2345'],
      source: `import { encodeFormalFunctionDefinition } from ${module};\nencodeFormalFunctionDefinition('opaque source text','function');\n`,
    },
    {
      name: 'valid-shared-numeric-function-correspondence', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, encodeSharedNumericFunctionDefinition,
  proveFormalConceptEquivalence, verifyFormalConceptEquivalence, readFormalConceptDefinition } from ${module};
import type { FormalConceptArchive } from ${module};
const rust = createTypedProgramLinks('fn add(x:f64)->f64{let y=x+1f64;y*2f64}fn main(){}','Rust',
  {rustSourceSignatureModel:'Rust source function signature 1'});
const javascript = createTypedProgramLinks('/** @param {number} x */ function add(x){const y=x+1;return y*2;}','JavaScript');
const first:FormalConceptArchive = encodeSharedNumericFunctionDefinition(rust,'add',{provenance:{consumer:'typed'}});
const second:FormalConceptArchive = encodeSharedNumericFunctionDefinition(
  {links:javascript.network.toLino(),root:javascript.root.asU64()},'add');
const certificate:FormalConceptArchive|null = proveFormalConceptEquivalence(first,second);
if(!certificate)throw new Error('the matched numeric functions need a checked correspondence');
export const contractEvidence = {matched:verifyFormalConceptEquivalence(first,second,certificate),
  model:readFormalConceptDefinition(first).model};
`,
    },
    {
      name: 'reject-source-text-in-place-of-shared-numeric-function-graph', expectedDiagnostics: ['TS2345'],
      source: `import { encodeSharedNumericFunctionDefinition } from ${module};\nencodeSharedNumericFunctionDefinition('opaque source text','function');\n`,
    },
    {
      name: 'reject-unknown-Rust-source-signature-profile', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks } from ${module};\ncreateTypedProgramLinks('fn main(){}','Rust',{rustSourceSignatureModel:'unknown model'});\n`,
    },
    {
      name: 'reject-undeclared-shared-numeric-function-selector', expectedDiagnostics: ['TS2353'],
      source: `import { createTypedProgramLinks, encodeSharedNumericFunctionDefinition } from ${module};\nencodeSharedNumericFunctionDefinition(createTypedProgramLinks('fn f()->bool{true}fn main(){}','Rust'),{identity:'f'});\n`,
    },
    {
      name: 'valid-selected-Rust-scalar-loop-profile', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
import type { TypedProgramLinksEmission } from ${module};
const packet = createTypedProgramLinks(${scalarLoopSource},'Rust',
  {rustScalarLoopModel:'closed primitive scalar loop 1',provenance:{consumer:'typed scalar loop'}});
const network = LinkNetwork.fromLino(packet.network.toLino());
const checked = decodeTypedProgramLinks(network,packet.root);
const javascript:TypedProgramLinksEmission = emitTypedProgramLinks(network,packet.root,'JavaScript');
const rust:TypedProgramLinksEmission = emitTypedProgramLinks(network,packet.root,'Rust');
export const contractEvidence = {language:javascript.language,code:javascript.code,rustCode:rust.code,
  declaration:checked.declarations.has('advance'),root:!!network.link(packet.root),
  proof:network.links().some(link=>link.metadata().term === 'source loop proof')};
`,
    },
    {
      name: 'valid-selected-JavaScript-scalar-loop-profile', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
import type { TypedProgramLinksEmission } from ${module};
const original = createTypedProgramLinks(${scalarLoopSource},'Rust',{rustScalarLoopModel:'closed primitive scalar loop 1'});
const actualSource = emitTypedProgramLinks(original.network,original.root,'JavaScript').code;
const packet = createTypedProgramLinks(actualSource,'JavaScript',
  {javaScriptScalarLoopModel:'closed primitive scalar loop 1',provenance:{consumer:'typed reverse scalar loop'}});
const network = LinkNetwork.fromLino(packet.network.toLino());
const checked = decodeTypedProgramLinks(network,packet.root);
const javascript:TypedProgramLinksEmission = emitTypedProgramLinks(network,packet.root,'JavaScript');
const rust:TypedProgramLinksEmission = emitTypedProgramLinks(network,packet.root,'Rust');
export const contractEvidence = {language:javascript.language,code:javascript.code,rustCode:rust.code,
  declaration:checked.declarations.has('advance'),root:!!network.link(packet.root),
  proof:network.links().some(link=>link.metadata().term === 'source loop proof')};
`,
    },
    {
      name: 'reject-unknown-Rust-scalar-loop-profile', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks } from ${module};\ncreateTypedProgramLinks('fn main(){}','Rust',{rustScalarLoopModel:'unknown model'});\n`,
    },
    {
      name: 'reject-unknown-JavaScript-scalar-loop-profile', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks } from ${module};\ncreateTypedProgramLinks('function keep(){return 1;}','JavaScript',{javaScriptScalarLoopModel:'unknown model'});\n`,
    },
    {
      name: 'valid-selected-borrowed-Rust-source-header', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
import type { TypedProgramLinksEmissionOptions, TypedProgramLinksEmission } from ${module};
const packet = createTypedProgramLinks("fn choose<'a,'b>(flag:&'a bool,value:&'b f64)->f64{if *flag{*value}else{-*value}}fn main(){}",'Rust',
  {rustSourceSignatureModel:'Rust source function signature 1'});
const network = LinkNetwork.fromLino(packet.network.toLino());
const options:TypedProgramLinksEmissionOptions = Object.freeze({rustSourceHeaderEmissionModel:'primitive borrowed Rust function header 1'} as const);
const selected:TypedProgramLinksEmission = emitTypedProgramLinks(network,packet.root,'Rust',options);
const legacy:TypedProgramLinksEmission = emitTypedProgramLinks(network,packet.root,'Rust');
const javascript:TypedProgramLinksEmission = emitTypedProgramLinks(network,packet.root,'JavaScript');
export const contractEvidence = {language:selected.language,code:javascript.code,rustCode:selected.code,legacyCode:legacy.code,
  root:!!network.link(packet.root),signature:network.links().some(link=>link.metadata().term === 'source function signature')};
`,
    },
    {
      name: 'valid-source-free-borrowed-Rust-header-value-edit', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
import type { TypedProgramLinksEmissionOptions, TypedProgramLinksEmission } from ${module};
const packet = createTypedProgramLinks('fn add_one(value:&f64)->f64{*value+1.0}fn main(){}','Rust',
  {rustSourceSignatureModel:'Rust source function signature 1'});
const network = LinkNetwork.fromLino(packet.network.toLino());
const original:TypedProgramLinksEmission = emitTypedProgramLinks(network,packet.root,'JavaScript');
const literals = network.links().filter(link=>link.metadata().term === 'string value' && link.metadata().definition === '1');
if(literals.length !== 1) throw new Error('the source-free edit must select exactly one scalar link');
literals[0].metadata().definition = '2.0';
const restored = LinkNetwork.fromLino(network.toLino());
const options:TypedProgramLinksEmissionOptions = {rustSourceHeaderEmissionModel:'primitive borrowed Rust function header 1'};
const selected:TypedProgramLinksEmission = emitTypedProgramLinks(restored,packet.root,'Rust',options);
const changed:TypedProgramLinksEmission = emitTypedProgramLinks(restored,packet.root,'JavaScript');
export const contractEvidence = {language:selected.language,originalCode:original.code,changedCode:changed.code,rustCode:selected.code,
  root:!!restored.link(packet.root),signature:restored.links().some(link=>link.metadata().term === 'source function signature')};
`,
    },
    {
      name: 'reject-unknown-Rust-source-header-emission-model', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks } from ${module};
const packet = createTypedProgramLinks('fn keep(value:&f64)->f64{*value}fn main(){}','Rust',
  {rustSourceSignatureModel:'Rust source function signature 1'});
emitTypedProgramLinks(packet.network,packet.root,'Rust',{rustSourceHeaderEmissionModel:'unknown model'});
`,
    },
    {
      name: 'valid-selected-Rust-declaration-loop-profile', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
const packet = createTypedProgramLinks('fn run(mut run:f64,limit:f64)->f64{loop{return if run<limit{(run,)=(run+1f64,);continue;}else{run};}}fn main(){}','Rust',
  {rustScalarLoopModel:'closed declaration scalar loop 1'});
const network = LinkNetwork.fromLino(packet.network.toLino());
const javascript = emitTypedProgramLinks(network,packet.root,'JavaScript');
const rust = emitTypedProgramLinks(network,packet.root,'Rust');
export const contractEvidence = {network,root:packet.root,code:javascript.code,rustCode:rust.code};
`,
    },
    {
      name: 'valid-selected-JavaScript-declaration-loop-profile', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
const actualSource = '/** @param {number} run @param {number} limit @returns {number} */function run(run,limit){for(;;){if(run<limit){const ml_scalar_write0=run+1;[run,limit]=[ml_scalar_write0,limit];continue;}else{return run;}}}';
const packet = createTypedProgramLinks(actualSource,'JavaScript',{javaScriptScalarLoopModel:'closed declaration scalar loop 1'});
const network = LinkNetwork.fromLino(packet.network.toLino());
const javascript = emitTypedProgramLinks(network,packet.root,'JavaScript');
const rust = emitTypedProgramLinks(network,packet.root,'Rust');
export const contractEvidence = {network,root:packet.root,code:javascript.code,rustCode:rust.code};
`,
    },
    {
      name: 'reject-unregistered-Rust-declaration-loop-version', expectedDiagnostics: ['TS2820'],
      source: `import { createTypedProgramLinks } from ${module};\ncreateTypedProgramLinks('fn main(){}','Rust',{rustScalarLoopModel:'closed declaration scalar loop 99'});\n`,
    },
    {
      name: 'reject-unregistered-JavaScript-declaration-loop-version', expectedDiagnostics: ['TS2820'],
      source: `import { createTypedProgramLinks } from ${module};\ncreateTypedProgramLinks('function keep(){return 1;}','JavaScript',{javaScriptScalarLoopModel:'closed declaration scalar loop 99'});\n`,
    },
    {
      name: 'valid-closed-borrowed-Rust-call-interface', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
import type { TypedProgramLinksEmissionOptions } from ${module};
const packet = createTypedProgramLinks('fn leaf(value:&f64)->f64{*value+1.0}fn caller(value:f64)->f64{leaf(&value)}fn main(){}','Rust',
  {rustSourceSignatureModel:'Rust source function signature 1'});
const network = LinkNetwork.fromLino(packet.network.toLino());
const options:TypedProgramLinksEmissionOptions = {rustSourceHeaderEmissionModel:'closed primitive borrowed Rust call interface 1'};
const selected = emitTypedProgramLinks(network,packet.root,'Rust',options);
const legacy = emitTypedProgramLinks(network,packet.root,'Rust');
const javascript = emitTypedProgramLinks(network,packet.root,'JavaScript');
export const contractEvidence = {network,root:packet.root,code:javascript.code,rustCode:selected.code,legacyCode:legacy.code};
`,
    },
    {
      name: 'valid-source-free-closed-borrowed-call-value-edit', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
import type { TypedProgramLinksEmissionOptions } from ${module};
const packet = createTypedProgramLinks('fn leaf(value:&f64)->f64{*value+1.0}fn caller(value:f64)->f64{leaf(&value)}fn main(){}','Rust',
  {rustSourceSignatureModel:'Rust source function signature 1'});
const network = LinkNetwork.fromLino(packet.network.toLino());
const original = emitTypedProgramLinks(network,packet.root,'JavaScript');
const literals = network.links().filter(link=>link.metadata().term === 'string value' && link.metadata().definition === '1');
if(literals.length !== 1) throw new Error('the call value edit needs exactly one scalar link');
literals[0].metadata().definition = '2';
const restored = LinkNetwork.fromLino(network.toLino());
const options:TypedProgramLinksEmissionOptions = {rustSourceHeaderEmissionModel:'closed primitive borrowed Rust call interface 1'};
const selected = emitTypedProgramLinks(restored,packet.root,'Rust',options);
const changed = emitTypedProgramLinks(restored,packet.root,'JavaScript');
export const contractEvidence = {network:restored,root:packet.root,originalCode:original.code,changedCode:changed.code,rustCode:selected.code};
`,
    },
    {
      name: 'reject-unregistered-closed-borrowed-call-version', expectedDiagnostics: ['TS2820'],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks } from ${module};
const packet = createTypedProgramLinks('fn keep(value:&f64)->f64{*value}fn main(){}','Rust',
  {rustSourceSignatureModel:'Rust source function signature 1'});
emitTypedProgramLinks(packet.network,packet.root,'Rust',{rustSourceHeaderEmissionModel:'closed primitive borrowed Rust call interface 99'});
`,
    },
    {
      name: 'valid-standard-qualified-binary64-owner-value-edit', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
const packet = createTypedProgramLinks('fn add_one(value:std::primitive::f64)->std::primitive::f64{value+1.0}fn main(){}','Rust',
  {rustPrimitiveOwnerModel:'standard primitive Rust binary64 owner 1',rustSourceSignatureModel:'Rust source function signature 1'});
const network = LinkNetwork.fromLino(packet.network.toLino());
const original = emitTypedProgramLinks(network,packet.root,'JavaScript');
const literals = network.links().filter(link=>link.metadata().term === 'string value' && link.metadata().definition === '1');
if(literals.length !== 1) throw new Error('the qualified owner value edit needs exactly one scalar link');
literals[0].metadata().definition = '2';
const restored = LinkNetwork.fromLino(network.toLino());
const changed = emitTypedProgramLinks(restored,packet.root,'JavaScript');
const rust = emitTypedProgramLinks(restored,packet.root,'Rust');
export const contractEvidence = {network:restored,root:packet.root,originalCode:original.code,changedCode:changed.code,rustCode:rust.code};
`,
    },
    {
      name: 'valid-standard-imported-binary64-owner', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
const packet = createTypedProgramLinks('use std::primitive::f64 as Real;fn keep(value:Real)->Real{value}fn main(){}','Rust',
  {rustPrimitiveOwnerModel:'standard primitive Rust binary64 owner 1',rustSourceSignatureModel:'Rust source function signature 1'});
const network = LinkNetwork.fromLino(packet.network.toLino());
const javascript = emitTypedProgramLinks(network,packet.root,'JavaScript');
const rust = emitTypedProgramLinks(network,packet.root,'Rust');
export const contractEvidence = {network,root:packet.root,code:javascript.code,rustCode:rust.code};
`,
    },
    {
      name: 'reject-unregistered-standard-binary64-owner-version', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks } from ${module};\ncreateTypedProgramLinks('fn main(){}','Rust',{rustPrimitiveOwnerModel:'standard primitive Rust binary64 owner 99'});\n`,
    },
    {
      name: 'valid-complete-repeated-JavaScript-scoped-binding-output', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
import type { TypedProgramLinksEmissionOptions } from ${module};
const packet = createTypedProgramLinks('fn run(mut run:f64,limit:f64)->f64{loop{return if run<limit{(run,)=(run+1f64,);continue;}else{run};}}fn main(){}','Rust',
  {rustScalarLoopModel:'closed declaration scalar loop 1'});
const options:TypedProgramLinksEmissionOptions = Object.freeze({javaScriptScopedBindingEmissionModel:'owned JavaScript scoped bindings 1'} as const);
let network = LinkNetwork.fromLino(packet.network.toLino());
let root = packet.root;
const codes:string[] = [];
for(let hop = 0; hop < 4; hop++) {
  const code = emitTypedProgramLinks(network,root,'JavaScript',options).code;
  codes.push(code);
  const replay = createTypedProgramLinks(code,'JavaScript',{javaScriptScalarLoopModel:'closed declaration scalar loop 1'});
  network = LinkNetwork.fromLino(replay.network.toLino());
  root = replay.root;
}
const literals = network.links().filter(link=>link.metadata().term === 'string value' && link.metadata().definition === '1');
if(literals.length !== 1) throw new Error('the complete module value edit needs exactly one scalar link');
literals[0].metadata().definition = '2';
const restored = LinkNetwork.fromLino(network.toLino());
const editedCode = emitTypedProgramLinks(restored,root,'JavaScript',options).code;
const replay = createTypedProgramLinks(editedCode,'JavaScript',{javaScriptScalarLoopModel:'closed declaration scalar loop 1'});
const replayCode = emitTypedProgramLinks(replay.network,replay.root,'JavaScript',options).code;
export const contractEvidence = {codes,editedCode,replayCode,network:replay.network,root:replay.root};
`,
    },
    {
      name: 'reject-unregistered-JavaScript-scoped-binding-emission-version', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks } from ${module};
const packet = createTypedProgramLinks('fn keep(value:f64)->f64{value}fn main(){}','Rust');
emitTypedProgramLinks(packet.network,packet.root,'JavaScript',{javaScriptScopedBindingEmissionModel:'owned JavaScript scoped bindings 99'});
`,
    },
    ...emptyOptionFixture.cases.map(entry => ({
      name: `valid-empty-option-owner-${entry.id}`, expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
const options = {rustEmptyOptionSourceModel:'closed empty primitive Rust option 1'} as const;
const packet = createTypedProgramLinks(${JSON.stringify(entry.source)},'Rust',{...options,provenance:{consumer:${JSON.stringify(entry.id)}}});
const network = LinkNetwork.fromLino(packet.network.toLino());
const javascript = emitTypedProgramLinks(network,packet.root,'JavaScript');
const rust = emitTypedProgramLinks(network,packet.root,'Rust');
export function reimportActualRustDefinitions(definitions:string) {
  const packet = createTypedProgramLinks(definitions+'\\nfn main(){}','Rust',options);
  const network = LinkNetwork.fromLino(packet.network.toLino());
  const javascript = emitTypedProgramLinks(network,packet.root,'JavaScript');
  return {network,root:packet.root,code:javascript.code};
}
export const contractEvidence = {network,root:packet.root,code:javascript.code,rustCode:rust.code};
`,
    })),
    {
      name: 'valid-source-free-empty-option-declaration-rename', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
const options = {rustEmptyOptionSourceModel:'closed empty primitive Rust option 1'} as const;
const packet = createTypedProgramLinks(${JSON.stringify(emptyOptionFixture.cases.find(entry => entry.id === 'prelude').source)},'Rust',options);
const network = LinkNetwork.fromLino(packet.network.toLino());
const original = emitTypedProgramLinks(network,packet.root,'JavaScript');
const names = network.links().filter(link=>link.metadata().term === 'string value' && link.metadata().definition === 'empty');
if(names.length === 0) throw new Error('the coherent empty declaration rename needs its actual name links');
for(const name of names) name.metadata().definition = 'vacant';
const renamed = LinkNetwork.fromLino(network.toLino());
const javascript = emitTypedProgramLinks(renamed,packet.root,'JavaScript');
const rust = emitTypedProgramLinks(renamed,packet.root,'Rust');
export function reimportActualRustDefinitions(definitions:string) {
  const packet = createTypedProgramLinks(definitions+'\\nfn main(){}','Rust',options);
  const network = LinkNetwork.fromLino(packet.network.toLino());
  return {network,root:packet.root,code:emitTypedProgramLinks(network,packet.root,'JavaScript').code};
}
export const contractEvidence = {network:renamed,root:packet.root,originalCode:original.code,code:javascript.code,rustCode:rust.code};
`,
    },
    {
      name: 'reject-unregistered-empty-option-source-version', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks } from ${module};
createTypedProgramLinks('fn empty()->Option<f64>{None::<f64>}fn main(){}','Rust',{rustEmptyOptionSourceModel:'closed empty primitive Rust option 99'});
`,
    },
    {
      name: 'valid-empty-option-selection-type-with-wrong-source-language', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks } from ${module};
export function createWrongLanguagePacket() {
  return createTypedProgramLinks('function empty(){return undefined;}','JavaScript',{rustEmptyOptionSourceModel:'closed empty primitive Rust option 1'});
}
`,
    },
    {
      name: 'valid-empty-option-independent-profile-composition-types', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks } from ${module};
export function createComposedSignaturePacket() {
  return createTypedProgramLinks('fn empty()->Option<f64>{None::<f64>}fn main(){}','Rust',
    {rustEmptyOptionSourceModel:'closed empty primitive Rust option 1',rustSourceSignatureModel:'Rust source function signature 1'});
}
export function createComposedPrimitivePacket() {
  return createTypedProgramLinks('fn empty()->Option<f64>{None::<f64>}fn main(){}','Rust',
    {rustEmptyOptionSourceModel:'closed empty primitive Rust option 1',rustPrimitiveOwnerModel:'standard primitive Rust binary64 owner 1'});
}
`,
    },
    ...typeNamespaceFixture.cases.map(entry => ({
      name: `valid-type-namespace-owner-${entry.id}`, expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
const options = {rustTypeNamespaceModel:'resolved Rust type namespace 1'} as const;
const packet = createTypedProgramLinks(${JSON.stringify(entry.source)},'Rust',
  {...options,provenance:{consumer:${JSON.stringify(entry.id)}}});
const network = LinkNetwork.fromLino(packet.network.toLino());
const program = decodeTypedProgramLinks(network,packet.root);
const javascript = emitTypedProgramLinks(network,packet.root,'JavaScript');
const rust = emitTypedProgramLinks(network,packet.root,'Rust');
export const contractEvidence = {network,root:packet.root,program,code:javascript.code,rustCode:rust.code};
export function reimportActualRustDefinitions(definitions:string) {
  const packet = createTypedProgramLinks(definitions+'\\nfn main(){}','Rust',options);
  const network = LinkNetwork.fromLino(packet.network.toLino());
  const javascript = emitTypedProgramLinks(network,packet.root,'JavaScript');
  return {network,root:packet.root,program:decodeTypedProgramLinks(network,packet.root),code:javascript.code};
}
`,
    })),
    {
      name: 'valid-source-free-type-namespace-value-edit', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks, LinkNetwork, LinkId } from ${module};
const packet = createTypedProgramLinks(${JSON.stringify(typeNamespaceFixture.cases[0].source)},'Rust',
  {rustTypeNamespaceModel:'resolved Rust type namespace 1'});
const network = LinkNetwork.fromLino(packet.network.toLino());
const relations = network.links().filter(link=>link.references().length===3);
const field = (parent:LinkId,label:string):LinkId => {
  const relation = relations.find(link=>link.references()[0].equals(parent)
    &&network.link(link.references()[1])?.metadata().term===label);
  if(!relation) throw new Error('missing actual owned field '+label);
  return relation.references()[2];
};
const declaration = relations.find(link=>network.link(link.references()[1])?.metadata().term==='full name'
  &&network.link(link.references()[2])?.metadata().definition==='nested.f64');
if(!declaration) throw new Error('missing actual value declaration');
const value = network.link(field(field(declaration.references()[0],'body'),'value'));
if(!value) throw new Error('missing actual Boolean value');
value.metadata().definition = 'false';
const restored = LinkNetwork.fromLino(network.toLino());
const javascript = emitTypedProgramLinks(restored,packet.root,'JavaScript');
const rust = emitTypedProgramLinks(restored,packet.root,'Rust');
export const contractEvidence = {network:restored,root:packet.root,code:javascript.code,rustCode:rust.code};
`,
    },
    {
      name: 'reject-unregistered-type-namespace-source-version', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks } from ${module};
createTypedProgramLinks('fn main(){}','Rust',{rustTypeNamespaceModel:'resolved Rust type namespace 99'});
`,
    },
    {
      name: 'valid-type-namespace-selection-type-with-wrong-source-language', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks } from ${module};
export function createWrongLanguagePacket() {
  return createTypedProgramLinks('function keep(value){return value;}','JavaScript',
    {rustTypeNamespaceModel:'resolved Rust type namespace 1'});
}
`,
    },
    {
      name: 'valid-type-namespace-independent-empty-option-profile-types', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks } from ${module};
export function createComposedEmptyOptionPacket() {
  return createTypedProgramLinks(${JSON.stringify(emptyOptionFixture.cases[0].source)},'Rust',
    {rustTypeNamespaceModel:'resolved Rust type namespace 1',rustEmptyOptionSourceModel:'closed empty primitive Rust option 1'});
}
`,
    },
    ...ownedTextFixture.cases.flatMap(entry => [
      ['moves', entry.source, ownedTextFixture.models.moves],
      ['copies', entry.replaySource, ownedTextFixture.models.copies],
    ].map(([selection, source, model]) => ({
      name: `valid-owned-text-${selection}-${entry.name}`, expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
import type { TypedProgramLinksEmission } from ${module};
const packet = createTypedProgramLinks(${JSON.stringify(source)},'Rust',
  {rustOwnedLocalStateModel:${JSON.stringify(model)},provenance:{consumer:${JSON.stringify(entry.name)}}});
const network = LinkNetwork.fromLino(packet.network.toLino());
const program = decodeTypedProgramLinks(network,packet.root);
const javascript:TypedProgramLinksEmission = emitTypedProgramLinks(network,packet.root,'JavaScript');
const rust:TypedProgramLinksEmission = emitTypedProgramLinks(network,packet.root,'Rust');
export function reimportActualRustDefinitions(definitions:string) {
  const packet = createTypedProgramLinks(definitions+'\\nfn main(){}','Rust',
    {rustOwnedLocalStateModel:${JSON.stringify(ownedTextFixture.models.copies)}});
  const network = LinkNetwork.fromLino(packet.network.toLino());
  return {network,root:packet.root,program:decodeTypedProgramLinks(network,packet.root),
    code:emitTypedProgramLinks(network,packet.root,'JavaScript').code};
}
export const contractEvidence = {network,root:packet.root,program,code:javascript.code,rustCode:rust.code};
`,
    }))),
    {
      name: 'valid-source-free-owned-text-value-edit', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork, LinkId } from ${module};
const packet = createTypedProgramLinks(${JSON.stringify(ownedTextFixture.cases.find(entry => entry.name === 'select').source)},'Rust',
  {rustOwnedLocalStateModel:${JSON.stringify(ownedTextFixture.models.moves)}});
const network = LinkNetwork.fromLino(packet.network.toLino());
const original = emitTypedProgramLinks(network,packet.root,'JavaScript');
const relations = network.links().filter(link=>link.references().length===3);
const field = (parent:LinkId,label:string):LinkId => {
  const relation = relations.find(link=>link.references()[0].equals(parent)
    &&network.link(link.references()[1])?.metadata().term===label);
  if(!relation) throw new Error('missing actual owned text field '+label);
  return relation.references()[2];
};
const kind = (node:LinkId) => network.link(field(node,'construct kind'))?.metadata().definition;
const declaration = relations.find(link=>network.link(link.references()[1])?.metadata().term==='full name'
  &&network.link(link.references()[2])?.metadata().definition==='select');
if(!declaration) throw new Error('missing actual owned text declaration');
let initializer:LinkId|null = null;
for(let node=field(declaration.references()[0],'body');kind(node)==='let';node=field(node,'body')) {
  const candidate = field(node,'value');
  if(kind(candidate)==='if') { initializer=candidate; break; }
}
if(!initializer) throw new Error('missing actual owned replacement branch');
const value = network.link(field(field(field(initializer,'then'),'value'),'value'));
if(!value||value.metadata().term!=='string value'||value.metadata().definition!=='9') throw new Error('missing actual owned replacement literal');
value.metadata().definition = 'edited';
const restored = LinkNetwork.fromLino(network.toLino());
const javascript = emitTypedProgramLinks(restored,packet.root,'JavaScript');
const rust = emitTypedProgramLinks(restored,packet.root,'Rust');
export function reimportActualRustDefinitions(definitions:string) {
  const packet = createTypedProgramLinks(definitions+'\\nfn main(){}','Rust',
    {rustOwnedLocalStateModel:${JSON.stringify(ownedTextFixture.models.copies)}});
  const network = LinkNetwork.fromLino(packet.network.toLino());
  return {network,root:packet.root,code:emitTypedProgramLinks(network,packet.root,'JavaScript').code};
}
export const contractEvidence = {network:restored,root:packet.root,program:decodeTypedProgramLinks(restored,packet.root),
  originalCode:original.code,code:javascript.code,rustCode:rust.code};
`,
    },
    {
      name: 'reject-unregistered-owned-text-model-version', expectedDiagnostics: ['TS2820'],
      source: `import { createTypedProgramLinks } from ${module};
createTypedProgramLinks(${JSON.stringify(ownedTextFixture.cases[0].source)},'Rust',
  {rustOwnedLocalStateModel:'closed owned text local state 99'});
`,
    },
    {
      name: 'valid-owned-text-selection-type-with-wrong-source-language', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks } from ${module};
export function createWrongLanguagePacket() {
  return createTypedProgramLinks('function keep(value){return value;}','JavaScript',
    {rustOwnedLocalStateModel:${JSON.stringify(ownedTextFixture.models.moves)}});
}
`,
    },
    ...[
      ['private', privateUnitFixture], ['public', publicUnitFixture],
    ].flatMap(([selection, fixture]) => fixture.cases.map(entry => ({
      name: `valid-pure-unit-${selection}-${entry.id}`, expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
import type { TypedProgramLinksEmission } from ${module};
const packet = createTypedProgramLinks(${JSON.stringify(entry.source)},'Rust',
  {rustPureUnitCallModel:${JSON.stringify(fixture.model)},provenance:{consumer:${JSON.stringify(entry.id)}}});
const network = LinkNetwork.fromLino(packet.network.toLino());
const program = decodeTypedProgramLinks(network,packet.root);
const javascript:TypedProgramLinksEmission = emitTypedProgramLinks(network,packet.root,'JavaScript');
const rust:TypedProgramLinksEmission = emitTypedProgramLinks(network,packet.root,'Rust');
export function reimportActualRustDefinitions(definitions:string) {
  const packet = createTypedProgramLinks(definitions+'\\nfn main(){}','Rust',
    {rustPureUnitCallModel:${JSON.stringify(fixture.model)}});
  const network = LinkNetwork.fromLino(packet.network.toLino());
  return {network,root:packet.root,program:decodeTypedProgramLinks(network,packet.root),
    code:emitTypedProgramLinks(network,packet.root,'JavaScript').code};
}
export const contractEvidence = {network,root:packet.root,program,code:javascript.code,rustCode:rust.code};
`,
    }))),
    ...publicUnitFixture.generated.map(entry => ({
      name: `valid-pure-unit-actual-JavaScript-${entry.id}`, expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
const originalSource = ${JSON.stringify(entry.original)};
const original = createTypedProgramLinks(originalSource,'JavaScript');
const originalProgram = decodeTypedProgramLinks(original.network,original.root);
const actualRust = emitTypedProgramLinks(original.network,original.root,'Rust');
const packet = createTypedProgramLinks(${JSON.stringify(entry.source)},'Rust',
  {rustPureUnitCallModel:${JSON.stringify(publicUnitFixture.model)}});
const network = LinkNetwork.fromLino(packet.network.toLino());
const program = decodeTypedProgramLinks(network,packet.root);
const javascript = emitTypedProgramLinks(network,packet.root,'JavaScript');
export function reimportActualRustDefinitions(definitions:string) {
  const packet = createTypedProgramLinks(definitions+'\\nfn main(){}','Rust',
    {rustPureUnitCallModel:${JSON.stringify(publicUnitFixture.model)}});
  const network = LinkNetwork.fromLino(packet.network.toLino());
  return {network,root:packet.root,program:decodeTypedProgramLinks(network,packet.root),
    code:emitTypedProgramLinks(network,packet.root,'JavaScript').code};
}
export const contractEvidence = {network,root:packet.root,program,code:javascript.code,
  originalSource,originalProgram,actualRustCode:actualRust.code};
`,
    })),
    ...[
      ['private', 'closed pure Rust unit calls 99'],
      ['public', 'closed public pure Rust unit calls 99'],
    ].map(([selection, model]) => ({
      name: `reject-unregistered-${selection}-pure-unit-model-version`, expectedDiagnostics: ['TS2820'],
      source: `import { createTypedProgramLinks } from ${module};
createTypedProgramLinks('fn main(){}','Rust',{rustPureUnitCallModel:${JSON.stringify(model)}});
`,
    })),
    ...[
      ['private', privateUnitFixture.model], ['public', publicUnitFixture.model],
    ].map(([selection, model]) => ({
      name: `valid-${selection}-pure-unit-selection-type-with-wrong-source-language`, expectedDiagnostics: [],
      source: `import { createTypedProgramLinks } from ${module};
export function createWrongLanguagePacket() {
  return createTypedProgramLinks('function empty(){}','JavaScript',{rustPureUnitCallModel:${JSON.stringify(model)}});
}
`,
    })),
    ...constFunctionFixture.cases.flatMap(entry => ['implicit', 'explicit'].map(selection => ({
      name: `valid-const-predicate-${selection}-${entry.name}`, expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
const options = {rustConstFunctionModel:${JSON.stringify(constFunctionFixture.model)}${selection === 'explicit' ? ",rustSourceSignatureModel:'Rust source function signature 1'" : ''}} as const;
const packet = createTypedProgramLinks(${JSON.stringify(entry.source)},'Rust',{...options,provenance:{consumer:${JSON.stringify(entry.name)}}});
const network = LinkNetwork.fromLino(packet.network.toLino());
const program = decodeTypedProgramLinks(network,packet.root);
const javascript = emitTypedProgramLinks(network,packet.root,'JavaScript');
const rust = emitTypedProgramLinks(network,packet.root,'Rust');
export function reimportActualRustDefinitions(definitions:string) {
  const packet = createTypedProgramLinks(definitions+'\\nfn main(){}','Rust',options);
  const network = LinkNetwork.fromLino(packet.network.toLino());
  return {network,root:packet.root,program:decodeTypedProgramLinks(network,packet.root),
    code:emitTypedProgramLinks(network,packet.root,'JavaScript').code};
}
export const contractEvidence = {network,root:packet.root,program,code:javascript.code,rustCode:rust.code};
`,
    }))),
    {
      name: 'valid-source-free-const-predicate-operation-edit', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork, LinkId } from ${module};
const options = {rustConstFunctionModel:${JSON.stringify(constFunctionFixture.model)}} as const;
const packet = createTypedProgramLinks(${JSON.stringify(constFunctionFixture.cases[0].source)},'Rust',options);
const network = LinkNetwork.fromLino(packet.network.toLino());
const original = emitTypedProgramLinks(network,packet.root,'JavaScript');
const relations = network.links().filter(link=>link.references().length===3);
const field = (parent:LinkId,label:string):LinkId => {
  const relation = relations.find(link=>link.references()[0].equals(parent)
    &&network.link(link.references()[1])?.metadata().term===label);
  if(!relation) throw new Error('missing actual const predicate field '+label);
  return relation.references()[2];
};
const declaration = relations.find(link=>network.link(link.references()[1])?.metadata().term==='full name'
  &&network.link(link.references()[2])?.metadata().definition===${JSON.stringify(constFunctionFixture.cases[0].name)});
if(!declaration) throw new Error('missing actual const predicate declaration');
const operation = network.link(field(field(declaration.references()[0],'body'),'operation'));
if(!operation||operation.metadata().definition!=='or') throw new Error('the actual editable predicate operation differs');
operation.metadata().definition = 'and';
const restored = LinkNetwork.fromLino(network.toLino());
const javascript = emitTypedProgramLinks(restored,packet.root,'JavaScript');
const rust = emitTypedProgramLinks(restored,packet.root,'Rust');
export function reimportActualRustDefinitions(definitions:string) {
  const packet = createTypedProgramLinks(definitions+'\\nfn main(){}','Rust',options);
  const network = LinkNetwork.fromLino(packet.network.toLino());
  return {network,root:packet.root,program:decodeTypedProgramLinks(network,packet.root),
    code:emitTypedProgramLinks(network,packet.root,'JavaScript').code};
}
export const contractEvidence = {network:restored,root:packet.root,program:decodeTypedProgramLinks(restored,packet.root),
  originalCode:original.code,code:javascript.code,rustCode:rust.code};
`,
    },
    {
      name: 'reject-unregistered-const-predicate-model-version', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks } from ${module};
createTypedProgramLinks(${JSON.stringify(constFunctionFixture.cases[0].source)},'Rust',
  {rustConstFunctionModel:'closed pure Rust const predicates 99'});
`,
    },
    {
      name: 'valid-const-predicate-selector-type-with-wrong-source-language', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks } from ${module};
export function createWrongLanguagePacket() {
  return createTypedProgramLinks('function predicate(value){return value;}','JavaScript',
    {rustConstFunctionModel:${JSON.stringify(constFunctionFixture.model)}});
}
`,
    },
    {
      name: 'reject-unregistered-explicit-const-predicate-signature-model', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks } from ${module};
createTypedProgramLinks(${JSON.stringify(constFunctionFixture.cases[0].source)},'Rust',
  {rustConstFunctionModel:${JSON.stringify(constFunctionFixture.model)},rustSourceSignatureModel:'Rust source function signature 99'});
`,
    },
    ...sourceTextByteFixture.cases.map((entry,index) => ({
      name: `valid-source-text-byte-facades-${index}`, expectedDiagnostics: [],
      source: `import { readSourceTextUnits, readSourceTextByteLength, readSourceTextByte, sliceSourceTextBytes } from ${module};
const text = ${JSON.stringify(String.fromCharCode(...entry.units))};
const units:number[] = readSourceTextUnits(text);
const length:number = readSourceTextByteLength(text);
const firstByte:number|null = readSourceTextByte(text,0);
const whole:string|null = sliceSourceTextBytes(text,0,length);
export function byteAt(offset:number):number|null { return readSourceTextByte(text,offset); }
export function sliceAt(start:number,end:number):string|null { return sliceSourceTextBytes(text,start,end); }
export const contractEvidence = {text,units,length,firstByte,whole};
`,
    })),
    {
      name: 'valid-source-text-byte-runtime-boundaries', expectedDiagnostics: [],
      source: `import { readSourceTextUnits, readSourceTextByteLength, readSourceTextByte, sliceSourceTextBytes } from ${module};
export function units(text:string):number[] { return readSourceTextUnits(text); }
export function length(text:string):number { return readSourceTextByteLength(text); }
export function byte(text:string,offset:number):number|null { return readSourceTextByte(text,offset); }
export function slice(text:string,start:number,end:number):string|null { return sliceSourceTextBytes(text,start,end); }
`,
    },
    ...[
      ['units', 'readSourceTextUnits(1)'], ['length', 'readSourceTextByteLength(1)'],
      ['byte', 'readSourceTextByte(1,0)'], ['slice', 'sliceSourceTextBytes(1,0,1)'],
    ].map(([selection, expression]) => ({
      name: `reject-source-text-byte-nonstring-${selection}`, expectedDiagnostics: ['TS2345'],
      source: `import { readSourceTextUnits, readSourceTextByteLength, readSourceTextByte, sliceSourceTextBytes } from ${module};\n${expression};\n`,
    })),
    ...[
      ['byte', "readSourceTextByte('A','0')"], ['slice', "sliceSourceTextBytes('A',0,'1')"],
    ].map(([selection, expression]) => ({
      name: `reject-source-text-byte-foreign-offset-${selection}`, expectedDiagnostics: ['TS2345'],
      source: `import { readSourceTextByte, sliceSourceTextBytes } from ${module};\nexport const contractEvidence = {value:${expression}};\n`,
    })),
    {
      name: 'reject-source-text-byte-null-as-number', expectedDiagnostics: ['TS2322'],
      source: `import { readSourceTextByte } from ${module};\nconst value:number = readSourceTextByte('',0);\nexport const contractEvidence = {value};\n`,
    },
    {
      name: 'reject-source-text-byte-null-as-string', expectedDiagnostics: ['TS2322'],
      source: `import { sliceSourceTextBytes } from ${module};\nconst value:string = sliceSourceTextBytes('A',1,0);\nexport const contractEvidence = {value};\n`,
    },
    ...runtimeShellFixture.cases.filter(entry => ['empty-program', 'ordered-repeated'].includes(entry.id)).map(entry => ({
      name: `valid-runtime-shell-whole-artifact-${entry.id}`, expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
const options = {rustRuntimeShellModel:${JSON.stringify(runtimeShellFixture.model)}} as const;
const original = createTypedProgramLinks(${JSON.stringify(entry.original)},'JavaScript');
const originalRust = emitTypedProgramLinks(original.network,original.root,'Rust');
const packet = createTypedProgramLinks(originalRust.code,'Rust',{...options,provenance:{consumer:${JSON.stringify(entry.id)}}});
const network = LinkNetwork.fromLino(packet.network.toLino());
const javascript = emitTypedProgramLinks(network,packet.root,'JavaScript');
const rust = emitTypedProgramLinks(network,packet.root,'Rust');
export function reimportWholeRustArtifact(source:string) {
  const packet = createTypedProgramLinks(source,'Rust',options);
  const network = LinkNetwork.fromLino(packet.network.toLino());
  return {network,root:packet.root,program:decodeTypedProgramLinks(network,packet.root),
    javascript:emitTypedProgramLinks(network,packet.root,'JavaScript'),rust:emitTypedProgramLinks(network,packet.root,'Rust')};
}
export const contractEvidence = {network,root:packet.root,program:decodeTypedProgramLinks(network,packet.root),
  javascript,rust,originalRustCode:originalRust.code};
`,
    })),
    {
      name: 'valid-source-free-runtime-shell-resource-edit', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
const packet = createTypedProgramLinks(${JSON.stringify(repeatedRuntime.fullEmittedRust)},'Rust',
  {rustRuntimeShellModel:${JSON.stringify(runtimeShellFixture.model)}});
const network = LinkNetwork.fromLino(packet.network.toLino());
const shift = network.links().find(link=>link.metadata().term==='string value'&&link.metadata().definition==='28');
if(!shift) throw new Error('the actual whole-runtime stack shift token is missing');
shift.metadata().definition = '20';
const restored = LinkNetwork.fromLino(network.toLino());
export const contractEvidence = {network:restored,root:packet.root,program:decodeTypedProgramLinks(restored,packet.root),
  javascript:emitTypedProgramLinks(restored,packet.root,'JavaScript'),rust:emitTypedProgramLinks(restored,packet.root,'Rust')};
`,
    },
    {
      name: 'reject-unregistered-runtime-shell-model-version', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks } from ${module};
createTypedProgramLinks(${JSON.stringify(repeatedRuntime.fullEmittedRust)},'Rust',
  {rustRuntimeShellModel:'closed pure Rust unit runtime shell 99'});
`,
    },
    {
      name: 'valid-runtime-shell-selector-type-with-wrong-source-language', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks } from ${module};
export function createWrongLanguagePacket() {
  return createTypedProgramLinks('function empty(){}','JavaScript',
    {rustRuntimeShellModel:${JSON.stringify(runtimeShellFixture.model)}});
}
`,
    },
    {
      name: 'valid-runtime-shell-selector-type-with-independent-source-profile', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks } from ${module};
export function createIncompatibleSourcePacket() {
  return createTypedProgramLinks(${JSON.stringify(repeatedRuntime.fullEmittedRust)},'Rust',
    {rustRuntimeShellModel:${JSON.stringify(runtimeShellFixture.model)},rustPureUnitCallModel:'closed public pure Rust unit calls 1'});
}
`,
    },
    {
      name: 'valid-runtime-shell-selector-type-with-independent-target-profile', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks } from ${module};
const packet = createTypedProgramLinks(${JSON.stringify(repeatedRuntime.fullEmittedRust)},'Rust',
  {rustRuntimeShellModel:${JSON.stringify(runtimeShellFixture.model)}});
export function emitIncompatibleTargetPacket() {
  return emitTypedProgramLinks(packet.network,packet.root,'Rust',
    {rustSourceHeaderEmissionModel:'primitive borrowed Rust function header 1'});
}
`,
    },
    {
      name: 'valid-runtime-shell-legacy-replay-boundaries', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, emitTypedProgramLinks } from ${module};
const source = ${JSON.stringify(repeatedRuntime.fullEmittedRust)};
const packet = createTypedProgramLinks(source,'Rust',{rustRuntimeShellModel:${JSON.stringify(runtimeShellFixture.model)}});
const javascript = emitTypedProgramLinks(packet.network,packet.root,'JavaScript');
export const replayBoundaryEvidence = {rustSource:source,javaScriptSource:javascript.code};
export function createWithoutSelectedRustModel() { return createTypedProgramLinks(source,'Rust'); }
export function reimportCompleteJavaScriptWithoutGeneratedBindingModel() {
  return createTypedProgramLinks(javascript.code,'JavaScript');
}
`,
    },
    ...ownedTupleCallFixture.cases.map((entry,index) => ({
      name: `valid-owned-tuple-call-whole-program-${index}`, expectedDiagnostics: [],
      source: `import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, LinkNetwork } from ${module};
const packet = createTypedProgramLinks(${JSON.stringify(entry.source)},'Rust',
  {rustOwnedLocalStateModel:${JSON.stringify(ownedTupleCallFixture.model)},provenance:{consumer:${JSON.stringify(entry.name)}}});
const network = LinkNetwork.fromLino(packet.network.toLino());
export function reimportActualEmittedRust(source:string,copies:boolean) {
  return createTypedProgramLinks(source,'Rust',{rustOwnedLocalStateModel:copies?'canonical owned text local copies 1':${JSON.stringify(ownedTupleCallFixture.model)}});
}
export const contractEvidence = {network,root:packet.root,program:decodeTypedProgramLinks(network,packet.root),
  javascript:emitTypedProgramLinks(network,packet.root,'JavaScript'),rust:emitTypedProgramLinks(network,packet.root,'Rust')};
`,
    })),
    {
      name: 'valid-source-free-owned-tuple-call-declaration-retarget', expectedDiagnostics: [],
      source: `${tupleCallGraphPrelude}
const previous = reference.references();
const report = network.applySubstitution(new SubstitutionRule(previous,[previous[0],previous[1],alternate]));
if(report.updated().length!==1) throw new Error('the actual initializer must have exactly one declaration reference');
const target = network.link(field(initializer,'target'));
if(!target) throw new Error('the actual initializer target fact is missing');
target.metadata().definition = 'alternate';
const restored = LinkNetwork.fromLino(network.toLino());
export const contractEvidence = {network:restored,root:packet.root,originalProgram,
  program:decodeTypedProgramLinks(restored,packet.root),javascript:emitTypedProgramLinks(restored,packet.root,'JavaScript'),
  rust:emitTypedProgramLinks(restored,packet.root,'Rust')};
`,
    },
    ...ownedTupleCallFixture.refusals.map((entry,index) => ({
      name: `valid-owned-tuple-call-selector-with-refused-source-${index}`, expectedDiagnostics: [],
      source: `import { createTypedProgramLinks } from ${module};
createTypedProgramLinks(${JSON.stringify(entry.source)},'Rust',{rustOwnedLocalStateModel:${JSON.stringify(ownedTupleCallFixture.model)}});
`,
    })),
    ...[
      ['call-only-retarget', `const previous=reference.references();network.applySubstitution(new SubstitutionRule(previous,[previous[0],previous[1],alternate]));`],
      ['fact-only-retarget', `const target=network.link(field(initializer,'target'));if(!target)throw new Error('missing target');target.metadata().definition='alternate';`],
      ['detached-declaration', `const original=network.link(seed);if(!original)throw new Error('missing declaration');
const cloned=network.insertLink(original.references(),original.metadata());
for(const relation of relations){const refs=relation.references();if(refs[0].equals(seed))network.insertLink([cloned,refs[1],refs[2]],relation.metadata());}
const previous=reference.references();network.applySubstitution(new SubstitutionRule(previous,[previous[0],previous[1],cloned]));`],
      ['string-header-width', `const integerList=network.link(field(field(field(field(seed,'source function signature'),'source signature result'),'type'),'elements'));
if(!integerList)throw new Error('missing actual result tuple');const integer=integerList.references()[1],width=field(integer,'bits');
const old=network.link(width);if(!old)throw new Error('missing actual integer width');
const scalar=network.insertLink([],old.metadata().withTerm('string value').withDefinition('32'));
const relation=relations.find(link=>link.references()[0].equals(integer)&&link.references()[2].equals(width));if(!relation)throw new Error('missing width relation');
const previous=relation.references();network.applySubstitution(new SubstitutionRule(previous,[previous[0],previous[1],scalar]));`],
      ['wrong-tuple-item-type', `const items=network.link(field(field(seed,'body'),'items'));if(!items)throw new Error('missing actual tuple items');
const previous=items.references();network.applySubstitution(new SubstitutionRule(previous,[previous[1],previous[1]]));`],
      ['reversed-field-modes', `const modes=network.link(field(initializer,'field modes'));if(!modes)throw new Error('missing actual ordered field modes');
const previous=modes.references();network.applySubstitution(new SubstitutionRule(previous,[previous[1],previous[0]]));`],
    ].map(([selection,mutation]) => ({
      name: `valid-owned-tuple-call-selector-with-malformed-links-${selection}`, expectedDiagnostics: [],
      source: `${tupleCallGraphPrelude}\n${mutation}\nexport const malformedPacket={network:LinkNetwork.fromLino(network.toLino()),root:packet.root};\n`,
    })),
    {
      name: 'reject-unregistered-owned-tuple-call-model-version', expectedDiagnostics: ['TS2820'],
      source: `import { createTypedProgramLinks } from ${module};
createTypedProgramLinks(${JSON.stringify(retargetTupleCall.source)},'Rust',{rustOwnedLocalStateModel:'closed owned text tuple initialization calls 99'});
`,
    },
    {
      name: 'valid-owned-tuple-call-selector-type-with-wrong-source-language', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks } from ${module};
export function createWrongLanguagePacket(){return createTypedProgramLinks('function empty(){}','JavaScript',
  {rustOwnedLocalStateModel:${JSON.stringify(ownedTupleCallFixture.model)}});}
`,
    },
    {
      name: 'reject-unregistered-owned-tuple-call-source-signature-model', expectedDiagnostics: ['TS2322'],
      source: `import { createTypedProgramLinks } from ${module};
createTypedProgramLinks(${JSON.stringify(retargetTupleCall.source)},'Rust',
  {rustOwnedLocalStateModel:${JSON.stringify(ownedTupleCallFixture.model)},rustSourceSignatureModel:'Rust source function signature 99'});
`,
    },
    {
      name: 'valid-owned-tuple-call-selector-type-with-independent-source-profile', expectedDiagnostics: [],
      source: `import { createTypedProgramLinks } from ${module};
export function createIncompatibleSourcePacket(){return createTypedProgramLinks(${JSON.stringify(retargetTupleCall.source)},'Rust',
  {rustOwnedLocalStateModel:${JSON.stringify(ownedTupleCallFixture.model)},rustPureUnitCallModel:'closed pure Rust unit calls 1'});}
`,
    },
  ];
}

export function checkPublicDeclarationContracts(compiler) {
  if (typeof compiler !== 'string' || !compiler.trim()) {
    throw new TypeError('Pass --compiler PATH to an existing TypeScript executable; this checker never installs dependencies.');
  }
  const version = spawnSync(compiler, ['--version'], { encoding: 'utf8', timeout: 20_000, maxBuffer: 256 * 1024 });
  if (version.error) throw new Error(`Cannot execute the required TypeScript compiler ${compiler}: ${version.error.message}`);
  const compilerVersion = /^Version (\S+)/mu.exec(version.stdout ?? '')?.[1];
  if (version.status !== 0 || !compilerVersion) {
    throw new Error(`The required compiler did not report its TypeScript version: ${version.stdout ?? ''}${version.stderr ?? ''}`);
  }
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-public-declarations-'));
  try {
    return publicDeclarationConsumers().map(({ name, source, expectedDiagnostics }) => {
      const file = join(directory, `${name}.mts`);
      writeFileSync(file, source);
      const result = spawnSync(compiler, ['--noEmit', '--strict', '--pretty', 'false', '--target', 'ES2022',
        '--module', 'NodeNext', '--moduleResolution', 'NodeNext', file],
      { encoding: 'utf8', timeout: 20_000, maxBuffer: 256 * 1024 });
      if (result.error) throw new Error(`Cannot execute the required TypeScript compiler ${compiler}: ${result.error.message}`);
      const diagnostics = `${result.stdout ?? ''}${result.stderr ?? ''}`;
      const codes = [...diagnostics.matchAll(/error (TS\d+):/gu)].map((match) => match[1]);
      // TypeScript 5 exits 2 for diagnostics; the pinned TypeScript 7 exits 1.
      // Both must still produce exactly the declared rejection diagnostics.
      const expectedExit = expectedDiagnostics.length ? [1, 2].includes(result.status) : result.status === 0;
      if (!expectedExit
        || JSON.stringify(codes) !== JSON.stringify(expectedDiagnostics)) {
        throw new Error(`${name}: compiler exit ${result.status}; expected diagnostics ${expectedDiagnostics.join(', ') || 'none'}\n${diagnostics}`);
      }
      return { name, compilerVersion, diagnostics: codes };
    });
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    const args = process.argv.slice(2);
    if (args.length !== 2 || args[0] !== '--compiler') throw new TypeError('Usage: node js/scripts/check-public-declaration-contracts.mjs --compiler PATH');
    const results = checkPublicDeclarationContracts(args[1]);
    console.log(`public declaration contracts: ${results.length} real consumer checks passed (TypeScript ${results[0].compilerVersion})`);
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
