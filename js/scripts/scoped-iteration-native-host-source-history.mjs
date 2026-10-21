// One certified source-only inverse before the unchanged required-field reader.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {serializeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);
export const SCOPED_ITERATION_NATIVE_SOURCE=Object.freeze({
  file:'rust/src/translation/javascript/declarations.rs',
  previousRevision:'d33e4a26dc873a1898cd954c6770abce8c1d9f02',
  revision:'c33d6e0af666d3b679b5ca420917f5cafb347613',
  previous:Object.freeze({bytes:38280,sha256:'573ac52363f87b48eb9526293990a2640a32be93df600bfbbd6867bb3c5cf2b0'}),
  current:Object.freeze({bytes:38337,sha256:'dfc79a3cb7003704e29948116ff4b27f05675f8da30aba23fe7692a46ab61267'}),
});
export const SCOPED_ITERATION_NATIVE_PREDECESSOR_REVISION='8ac3f197b537b63c03fbe14e4e7ad52a9db7eeb1';
export const SCOPED_ITERATION_NATIVE_LOOP_SOURCE=Object.freeze({
  file:'rust/src/translation/javascript/loops.rs',previousRevision:'d33e4a26dc873a1898cd954c6770abce8c1d9f02',revision:'c33d6e0af666d3b679b5ca420917f5cafb347613',
  previous:Object.freeze({"bytes":19047,"sha256":"610bd8dc1396245182b3dc93d8ea798c948e507ab38eba9353eb836eaaa4b820"}),current:Object.freeze({"bytes":19151,"sha256":"e0238bc991444011e70ec5bbf41080442586bb6cbeddb066d1feb7d850ee9ec2"}),
});
const predecessorPacket={"bytes":593225,"sha256":"e1948cf3bfcd36baac5ab585d838fd4b8f40cd9dc89160a0794d369197648db4"};
export const SCOPED_ITERATION_NATIVE_OUTPUT='parity/self-translation/scoped-iteration-native-host-source-history.lino';
const packetSha256='80f6f0086f0eb34a47b87ec474febbf2aaf03f04f8faa14993f6c0b126f7e841';
const anchor='                super::owned_array_append::lower_owned_array_appends(\n'
  +'                    &mut statements,\n'
  +'                    &params,\n'
  +'                    &self.owned_array_readers,\n';
const addition='                    &self.owned_array_iteration_sources,\n';
let cached;
export const pinScopedIterationNativeSource=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const exactKeys=(value,keys)=>assert.deepEqual(Object.keys(value).sort(),[...keys].sort());

export function deriveScopedIterationNativeHostSource(previous,current){
  const specification=SCOPED_ITERATION_NATIVE_SOURCE;
  assert.deepEqual(pinScopedIterationNativeSource(previous),specification.previous);
  assert.deepEqual(pinScopedIterationNativeSource(current),specification.current);
  assert.equal(Buffer.from(previous).toString('utf8'),previous);assert.equal(Buffer.from(current).toString('utf8'),current);
  assert.equal(previous.split(anchor).length,2);assert.equal(previous.includes(addition),false);
  const start=previous.indexOf(anchor)+anchor.length;
  assert.equal(current,previous.slice(0,start)+addition+previous.slice(start),'only the actual parser-owned iteration observation is added');
  assert.equal(current.slice(start,start+addition.length),addition);
  assert.equal(current.slice(0,start)+current.slice(start+addition.length),previous,'every complete predecessor byte survives');
  return[{start,removed:'',inserted:addition}];
}

export function verifyScopedIterationNativeHostSourceHistory(packet){
  exactKeys(packet,['schemaVersion','model','file','previousRevision','revision','coordinateUnit','previous','current','edits','boundary','predecessor','additionalSources']);
  const specification=SCOPED_ITERATION_NATIVE_SOURCE;
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'scoped iteration native historical source 1');
  for(const key of ['file','previousRevision','revision'])assert.equal(packet[key],specification[key]);
  assert.equal(packet.coordinateUnit,'UTF-16 code units');
  for(const key of ['previous','current']){
    const row=packet[key];exactKeys(row,['source','bytes','sha256','certificate']);
    assert.equal(typeof row.source,'string');assert.deepEqual(pinScopedIterationNativeSource(row.source),specification[key]);
    assert.deepEqual({bytes:row.bytes,sha256:row.sha256},specification[key]);
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:key==='previous'?specification.previousRevision:specification.revision,path:specification.file});
  }
  assert.deepEqual(packet.edits,deriveScopedIterationNativeHostSource(packet.previous.source,packet.current.source));
  assert.equal(typeof packet.boundary,'string');assert.ok(packet.boundary.length>0);
  exactKeys(packet.predecessor,['revision','bytes','sha256','certificate']);
  assert.equal(packet.predecessor.revision,SCOPED_ITERATION_NATIVE_PREDECESSOR_REVISION);
  assert.deepEqual({bytes:packet.predecessor.bytes,sha256:packet.predecessor.sha256},predecessorPacket);
  const {predecessor,additionalSources,...previous}=packet;
  const original=Buffer.from(serializeProjectData(previous,SCOPED_ITERATION_NATIVE_OUTPUT));
  assert.deepEqual(pinScopedIterationNativeSource(original),predecessorPacket,'the entire first declaration source packet survives exactly');
  verifyGitArtifactCertificate(predecessor.certificate,original,{revision:SCOPED_ITERATION_NATIVE_PREDECESSOR_REVISION,path:SCOPED_ITERATION_NATIVE_OUTPUT});
  assert.ok(Array.isArray(additionalSources));assert.equal(additionalSources.length,1);
  verifyScopedIterationNativeLoopSource(additionalSources[0]);
  return packet;
}

export function readScopedIterationNativeHostSourceHistory(){
  if(!cached){
    const bytes=readFileSync(new URL(SCOPED_ITERATION_NATIVE_OUTPUT,root));
    assert.equal(pinScopedIterationNativeSource(bytes).sha256,packetSha256,'the whole source-only packet remains exact');
    cached=verifyScopedIterationNativeHostSourceHistory(readBootstrapStructuredDataLino(bytes.toString('utf8')));
  }
  return structuredClone(cached);
}

export function restoreScopedIterationNativeHostSource(source,file,reader){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');assert.equal(typeof reader,'string');
  if(file===SCOPED_ITERATION_NATIVE_LOOP_SOURCE.file&&reader==='mutable array binding host'){
    const actual=pinScopedIterationNativeSource(source),expected=SCOPED_ITERATION_NATIVE_LOOP_SOURCE.current;
    if(actual.bytes!==expected.bytes||actual.sha256!==expected.sha256)return source;
    const row=readScopedIterationNativeHostSourceHistory().additionalSources[0];assert.equal(source,row.current.source);
    return row.previous.source;
  }
  if(file!==SCOPED_ITERATION_NATIVE_SOURCE.file||reader!=='required field native host')return source;
  const actual=pinScopedIterationNativeSource(source),expected=SCOPED_ITERATION_NATIVE_SOURCE.current;
  if(actual.bytes!==expected.bytes||actual.sha256!==expected.sha256)return source;
  const packet=readScopedIterationNativeHostSourceHistory();assert.equal(source,packet.current.source);
  return packet.previous.source;
}


const loopAnchor='        declarations.push(Stmt::Block {\n            body,\n            inline: false,\n            span: place,\n        });\n';
const loopAddition='        self.owned_array_iteration_sources\n            .insert((place, values.clone(), index.clone()));\n';
export function deriveScopedIterationNativeLoopSource(previous,current){
  const specification=SCOPED_ITERATION_NATIVE_LOOP_SOURCE;
  assert.deepEqual(pinScopedIterationNativeSource(previous),specification.previous);assert.deepEqual(pinScopedIterationNativeSource(current),specification.current);
  assert.equal(previous.split(loopAnchor).length,2);assert.equal(previous.includes(loopAddition),false);
  const start=previous.indexOf(loopAnchor)+loopAnchor.length;
  assert.equal(current,previous.slice(0,start)+loopAddition+previous.slice(start),'only the actual for-of source observation is added');
  assert.equal(current.slice(0,start)+current.slice(start+loopAddition.length),previous);
  return[{start,removed:'',inserted:loopAddition}];
}
export function verifyScopedIterationNativeLoopSource(row){
  exactKeys(row,['file','previousRevision','revision','coordinateUnit','previous','current','edits']);
  const specification=SCOPED_ITERATION_NATIVE_LOOP_SOURCE;
  for(const key of['file','previousRevision','revision'])assert.equal(row[key],specification[key]);
  assert.equal(row.coordinateUnit,'UTF-16 code units');
  for(const key of['previous','current']){
    exactKeys(row[key],['source','bytes','sha256','certificate']);
    assert.deepEqual(pinScopedIterationNativeSource(row[key].source),specification[key]);
    assert.deepEqual({bytes:row[key].bytes,sha256:row[key].sha256},specification[key]);
    verifyGitArtifactCertificate(row[key].certificate,Buffer.from(row[key].source),{revision:key==='previous'?specification.previousRevision:specification.revision,path:specification.file});
  }
  assert.deepEqual(row.edits,deriveScopedIterationNativeLoopSource(row.previous.source,row.current.source));return row;
}
