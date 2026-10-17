import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {bulkLanguageRow,missingFeatures,renderMatrix} from '../scripts/run-grammar-bulk-pipeline.mjs';

// Tiny injected sources exercise the real importer, compiler and report path.
// They establish component scheduling, not support for the named languages.
test('bulk audits preserve every Markdown, TSX and repeated catalog grammar occurrence without merging component roots',async()=>{
  const calls=[],snapshots=[],sources=new Map([
    ['markdown',JSON.stringify({name:'block_control',rules:{document:{type:'STRING',value:'x'}}})],
    ['markdown_inline',JSON.stringify({name:'inline_control',rules:{inline:{type:'STRING',value:'y'}}})],
    ['typescript',JSON.stringify({name:'typescript_control',rules:{program:{type:'STRING',value:'x'}}})],
    ['tsx',JSON.stringify({name:'tsx_control',rules:{jsx:{type:'STRING',value:'x'}}})],
  ]);
  const entries=[
    {name:'Markdown',grammars:['markdown','markdown_inline'],source:'x'},
    {name:'TypeScript',grammars:['typescript','tsx'],source:'x'},
    {name:'Repeated control',grammars:['markdown','markdown','missing'],source:'x'},
  ];
  const options={
    async loadTreeSitterSource(id,position,entry){
      calls.push([entry.name,position,id]);
      if(!sources.has(id))throw new Error(`synthetic missing source ${id}`);
      return{origin:`synthetic component control ${id}`,text:sources.get(id)};
    },
    async loadGrammarsV4(){return null;},loadAdditionalSources(){return[];},
  };
  const rows=[];
  for(const entry of entries)rows.push(await bulkLanguageRow(entry,row=>snapshots.push(structuredClone(row)),options));
  assert.deepEqual(calls,entries.flatMap(entry=>entry.grammars.map((id,position)=>[entry.name,position,id])));
  assert.equal(calls.length,7,'repeated IDs are distinct catalog occurrences, not deduplicated work');
  for(let index=0;index<entries.length;index++){
    const entry=entries[index],row=rows[index];
    assert.equal(row.language,entry.name);assert.deepEqual(row.grammars,entry.grammars);
    assert.deepEqual(row.treeSitterComponents.map(({id,position})=>[id,position]),entry.grammars.map((id,position)=>[id,position]));
    assert.equal(row.treeSitterComponents[0].outcome,row.treeSitter,'the original primary row object is retained');
    assert.equal(row.treeSitter.stage,'done');assert.equal(row.treeSitter.sample,'accepted');
    assert.deepEqual(row.merge.sources,['tree-sitter'],'component roots do not enter an equivalence-source merge');
    assert.deepEqual(row.conceptSources.map(({id})=>id),[`${entry.name.replace(/[^A-Za-z0-9_-]/gu,'_')}--tree-sitter`]);
    assert.equal(row.componentConceptSources.length,index===2?0:1,'repeated identical pinned source definitions retain separate occurrence audits without duplicate global source identities');
    for(const source of row.componentConceptSources)assert.ok(source.grammarLinks.includes('(rule '));
    for(const component of row.treeSitterComponents){
      assert.equal(component.includedInPrimaryMerge,component.position===0);
      assert.equal(component.sampleSha256,createHash('sha256').update(entry.source).digest('hex'));
      assert.equal(component.sampleBytes,Buffer.byteLength(entry.source));
      if(!sources.has(component.id)){
        assert.equal(component.outcome.stage,'source');
        assert.equal(component.outcome.error,'synthetic missing source missing');
      }else{
        assert.equal(component.outcome.origin,`synthetic component control ${component.id}`);
        assert.equal(component.outcome.sha256,createHash('sha256').update(sources.get(component.id)).digest('hex'));
        assert.equal(component.outcome.stage,'done');assert.equal(component.outcome.compiled,true);
        assert.deepEqual(component.outcome.unsupported,[]);
        assert.ok(component.outcome.compileMs>=0);assert.ok(component.outcome.parseMs>=0);
      }
    }
    row.missing=missingFeatures(row);
  }
  const inline=rows[0].treeSitterComponents[1];
  assert.equal(rows[2].treeSitterComponents[1].sourceOrigin,rows[2].treeSitterComponents[0].sourceOrigin);
  assert.equal(inline.sampleRole,'catalog-document-probe-without-composition');
  assert.equal(inline.outcome.sample,'rejected');
  assert.match(inline.outcome.rejection,/^syntax at /u);
  assert.ok(rows[0].missing.some(({stage})=>stage==='tree-sitter component 1:markdown_inline sample'));
  assert.ok(rows[2].missing.some(({stage,feature})=>stage==='tree-sitter component 2:missing source'&&feature==='synthetic missing source missing'));
  assert.ok(snapshots.some(row=>row.treeSitterComponents[1]?.outcome.stage==='compile'));
  const summary={treeSitterImported:3,treeSitterCompiled:3,treeSitterAccepted:3,rowsMatch:0,grammarsV4Imported:0,grammarsV4Mapped:0,grammarsV4Compiled:0,grammarsV4Accepted:0,merged:3,mergedFromSeveral:0,sharingRules:0,sharedRules:0};
  const matrix=renderMatrix({languages:rows,summary,grammarsV4Revision:'synthetic',grammarsV4Notes:{}});
  assert.match(matrix,/\| Markdown \| 1 \| 0 \| yes \| accepted \|/u,'the existing first-source summary remains in place');
  assert.match(matrix,/## Catalog grammar component audits/u);
  assert.match(matrix,/\| Markdown \| 1 \| markdown_inline \|/u);
  assert.match(matrix,/\| TypeScript \| 1 \| tsx \|/u);
  assert.match(matrix,/\| Repeated control \| 2 \| missing \|/u);
});

test('unfinished component audits remain visible without attributing a timeout to an unstarted component',()=>{
  const primary={stage:'done',compiled:true,sample:'accepted',unsupported:[]};
  const row={treeSitter:primary,treeSitterComponents:[
    {id:'host',position:0,outcome:primary},
    {id:'inline',position:1,outcome:{stage:'import',timedOut:true}},
    {id:'later',position:2,outcome:{stage:'pending'}},
  ]};
  assert.deepEqual(missingFeatures(row),[
    {stage:'tree-sitter component 1:inline import',feature:'timed out'},
    {stage:'tree-sitter component 2:later source',feature:'component audit did not start'},
  ]);
});
