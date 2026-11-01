// Deferred CI producer. Authoring this source executes neither the original
// programs nor the frontend. Its runtime output is never a checked-in result.
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {once} from 'node:events';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {LinkNetwork} from '../src/network.js';
import {readRustStandardSumConcreteControl} from '../tests/support/rust-standard-sum-concrete.js';
import {readRustOriginalCompilerExpectation,describeRustOriginalCompilerClassificationCoverage} from '../tests/support/rust-original-compiler-classifications.js';
import {rustStandardSumIntegrationOriginals as originals,rustStandardSumIntegrationRefusals as refusals,
 rustStandardSumIntegrationEmissionRefusals as emissionRefusals,
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
const originalCompilerCoverage=describeRustOriginalCompilerClassificationCoverage([...refusals,...emissionRefusals],versions);
// This reports declared source coverage/obligations only. No original compiler
// outcome is known to this producer, and historical inputs are never relabeled.
process.stderr.write('Declared original Rust refusal compiler coverage: '+JSON.stringify(originalCompilerCoverage)+'\n');
const header=JSON.stringify({versions,targets,transports,previousRefusals,nativeControls,originalCompilerCoverage});
await write(header.slice(0,-1)+',"cases":[');let first=true;
for(const original of originals){
 assert.ok(Buffer.byteLength(original.source)<=65536,'finite complete original source input');
 const compilerExpectation=readRustOriginalCompilerExpectation(original,versions);
 if(compilerExpectation)assert.equal(compilerExpectation.outcome,'accept',original.name+': a retained positive original must keep its compiler acceptance contract');
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
 const compilerExpectation=readRustOriginalCompilerExpectation(original,versions);
 const selections=[],concrete=readRustStandardSumConcreteControl(original.source);
 for(const version of versions){
  let failure;try{checkProgram(parseRust(original.source,{sourceVersion:version}));}catch(error){failure=error;}
  assert.ok(failure,original.name+': original refusal remains represented');
  assert.ok((original.diagnosticKinds??['unsupported','type']).includes(failure.kind),original.name);
  if(compilerExpectation?.outcome==='accept')assert.equal(failure.kind,'unsupported',original.name+': valid original Rust refusal is an explicit interpreter profile boundary');
  if(original.reasonIncludes)assert.ok(failure.reason.includes(original.reasonIncludes),original.name);
  assert.throws(()=>createTypedProgramLinks(original.source,'Rust',{rustSourceVersion:version}),undefined,original.name);
  selections.push({version,kind:failure.kind,reason:failure.reason,span:failure.span??null});
 }
 await write((first?'':',')+JSON.stringify({...original,concrete,selections}));first=false;
}
await write('],"emissionRefusals":[');first=true;
for(const original of emissionRefusals){
 const compilerExpectation=readRustOriginalCompilerExpectation(original,versions);assert.equal(compilerExpectation.outcome,'accept');
 const selections=[],concrete=readRustStandardSumConcreteControl(original.source);
 for(const version of versions){
  const surface=parseRust(original.source,{sourceVersion:version}),program=checkProgram(surface);
  const packet=createTypedProgramLinks(original.source,'Rust',{rustSourceVersion:version});
  const text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network);
  const graphs=[packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)],diagnostics=[];
  for(let at=0;at<graphs.length;at+=1){
   assert.equal(graphs[at].toLino(),text);
   const restored=decodeTypedProgramLinks(graphs[at],packet.root);assert.deepEqual(restored.items,program.items);assert.deepEqual(restored.main,program.main);
   assert.deepEqual(encodeLinkNetworkBinary(graphs[at]),binary);
   for(const target of original.targets){
    let failure;try{emitTypedProgramLinks(graphs[at],packet.root,target);}catch(error){failure=error;}
    assert.ok(failure,original.name+'/'+version+'/'+transports[at]+'/'+target+': actual backend refusal');
    assert.ok(original.diagnosticKinds.includes(failure.kind));assert.ok(failure.reason.length>0);
    diagnostics.push({transport:transports[at],target,kind:failure.kind,reason:failure.reason,span:failure.span??null});
   }
  }
  selections.push({version,surface,root:packet.root.asU64(),links:text,binary:[...binary],diagnostics});
 }
 await write((first?'':',')+JSON.stringify({...original,concrete,selections}));first=false;
}
await write(']}\n');
