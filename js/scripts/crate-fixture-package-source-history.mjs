// Complete crate-input source epochs; current packaging retains its own checks.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {readNativeCompilerHostSourceIdentity} from './native-compiler-host-source-history.mjs';
export const CRATE_FIXTURE_PACKAGE_SOURCE=Object.freeze({file:'rust/Cargo.toml',previousRevision:'48ada09da9a4fe63bf7c9dfdf57624998a8da516',currentRevision:'37ef9ed1f84bd8f6e5506dd5f382aa8483234dc2',history:'parity/self-translation/crate-fixture-package-source-history.lino'});
export const CRATE_FIXTURE_PACKAGE_CHANGE=Object.freeze({before:'    "src/data/guarded-private-array-write-program.lino",\n',after:'    "src/data/guarded-private-array-write-program.lino",\n    "src/data/empty-module-collections.lino",\n    "src/data/numeric-module-bindings.lino",\n'});
let snapshot;
export function verifyCrateFixturePackageSourceHistory(packet){
 assert.deepEqual(Object.keys(packet).sort(),['schemaVersion','model','file','previousRevision','currentRevision','previous','current','change','boundary'].sort());
 assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'complete crate fixture package source 1');
 for(const key of ['file','previousRevision','currentRevision'])assert.equal(packet[key],CRATE_FIXTURE_PACKAGE_SOURCE[key]);
 assert.deepEqual(packet.change,CRATE_FIXTURE_PACKAGE_CHANGE);
 for(const side of ['previous','current']){
  const row=packet[side];assert.deepEqual(Object.keys(row).sort(),['source','bytes','sha256','certificate'].sort());
  assert.deepEqual(readNativeCompilerHostSourceIdentity(row.source),{bytes:row.bytes,sha256:row.sha256});
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:packet[side+'Revision'],path:packet.file});
 }
 const{before,after}=packet.change;assert.equal(packet.previous.source.split(before).length,2);
 assert.equal(packet.previous.source.replace(before,()=>after),packet.current.source);
 assert.equal(packet.current.source.split(after).length,2);
 assert.equal(packet.current.source.replace(after,()=>before),packet.previous.source);
 assert.equal(typeof packet.boundary,'string');assert.ok(packet.boundary.length>0);return packet;
}
export function readCrateFixturePackageSourceHistory(){
 const text=readFileSync(new URL('../../'+CRATE_FIXTURE_PACKAGE_SOURCE.history,import.meta.url),'utf8');
 if(snapshot?.text!==text)snapshot={text,packet:verifyCrateFixturePackageSourceHistory(readBootstrapStructuredDataLino(text))};
 return structuredClone(snapshot.packet);
}
export function restoreCrateFixturePackageSource(source,file){
 assert.equal(typeof source,'string');assert.equal(typeof file,'string');
 if(file!==CRATE_FIXTURE_PACKAGE_SOURCE.file)return source;
 const packet=readCrateFixturePackageSourceHistory();return source===packet.current.source?packet.previous.source:source;
}
