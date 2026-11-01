// Complete future frontend/codec/target assertions. Imported only by deferred
// tests/producers, never executed while authoring this source batch.
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseJavaScript} from '../../src/translation/javascript.js';
import {checkProgram} from '../../src/translation/check.js';
import {emitJavaScript} from '../../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../../src/typed-program-links.js';
import {LinkNetwork} from '../../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../src/link-network-binary.js';
import {collectionCopyTargets as targets,collectionCopyTransports as transports} from '../fixtures/collection-copies.mjs';
import {readCollectionCopyConcreteControl} from './collection-copy-concrete.js';
export function requireCollectionCopyNodeVersion(){const[major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,`uncovered collection-copy source Node ${process.versions.node}; >=22.15 <23 || >=24 <25`);}
export function collectionCopyStdout(source){const lines=[];vm.runInNewContext(source,{console:{log:(...values)=>lines.push(values.map(String).join(' '))}},{timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
function objects(root){const seen=new Set(),pending=[root],result=[];while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;assert.ok(seen.size<100000);seen.add(value);result.push(value);pending.push(...(value instanceof Map?[...value.values()]:Object.values(value)));}return result;}
export function readCollectionCopyOriginalControl(row){
 assert.equal(row.sourceVersionRange,'ECMAScript 2024-2025; Node >=22.15 <23 || >=24 <25');
 requireCollectionCopyNodeVersion();const concrete=readCollectionCopyConcreteControl(row.source),originalStdout=collectionCopyStdout(row.source);assert.equal(originalStdout,row.stdout,row.name+': independent complete original');
 const surface=parseJavaScript(row.source),program=checkProgram(surface),observedStdout=collectionCopyStdout(emitJavaScript(program).text);assert.equal(observedStdout,row.stdout,row.name+': actual frontend');
 const packet=createTypedProgramLinks(row.source,'JavaScript'),links=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network);
 const variants=[packet.network,LinkNetwork.fromLino(links),decodeLinkNetworkBinary(binary)],emissions={};assert.equal(variants.length,transports.length);
 for(let at=0;at<variants.length;at++){
  const network=variants[at];assert.equal(network.toLino(),links);const restored=decodeTypedProgramLinks(network,packet.root);
  for(const owner of row.singleCallOwners){const declaration=restored.declarations.get(row.singleCallFunction??'copy');assert.ok(declaration);assert.equal(objects(declaration.body).filter(node=>node.k==='call'&&node.fn===owner).length,1,row.name+': actual source operand call once');}
  for(const target of targets){const emitted=emitTypedProgramLinks(network,packet.root,target);assert.ok(emitted.code.length);if(at===0)emissions[target]=emitted;else assert.deepEqual(emitted,emissions[target],row.name+'/'+transports[at]+'/'+target);if(target==='JavaScript')assert.equal(collectionCopyStdout(emitted.code),row.stdout,row.name);}
 }
 assert.deepEqual(encodeLinkNetworkBinary(variants[2]),binary);
 return {...row,concrete,surface,root:packet.root.asU64(),links,binary:[...binary],emissions,originalStdout,observedStdout};
}
export function readCollectionCopyRefusalControl(row){
 assert.equal(row.sourceVersionRange,'ECMAScript 2024-2025; Node >=22.15 <23 || >=24 <25');
 requireCollectionCopyNodeVersion();const concrete=readCollectionCopyConcreteControl(row.source);let failure,originalObservedStdout=null;
 if(row.originalStdout!==undefined){originalObservedStdout=collectionCopyStdout(row.source);assert.equal(originalObservedStdout,row.originalStdout,row.name+': independently retained source behavior');}
 try{checkProgram(parseJavaScript(row.source));}catch(error){failure=error;}
 assert.ok(failure,row.name+': actual source boundary');assert.ok(['unsupported','type'].includes(failure.kind),row.name);if(row.reasonIncludes)assert.ok(failure.reason.includes(row.reasonIncludes));
 assert.throws(()=>createTypedProgramLinks(row.source,'JavaScript'),error=>['unsupported','type'].includes(error.kind));
 return {...row,concrete,originalObservedStdout,kind:failure.kind,reason:failure.reason,span:failure.span??null};
}
