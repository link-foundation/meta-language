import assert from 'node:assert/strict';
import test from 'node:test';
import {LinkNetwork,createTypedProgramLinks,readTypedProgramBindingIdentities,encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/index.js';

test('public binding identities retain unused lexical owners through text and binary reloads',()=>{
  const source='function calculate(value,unused){const constant=1;const increment=()=>value+constant;return increment();}const result=calculate(2,3);console.log(result);';
  const archive=createTypedProgramLinks(source,'JavaScript');
  const expected=readTypedProgramBindingIdentities(archive.network,archive.root).map(id=>id.asU64());
  assert.equal(new Set(expected).size,5);
  for(const network of [LinkNetwork.fromLino(archive.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(archive.network))]){
    const identities=readTypedProgramBindingIdentities(network,archive.root);
    assert.deepEqual(identities.map(id=>id.asU64()),expected);
    const names=identities.map(id=>{
      const name=network.fieldRelations().find(relation=>relation.parent.equals(id)&&relation.label==='name');
      assert.ok(name);return network.link(name.child).metadata().definition;
    }).sort();
    assert.deepEqual(names,['constant','increment','result','unused','value']);
    assert.ok(network.links().every(link=>!link.metadata().definition?.includes(source)));
  }
});
