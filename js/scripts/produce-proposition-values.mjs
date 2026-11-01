// Future CI producer only. No represented program runs while drafting this file.
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {propositionValueSurfacePrograms as programs,propositionValueSurfaceRefusals as refusals} from '../tests/fixtures/proposition-values.mjs';
import {createControlDiagnostics,requireControlProduct as product,readControlDiagnosticFallback} from '../tests/support/control-diagnostics.js';
const targets=['JavaScript','Rust','Lean','Rocq'],transports=['original','text','binary'];
function stdout(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
function positive(row){
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt,record={...row,transportEmissions:{}};
 const checked=attempt('actual Surface checking',()=>checkProgram(structuredClone(row.program)));
 const direct=attempt('independent checked JavaScript emission',()=>emitJavaScript(product(checked,'actual Surface checking')).text);
 const observed=attempt('checked JavaScript execution',()=>stdout(product(direct,'checked JavaScript emission')));
 attempt('independent literal stdout',()=>assert.equal(product(observed,'checked JavaScript execution'),row.stdout));
 const packet=attempt('typed graph construction',()=>encodeTypedProgramLinks(product(checked,'actual Surface checking')));
 record.root=attempt('typed root identity',()=>product(packet,'typed graph construction').root.asU64());
 record.links=attempt('typed text encoding',()=>product(packet,'typed graph construction').network.toLino());
 const binary=attempt('typed binary encoding',()=>encodeLinkNetworkBinary(product(packet,'typed graph construction').network));
 if(binary!==undefined)record.binary=[...binary];
 const variants=[attempt('original graph availability',()=>product(packet,'typed graph construction').network),attempt('text graph reload',()=>LinkNetwork.fromLino(product(record.links,'typed text encoding'))),attempt('binary graph reload',()=>decodeLinkNetworkBinary(product(binary,'typed binary encoding')))];
 for(let index=0;index<transports.length;index+=1){
  const transport=transports[index],network=variants[index];record.transportEmissions[transport]={};
  attempt(transport+'/exact text graph',()=>assert.equal(product(network,transport+' graph').toLino(),product(record.links,'typed text encoding')));
  attempt(transport+'/exact binary graph',()=>assert.deepEqual(encodeLinkNetworkBinary(product(network,transport+' graph')),product(binary,'typed binary encoding')));
  const restored=attempt(transport+'/actual typed decode',()=>decodeTypedProgramLinks(product(network,transport+' graph'),product(packet,'typed graph construction').root));
  attempt(transport+'/checked items',()=>assert.deepEqual(product(restored,transport+'/actual typed decode').items,product(checked,'actual Surface checking').items));
  attempt(transport+'/checked main',()=>assert.deepEqual(product(restored,transport+'/actual typed decode').main,product(checked,'actual Surface checking').main));
  for(const target of targets){
   const emitted=attempt(transport+'/'+target+'/emission',()=>emitTypedProgramLinks(product(network,transport+' graph'),product(packet,'typed graph construction').root,target));
   if(emitted!==undefined)record.transportEmissions[transport][target]=emitted;
   attempt(transport+'/'+target+'/nonempty code',()=>assert.ok(product(emitted,target+'/emission').code.length>0));
   if(index>0)attempt(transport+'/'+target+'/exact original emission',()=>assert.deepEqual(product(emitted,target+'/emission'),product(record.transportEmissions.original[target],target+'/original emission')));
   if(target==='JavaScript')attempt(transport+'/'+target+'/literal stdout',()=>assert.equal(stdout(product(emitted,target+'/emission').code),row.stdout));
  }
 }
 record.diagnostics={assertions:diagnostic.assertions,failures:diagnostic.failures};return record;
}
function refusal(row){
 const diagnostic=createControlDiagnostics();
 diagnostic.attempt('actual Surface refusal',()=>assert.throws(()=>checkProgram(structuredClone(row.program)),error=>{assert.ok(['type','unsupported'].includes(error.kind));assert.ok(error.reason.includes(row.reasonIncludes));return true;}));
 return {...row,diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}};
}
const diagnostic=createControlDiagnostics();
diagnostic.attempt('active Node source range',()=>{const[major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered proposition source Node '+process.versions.node+'; >=22.15 <23 || >=24 <25');});
diagnostic.attempt('complete declared Surface inventory',()=>{assert.equal(programs.length,6);assert.equal(refusals.length,3);});
const envelope={model:'proposition value complete Surface controls 1',nodeVersion:process.versions.node,targets,transports,cases:programs.map(row=>readControlDiagnosticFallback(row,()=>positive(row))),refusals:refusals.map(row=>readControlDiagnosticFallback(row,()=>refusal(row))),diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}};
const serialized=JSON.stringify(envelope);
if(Buffer.byteLength(serialized,'utf8')>32*1024*1024)throw new Error('proposition Surface diagnostic envelope exceeds its explicit byte budget');
process.stdout.write(serialized+'\n');
if([...diagnostic.failures,...envelope.cases.flatMap(row=>row.diagnostics.failures),...envelope.refusals.flatMap(row=>row.diagnostics.failures)].length)process.exitCode=1;
