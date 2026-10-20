import assert from 'node:assert/strict';
import {readFileSync,existsSync,mkdtempSync,rmSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import vm from 'node:vm';
import test from 'node:test';
import {generateConcreteProgramNativeInputs,generateConcreteProgramNativeTemplate,applyNativeConsumerChanges} from '../scripts/generate-concrete-program-native-inputs.mjs';
import {generateNativeGraphSourceHistory} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-native-graph-source-history.mjs';
import {restoreNativeGraphSourceHistory} from '../scripts/native-graph-source-history.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {LinkNetwork} from '../src/network.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkId} from '../src/primitives.js';
const read=file=>readFileSync(new URL('../../'+file,import.meta.url),'utf8');

test('five shared native graph inputs retain complete concrete fields and independent original and edited results',async()=>{
  const manifest=await generateConcreteProgramNativeInputs({check:true});
  assert.deepEqual(manifest.cases.map(row=>row.language),['JavaScript','Rust','TypeScript','JavaScript','Rust']);
  assert.deepEqual(manifest.cases.map(row=>row.variant),['original checked program','original checked program',
    'original checked program','edited checked program','edited checked program']);
  for(const row of manifest.cases){
    const source=read(row.path),network=LinkNetwork.fromLino(source),root=LinkId.fromU64(Number(row.root));
    assert.equal(network.toLino(),source);
    assert.equal(network.fieldRelations().filter(field=>field.label==='concrete syntax').length,row.language==='TypeScript'?2:1);
    const lines=[];
    vm.runInNewContext(emitTypedProgramLinks(network,root,'JavaScript').code,
      {console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});
    assert.deepEqual(lines,row.expectedLines);
    assert.match(emitTypedProgramLinks(network,root,'Rust').code,/fn main/u);
  }
  assert.match(read('rust/tests/unit/mod.rs'),/^mod translation_concrete_program_graphs;$/mu);
  const rust=read('rust/tests/unit/translation_concrete_program_graphs.rs');
  assert.deepEqual([...rust.matchAll(/check_concrete_program_graph\(([0-9]+)\);/gu)].map(([,value])=>Number(value)),[0,1,2,3,4]);
});

test('generated native consumer bodies invert to their complete certified original templates',()=>{
  const template=generateConcreteProgramNativeTemplate({check:true});
  const originalRust=template.sources.find(row=>row.file.endsWith('.rs')).sourceText;
  const rust=read('rust/tests/unit/translation_concrete_program_graphs.rs'),marker='\n#[test]\n';
  const invert=changes=>changes.toReversed().map(row=>({before:row.after,after:row.before,occurrences:row.occurrences}));
  const helper=applyNativeConsumerChanges(rust.slice(0,rust.indexOf(marker)),invert(template.rustChanges));
  assert.equal(helper+originalRust.slice(originalRust.indexOf(marker)),originalRust);
  const javascript=read('experiments/concrete-program-graphs/run-rust-emissions.mjs');
  assert.equal(applyNativeConsumerChanges(javascript,invert(template.javascriptChanges)),template.sources.find(row=>row.file.endsWith('.mjs')).sourceText);
  assert.throws(()=>applyNativeConsumerChanges('unknown source',template.javascriptChanges),assert.AssertionError);
});

test('native graph registration preserves every original source guard and exposes unknown source edits',()=>{
  generateNativeGraphSourceHistory({check:true});
  const history=readProjectData(new URL('../../parity/self-translation/concrete-program-native-source-sources-history.lino',import.meta.url));
  assert.equal(history.sources.length,4);
  for(const row of history.sources){
    const actual=read(row.file);assert.equal(Buffer.byteLength(actual),row.current.bytes);
    assert.equal(createHash('sha256').update(actual).digest('hex'),row.current.sha256);
    assert.equal(restoreNativeGraphSourceHistory(row.file,actual),row.sourceText);
    assert.equal(restoreNativeGraphSourceHistory(row.file,row.sourceText),row.sourceText);
    assert.throws(()=>restoreNativeGraphSourceHistory(row.file,actual+'\n# unknown edit\n'),assert.AssertionError);
  }
  assert.equal(restoreNativeGraphSourceHistory('unknown/source.js','visible source'),'visible source');
});

test('the generated native transport refuses local execution before resolving a compiler or creating outputs',(t)=>{
  const directory=mkdtempSync(join(tmpdir(),'meta-language-concrete-native-guard-'));
  t.after(()=>rmSync(directory,{recursive:true}));
  const output=new URL('../../issue-195-artifacts/typescript-typed-graph-native/concrete-program-graphs',import.meta.url);
  const existed=existsSync(output);
  const result=spawnSync(process.execPath,[new URL('../../experiments/concrete-program-graphs/run-rust-emissions.mjs',import.meta.url).pathname,
    '--case','0','--results-dir','issue-195-results'],{cwd:directory,env:{...process.env,CI:'false',GITHUB_ACTIONS:'false',PATH:''},encoding:'utf8'});
  assert.equal(result.status,1);assert.match(result.stderr,/GitHub CI-only/u);
  assert.equal(existsSync(join(directory,'issue-195-results')),false);assert.equal(existsSync(output),existed);
});
