import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {LinkNetwork} from '../src/network.js';
import {encodeStructuredDataLinks,decodeStructuredDataLinks} from '../src/structured-data-links.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readNativeCompilerHostSourceHistory,verifyNativeCompilerHostSourceHistory,readNativeCompilerHostSourceIdentity,restoreNativeCompilerHostSource} from '../scripts/native-compiler-host-source-history.mjs';
import {generateNativeCompilerHostSourceHistory} from '../scripts/generate-native-compiler-host-source-history.mjs';
import {generateBooleanDeclarationValueHosts} from '../scripts/generate-boolean-declaration-value-hosts.mjs';
const root=new URL('../../',import.meta.url),read=file=>readFileSync(new URL(file,root),'utf8');

test('every complete compiler source epoch retains independent membership exact old bytes and actual current source',()=>{
 const packet=readNativeCompilerHostSourceHistory();assert.equal(packet.sources.length,4);
 assert.deepEqual(generateNativeCompilerHostSourceHistory({check:true}),packet);
 for(const row of packet.sources){
  assert.equal(read(row.file),row.current.source);assert.notEqual(row.current.source,row.previous.source);
  assert.equal(restoreNativeCompilerHostSource(row.current.source,row.file),row.previous.source);
  for(const source of [row.current.source+'\n',row.current.source.slice(1),row.previous.source])assert.equal(restoreNativeCompilerHostSource(source,row.file),source);
  assert.equal(restoreNativeCompilerHostSource(row.current.source,row.file+'.unknown'),row.current.source);
 }
});

test('crate fixture packaging preserves complete certified old bytes and rejects changed source authority',async()=>{
 const {CRATE_FIXTURE_PACKAGE_SOURCE,readCrateFixturePackageSourceHistory,verifyCrateFixturePackageSourceHistory,restoreCrateFixturePackageSource}=await import('../scripts/crate-fixture-package-source-history.mjs');
 const {generateCrateFixturePackageSourceHistory}=await import('../scripts/generate-native-compiler-host-source-history.mjs');
 const packet=readCrateFixturePackageSourceHistory();assert.deepEqual(generateCrateFixturePackageSourceHistory({check:true}),packet);
 assert.equal(read(CRATE_FIXTURE_PACKAGE_SOURCE.file),packet.current.source);
 assert.equal(restoreCrateFixturePackageSource(packet.current.source,packet.file),packet.previous.source);
 assert.equal(restoreCrateFixturePackageSource(packet.previous.source,packet.file),packet.previous.source);
 assert.equal(restoreCrateFixturePackageSource(packet.current.source+'\n',packet.file),packet.current.source+'\n');
 assert.equal(restoreCrateFixturePackageSource(packet.current.source,'unknown.toml'),packet.current.source);
 for(const mutate of [value=>{value.currentRevision='0'.repeat(40);},value=>{value.file='unknown.toml';},value=>{value.current.certificate.path='unknown.toml';},value=>{value.previous.certificate.revision='0'.repeat(40);},value=>{value.change.after+='\n';},value=>{value.current.source+='\n';Object.assign(value.current,readNativeCompilerHostSourceIdentity(value.current.source));}]){
  const changed=structuredClone(packet);mutate(changed);assert.throws(()=>verifyCrateFixturePackageSourceHistory(changed));
 }
 const detached=readCrateFixturePackageSourceHistory();detached.current.source='caller mutation';
 assert.deepEqual(readCrateFixturePackageSourceHistory(),packet);
 const encoded=encodeStructuredDataLinks(packet),canonical=LinkNetwork.fromLino(encoded.network.toLino());
 for(const network of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
  const decoded=decodeStructuredDataLinks(network,encoded.root);assert.deepEqual(decoded,packet);verifyCrateFixturePackageSourceHistory(decoded);
 }
});

test('coherent current metadata foreign certificates changed edits and lost source fields refuse',()=>{
 const packet=readNativeCompilerHostSourceHistory();
 for(const mutate of [value=>{value.sources.reverse();},value=>{value.currentRevision='0'.repeat(40);},value=>{value.sources[0].current.source+='\n';Object.assign(value.sources[0].current,readNativeCompilerHostSourceIdentity(value.sources[0].current.source));},value=>{value.sources[1].current.certificate.path=value.sources[0].file;},value=>{value.sources[0].changes=[];},value=>{value.sources[3].previous.source=value.sources[3].current.source;},value=>{value.extra='unrepresented';}]){
  const changed=structuredClone(packet);mutate(changed);assert.throws(()=>verifyNativeCompilerHostSourceHistory(changed));
 }
 const changed=readNativeCompilerHostSourceHistory();changed.sources[0].previous.source+='changed';assert.notDeepEqual(readNativeCompilerHostSourceHistory(),changed);
});

test('historical Boolean author preserves every actual compiler-corrected host in both modes',()=>{
 const files=['rust/src/formal_concept_definition.rs','rust/src/formal_function_links.rs','rust/src/shared_numeric_function_model.rs'],before=files.map(read);
 generateBooleanDeclarationValueHosts({check:true});generateBooleanDeclarationValueHosts();
 assert.deepEqual(files.map(read),before);
});

test('both Links codecs preserve every complete compiler host and ABI artifact certificate',()=>{
 const packet=readNativeCompilerHostSourceHistory(),saved=encodeStructuredDataLinks(packet),canonical=LinkNetwork.fromLino(saved.network.toLino());
 for(const network of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
  const restored=decodeStructuredDataLinks(network,saved.root);assert.deepEqual(restored,packet);verifyNativeCompilerHostSourceHistory(restored);
 }
});

test('active root succession retains every current export and the entire earlier source recipe',async()=>{
 const {outcomeCrateRootPath,readOutcomeCrateRootSourcePair,restoreOutcomeCrateRootSource,checkOutcomeCrateRootSourceReceipt}=await import('../../experiments/grammar-outcome-network/native-transport/diagnostic-order/crate-root-source-view.mjs');
 const pair=readOutcomeCrateRootSourcePair(),source=read(outcomeCrateRootPath),row=readNativeCompilerHostSourceHistory().sources.find(value=>value.file===outcomeCrateRootPath);
 assert.equal(pair.successor.source,row.previous.source);
 const current=restoreOutcomeCrateRootSource(outcomeCrateRootPath,source);
 assert.equal(current.current,source);assert.equal(current.predecessor,pair.predecessor.source);
 assert.equal(current.receipt.profile,'complete grammar transport crate root source successor 2');
 assert.deepEqual(current.receipt.compilerSourceSuccession,row);
 assert.ok(current.current.includes('create_typed_program_links_with_rust_shared_array_handle_source_model'));
 assert.deepEqual(checkOutcomeCrateRootSourceReceipt(outcomeCrateRootPath,source,current.receipt),current.receipt);
 assert.equal(restoreOutcomeCrateRootSource(outcomeCrateRootPath,pair.successor.source).receipt.profile,'complete grammar transport crate root source successor 1');
 assert.throws(()=>restoreOutcomeCrateRootSource(outcomeCrateRootPath,source+'\n'));
 const changed=structuredClone(current.receipt);changed.compilerSourceSuccession.current.certificate.path='unknown.rs';
 assert.throws(()=>checkOutcomeCrateRootSourceReceipt(outcomeCrateRootPath,source,changed));
});
