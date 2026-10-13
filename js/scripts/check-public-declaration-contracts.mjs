#!/usr/bin/env node
// Compile bounded public consumers with an explicitly supplied existing compiler.
// This checker never downloads a compiler or installs consumer dependencies.
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const publicModule = fileURLToPath(new URL('../src/index.js', import.meta.url));

export function publicDeclarationConsumers(moduleSpecifier = publicModule) {
  const module = JSON.stringify(moduleSpecifier);
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
