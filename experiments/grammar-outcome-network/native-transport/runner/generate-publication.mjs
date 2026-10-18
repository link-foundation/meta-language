// Convert the reviewed packet automatically; subsequent checks replay exact
// publication edits against its retained whole sources, without executing them.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import {dirname,resolve} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {readProjectData,writeProjectData,checkProjectData} from '../../../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../../../js/src/structured-data-links.js';

const directory=dirname(fileURLToPath(import.meta.url));
const path=resolve(directory,'publication.lino');
const originalPins={
  'policy.mjs':'cab0eda69d1fe2be2cbd4dde4b4997eaf70dd420eb2fe749a13b64ed5803b238',
  'plan.mjs':'8fa56f05a469fd5dcb74296f031c4769d88ddaf2630b805de8e00d2586173ae7',
  'run.mjs':'2aa8b535acba2356e8424b77d92ca9dc988351e619f5331d68fa29081c8f7918',
  'runner.test.mjs':'10db6b9e540a15fb4c7e49961762ca2f1c275db0eb9aa94d706d25ea492d5635',
  'README.md':'0e9eca40a2d1de5f175a1c156a7f5233da18b3fd03857ed8dbaf4004cbc56d29',
};
const newSourceFiles=['generate-publication.mjs','provision-assets.mjs','release-restored-target.mjs','teardown.mjs'];
const hash=value=>createHash('sha256').update(value).digest('hex');

export function nativeOraclePolicy(baseCommit,repository=resolve(directory,'../../../..')) {
  const frozen=file=>{
    const gitObject=execFileSync('git',['-C',repository,'rev-parse',baseCommit+':'+file],{encoding:'utf8',timeout:15000}).trim();
    const bytes=execFileSync('git',['-C',repository,'cat-file','blob',gitObject],{timeout:15000,maxBuffer:8*1024*1024});
    return{file,gitObject,bytes:bytes.length,sha256:hash(bytes),source:bytes};
  };
  const manifest=frozen('parity/grammars/external-assets.lino');
  const provisioner=frozen('js/scripts/external-grammar-cache.mjs');
  const registry=readStructuredDataLino(manifest.source.toString('utf8'));
  const closureInputs=['rust/Cargo.toml','rust/oracles/agda-layout-source-oracle/build.rs',
    'rust/oracles/cmake-source-oracle/build.rs','rust/oracles/native-source-oracles/build.rs',
    'js/scripts/agda-queue-safe-oracle.mjs','rust/vendor/tree-sitter-agda-layout/source-manifest.lino',
    'rust/vendor/tree-sitter-agda-layout/layout-state.patch'].map(frozen);
  const families=closureInputs.slice(1,3).map(({source})=>{
    const matches=[...source.toString('utf8').matchAll(/PathBuf::from\("\.\.\/\.\.\/vendor\/tree-sitter-([a-z-]+)\/src"\)/gu)];
    assert.equal(matches.length,1,'one exact native oracle vendor root');return matches[0][1];
  });
  const names=/const VENDORED_GRAMMARS: &\[&str\] = &\[([^\]]+)\];/u.exec(closureInputs[3].source.toString('utf8'));
  assert.ok(names,'exact native source oracle grammar inventory');
  families.push(...[...names[1].matchAll(/"([a-z-]+)"/gu)].map(match=>match[1]));
  const assets=Object.entries(registry.assets).filter(([path,asset])=>asset.group==='native-oracles'
    &&families.some(name=>path.startsWith('rust/vendor/tree-sitter-'+name+'/src/'))
    &&/\/(?:parser\.c\.gz|scanner\.c|scanner\.c\.upstream\.gz|tree_sitter\/[a-z_]+\.h)$/u.test(path))
    .map(([path,asset])=>({path,...asset}));
  assert.equal(assets.length,18,'frozen four-family native build input closure');
  const pin=({source,...record})=>record;
  return{schemaVersion:1,group:'native-oracles',manifest:pin(manifest),provisioner:pin(provisioner),
    closureInputs:closureInputs.map(pin),families,assets,
    timeoutMs:90000,jobs:2,requestTimeoutMs:10000};
}

export function publicationEdit(original,current) {
  let start=0,end=original.length,finish=current.length;
  while(start<end&&start<finish&&original[start]===current[start])start++;
  while(end>start&&finish>start&&original[end-1]===current[finish-1]){end--;finish--;}
  return{start,end,removed:original.slice(start,end),inserted:current.slice(start,finish)};
}

export function replayPublication(original,edit) {
  assert.ok(Number.isSafeInteger(edit.start)&&Number.isSafeInteger(edit.end)&&edit.start>=0&&edit.end>=edit.start&&edit.end<=original.length);
  assert.equal(original.slice(edit.start,edit.end),edit.removed,'exact publication source interval');
  const current=original.slice(0,edit.start)+edit.inserted+original.slice(edit.end);
  const inverse=current.slice(0,edit.start)+edit.removed+current.slice(edit.start+edit.inserted.length);
  assert.equal(inverse,original,'whole original publication inverse');
  return current;
}

export function checkRunnerPublication(packet=readProjectData(path)) {
  assert.equal(packet.schemaVersion,1);
  assert.deepEqual(packet.originalSources.map(source=>source.file),Object.keys(originalPins));
  for(const original of packet.originalSources) {
    assert.equal(hash(original.source),originalPins[original.file],'original reviewed runner source');
    assert.equal(original.bytes,Buffer.byteLength(original.source));
    assert.equal(original.sha256,originalPins[original.file]);
    const published=packet.publication.find(source=>source.file===original.file);
    assert.ok(published,'published source registration');
    const current=readFileSync(resolve(directory,original.file),'utf8');
    assert.equal(replayPublication(original.source,published.edit),current,'exact published runner source');
    assert.equal(published.sha256,hash(current));assert.equal(published.bytes,Buffer.byteLength(current));
    assert.deepEqual(published.edit,publicationEdit(original.source,current),'canonical complete publication delta');
  }
  assert.equal(hash(JSON.stringify(packet.originalPolicy)),'8a12deb1cd59946232fe2d9b65fdb75aa9ab214e355d935ef05539bf3e023791');
  assert.deepEqual(packet.newSources.map(source=>source.file),newSourceFiles);
  for(const source of packet.newSources) {
    const bytes=readFileSync(resolve(directory,source.file));
    assert.equal(source.bytes,bytes.length,'published adapter bytes');assert.equal(source.sha256,hash(bytes),'published adapter hash');
  }
  assert.deepEqual(packet.nativeOraclePolicy,nativeOraclePolicy(packet.originalPolicy.baseCommit),'exact frozen external oracle closure');
  checkProjectData(path,packet);
  return packet;
}

async function main() {
  const args=process.argv.slice(2);
  if(args.length===1&&args[0]==='--check'){checkRunnerPublication();console.log('Runner publication preserves every reviewed source and exact inverse.');return;}
  assert.ok(args.length===0||(args.length===2&&args[0]==='--original-dir'));
  let packet;
  if(args.length) {
    const originalDirectory=resolve(args[1]);
    const originalSources=Object.entries(originalPins).map(([file,sha256])=>{
      const source=readFileSync(resolve(originalDirectory,file),'utf8');assert.equal(hash(source),sha256);
      return{file,bytes:Buffer.byteLength(source),sha256,source};
    });
    // Only this fixed authenticated module is evaluated during the one-time
    // conversion. Normal --check and runtime loading never evaluate archives.
    const {overlayPolicy}=await import('data:text/javascript;base64,'+Buffer.from(originalSources[0].source).toString('base64'));
    packet={schemaVersion:1,scope:'Source publication provenance; no native execution evidence.',originalPolicy:overlayPolicy,originalSources,publication:[]};
  } else packet=readProjectData(path);
  packet.nativeOraclePolicy=nativeOraclePolicy(packet.originalPolicy.baseCommit);
  packet.newSources=newSourceFiles.map(file=>{
    const bytes=readFileSync(resolve(directory,file));return{file,bytes:bytes.length,sha256:hash(bytes)};
  });
  packet.publication=packet.originalSources.map(original=>{
    const current=readFileSync(resolve(directory,original.file),'utf8');
    return{file:original.file,bytes:Buffer.byteLength(current),sha256:hash(current),edit:publicationEdit(original.source,current)};
  });
  writeProjectData(path,packet);checkRunnerPublication(packet);
}
if(process.argv[1]&&pathToFileURL(resolve(process.argv[1])).href===import.meta.url)await main();
