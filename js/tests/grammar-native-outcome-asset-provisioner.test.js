import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {gzipSync} from 'node:zlib';
import test from 'node:test';
import {readActiveAssetProvisioner,checkNativeOracleAssetBytes} from '../../experiments/grammar-outcome-network/native-transport/runner/provision-assets.mjs';
import {runnerPublication} from '../../experiments/grammar-outcome-network/native-transport/runner/policy.mjs';
import {sha256} from '../../experiments/grammar-outcome-network/native-transport/runner/plan.mjs';

const root=fileURLToPath(new URL('../../',import.meta.url));

test('the actual frozen downloader source stays authenticated while the active tool has its own exact parent pin',()=>{
  const original=runnerPublication.nativeOraclePolicy.provisioner,active=readFileSync(path.join(root,original.file));
  const policy={provisioner:original,activeProvisioner:{file:original.file,bytes:active.length,sha256:sha256(active)}};
  assert.equal(readActiveAssetProvisioner(root,policy),path.join(root,original.file));
  const stale=structuredClone(policy);stale.activeProvisioner.sha256='0'.repeat(64);
  assert.throws(()=>readActiveAssetProvisioner(root,stale),/active asset downloader/u);
  const changed=structuredClone(policy);changed.provisioner.sha256='0'.repeat(64);
  assert.throws(()=>readActiveAssetProvisioner(root,changed));
});

test('frozen and active downloader identities remain distinct and neither can replace the other in the request',()=>{
  const frozen=Buffer.from('frozen acquisition control'),active=Buffer.from('current acquisition control');
  const policy={provisioner:{file:'js/scripts/external-grammar-cache.mjs',gitObject:'a'.repeat(40),bytes:frozen.length,sha256:sha256(frozen)},
    activeProvisioner:{file:'js/scripts/external-grammar-cache.mjs',bytes:active.length,sha256:sha256(active)}};
  const inputs={readFrozen:()=>({bytes:frozen}),readCurrent:()=>active};
  assert.equal(readActiveAssetProvisioner('/synthetic',policy,inputs),'/synthetic/js/scripts/external-grammar-cache.mjs');
  for(const mutate of [value=>{value.activeProvisioner.file='js/scripts/other.mjs';},value=>{value.provisioner.bytes++;},
    value=>{value.provisioner.sha256=policy.activeProvisioner.sha256;},value=>{value.activeProvisioner.sha256=policy.provisioner.sha256;},
    value=>{value.activeProvisioner.bytes++;}]){
    const altered=structuredClone(policy);mutate(altered);assert.throws(()=>readActiveAssetProvisioner('/synthetic',altered,inputs));
  }
});

test('every final oracle distribution and decoded compiler input has independent exact byte checks',()=>{
  const bytes=Buffer.from('synthetic native source transport control\n'),compressed=gzipSync(bytes);
  const plain={encoding:'identity',bytes:bytes.length,sha256:sha256(bytes)},gzip={encoding:'gzip',bytes:compressed.length,sha256:sha256(compressed),decodedBytes:bytes.length,decodedSha256:sha256(bytes)};
  assert.deepEqual(checkNativeOracleAssetBytes(plain,bytes),{bytes:bytes.length,sha256:sha256(bytes)});
  assert.deepEqual(checkNativeOracleAssetBytes(gzip,compressed),{bytes:compressed.length,sha256:sha256(compressed)});
  for(const asset of [{...plain,encoding:'foreign'},{...plain,bytes:plain.bytes+1},{...plain,sha256:'0'.repeat(64)},
    {...gzip,decodedBytes:gzip.decodedBytes+1},{...gzip,decodedBytes:gzip.decodedBytes-1},{...gzip,decodedSha256:'0'.repeat(64)},
    {...gzip,decodedBytes:64*1024*1024+1}])assert.throws(()=>checkNativeOracleAssetBytes(asset,asset.encoding==='identity'?bytes:compressed));
  const invalid=compressed.subarray(0,-1);assert.throws(()=>checkNativeOracleAssetBytes({...gzip,bytes:invalid.length,sha256:sha256(invalid)},invalid));
});
