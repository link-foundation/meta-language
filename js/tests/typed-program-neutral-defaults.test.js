import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {readProgramTypeNeutralDefaults} from '../scripts/read-program-type-neutral-defaults.mjs';
import {generateTypedProgramNeutralDefaultFixture} from '../scripts/generate-typed-program-neutral-default-fixtures.mjs';
import {LinkNetwork} from '../src/network.js';
import {LinkId} from '../src/primitives.js';
import {decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {readProgramTypeNeutralFieldDefault} from '../src/typed-program-link-rules.js';
import {FLOAT,array,record,map,set,typeKey,sameType} from '../src/translation/types.js';

const fixture=readProjectData(new URL('../../parity/fixtures/typed-program-neutral-type-defaults.lino',import.meta.url));

test('type neutral defaults are derived from the actual serde-owned bool fields and keep every off-diagonal refusal',()=>{
  const guarded=readProgramTypeNeutralDefaults();assert.deepEqual(guarded,fixture.sourceGuard);assert.equal(guarded.fields.length,5);
  for(const row of fixture.decisions)assert.equal(readProgramTypeNeutralFieldDefault(row.kind,row.field),row.expected,`${row.kind}/${row.field}`);
  assert.equal(fixture.decisions.length,66);
  generateTypedProgramNeutralDefaultFixture({check:true});
});

test('false and absent facets share actual type meaning while true retains its existing identity',()=>{
  for(const type of [array(FLOAT),record([{name:'value',type:FLOAT}]),map(FLOAT,FLOAT),set(FLOAT)]){
    const falseType={...type,reference:false},trueType={...type,reference:true};
    assert.equal(typeKey(falseType),typeKey(type));assert.equal(sameType(falseType,type),true);
    assert.notEqual(typeKey(trueType),typeKey(type));assert.equal(sameType(trueType,type),false);
  }
  const ordinary=record([{name:'value',type:FLOAT}]);
  assert.equal(typeKey({...ordinary,open:false}),typeKey(ordinary));
  assert.notEqual(typeKey({...ordinary,open:true}),typeKey(ordinary));
});

test('explicit false type fields survive actual checked text and binary archives without altering observable source behavior',()=>{
  for(const row of fixture.cases){
    const root=LinkId.fromU64(BigInt(row.archive.root));
    const expected=vm.runInNewContext(row.source+'\n'+row.probe,{}, {timeout:1000});assert.equal(expected,row.expected);
    for(const network of [LinkNetwork.fromLino(row.archive.text),decodeLinkNetworkBinary(Buffer.from(row.archive.binaryHex,'hex'))]){
      const program=decodeTypedProgramLinks(network,root);
      assert.deepEqual([...program.declarations.keys()],['arrayIdentity','recordIdentity','mapIdentity','setIdentity']);
      for(const entry of program.declarations.values()){
        assert.equal(entry.params[0].type.reference,false);assert.equal(entry.ret.reference,false);
        if(entry.ret.kind==='record')assert.equal(entry.ret.open,false);
      }
      const emitted=emitTypedProgramLinks(network,root,'JavaScript');
      assert.equal(vm.runInNewContext(emitted.code+'\n'+row.probe,{}, {timeout:1000}),expected);
    }
  }
});
