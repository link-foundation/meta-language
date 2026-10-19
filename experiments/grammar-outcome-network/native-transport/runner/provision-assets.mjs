// CI-only acquisition into stopped-task scratch, using the exact frozen registry
// and unchanged existing checksum-verifying downloader. Never store sources here.
import assert from 'node:assert/strict';
import {existsSync,lstatSync,readFileSync,realpathSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {readProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {requireNativeCI} from './run.mjs';
import {bytesAt,sha256,within} from './plan.mjs';

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
  assert.equal(sha256(bytesAt(repository,policy.provisioner.file)),policy.provisioner.sha256,'unchanged frozen asset downloader');
  const {downloadExternalGrammarAssets,externalGrammarPath}=await import(pathToFileURL(join(repository,policy.provisioner.file)));
  const manifest={schemaVersion:1,assets:Object.fromEntries(policy.assets.map(({path,...asset})=>{
    assert.ok(/^rust\/vendor\/tree-sitter-[a-z-]+\/src\//u.test(path),'only native oracle source assets');
    assert.equal(asset.group,policy.group);return[path,asset];
  }))};
  const context={root:overlay,cacheRoot:join(work,'external-grammar-assets'),manifest};
  const captured={schemaVersion:1,scope:'Authenticated temporary native oracle build inputs; no grammar capability evidence.',
    manifest:policy.manifest,provisioner:policy.provisioner,closureInputs:policy.closureInputs,files:[]};
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
      captured.files.push({path:asset.path,bytes:bytes.length,sha256:sha256(bytes),
        ...(asset.encoding==='gzip'?{decodedBytes:asset.decodedBytes,decodedSha256:asset.decodedSha256}:{})});
    }
    writeProjectData(receipt,captured);
  }
  if(failure)throw failure;
  assert.equal(captured.files.length,policy.assets.length,'every registered build input was authenticated');
  console.log('Authenticated '+captured.files.length+' temporary native oracle source assets.');
}
if(process.argv[1]&&pathToFileURL(resolve(process.argv[1])).href===import.meta.url)await main();
