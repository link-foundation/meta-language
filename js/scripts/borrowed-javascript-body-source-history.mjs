// Source conservation only: the actual borrowed native host keeps its own gates.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
export const BORROWED_JAVASCRIPT_BODY_SOURCE=Object.freeze({file:'rust/src/translation/javascript/declarations.rs',previousRevision:'c33d6e0af666d3b679b5ca420917f5cafb347613',currentRevision:'48ada09da9a4fe63bf7c9dfdf57624998a8da516'});
export const BORROWED_JAVASCRIPT_BODY_HISTORY='parity/self-translation/borrowed-javascript-body-source-history.lino';
export const BORROWED_JAVASCRIPT_BODY_CHANGES=Object.freeze([
 {before:'Body::RecordParameters(Box::new(records)),',after:'&Body::RecordParameters(Box::new(records)),',count:1},
 {before:'&tokens, Body::Block, is_async',after:'&tokens, &Body::Block, is_async',count:2},
 {before:'&tokens, Body::Arrow, is_async',after:'&tokens, &Body::Arrow, is_async',count:1},
 {before:'body: Body,',after:'body: &Body,',count:1},
 {before:'match &body {',after:'match body {',count:2},
].map(Object.freeze));
let cached;
export function readBorrowedJavaScriptBodySourceIdentity(source){return{bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')};}
export function deriveBorrowedJavaScriptBodySource(previous){
 let current=previous;
 for(const{before,after,count}of BORROWED_JAVASCRIPT_BODY_CHANGES){assert.equal(current.split(before).length-1,count);current=current.replaceAll(before,after);}
 let inverse=current;
 for(const{before,after,count}of [...BORROWED_JAVASCRIPT_BODY_CHANGES].reverse()){assert.equal(inverse.split(after).length-1,count);inverse=inverse.replaceAll(after,before);}
 assert.equal(inverse,previous,'every complete preceding source byte survives');return current;
}
export function verifyBorrowedJavaScriptBodySourceHistory(packet){
 assert.deepEqual(Object.keys(packet).sort(),['schemaVersion','model','file','previousRevision','currentRevision','previous','current','changes','boundary'].sort());
 assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'borrowed JavaScript function body native source 1');
 for(const key of ['file','previousRevision','currentRevision'])assert.equal(packet[key],BORROWED_JAVASCRIPT_BODY_SOURCE[key]);
 assert.deepEqual(packet.changes,BORROWED_JAVASCRIPT_BODY_CHANGES);
 for(const side of ['previous','current']){
  const row=packet[side];assert.deepEqual(Object.keys(row).sort(),['source','bytes','sha256','certificate'].sort());
  assert.equal(typeof row.source,'string');assert.deepEqual(readBorrowedJavaScriptBodySourceIdentity(row.source),{bytes:row.bytes,sha256:row.sha256});
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:packet[side+'Revision'],path:packet.file});
 }
 assert.equal(deriveBorrowedJavaScriptBodySource(packet.previous.source),packet.current.source);
 assert.equal(typeof packet.boundary,'string');assert.ok(packet.boundary.length>0);return packet;
}
export function readBorrowedJavaScriptBodySourceHistory(){
 cached??=verifyBorrowedJavaScriptBodySourceHistory(readBootstrapStructuredDataLino(readFileSync(new URL('../../'+BORROWED_JAVASCRIPT_BODY_HISTORY,import.meta.url),'utf8')));
 return structuredClone(cached);
}
/** Only a complete certified successor can supply this immediate predecessor. */
export function restoreBorrowedJavaScriptBodySource(source,file){
 assert.equal(typeof source,'string');assert.equal(typeof file,'string');
 if(file!==BORROWED_JAVASCRIPT_BODY_SOURCE.file)return source;
 const packet=readBorrowedJavaScriptBodySourceHistory();return source===packet.current.source?packet.previous.source:source;
}
