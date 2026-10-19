// Probe the actual portable translator before implementing the certificate VM.
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/index.js';
export const source=`/** @param {number[]} tokens @param {number[]} transitions @param {number[]} owners @returns {number[]} */
export function probeTextCertificate(tokens,transitions,owners){
 let state=0;
 let consumed=0;
 let valid=true;
 for(let index=0;index<tokens.length;index++){
  const token=tokens[index]??-1;
  let next=-1;
  for(let row=0;row<transitions.length;row=row+3){
   if((transitions[row]??-1)===state&&(transitions[row+1]??-1)===token)next=transitions[row+2]??-1;
  }
  if(next<0)valid=false;
  state=next;
  consumed=consumed+1;
 }
 for(let row=0;row<owners.length;row=row+2){if((owners[row]??-1)!==(owners[row+1]??-1))valid=false;}
 return [valid?1:0,consumed,state];
}
`;
const checked=checkProgram(parseJavaScript(source));
const packet=encodeTypedProgramLinks(checked),restored=decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(LinkNetwork.fromLino(packet.network.toLino()))),packet.root),emitted=emitJavaScript(restored),context={};
vm.runInNewContext(emitted.text,context);const name=emitted.mappings.find(row=>row.source==='probeTextCertificate').target;
assert.deepEqual(Array.from(context[name]([4,5],[0,4,1,1,5,2],[11,11,23,23])),[1,2,2]);
assert.deepEqual(Array.from(context[name]([4,5],[0,4,1,1,5,2],[11,11,23,24])),[0,2,2]);
assert.deepEqual(Array.from(context[name]([4,6],[0,4,1,1,5,2],[11,11])),[0,2,-1]);
const rust=emitRust(checked);assert.ok(rust.definitions.some(definition=>definition.includes('pub fn probe_text_certificate')));console.log('Portable certificate probe: whole numeric-array token automaton, nested scans, actual-owner equality and ordered array result translate automatically; saved text/binary replay executes all three controls.');
