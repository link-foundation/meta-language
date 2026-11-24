// Future independent syntax and complete primitive array inclusion controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
test('exponentiation rejects each original unparenthesized unary base at syntax time',()=>{
 const fixture=readProjectData(new URL('../../parity/fixtures/exponentiation.lino',import.meta.url));
 for(const row of fixture.syntaxCases){
  assert.throws(()=>runInNewContext(row.source.replace(/^export /gmu,'')),error=>error.name==='SyntaxError');
  assert.throws(()=>parseJavaScript(row.source),error=>error.kind===row.kind);
 }
});
test('formal array inclusion retains primitive SameValueZero, optional absence and relative start positions',()=>{
 for(const [type,values,search] of [['number','[NaN,-0,1]','NaN'],['string','["a","\\ud800"]','"\\ud800"'],['bigint','[0n,1n]','1n'],['boolean','[false,true]','false'],['number|null|undefined','[NaN,null,undefined,-0]','undefined']]){
  const source='/** @param {('+type+')[]} values @param {'+type+'} search @param {number} position @returns {boolean} */ function includesPrimitive(values,search,position){return values.includes(search,position);}\n';
  for(const position of ['0','-1','-Infinity','Infinity','NaN','1.9']){
   const program=checkProgram(parseJavaScript(source+'console.log(includesPrimitive('+values+','+search+','+position+'));')),packet=encodeTypedProgramLinks(program);
   for(const target of ['JavaScript','Rust','Lean','Rocq']){
    const expected=emitTypedProgramLinks(packet.network,packet.root,target);
    for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
   }
  }
 }
});
