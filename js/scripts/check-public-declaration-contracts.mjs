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
      if (result.status !== (expectedDiagnostics.length ? 2 : 0)
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
