import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {spawnSync} from 'node:child_process';
import {mkdtempSync,rmSync,existsSync} from 'node:fs';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import {createTypeScriptSemanticGraph} from '../../experiments/typescript-typed-graph/projection.mjs';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {TYPED_PROGRAM_LINKS_SCHEMA,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {compileGrammar} from '../src/grammar.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {nativeGrammarText} from '../src/native-grammar-parser.js';
import {generateTypeScriptTypedGraphFixtures,generateTypeScriptTypedGraphArchives} from '../scripts/generate-typescript-typed-graph-fixtures.mjs';

const schema=TYPED_PROGRAM_LINKS_SCHEMA;
const labelOf=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
const fieldOf=label=>Object.entries(schema.fields).find(([,value])=>value===label)?.[0]
  ??label.replace(/ ([a-z])/gu,(_,letter)=>letter.toUpperCase());

// Test-only generic data inspection. No source string, host pointer or side
// graph is available after reload. Semantic checking uses the production codec.
function originData(network,root) {
  const fields=network.fieldRelations();
  const origin=fields.find(row=>row.parent.equals(root)&&row.label==='provenance');
  assert.ok(origin);
  function value(id) {
    const link=network.link(id),metadata=link.metadata(),term=metadata.term;
    if(term===schema.containers.array)return link.references().map(value);
    if(term===schema.containers.record){
      const entries=fields.filter(row=>row.parent.equals(id)).map(row=>[fieldOf(row.label),value(row.child)]);
      assert.equal(new Set(entries.map(([key])=>key)).size,entries.length);
      return Object.fromEntries(entries);
    }
    if(term===schema.scalars.string)return metadata.definition;
    if(term===schema.scalars.number)return Number(metadata.definition);
    if(term===schema.scalars.boolean)return metadata.definition==='true';
    if(term===schema.scalars.null)return null;
    throw new Error('unexpected concrete provenance scalar '+term);
  }
  return value(origin.child);
}

function concreteBytes(tree) {
  if(tree.type==='missing')return Buffer.alloc(0);
  if(tree.type==='token'||tree.type==='error')return tree.text===null?Buffer.from(tree.hex,'hex'):Buffer.from(tree.text);
  return Buffer.concat((tree.children??[tree.root]).map(concreteBytes));
}

function printed(code) {
  const lines=[];
  vm.runInNewContext(code,{console:{log:value=>lines.push(String(value))},process:{argv:[]}}, {timeout:1000});
  return lines;
}

test('TypeScript typed graphs retain complete original and lowered CSTs through text and binary Links reload',async()=>{
  const fixture=generateTypeScriptTypedGraphFixtures({check:true});
  const nativeInputs=await generateTypeScriptTypedGraphArchives({check:true});
  assert.equal(nativeInputs.cases.length,4);
  const parser=compileGrammar(parseGrammarLinks(nativeGrammarText('native-typescript')));
  for(const {source,expectedLines:expected}of fixture.cases){
    const packet=createTypeScriptSemanticGraph(source,{utf16Strings:true,provenance:{purpose:'independent source retention'}});
    for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),
      decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
      const program=decodeTypedProgramLinks(network,packet.root),origin=originData(network,packet.root);
      assert.equal(program.sourceLanguage,'JavaScript','the semantic frontend language is explicitly the lowered language');
      assert.equal(origin.sourceProfile.language,'TypeScript');
      assert.equal(origin.sourceProfile.model,'typescript-portable-typing-v1');
      assert.deepEqual(origin.userProvenance,{purpose:'independent source retention'});
      assert.deepEqual(origin.sourceProfile.concreteSyntax.tree,parser.parseTree(source).tree);
      assert.ok(concreteBytes(origin.sourceProfile.concreteSyntax.tree).equals(Buffer.from(source)));
      const lowered=origin.sourceProfile.lowering;
      assert.equal(lowered.language,'JavaScript');
      assert.ok(lowered.erasures.length>0);assert.ok(lowered.annotations.length>0);
      const loweredSource=concreteBytes(lowered.concreteSyntax.tree).toString();
      assert.deepEqual(printed(loweredSource),expected);
      assert.deepEqual(printed(emitTypedProgramLinks(network,packet.root,'JavaScript').code),expected);
      const rust=emitTypedProgramLinks(network,packet.root,'Rust');
      assert.equal(rust.language,'Rust');assert.match(rust.code,/fn main/u);
    }
  }
});

test('editing the checked program changes emitted behavior while its TypeScript origin remains exact',()=>{
  const {edit}=generateTypeScriptTypedGraphFixtures({check:true}),{source}=edit;
  const packet=createTypeScriptSemanticGraph(source),network=LinkNetwork.fromLino(packet.network.toLino());
  const operation=network.fieldRelations().find(row=>row.label===labelOf('op')&&network.link(row.child).metadata().definition===edit.before);
  assert.ok(operation);const link=network.link(operation.child);link.setMetadata(link.metadata().withDefinition(edit.after));
  assert.deepEqual(printed(emitTypedProgramLinks(network,packet.root,'JavaScript').code),edit.expectedLines);
  assert.ok(concreteBytes(originData(network,packet.root).sourceProfile.concreteSyntax.tree).equals(Buffer.from(source)));
});

test('unsupported runtime constructs contradictory annotations and non-scalar source never become semantic graphs',()=>{
  const fixture=generateTypeScriptTypedGraphFixtures({check:true});
  for(const source of fixture.refusals)assert.throws(()=>createTypeScriptSemanticGraph(source));
  const {prefix,units,suffix}=fixture.nonScalarSource;
  assert.throws(()=>createTypeScriptSemanticGraph(prefix+String.fromCharCode(...units)+suffix),/Unicode scalar/u);
});

test('native TypeScript graph execution refuses outside CI before creating output or resolving a compiler',(t)=>{
  const directory=mkdtempSync(join(tmpdir(),'meta-language-typescript-native-guard-'));
  t.after(()=>rmSync(directory,{recursive:true}));
  const outputs=['issue-195-results/artifacts/typescript-typed-graph-native','issue-195-artifacts/typescript-typed-graph-native']
    .map(path=>new URL('../../'+path,import.meta.url));
  const originalOutput=outputs.map(existsSync);
  // No compiler or Git executable is on PATH even if this guard regresses.
  for(const [file,args]of [
    ['run-native.mjs',['--results-dir','issue-195-results']],
    ['run-rust-emissions.mjs',['--case','0','--results-dir','issue-195-results']],
  ]){
    const result=spawnSync(process.execPath,[new URL('../../experiments/typescript-typed-graph/'+file,import.meta.url).pathname,...args],
      {cwd:directory,env:{...process.env,CI:'false',GITHUB_ACTIONS:'false',PATH:''},encoding:'utf8'});
    assert.equal(result.status,1);assert.match(result.stderr,/GitHub CI-only/u);
  }
  assert.equal(existsSync(join(directory,'issue-195-results')),false);
  assert.deepEqual(outputs.map(existsSync),originalOutput);
});
