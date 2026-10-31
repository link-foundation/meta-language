// Deferred authoring-boundary controls; never import during full drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {mkdtempSync,mkdirSync,writeFileSync,rmSync,symlinkSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {execFileSync} from 'node:child_process';
import {draftOwnSourceConstructInventory} from '../scripts/draft-own-source-construct-inventory.mjs';
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
function fixture(run){
 const directory=mkdtempSync(join(tmpdir(),'meta-language-own-source-inventory-'));
 const git=args=>execFileSync('git',['-c','gc.auto=0','-c','core.hooksPath=/dev/null','-C',directory,...args],{stdio:'pipe'});
 try{
  mkdirSync(join(directory,'js/src'),{recursive:true});mkdirSync(join(directory,'rust/src'),{recursive:true});
  writeFileSync(join(directory,'js/src/actual.js'),'throw new Error("the inspected source must never execute");\nexport class Actual {}');
  writeFileSync(join(directory,'rust/src/actual.rs'),'compile_error!("the inspected source must never compile");\ntype Alias=bool;');
  writeFileSync(join(directory,'outside.js'),'outside inventory scope');git(['init','--object-format=sha1']);git(['add','--','js/src/actual.js','rust/src/actual.rs','outside.js']);git(['-c','user.name=Fixture author','-c','user.email=fixture@example.invalid','commit','-m','Independent source inventory originals']);
  run(directory,git);
 }finally{rmSync(directory,{recursive:true,force:true});}
}
test('only actual tracked roots contribute and code is treated as source text',()=>fixture(directory=>{
 writeFileSync(join(directory,'js/src/untracked.js'),'class Untracked {}');
 const value=draftOwnSourceConstructInventory(directory);assert.equal(value.files.length,2);assert.deepEqual(value.files.map(row=>row.path),['js/src/actual.js','rust/src/actual.rs']);
 assert.equal(value.schemaVersion,2);assert.equal(value.objectFormat,'sha1');assert.equal(value.gitVersionRange,'>=2.29.0 <3.0.0');assert.match(value.sourceState,/working-tree bytes/u);
 assert.match(value.revision,/^[a-f0-9]{40}$/u);for(const file of value.files){assert.match(file.sha256,/^[a-f0-9]{64}$/u);assert.ok(file.bytes>0);}
 const signal=value.signals.find(row=>row.name==='class declaration');assert.equal(signal.matchingFiles,1);assert.equal(signal.samples[0].path,'js/src/actual.js');assert.equal(signal.samples[0].firstLfLine,2);
 assert.deepEqual(value.files[0].signals.find(row=>row.id===signal.id),{id:signal.id,count:1,firstOffset:67,firstLfLine:2});
}));
test('working source changes keep their own digest and the distinct Git index identity',()=>fixture(directory=>{
 const original=draftOwnSourceConstructInventory(directory),first=original.files[0];
 writeFileSync(join(directory,'js/src/actual.js'),'class Replacement {}\nclass ReplacementTwo {}');
 const changed=draftOwnSourceConstructInventory(directory),next=changed.files[0];
 assert.equal(changed.revision,original.revision);assert.equal(next.indexObject,first.indexObject);assert.notEqual(next.sha256,first.sha256);
 const signal=changed.signals.find(row=>row.name==='class declaration');assert.equal(signal.totalOccurrences,2);assert.equal(next.signals.find(row=>row.id===signal.id).count,2);
}));
test('budgets and root traversal refuse without a partial inventory',()=>fixture(directory=>{
 for(const options of [{byteLimit:1},{fileLimit:1},{sampleLimit:65},{byteLimit:0},{roots:['../outside']},{roots:['/host']},{roots:[':(exclude)js']},{roots:[]}])assert.throws(()=>draftOwnSourceConstructInventory(directory,options));
}));
test('tracked symlinks and non-UTF8 source refuse',()=>fixture((directory,git)=>{
 symlinkSync(join(directory,'outside.js'),join(directory,'js/src/alias.js'));git(['add','--','js/src/alias.js']);assert.throws(()=>draftOwnSourceConstructInventory(directory),/ordinary tracked source index entry/u);
 rmSync(join(directory,'js/src/alias.js'));git(['rm','--cached','--','js/src/alias.js']);writeFileSync(join(directory,'rust/src/invalid.rs'),Buffer.from([255]));git(['add','--','rust/src/invalid.rs']);assert.throws(()=>draftOwnSourceConstructInventory(directory));
}));
