// CI-only acquisition into stopped-task scratch, using the exact frozen registry
// and unchanged existing checksum-verifying downloader. Never store sources here.
import assert from 'node:assert/strict';
import {existsSync,lstatSync,readFileSync,realpathSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {gunzipSync} from 'node:zlib';
import {readProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {requireNativeCI} from './run.mjs';
import {bytesAt,frozenSourceBytes,sha256,within} from './plan.mjs';

/** Keep frozen source membership and the active acquisition tool distinct. */
export function readActiveAssetProvisioner(repository,policy,{readFrozen=frozenSourceBytes,readCurrent=bytesAt}={}) {
  assert.equal(policy.activeProvisioner.file,policy.provisioner.file,'the parent selects the same acquisition tool path');
  const frozen=readFrozen(repository,{path:policy.provisioner.file,gitObject:policy.provisioner.gitObject,sha256:policy.provisioner.sha256});
  assert.equal(frozen.bytes.length,policy.provisioner.bytes);
  assert.equal(sha256(frozen.bytes),policy.provisioner.sha256,'unchanged frozen asset downloader');
  const active=readCurrent(repository,policy.activeProvisioner.file);
  assert.equal(active.length,policy.activeProvisioner.bytes);
  assert.equal(sha256(active),policy.activeProvisioner.sha256,'exact parent-captured active asset downloader');
  return join(repository,policy.activeProvisioner.file);
}

/** Independent final native compiler input check, including decompressed assets. */
export function checkNativeOracleAssetBytes(asset,bytes) {
  assert.ok(['identity','gzip'].includes(asset.encoding),'covered frozen native oracle asset encoding');
  assert.equal(bytes.length,asset.bytes,'exact native oracle distribution byte count');
  assert.equal(sha256(bytes),asset.sha256,'exact native oracle distribution bytes');
  if(asset.encoding==='gzip'){
    assert.ok(Number.isSafeInteger(asset.decodedBytes)&&asset.decodedBytes>=0&&asset.decodedBytes<=64*1024*1024,'bounded frozen native oracle decoded bytes');
    const decoded=gunzipSync(bytes,{maxOutputLength:Math.max(1,asset.decodedBytes+1)});
    assert.equal(decoded.length,asset.decodedBytes,'exact native oracle decoded byte count');
    assert.equal(sha256(decoded),asset.decodedSha256,'exact native oracle compiler input bytes');
  }
  return{bytes:bytes.length,sha256:sha256(bytes)};
}

export function requireOwnedAssetScratch({repository,work,overlay,request,receipt},environment=process.env) {
  const temporary=requireNativeCI({repository},environment);
  work=realpathSync(work);overlay=realpathSync(overlay);
  assert.ok(within(temporary,work)&&within(work,overlay),'external owned scratch only');
  assert.ok(within(work,request)&&within(repository,receipt),'request and retained receipt owners');
  assert.equal(readProjectData(join(work,'.meta-language-bulk95-owned.lino')).repository,repository);
  assert.ok(lstatSync(overlay).isDirectory()&&!lstatSync(overlay).isSymbolicLink());
  assert.ok(!existsSync(join(overlay,'.git')),'scratch asset root must not be a checkout');
  return{work,overlay};
}

async function main() {
  const args=process.argv.slice(2);
  assert.equal(args.length,12);
  const options={};
  for(let index=0;index<args.length;index+=2) {
    assert.ok(['--repository','--work','--overlay','--request','--request-sha256','--receipt'].includes(args[index]));
    const name=args[index].slice(2);assert.ok(!options[name]);options[name]=args[index+1];
  }
  const repository=realpathSync(options.repository),work=realpathSync(options.work),overlay=realpathSync(options.overlay);
  const request=resolve(options.request),receipt=resolve(options.receipt);
  requireOwnedAssetScratch({repository,work,overlay,request,receipt});
  assert.equal(sha256(readFileSync(request)),options['request-sha256'],'exact parent-authored frozen asset request');
  const policy=readProjectData(request);
  assert.equal(policy.schemaVersion,1);assert.equal(policy.group,'native-oracles');
  const provisioner=readActiveAssetProvisioner(repository,policy);
  const {downloadExternalGrammarAssets,externalGrammarPath}=await import(pathToFileURL(provisioner));
  const manifest={schemaVersion:1,assets:Object.fromEntries(policy.assets.map(({path,...asset})=>{
    assert.ok(/^rust\/vendor\/tree-sitter-[a-z-]+\/src\//u.test(path),'only native oracle source assets');
    assert.equal(asset.group,policy.group);return[path,asset];
  }))};
  const context={root:overlay,cacheRoot:join(work,'external-grammar-assets'),manifest};
  const captured={schemaVersion:1,scope:'Authenticated temporary native oracle build inputs; no grammar capability evidence.',
    manifest:policy.manifest,provisioner:policy.provisioner,activeProvisioner:policy.activeProvisioner,closureInputs:policy.closureInputs,files:[]};
  let failure;
  try {
    await downloadExternalGrammarAssets({...context,assets:policy.assets.map(asset=>asset.path),materialize:true,
      jobs:policy.jobs,timeoutMs:policy.requestTimeoutMs});
  } catch(error) {failure=error;captured.error={name:error.name,message:error.message};}
  finally {
    for(const asset of policy.assets) {
      if(!existsSync(join(overlay,asset.path)))continue;
      const actual=externalGrammarPath(asset.path,context);assert.ok(within(overlay,realpathSync(actual)),'asset is materialized in owned overlay');
      const bytes=readFileSync(actual);
      const checked=checkNativeOracleAssetBytes(asset,bytes);
      captured.files.push({path:asset.path,...checked,
        ...(asset.encoding==='gzip'?{decodedBytes:asset.decodedBytes,decodedSha256:asset.decodedSha256}:{})});
    }
    writeProjectData(receipt,captured);
  }
  if(failure)throw failure;
  assert.equal(captured.files.length,policy.assets.length,'every registered build input was authenticated');
  console.log('Authenticated '+captured.files.length+' temporary native oracle source assets.');
}
if(process.argv[1]&&pathToFileURL(resolve(process.argv[1])).href===import.meta.url)await main();
