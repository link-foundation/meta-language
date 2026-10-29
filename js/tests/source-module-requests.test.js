// Deferred CST/graph controls. Authors parse this module without importing it.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync,mkdtempSync,mkdirSync,writeFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {execFileSync} from 'node:child_process';
import {runCommandLine} from '../src/cli.js';
import {materializeProjectTranslationInputs} from '../scripts/draft-project-translation.mjs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../scripts/project-data.mjs';
import {analyzeSourceModuleRequests} from '../src/source-module-requests.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/source-module-request-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/source-module-requests.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.node);
for(const row of fixture.grammarProfiles)assert.equal(createHash('sha256').update(readFileSync(new URL('../../'+row.path,import.meta.url))).digest('hex'),row.sha256,'uncovered native grammar revision '+row.path);
for(const row of fixture.roles)test('module request owner '+row.term,()=>assert.equal(rules.acceptSourceModuleRequestTerm(row.term),row.expected));
for(const row of fixture.scopes)test('module request type context '+row.term,()=>assert.equal(rules.acceptSourceModuleRequestTypeScope(row.term),row.expected));
for(const row of fixture.literals)test('module request literal '+row.term,()=>assert.equal(rules.acceptSourceModuleRequestLiteral(row.term),row.expected));
for(const row of fixture.callees)test('module request callee '+row.term,()=>assert.equal(rules.acceptSourceModuleRequestImportCallee(row.term),row.expected));
for(const row of fixture.kinds)test('module request kind '+row.code,()=>assert.equal(rules.readSourceModuleRequestKind(row.code),row.expected));
for(const [index,row]of fixture.codes.entries())test('module request phase '+index,()=>assert.equal(rules.readSourceModuleRequestCode(row.language,row.term,row.second,row.third,row.context),row.expected));
for(const [index,row]of fixture.grammars.entries())test('module request grammar '+index,()=>assert.equal(rules.readSourceModuleRequestGrammar(row.language,row.tsx),row.expected));
for(const [index,row]of fixture.commands.entries())test('module request arguments '+index,()=>assert.deepEqual(rules.planSourceModuleRequestCommand(row.args),row.expected));
for(const row of fixture.cases)test(row.name,()=>{
 const packet=analyzeSourceModuleRequests(row.source,row.language,{tsx:row.tsx});assert.equal(packet.coordinateUnit,'UTF-8 bytes');
 assert.deepEqual(packet.requests.map(({linkId,syntaxLinkId,...request})=>request),row.expected);
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  assert.equal(network.reconstructText(),row.source);
  assert.deepEqual(network.link(packet.root).references().map(id=>String(id.asU64())),packet.requests.map(request=>request.linkId));
  for(const request of packet.requests){const relation=network.link(request.linkId),references=relation.references();assert.equal(references.length,3);assert.equal(network.link(references[0]).metadata().term,request.kind);assert.equal(String(references[1].asU64()),request.syntaxLinkId);const span=network.link(references[1]).metadata().span.byteRange;assert.deepEqual([span.start,span.end],[request.start,request.end]);const value=network.link(references[2]);if(request.specifierUnits===null)assert.equal(value.metadata().term,'computed module specifier');else assert.deepEqual(value.references().map(id=>Number(network.link(id).metadata().definition)),request.specifierUnits);}
 }
});
test('resource and syntax profile refusals remain explicit',()=>{
 assert.throws(()=>analyzeSourceModuleRequests('','Rust'),/expected scalar JavaScript or TypeScript source/u);
 assert.throws(()=>analyzeSourceModuleRequests('','JavaScript',{tsx:true}),/invalid syntax profile/u);
 assert.throws(()=>analyzeSourceModuleRequests('import "a";','JavaScript',{byteLimit:1}),/source byte limit exceeded/u);
 assert.throws(()=>analyzeSourceModuleRequests('import "a";','JavaScript',{workLimit:1}),/request work limit exceeded/u);
 assert.throws(()=>analyzeSourceModuleRequests('','JavaScript',{workLimit:0}),/invalid resource limit/u);
});

// This temporary repository is created only when CI executes the deferred test.
test('tracked project discovery keeps reexports JSX TSX and unknown edges',()=>{
 const root=mkdtempSync(join(tmpdir(),'meta-language-module-requests-'));
 try{
  mkdirSync(join(root,'src'));
  const files={
   'src/a.js':'export const value=7;',
   'src/b.js':'export {value} from "./a.js";',
   'src/view.jsx':'export const view=<main>{import("./b.js")}</main>;',
   'src/view.tsx':'export const view=<main>{import("./b.js")}</main>;',
   'src/types.ts':'export type {Value} from "./a.js";',
   'src/dynamic.js':'export function load(name){return import("./"+name);}'
  };
  for(const [name,source]of Object.entries(files))writeFileSync(join(root,name),source);
  const git=args=>execFileSync('git',['-c','core.hooksPath=/dev/null','-c','commit.gpgSign=false','-c','user.name=Module Request Fixture','-c','user.email=module-request@example.invalid',...args],{cwd:root,stdio:'ignore'});
  git(['init','--quiet']);git(['add','src']);git(['commit','--quiet','-m','independent module request fixture']);
  const inventory=materializeProjectTranslationInputs(root),inputs=new Map(inventory.inputs.map(input=>[input.identifier,input]));
  assert.deepEqual([...inputs.keys()].sort(),Object.keys(files).sort());
  for(const name of ['src/b.js','src/view.jsx','src/view.tsx','src/types.ts'])assert.equal(inputs.get(name).imports[name.endsWith('b.js')||name.endsWith('types.ts')?'./a.js':'./b.js'],name.endsWith('b.js')||name.endsWith('types.ts')?'src/a.js':'src/b.js');
  assert.equal(inventory.unresolved.length,1);assert.equal(inventory.unresolved[0].identifier,'src/dynamic.js');assert.equal(inventory.unresolved[0].kind,'dynamic module import');assert.equal(inventory.unresolved[0].reason,'computed module specifier requires a source interpretation');
  assert.equal(inventory.files.find(file=>file.identifier==='src/types.ts').moduleRequests[0].kind,'type module export');
  assert.deepEqual(inputs.get('src/dynamic.js').sourceIssues,['computed module specifier requires a source interpretation']);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test('public CLI emits complete concrete Links text or binary',()=>{
 const source='export {value} from "./a.js";';
 for(const binary of [false,true]){const result=runCommandLine(['module-requests','source.js',...(binary?['--binary']:[])],{readFile:path=>{assert.equal(path,'source.js');return source;}});assert.equal(result.exitCode,0);assert.equal(result.stderr,'');const network=binary?decodeLinkNetworkBinary(result.stdout):LinkNetwork.fromLino(result.stdout);assert.equal(network.reconstructText(),source);assert.equal(network.links().filter(link=>link.metadata().term==='source module request').length,1);}
 assert.equal(runCommandLine(['module-requests','--help']).exitCode,0);assert.equal(runCommandLine(['module-requests']).exitCode,2);
});
