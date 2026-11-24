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
import {createControlDiagnostics,requireControlProduct as product,finishControlDiagnostics} from './control-diagnostics.js';
export function requireCollectionCopyNodeVersion(){const[major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,`uncovered collection-copy source Node ${process.versions.node}; >=22.15 <23 || >=24 <25`);}
export function collectionCopyStdout(source){const lines=[];vm.runInNewContext(source,{console:{log:(...values)=>lines.push(values.map(String).join(' '))}},{timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
function objects(root){const seen=new Set(),pending=[root],result=[];while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;assert.ok(seen.size<100000);seen.add(value);result.push(value);pending.push(...(value instanceof Map?[...value.values()]:Object.values(value)));}return result;}
export function readCollectionCopyOriginalDiagnostic(row){
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt,record={...row,emissions:{},transportEmissions:{}};
 attempt('declared source version',()=>assert.equal(row.sourceVersionRange,'ECMAScript 2024-2025; Node >=22.15 <23 || >=24 <25'));
 attempt('active source version',requireCollectionCopyNodeVersion);
 record.concrete=attempt('complete source CST and physical transports',()=>readCollectionCopyConcreteControl(row.source));
 record.originalStdout=attempt('original source execution',()=>collectionCopyStdout(row.source));
 attempt('literal original stdout',()=>assert.equal(product(record.originalStdout,'original source execution'),row.stdout,row.name+': independent complete original'));
 const surface=attempt('source frontend parse',()=>parseJavaScript(row.source));record.surface=surface;
 const program=attempt('source frontend check',()=>checkProgram(product(surface,'source frontend parse')));
 record.observedStdout=attempt('frontend JavaScript execution',()=>collectionCopyStdout(emitJavaScript(product(program,'source frontend check')).text));
 attempt('literal frontend stdout',()=>assert.equal(product(record.observedStdout,'frontend JavaScript execution'),row.stdout,row.name+': actual frontend'));
 if(row.prototypeMethodsExpected!==undefined){
  attempt('prototype table and checked instance layout',()=>{
  product(surface,'source frontend parse');product(program,'source frontend check');
  const methods=surface.sourceClassMethods??[];assert.equal(methods.length>0,row.prototypeMethodsExpected);assert.equal(methods.length%3,0);assert.equal(Object.hasOwn(program,'sourceClassMethods'),false);
  for(let at=0;at<methods.length;at+=3){const[brand,method,helper]=methods.slice(at,at+3),entry=program.declarations.get(helper);assert.equal(entry?.k,'fn');assert.equal(entry.params[0].type.kind,'record');assert.ok(entry.params[0].type.fields.some(field=>field.name===brand));
   for(const declaration of program.declarations.values())if(declaration.k==='fn'&&declaration.ret.kind==='record'&&declaration.ret.fields.some(field=>field.name===brand))assert.ok(!declaration.ret.fields.some(field=>field.name===method),row.name+': prototype method is absent from instance data');
  }
  });
 }
 const packet=attempt('typed archive construction',()=>createTypedProgramLinks(row.source,'JavaScript'));
 record.root=attempt('typed root identity',()=>product(packet,'typed archive construction').root.asU64());
 record.links=attempt('typed text encoding',()=>product(packet,'typed archive construction').network.toLino());
 const binary=attempt('typed binary encoding',()=>encodeLinkNetworkBinary(product(packet,'typed archive construction').network));if(binary!==undefined)record.binary=[...binary];
 const variants=[attempt('original graph availability',()=>product(packet,'typed archive construction').network),attempt('typed text reload',()=>LinkNetwork.fromLino(product(record.links,'typed text encoding'))),attempt('typed binary reload',()=>decodeLinkNetworkBinary(product(binary,'typed binary encoding')))];
 attempt('declared transport cardinality',()=>assert.equal(variants.length,transports.length));
 for(let at=0;at<transports.length;at++){
  const transport=transports[at],network=variants[at];record.transportEmissions[transport]={};
  attempt(transport+'/exact text graph',()=>assert.equal(product(network,transport+' graph').toLino(),product(record.links,'typed text encoding')));
  const restored=attempt(transport+'/typed decode',()=>decodeTypedProgramLinks(product(network,transport+' graph'),product(packet,'typed archive construction').root));
  for(const owner of row.singleCallOwners)attempt(transport+'/operand single call/'+owner,()=>{const declaration=product(restored,transport+'/typed decode').declarations.get(row.singleCallFunction??'copy');assert.ok(declaration);assert.equal(objects(declaration.body).filter(node=>node.k==='call'&&node.fn===owner).length,1,row.name+': actual source operand call once');});
  for(const target of targets){
   const emitted=attempt(transport+'/'+target+'/emission',()=>emitTypedProgramLinks(product(network,transport+' graph'),product(packet,'typed archive construction').root,target));
   if(emitted!==undefined){record.transportEmissions[transport][target]=emitted;if(at===0)record.emissions[target]=emitted;}
   attempt(transport+'/'+target+'/nonempty target',()=>assert.ok(product(emitted,transport+'/'+target+'/emission').code.length));
   if(at!==0)attempt(transport+'/'+target+'/transport equality',()=>assert.deepEqual(product(emitted,transport+'/'+target+'/emission'),product(record.emissions[target],'original/'+target+'/emission'),row.name+'/'+transport+'/'+target));
   if(target==='JavaScript')attempt(transport+'/'+target+'/literal stdout',()=>assert.equal(collectionCopyStdout(product(emitted,transport+'/'+target+'/emission').code),row.stdout,row.name));
  }
 }
 attempt('exact binary replay',()=>assert.deepEqual(encodeLinkNetworkBinary(product(variants[2],'typed binary reload')),product(binary,'typed binary encoding')));
 record.diagnostics={assertions:diagnostic.assertions,failures:diagnostic.failures};return record;
}
export function readCollectionCopyOriginalControl(row){return finishControlDiagnostics(readCollectionCopyOriginalDiagnostic(row));}
export function readCollectionCopyRefusalDiagnostic(row){
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt,record={...row,originalObservedStdout:null};
 attempt('declared source version',()=>assert.equal(row.sourceVersionRange,'ECMAScript 2024-2025; Node >=22.15 <23 || >=24 <25'));
 attempt('active source version',requireCollectionCopyNodeVersion);
 record.concrete=attempt('complete source CST and physical transports',()=>readCollectionCopyConcreteControl(row.source));
 if(row.originalStdout!==undefined){record.originalObservedStdout=attempt('original source execution',()=>collectionCopyStdout(row.source));attempt('literal original stdout',()=>assert.equal(product(record.originalObservedStdout,'original source execution'),row.originalStdout,row.name+': independently retained source behavior'));}
 let failure;try{checkProgram(parseJavaScript(row.source));}catch(error){failure=error;}
 if(failure){record.kind=failure.kind;record.reason=failure.reason;record.span=failure.span??null;}
 attempt('actual frontend refusal',()=>assert.ok(failure,row.name+': actual source boundary'));
 attempt('frontend refusal classification',()=>assert.ok(['unsupported','type'].includes(product(failure,'actual frontend refusal').kind),row.name));
 if(row.reasonIncludes)attempt('literal refusal diagnostic',()=>assert.ok(product(failure,'actual frontend refusal').reason.includes(row.reasonIncludes)));
 attempt('typed archive refusal',()=>assert.throws(()=>createTypedProgramLinks(row.source,'JavaScript'),error=>['unsupported','type'].includes(error.kind)));
 record.diagnostics={assertions:diagnostic.assertions,failures:diagnostic.failures};return record;
}
export function readCollectionCopyRefusalControl(row){return finishControlDiagnostics(readCollectionCopyRefusalDiagnostic(row));}
