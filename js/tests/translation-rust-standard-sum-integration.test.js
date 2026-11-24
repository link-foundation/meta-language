// Deferred end-to-end source controls. Do not run during complete-code drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readRustStandardSumConcreteControl} from './support/rust-standard-sum-concrete.js';
import {rustStandardSumIntegrationVersions as versions,rustStandardSumIntegrationTargets as targets,
 rustStandardSumIntegrationTransports as transports,rustStandardSumIntegrationOriginals as originals,
 rustStandardSumIntegrationRefusals as refusals,rustStandardSumIntegrationNativeControls as nativeControls,
 rustStandardSumIntegrationEmissionRefusals as emissionRefusals,
 rustStandardSumIntegrationPreviousRefusals as previousRefusals} from './fixtures/rust-standard-sum-integration.mjs';

function stdout(code){
 const lines=[];
 vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});
 return lines.length?lines.join('\n')+'\n':'';
}
function objects(root){
 const pending=[root],seen=new Set(),result=[];
 while(pending.length){
  const value=pending.pop();if(value===null||typeof value!=='object'||seen.has(value))continue;
  assert.ok(seen.size<100000,'bounded source control traversal');seen.add(value);result.push(value);
  for(const child of Object.values(value))if(child!==null&&typeof child==='object')pending.push(child);
 }
 return result;
}
function assertNoPrivateMarkers(root,label){
 assert.ok(objects(root).every(node=>node.k!=='rustStandardSumTry'),label+': no private completion escapes');
}

for(const row of originals)for(const sourceVersion of versions){
 test(`standard sum actual source integration: ${row.name} / Rust ${sourceVersion}`,()=>{
  assert.equal(readRustStandardSumConcreteControl(row.source).language,'Rust');
  const surface=parseRust(row.source,{sourceVersion});assertNoPrivateMarkers(surface,row.name);
  const checked=checkProgram(surface);assertNoPrivateMarkers(checked.items,row.name);
  assert.equal(stdout(emitJavaScript(checked).text),row.stdout,'actual parsed/checker emission versus independent literal stdout');
  const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion,provenance:{fixture:row.name}});
  const text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network);
  const variants=[packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)];
  assert.equal(variants.length,transports.length);
  for(let index=0;index<variants.length;index+=1){
   const network=variants[index],label=row.name+'/'+sourceVersion+'/'+transports[index];
   assert.equal(network.toLino(),text,label+': full typed graph retention');
   const program=decodeTypedProgramLinks(network,packet.root);assertNoPrivateMarkers(program.items,label);
   if(row.nominalOwner){
    const owner=program.declarations.get(row.nominalOwner);assert.ok(owner,label+': actual local datatype owner');
    assert.equal(owner.k,'data');assert.deepEqual(owner.ctors.map(ctor=>ctor.name),['Some','None'],
     label+': local nominal owner retains its actual declaration constructor order');
    assert.equal(owner.fullName,row.nominalOwner,label+': actual qualified datatype identity');
    for(const constructor of ['Some','None'])assert.ok(objects([program.items,program.main]).some(node=>
     node.k==='ctor'&&node.data===row.nominalOwner&&node.ctor===constructor),
    label+': actual constructor relationship '+row.nominalOwner+'.'+constructor);
   }
   for(const owner of row.singleCallOwners){
    const step=program.declarations.get(row.singleCallFunction??'step');assert.ok(step,label+': actual owning function declaration');
    assert.equal(objects(step.body).filter(node=>node.k==='call'&&node.fn===owner).length,1,
     label+': exactly one represented operand call '+owner);
   }
   for(const target of targets){
    const emitted=emitTypedProgramLinks(network,packet.root,target);
    assert.equal(typeof emitted.code,'string',label+'/'+target);assert.ok(emitted.code.length>0,label+'/'+target);
    assert.ok(!emitted.code.includes('rustStandardSumTry'),label+'/'+target+': private marker never emitted');
    if(target==='JavaScript')assert.equal(stdout(emitted.code),row.stdout,label+': selected continuation exact stdout');
   }
  }
  assert.deepEqual(encodeLinkNetworkBinary(variants[2]),binary,'binary source-free replay remains exact');
 });
}

for(const row of refusals)for(const sourceVersion of versions){
 test(`standard sum actual source refusal: ${row.name} / Rust ${sourceVersion}`,()=>{
  assert.equal(readRustStandardSumConcreteControl(row.source).language,'Rust');
  for(const read of [()=>checkProgram(parseRust(row.source,{sourceVersion})),
   ()=>createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion})]){
   assert.throws(read,error=>{
    assert.ok((row.diagnosticKinds??['unsupported','type']).includes(error.kind),row.name+': actual checked source refusal');
    if(row.reasonIncludes)assert.ok(error.reason.includes(row.reasonIncludes),row.name+': preserve specific source boundary');
    return true;
   });
  }
 });
}

for(const row of emissionRefusals)for(const sourceVersion of versions)test(`standard sum backend boundary: ${row.name} / Rust ${sourceVersion}`,()=>{
 assert.equal(readRustStandardSumConcreteControl(row.source).language,'Rust');
 const program=checkProgram(parseRust(row.source,{sourceVersion}));
 const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion});
 const text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network);
 const graphs=[packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)];
 for(const [at,graph]of graphs.entries()){
  assert.equal(graph.toLino(),text);assert.deepEqual(encodeLinkNetworkBinary(graph),binary);
  const restored=decodeTypedProgramLinks(graph,packet.root);assert.deepEqual(restored.items,program.items);assert.deepEqual(restored.main,program.main);
  for(const target of row.targets)assert.throws(()=>emitTypedProgramLinks(graph,packet.root,target),error=>{
   assert.ok(row.diagnosticKinds.includes(error.kind),row.name+'/'+transports[at]+'/'+target);assert.ok(error.reason.length>0);return true;
  });
 }
});

// Retention assertions keep every planned native comparison attached to all
// originals/refusals. They are not native observations or an execution claim.
test('native follow-up retains every complete original, literal oracle and transport/target cell',()=>{
 assert.deepEqual(transports,['original','text','binary']);
 assert.deepEqual(targets,['JavaScript','Rust','Lean','Rocq']);
 assert.deepEqual(versions,['1.90.0','1.99.0']);
 assert.equal(nativeControls.length,7);
 for(const previous of previousRefusals){assert.ok(originals.some(row=>row.source===previous.source),'retain exact formerly refused original');assert.ok(previous.reasonIncludes.length>0);}
 for(const row of originals){assert.ok(row.source.length>0);assert.equal(typeof row.stdout,'string');assert.equal(row.sourceVersionRange,'>=1.90.0 <1.100.0');}
 for(const row of refusals)assert.ok(row.source.length>0);
 for(const row of emissionRefusals){assert.ok(row.source.length>0);assert.deepEqual(row.targets,['Rust','Lean']);assert.equal(row.originalCompilerExpectation.outcome,'accept');}
});
