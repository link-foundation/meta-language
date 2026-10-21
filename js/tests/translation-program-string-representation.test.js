import assert from 'node:assert/strict';
import test from 'node:test';
import {acceptProgramStringRepresentationFlag,selectProgramStringRepresentation} from '../src/translation/program-string-representation-rules.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {generateProgramStringRepresentationRules} from '../scripts/generate-program-string-representation-rules.mjs';
import {generateProgramStringRepresentationHosts} from '../../experiments/program-string-representation/generate.mjs';
const source='/** @param {number} value @returns {number} */\nfunction identity(value) { return value; }\nconsole.log(identity(3));';

test('shared string representation decisions retain all boolean choices and automatically generated complete native definitions',()=>{
  for(const present of [false,true])for(const kind of ['undefined','boolean','number','string','object','function','symbol','bigint']){
    assert.equal(acceptProgramStringRepresentationFlag(kind,present),!present||kind==='boolean');
  }
  for(const program of [false,true])for(const explicit of [false,true])for(const required of [false,true]){
    assert.equal(selectProgramStringRepresentation(program,explicit,required),program||explicit||required);
  }
  const {policy,files}=generateProgramStringRepresentationRules({check:true});assert.equal(policy.declarations.length,2);assert.ok(files.length>0);
  const hosts=generateProgramStringRepresentationHosts({check:true});assert.equal(hosts.sources.length,9);
});

test('explicit true false and absence remain distinct through whole-program rechecking both archives and actual existing emitters',()=>{
  const initial=createTypedProgramLinks(source,'JavaScript');
  for(const flag of [undefined,false,true]){
    const program=decodeTypedProgramLinks(initial.network,initial.root);
    if(flag!==undefined)program.utf16Strings=flag;
    assert.equal(recheckLinkedTypedProgram(program,{wholeProgram:true}),program);
    const packet=encodeTypedProgramLinks(program);
    for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
      const restored=decodeTypedProgramLinks(network,packet.root);assert.equal(Object.hasOwn(restored,'utf16Strings'),flag!==undefined);assert.equal(restored.utf16Strings,flag);
      assert.equal(recheckLinkedTypedProgram(restored,{wholeProgram:true}),restored);
      const before=network.toLino(),javascript=emitTypedProgramLinks(network,packet.root,'JavaScript'),native=emitTypedProgramLinks(network,packet.root,'Rust');
      assert.ok(javascript.code.includes('identity'));assert.ok(native.code.includes('identity'));
      assert.equal(native.encodings.some(row=>row.id==='utf16-strings'),flag===true);
      assert.equal(network.toLino(),before);
    }
  }
});

test('malformed representation flags refuse without weakening checked types binding ownership or other program fields',()=>{
  const initial=createTypedProgramLinks(source,'JavaScript');
  for(const flag of [undefined,null,0,1,'false',[],{}]){
    const program=decodeTypedProgramLinks(initial.network,initial.root);program.utf16Strings=flag;
    assert.throws(()=>encodeTypedProgramLinks(program),/explicit boolean/u);
    assert.throws(()=>recheckLinkedTypedProgram(program,{wholeProgram:true}),/explicit boolean/u);
  }
  const changed=decodeTypedProgramLinks(initial.network,initial.root);changed.utf16Strings=true;
  const declaration=changed.declarations.get('identity');declaration.ret={kind:'string'};
  assert.throws(()=>encodeTypedProgramLinks(changed));
});
