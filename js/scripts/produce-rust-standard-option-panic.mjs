// Deferred CI producer. Authoring this file executes no source or control.
// JSON is process IPC only; complete graph meanings remain in Links snapshots.
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as methods from '../src/translation/rust-standard-sum-method-rules.js';
import * as panic from '../src/translation/rust-panic-value-rules.js';
import {rustStandardOptionPanicMethodDecisions as decisions,rustStandardOptionPanicMethodOriginals as originals,
 rustStandardOptionPanicMethodRefusals as refusals,rustStandardOptionPanicMethodSources as sources} from '../tests/fixtures/rust-standard-option-panic-methods.mjs';

const endpoints=['1.90.0','1.99.0'],targets=['JavaScript','Rust','Lean','Rocq'],transports=['original','text','binary'];
const args=process.argv.slice(2);
assert.ok(args.length===0||args.length===2&&args[0]==='--source-version'&&endpoints.includes(args[1]),'select exactly one declared source endpoint or the complete endpoint batch');
const versions=args.length?[args[1]]:endpoints;
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===24||major===22&&minor>=15,'uncovered producer Node '+process.versions.node);
assert.equal(originals.length,5);assert.equal(decisions.length,18);assert.equal(refusals.length,4);

function observe(code){
 const lines=[];let failure=null;
 try{vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});}
 catch(error){assert.equal(error.name,'Error','a represented Rust panic must keep the ordinary error class');assert.equal(typeof error.message,'string');failure=error.message;}
 return {stdout:lines.length?lines.join('\n')+'\n':'',panic:failure};
}
function expected(row){return {stdout:row.stdout,panic:row.panic??null};}
const decisionResults=decisions.map(row=>{
 const operation=methods[row.operation]??panic[row.operation];assert.equal(typeof operation,'function',row.operation);
 const actual=operation(...row.arguments);assert.deepEqual(actual,row.expected,row.operation);return {...row,actual};
});
const cases=[];
for(const row of originals){
 assert.ok(Buffer.byteLength(row.source)<=65536,'bounded complete original');
 const concrete=LinkNetwork.parse(row.source,'Rust'),concreteText=concrete.toLino(),concreteBinary=encodeLinkNetworkBinary(concrete);
 for(const graph of [concrete,LinkNetwork.fromLino(concreteText),decodeLinkNetworkBinary(concreteBinary)])assert.equal(graph.reconstructText(),row.source,row.name+': exact original CST');
 const selections=[];
 for(const version of versions){
  // Parsing/checking is outside runtime observation: a refusal cannot stand in
  // for the independently declared runtime panic payload.
  const surface=parseRust(row.source,{sourceVersion:version}),checked=checkProgram(surface);
  const direct=observe(emitJavaScript(checked).text);assert.deepEqual(direct,expected(row),row.name+': actual frontend/checker execution');
  const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:version,provenance:{fixture:row.name}});
  const text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network),emissions={};
  const graphs=[packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)],observed=[];
  for(let index=0;index<graphs.length;index+=1){
   const graph=graphs[index];assert.equal(graph.toLino(),text);decodeTypedProgramLinks(graph,packet.root);
   for(const target of targets){
    const emission=emitTypedProgramLinks(graph,packet.root,target);
    if(index===0)emissions[target]=emission;else assert.deepEqual(emission,emissions[target],row.name+'/'+version+'/'+transports[index]+'/'+target);
   }
   const result=observe(emitTypedProgramLinks(graph,packet.root,'JavaScript').code);assert.deepEqual(result,expected(row),row.name+'/'+transports[index]);observed.push(result);
  }
  assert.deepEqual(encodeLinkNetworkBinary(graphs[2]),binary);
  selections.push({version,surface,root:packet.root.asU64(),links:text,binary:[...binary],emissions,direct,observed});
 }
 cases.push({...row,concrete:{links:concreteText,binary:[...concreteBinary]},selections});
}
const rejected=[];
for(const row of refusals){
 const selections=[];
 for(const version of versions){
  let error;try{checkProgram(parseRust(row.source,{sourceVersion:version}));}catch(failure){error=failure;}
  assert.ok(error,row.name+': actual refusal required');assert.ok(['syntax','type','unsupported'].includes(error.kind),row.name);
  assert.throws(()=>createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:version}),undefined,row.name);
  selections.push({version,kind:error.kind,reason:error.reason,span:error.span??null});
 }
 rejected.push({...row,selections});
}
process.stdout.write(JSON.stringify({versions,targets,transports,sources,decisions:decisionResults,cases,refusals:rejected}));
