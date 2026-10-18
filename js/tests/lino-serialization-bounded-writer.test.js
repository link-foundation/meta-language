import assert from 'node:assert/strict';
import test from 'node:test';
import {execFileSync} from 'node:child_process';
import {LinkId} from '../src/primitives.js';
import {encodeNetworkLino as candidate} from '../src/lino-serialization-write.js';
import {encodeLinkStatement} from '../src/lino-serialization-write.js';

// Exact preceding algorithm provides an independent whole-output comparison.
function previousNetworkWriter(links, registeredIds) {
  let output = '';
  for (const link of links) {
    const id = link.id().asU64();
    output += `${encodeLinkStatement(id, link.references(), link.metadata(), registeredIds.has(id))}\n`;
  }
  return output;
}

function input(identities,trace){
 const links=identities.map((id,index)=>({
  id(){trace.push(['identity',id]);return LinkId.from(id);},
  references(){trace.push(['references',id]);return [id,1].map(LinkId.from);},
  metadata(){trace.push(['metadata',id]);return {
   linkType:index%61===0?'custom-\ud800-λ':'semantic',named:index%2===0,
   term:' opaque α😀\n% (quoted) \ud800 ',definition:index%3===0?'':'payload '+index,
   language:'language with spaces',span:{byteRange:{start:index,end:index+1},start:{row:1,column:index},end:{row:1,column:index+1}},
   flags:{isError:index%2===0,hasError:index%3===0,isMissing:index%5===0,isExtra:index%7===0},
  };},
 }));
 return{links,registered:{has(id){trace.push(['registered',id]);return id%3===0;}}};
}

test('bounded canonical writer retains every statement and host observation across batch boundaries',()=>{
 const historical=execFileSync('git',['show','d1fa44c2803602ca9a08c24848de184f2c39a800:js/src/lino-serialization-write.js'],{cwd:new URL('../../',import.meta.url),encoding:'utf8'});
 assert.equal(historical.split('export '+previousNetworkWriter.toString().replace('previousNetworkWriter','encodeNetworkLino')).length,2,'the comparison retains the entire exact preceding writer body');
 for(const count of[0,1,2,511,512,513,1024,1025]){
  const identities=Array.from({length:count},(_,at)=>at+1),left=[],right=[];
  const before=input(identities,left),after=input(identities,right);
  const expected=previousNetworkWriter(before.links,before.registered),actual=candidate(after.links,after.registered);
  assert.equal(actual,expected,'all UTF-16 code units, escaping, flags, spans, registration and line endings at '+count);
  assert.deepEqual(right,left,'every actual getter and membership call retains its complete order');
  if(count)assert.ok(actual.includes('\ud800'),'unknown type tokens retain their existing unpaired UTF-16 code unit');
 }
});

test('bounded canonical writer retains supplied order repeated identities and thrown errors',()=>{
 const left=[],right=[],before=input([4,2,4,1],left),after=input([4,2,4,1],right);
 assert.equal(candidate(after.links,after.registered),previousNetworkWriter(before.links,before.registered));
 assert.deepEqual(right,left);
 for(const write of[previousNetworkWriter,candidate]){
  const trace=[],value=input([1,2,3],trace),error=new Error('the actual metadata owner failed');
  value.links[1].metadata=()=>{throw error;};
  assert.throws(()=>write(value.links,value.registered),actual=>actual===error);
  assert.deepEqual(trace,[['identity',1],['references',1],['metadata',1],['registered',1],['identity',2],['references',2]]);
 }
});
