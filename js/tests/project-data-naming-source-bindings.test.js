import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {NAMING_SOURCE_BINDING_STORAGE as specification,namingSourceBindingLiteralChunks,verifyNamingSourceBindingData,readNamingSourceBindings} from '../scripts/naming-source-bindings.mjs';
import {generateNamingSourceBindingData} from '../scripts/generate-naming-source-bindings.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {structuredDataToLino,encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const root=new URL('../../',import.meta.url);

test('naming Links storage reconstructs the complete original committed source and preserves every ordered binding',()=>{
  const checked=generateNamingSourceBindingData({check:true});
  assert.ok(checked.rows>=3727);assert.ok(checked.source.bytes<6*1024*1024);
  const rows=readNamingSourceBindings(),migration=readProjectData(new URL(specification.migration,root));
  const proof=verifyNamingSourceBindingData(rows,migration,readFileSync(new URL(specification.module,root)));
  assert.equal(proof.baselineRows,3727);assert.equal(proof.original.bytes,54076530);
  assert.equal(migration.certificate.revision,'5a57b666fb54b151249896d90f1e63d7a8866fcf');
  const appended={id:'independent continuation input',source:'separate source',name:'separate operation'};
  assert.equal(verifyNamingSourceBindingData([...rows,appended],migration,readFileSync(new URL(specification.module,root))).rows,rows.length+1);
  const first=rows[0].source;rows[0].source='caller mutation';rows.reverse();
  assert.equal(readNamingSourceBindings()[0].source,first,'caller data cannot change the verified private cache');
});

test('naming data preserves scalar code units object ownership and exact source literal spelling through both archive formats',()=>{
  const first={sourceContext:'line\u2028separator\ud800',sourceRange:{start:0,end:2}},second={...first,sourceRange:{start:0,end:2}};
  const rows=[first,second],literal=Buffer.concat([...namingSourceBindingLiteralChunks(rows)]).toString();
  assert.equal(literal,JSON.stringify(rows,null,2));
  const encoded=encodeStructuredDataLinks(rows),network=LinkNetwork.fromLino(structuredDataToLino(rows));
  for(const graph of [network,decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]){
    const restored=decodeStructuredDataLinks(graph,encoded.root);assert.deepEqual(restored,rows);
    assert.notEqual(restored[0],restored[1]);assert.notEqual(restored[0].sourceRange,restored[1].sourceRange);
    assert.equal(Buffer.concat([...namingSourceBindingLiteralChunks(restored)]).toString(),literal);
  }
});

test('naming storage refuses missing reordered or changed original bindings changed modules and forged original membership',()=>{
  const rows=readNamingSourceBindings(),migration=readProjectData(new URL(specification.migration,root)),source=readFileSync(new URL(specification.module,root));
  assert.throws(()=>verifyNamingSourceBindingData(rows.slice(0,specification.baselineRows-1),migration,source));
  const oversized=structuredClone(migration);oversized.original.bytes=2**40;
  assert.throws(()=>verifyNamingSourceBindingData(rows,oversized,source),/bounded complete original source/u);
  assert.throws(()=>verifyNamingSourceBindingData(rows,migration,Buffer.concat([source,Buffer.from('\n// unknown edit\n')])));
  const changed=[...rows];[changed[0],changed[1]]=[changed[1],changed[0]];
  assert.throws(()=>verifyNamingSourceBindingData(changed,migration,source));
  changed[0]={...rows[0],sourceText:rows[0].sourceText+' changed'};changed[1]=rows[1];
  assert.throws(()=>verifyNamingSourceBindingData(changed,migration,source));
  const forged=structuredClone(migration);forged.certificate.path='js/scripts/another-source.mjs';
  assert.throws(()=>verifyNamingSourceBindingData(rows,forged,source));
});
