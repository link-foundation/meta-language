// Deferred CI producer. Authoring this source executes neither the original
// programs nor the frontend. Its runtime output is never a checked-in result.
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {once} from 'node:events';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {encodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readRustStandardSumConcreteControl} from '../tests/support/rust-standard-sum-concrete.js';
import {rustStandardSumIntegrationOriginals as originals,rustStandardSumIntegrationRefusals as refusals,
 rustStandardSumIntegrationVersions as versions,rustStandardSumIntegrationTargets as targets,
 rustStandardSumIntegrationTransports as transports,rustStandardSumIntegrationPreviousRefusals as previousRefusals,
 rustStandardSumIntegrationNativeControls as nativeControls} from '../tests/fixtures/rust-standard-sum-integration.mjs';

function stdout(code){
 const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});
 return lines.length?lines.join('\n')+'\n':'';
}
const [nodeMajor,nodeMinor]=process.versions.node.split('.').map(Number);
assert.ok(nodeMajor===24||nodeMajor===22&&nodeMinor>=15,`uncovered integration producer Node ${process.versions.node}`);
async function write(text){if(!process.stdout.write(text))await once(process.stdout,'drain');}
// Keep one complete original's observations live at a time. JSON is existing
// process IPC, not a repository artifact; the graph values remain Links data.
const header=JSON.stringify({versions,targets,transports,previousRefusals,nativeControls});
await write(header.slice(0,-1)+',"cases":[');let first=true;
for(const original of originals){
 assert.ok(Buffer.byteLength(original.source)<=65536,'finite complete original source input');
 const selections=[],concrete=readRustStandardSumConcreteControl(original.source);
 for(const version of versions){
  const surface=parseRust(original.source,{sourceVersion:version}),program=checkProgram(surface);
  const observedStdout=stdout(emitJavaScript(program).text);assert.equal(observedStdout,original.stdout,original.name);
  const packet=createTypedProgramLinks(original.source,'Rust',{rustSourceVersion:version,provenance:{fixture:original.name}});
  const emissions=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
  assert.equal(stdout(emissions.JavaScript.code),original.stdout,original.name+': actual typed emission');
  selections.push({version,surface,root:packet.root.asU64(),links:packet.network.toLino(),
   binary:[...encodeLinkNetworkBinary(packet.network)],emissions,observedStdout});
 }
 await write((first?'':',')+JSON.stringify({...original,concrete,selections}));first=false;
}
await write('],"refusals":[');first=true;
for(const original of refusals){
 const selections=[],concrete=readRustStandardSumConcreteControl(original.source);
 for(const version of versions){
  let failure;try{checkProgram(parseRust(original.source,{sourceVersion:version}));}catch(error){failure=error;}
  assert.ok(failure,original.name+': original refusal remains represented');
  assert.ok(['unsupported','type'].includes(failure.kind),original.name);
  if(original.reasonIncludes)assert.ok(failure.reason.includes(original.reasonIncludes),original.name);
  assert.throws(()=>createTypedProgramLinks(original.source,'Rust',{rustSourceVersion:version}),undefined,original.name);
  selections.push({version,kind:failure.kind,reason:failure.reason,span:failure.span??null});
 }
 await write((first?'':',')+JSON.stringify({...original,concrete,selections}));first=false;
}
await write(']}\n');
