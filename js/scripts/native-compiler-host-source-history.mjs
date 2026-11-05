// Complete compiler-repair source epochs, retained only for historical readers.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
export const NATIVE_COMPILER_HOST_PREVIOUS='be0a9d76196075e6b008c05a8c63b4b4611237e7';
export const NATIVE_COMPILER_HOST_CURRENT='48ada09da9a4fe63bf7c9dfdf57624998a8da516';
export const NATIVE_COMPILER_HOST_HISTORY='parity/self-translation/native-compiler-host-source-history.lino';
export const NATIVE_COMPILER_HOST_SOURCES=Object.freeze([
 {file:'rust/src/lib.rs',changes:[{before:'    create_typed_program_links_with_rust_scalar_loop_model,\n',after:'    create_typed_program_links_with_rust_scalar_loop_model,\n    create_typed_program_links_with_rust_shared_array_handle_source_model,\n'}]},
 {file:'rust/src/formal_concept_definition.rs',changes:[{before:'pub(crate) fn encode_formal_function_declaration_context(',after:'pub fn encode_formal_function_declaration_context('}]},
 {file:'rust/src/shared_numeric_function_model.rs',changes:[{before:'.flat_map(|link| link.references())',after:'.flat_map(crate::Link::references)'}]},
 {file:'parity/self-translation/ascii-text-predicate-rules-native.lino',changes:[],artifact:true},
].map(row=>Object.freeze({...row,changes:Object.freeze(row.changes.map(Object.freeze))})));
let cached;
export function readNativeCompilerHostSourceIdentity(source){return{bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')};}
export function verifyNativeCompilerHostSourceHistory(packet){
 assert.deepEqual(Object.keys(packet).sort(),['schemaVersion','model','previousRevision','currentRevision','sources','boundary'].sort());
 assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'complete native compiler historical host sources 1');
 assert.equal(packet.previousRevision,NATIVE_COMPILER_HOST_PREVIOUS);assert.equal(packet.currentRevision,NATIVE_COMPILER_HOST_CURRENT);
 assert.deepEqual(packet.sources.map(row=>row.file),NATIVE_COMPILER_HOST_SOURCES.map(row=>row.file));
 for(const [index,row]of packet.sources.entries()){
  const specification=NATIVE_COMPILER_HOST_SOURCES[index];assert.deepEqual(Object.keys(row).sort(),['file','previous','current','changes'].sort());assert.deepEqual(row.changes,specification.changes);
  for(const side of ['previous','current']){
   const value=row[side];assert.deepEqual(Object.keys(value).sort(),['source','bytes','sha256','certificate'].sort());assert.equal(typeof value.source,'string');
   assert.deepEqual(readNativeCompilerHostSourceIdentity(value.source),{bytes:value.bytes,sha256:value.sha256});
   verifyGitArtifactCertificate(value.certificate,Buffer.from(value.source),{revision:packet[side+'Revision'],path:row.file});
  }
  if(specification.artifact){
   const before=readBootstrapStructuredDataLino(row.previous.source),after=readBootstrapStructuredDataLino(row.current.source);
   const{files:oldFiles,...oldFields}=before,{files:newFiles,...newFields}=after;
   assert.deepEqual(newFields,oldFields,'every checked program, source, replay, interface and original field stays exact');
   assert.deepEqual(newFiles.map(file=>file.path),oldFiles.map(file=>file.path));
   for(const file of [...oldFiles,...newFiles]){assert.deepEqual(Object.keys(file).sort(),['path','source','bytes','sourceSha256'].sort());assert.deepEqual(readNativeCompilerHostSourceIdentity(file.source),{bytes:file.bytes,sha256:file.sourceSha256});}
  }else{
   let current=row.previous.source;
   for(const{before,after}of row.changes){assert.equal(current.split(before).length,2);current=current.replace(before,()=>after);}
   assert.equal(current,row.current.source);
   for(const{before,after}of [...row.changes].reverse()){assert.equal(current.split(after).length,2);current=current.replace(after,()=>before);}
   assert.equal(current,row.previous.source,'every complete preceding byte survives');
  }
 }
 assert.equal(typeof packet.boundary,'string');assert.ok(packet.boundary.length>0);return packet;
}
export function readNativeCompilerHostSourceHistory(){
 cached??=verifyNativeCompilerHostSourceHistory(readBootstrapStructuredDataLino(readFileSync(new URL('../../'+NATIVE_COMPILER_HOST_HISTORY,import.meta.url),'utf8')));return structuredClone(cached);
}
export function restoreNativeCompilerHostSource(source,file){
 assert.equal(typeof source,'string');assert.equal(typeof file,'string');
 if(!NATIVE_COMPILER_HOST_SOURCES.some(row=>row.file===file))return source;
 const row=readNativeCompilerHostSourceHistory().sources.find(row=>row.file===file);return source===row.current.source?row.previous.source:source;
}
