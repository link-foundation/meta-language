// Deferred metadata-authoring controls; do not execute during the full draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync,mkdtempSync,rmSync,symlinkSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readProjectData,writeProjectData} from '../scripts/project-data.mjs';
import {readSourceDefinitionCatalogAuthorCache,readSourceDefinitionCatalogAuthorEntry} from '../scripts/source-definition-catalog-authoring.mjs';
const fixture=readProjectData(new URL('../../parity/fixtures/source-definition-catalog-authoring.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered source-author Node '+process.versions.node+'; '+fixture.versions.Node);
test('only completely matching authored entry metadata permits writing reuse',()=>{
 const bytes=Buffer.from(fixture.source),expected=fixture.preceding;
 assert.deepEqual(readSourceDefinitionCatalogAuthorEntry(fixture.asset,bytes,{previous:expected}),expected);
 for(const row of fixture.invalidCaches)assert.deepEqual(readSourceDefinitionCatalogAuthorEntry(fixture.asset,bytes,{previous:{...expected,root:'2',...row.edit}}),expected,row.name);
 const changed=Buffer.from(fixture.changed),entry=readSourceDefinitionCatalogAuthorEntry(fixture.asset,changed,{previous:expected});
 assert.equal(entry.root,'2');assert.equal(entry.bytes,changed.length);assert.equal(entry.sha256,createHash('sha256').update(changed).digest('hex'));
});
test('complete checks ignore every cached root and retain duplicate-root refusal',()=>{
 const bytes=Buffer.from(fixture.changed),previous={...fixture.preceding,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
 assert.equal(readSourceDefinitionCatalogAuthorEntry(fixture.asset,bytes,{previous,complete:true}).root,'2');
 const duplicate=Buffer.from(fixture.duplicate),matching={...fixture.preceding,bytes:duplicate.length,sha256:createHash('sha256').update(duplicate).digest('hex')};
 assert.throws(()=>readSourceDefinitionCatalogAuthorEntry(fixture.asset,duplicate,{previous:matching,complete:true}),/one actual existing schema root/u);
});
test('default commands retain their original complete paths and cannot select a cached check',()=>{
 const manifest=JSON.parse(readFileSync(new URL('../package.json',import.meta.url),'utf8'));
 assert.equal(manifest.scripts['draft:source-definition-catalog'],'node scripts/draft-source-definition-catalog.mjs --draft');
 assert.equal(manifest.scripts['check:source-definition-catalog'],'node scripts/draft-source-definition-catalog.mjs --check');
 assert.equal(manifest.scripts['draft:source-definition-catalog:incremental'],'node scripts/draft-source-definition-catalog.mjs --draft --reuse-roots');
 assert.throws(()=>execFileSync(process.execPath,[new URL('../scripts/draft-source-definition-catalog.mjs',import.meta.url).pathname,'--check','--reuse-roots'],{stdio:'pipe',timeout:20000}),error=>error.status!==0&&String(error.stderr).includes('use --draft or --check'));
});
test('authored cache files are optional bounded Links snapshots with distinct assets',()=>{
 const directory=mkdtempSync(join(tmpdir(),'meta-language-catalog-author-'));
 try{
  const file=join(directory,'snapshot.lino'),snapshot={schemaVersion:1,model:'complete qualified typed source programs 1',entries:[fixture.preceding]};
  assert.equal(readSourceDefinitionCatalogAuthorCache(file).size,0);assert.equal(readSourceDefinitionCatalogAuthorCache(file,true).size,0);
  writeProjectData(file,snapshot);assert.deepEqual([...readSourceDefinitionCatalogAuthorCache(file,true).values()],[fixture.preceding]);
  writeProjectData(file,{...snapshot,entries:[fixture.preceding,fixture.preceding]});assert.throws(()=>readSourceDefinitionCatalogAuthorCache(file,true),/distinct actual assets/u);
  writeProjectData(file,{...snapshot,schemaVersion:2});assert.throws(()=>readSourceDefinitionCatalogAuthorCache(file,true),/original schema/u);
  writeFileSync(file,'x'.repeat(4*1024*1024+1));assert.equal(readSourceDefinitionCatalogAuthorCache(file,true).size,0);
  const alias=join(directory,'alias.lino');symlinkSync(directory,alias,process.platform==='win32'?'junction':'dir');assert.throws(()=>readSourceDefinitionCatalogAuthorCache(alias,true),/regular file/u);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
