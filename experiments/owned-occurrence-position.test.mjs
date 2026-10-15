import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {acceptProgramOrderedPosition} from '../js/src/program-numeric-metadata-rules.js';
import {acceptTupleIndex} from '../js/src/translation/primitive-value-rules.js';
import {ownedOccurrencePositionArchive,ownedOccurrencePositionFixture} from './generate-owned-occurrence-position-fixture.mjs';
import {decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks} from '../js/src/typed-program-links.js';
import {LinkNetwork} from '../js/src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../js/src/link-network-binary.js';
const fixture=readProjectData(new URL('./owned-occurrence-position-fixture.lino',import.meta.url));
function observe(code,entry){const context={};vm.runInNewContext(code,context,{timeout:1000});for(const[text,count,flag,expected]of entry.observations)assert.equal(context[entry.name](text,BigInt(count),flag),expected,entry.id);}
test('ordered path positions reuse actual tuple index decisions and retain either zero sign',()=>{
 for(const index of[-0,0,1,2,-1,0.5,NaN,Infinity,Number.MAX_SAFE_INTEGER,Number.MAX_SAFE_INTEGER+1])for(const length of[0,1,2,3,Number.MAX_SAFE_INTEGER])assert.equal(acceptProgramOrderedPosition(index,length),acceptTupleIndex(index,length));
 assert.equal(acceptProgramOrderedPosition(-0,1),true);
});
test('all five source archives retain actual numeric occurrence aliases through text and binary reload',()=>{
 assert.deepEqual(ownedOccurrencePositionFixture(),fixture);let count=0;
 for(const entry of fixture.cases){
  const baseline=decodeTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root);
  for(const edit of entry.aliases){
   const packet=ownedOccurrencePositionArchive(entry,edit),binary=decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network));
   for(const network of[packet.network,binary]){
    const program=decodeTypedProgramLinks(network,packet.root),expected=structuredClone(baseline);
    expected.items[0].sourceLocalState.reads[entry.readIndex].path[entry.pathPartIndex]=Number(edit.spelling);assert.deepEqual(program,expected,edit.id);
    const actual=program.items[0].sourceLocalState.reads[entry.readIndex].path[entry.pathPartIndex];assert.equal(Object.is(actual,-0),Object.is(Number(edit.spelling),-0));
    const rebuilt=encodeTypedProgramLinks(program);assert.deepEqual(decodeTypedProgramLinks(LinkNetwork.fromLino(rebuilt.network.toLino()),rebuilt.root),expected);
    for(const target of['JavaScript','Rust'])assert.equal(emitTypedProgramLinks(network,packet.root,target).code,emitTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root,target).code);
    observe(emitTypedProgramLinks(network,packet.root,'JavaScript').code,entry);
   }
   count++;
  }
 }
 assert.equal(count,25);
});
test('wrong, nonintegral, unsafe and string occurrence positions retain their exact refusal boundary',()=>{
 for(const entry of fixture.cases)for(const edit of entry.invalid){const packet=ownedOccurrencePositionArchive(entry,edit);for(const network of[packet.network,decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){assert.throws(()=>decodeTypedProgramLinks(network,packet.root),undefined,edit.id);for(const target of['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(network,packet.root,target),undefined,edit.id+target);}}
});
