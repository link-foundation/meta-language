import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {LinkMetadata,LinkType} from '../src/primitives.js';

const controls=readProjectData(new URL('../../parity/fixtures/concept-seed-index-controls.lino',import.meta.url));

async function readBoundedConceptSeedNetworks(){
  const {baseline}=controls,base=new URL('../../'+baseline.file,import.meta.url);
  assert.equal(baseline.revision,'8b95e57d696c755dc14705962f783d21f69e89fb');
  assert.equal(baseline.file,'js/src/network.js');
  assert.equal(baseline.bytes,38071);
  assert.equal(baseline.sha256,'75a1991b8d99a947c7e0b72c9b142841c469f0ccefb44a46c1b0e391df971cc9');
  assert.equal(Buffer.byteLength(baseline.source),baseline.bytes);
  assert.equal(createHash('sha256').update(baseline.source).digest('hex'),baseline.sha256);
  assert.equal(controls.records.length,4);
  assert.deepEqual(controls.records.map(row=>row.id),['grammar.rule','grammar.character-class','loop','operation.parse']);
  assert.equal(controls.records.reduce((sum,row)=>sum+row.sourceAliases.length,0),24);
  // Only these declared four complete records are the bounded control input.
  // Both complete network modules execute their actual seeding methods; the
  // shipped-register tests remain unchanged and run over every record in CI.
  const data='export {FORMER_CONCEPT_ID_VOCABULARY,conceptRecord,currentConceptId,insertConceptRecord} from '+JSON.stringify(new URL('../src/concept-records.js',import.meta.url).href)+';\nexport const CONCEPT_RECORDS='+JSON.stringify(controls.records)+';';
  const records='data:text/javascript;base64,'+Buffer.from(data).toString('base64');
  const module=source=>{
    const rewritten=source.replace(/\bfrom (['"])(\.[^'"\n]+)\1/gu,(_,quote,path)=>{
      const address=new URL(path,base).href;
      return 'from '+quote+(path==='./concept-records.js'?records:address)+quote;
    }).replaceAll('import.meta.url',JSON.stringify(base.href));
    return 'data:text/javascript;base64,'+Buffer.from(rewritten).toString('base64');
  };
  const original=await import(module(baseline.source));
  const current=await import(module(readFileSync(base,'utf8')));
  return {original:original.LinkNetwork,current:current.LinkNetwork};
}

function prepareConceptSeedNetwork(Network,duplicates){
  const network=new Network(),record=controls.records[0],alias=record.sourceAliases[0];
  if(duplicates){
    const concept=network.internConcept(record.id,'The original definition.');
    const vocabulary=network.insertTypedPoint(LinkType.Type,'external-identifier:'+alias.source);
    const metadata=LinkMetadata.new().withLinkType(LinkType.Semantic).withTerm(alias.name).withLanguage(alias.source);
    network.insertLinkWithOptionalId(900,[concept,vocabulary],metadata.clone());
    network.insertLinkWithOptionalId(500,[concept,vocabulary],metadata.clone());
    network.insertLink([concept,vocabulary],metadata.withLinkType(LinkType.Dynamic));
    network.insertLink([concept,vocabulary,concept],metadata);
  }
  let calls=0,slots=0;
  const links=network.links.bind(network);
  network.links=()=>{const value=links();calls++;slots+=value.length;return value;};
  return {network,read:()=>({calls,slots}),reset:()=>{calls=0;slots=0;}};
}

test('bulk concept seeding preserves frozen graphs and ordered semantic pair reuse',async()=>{
  const {original,current}=await readBoundedConceptSeedNetworks();
  for(const duplicates of[false,true]){
    const expected=prepareConceptSeedNetwork(original,duplicates),actual=prepareConceptSeedNetwork(current,duplicates);
    for(let run=0;run<2;run++){
      expected.reset();actual.reset();
      assert.deepEqual(actual.network.seedConceptRecords(),expected.network.seedConceptRecords());
      const before=expected.read(),after=actual.read();
      assert.equal(after.calls,1,'one derived scan per bounded seeding call');
      assert.ok(before.calls>after.calls);assert.ok(before.slots>after.slots);
      assert.equal(actual.network.toLino(),expected.network.toLino());
      assert.deepEqual(Object.keys(actual.network),Object.keys(expected.network),'the index adds no persistent network field');
    }
    if(duplicates){
      const alias=controls.records[0].sourceAliases[0];
      for(const prepared of[expected,actual]){
        const network=prepared.network,concept=network.findTerm(controls.records[0].id);
        assert.equal(network.insertConceptAlias(concept,alias.source,alias.name).asU64(),500,'the original link identity order selects the first duplicate');
        network.link(500).setMetadata(network.link(500).metadata().withTerm('changed alias'));
        assert.equal(network.insertConceptAlias(concept,alias.source,alias.name).asU64(),900,'later graph edits are read from actual links');
        network.link(900).setMetadata(network.link(900).metadata().withLanguage('changed vocabulary'));
        assert.notEqual(network.insertConceptAlias(concept,alias.source,alias.name).asU64(),900);
        network.seedConceptRecords();
      }
      assert.equal(actual.network.toLino(),expected.network.toLino());
    }
  }
});

test('bulk seeding releases derived indexes after nested calls and failures',async()=>{
  const {original,current}=await readBoundedConceptSeedNetworks();
  for(const nested of[false,true]){
    const outcomes=[];
    for(const Network of[original,current]){
      const network=new Network(),method=network._insertConceptSyntaxMapping;
      let calls=0,entered=false,report;
      network._insertConceptSyntaxMapping=function(...args){
        calls++;
        if(nested&&!entered){entered=true;report=this.seedConceptRecords();}
        if(!nested&&calls===2)throw new Error('bounded seeding failure');
        return method.apply(this,args);
      };
      if(nested)network.seedConceptRecords();
      else assert.throws(()=>network.seedConceptRecords(),/bounded seeding failure/u);
      delete network._insertConceptSyntaxMapping;
      const record=controls.records[0],alias=record.sourceAliases[0],concept=network.findTerm(record.id);
      const first=network.insertConceptAlias(concept,alias.source,alias.name);
      network.link(first).setMetadata(network.link(first).metadata().withTerm('changed after scope'));
      const fresh=network.insertConceptAlias(concept,alias.source,alias.name);
      assert.ok(!fresh.equals(first),'the completed or failed scope leaves no stale lookup');
      const final=network.seedConceptRecords();
      outcomes.push({report,final,source:network.toLino()});
    }
    assert.deepEqual(outcomes[1],outcomes[0]);
  }
});
