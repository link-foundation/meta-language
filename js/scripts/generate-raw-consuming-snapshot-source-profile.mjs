// Small normal source capture; freshness uses stored certificates without Git.
import assert from 'node:assert/strict';
import {existsSync,readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {RAW_CONSUMING_SNAPSHOT_REVISION,RAW_CONSUMING_SNAPSHOT_OUTPUT,
  RAW_CONSUMING_SNAPSHOT_FILES,RAW_CONSUMING_SNAPSHOT_BOUNDARY,pinRawConsumingSnapshotSource,
  deriveRawConsumingSnapshotSourceInterval,verifyRawConsumingSnapshotSourceStructure,
  readRawConsumingSnapshotSourceProfile,readCertifiedRawConsumingSnapshotSources,
  readArchivedRawConsumingSnapshotSources} from './raw-consuming-snapshot-source-profile.mjs';
const root=new URL('../../',import.meta.url),helper='js/scripts/raw-consuming-snapshot-source-profile.mjs';

export function generateRawConsumingSnapshotSourceProfile({check=false}={}){
  let previous;
  const output=new URL(RAW_CONSUMING_SNAPSHOT_OUTPUT,root);
  if(check){previous=readRawConsumingSnapshotSourceProfile().previous;}
  else if(existsSync(output)){previous=readCertifiedRawConsumingSnapshotSources();}
  else{
    previous=RAW_CONSUMING_SNAPSHOT_FILES.slice(0,2).map(({file})=>{
      const{blob,certificate}=captureGitArtifactCertificate(RAW_CONSUMING_SNAPSHOT_REVISION,file);
      assert.ok(Buffer.from(blob.toString('utf8')).equals(blob));
      return{file,source:blob.toString('utf8'),...pinRawConsumingSnapshotSource(blob),certificate};
    });
    previous.push(...readArchivedRawConsumingSnapshotSources());
  }
  const sources=RAW_CONSUMING_SNAPSHOT_FILES.map((row,index)=>{
    const bytes=readFileSync(new URL(row.file,root)),source=bytes.toString('utf8');
    assert.ok(Buffer.from(source).equals(bytes),'current source retains exact UTF-8 bytes');
    return{file:row.file,source,...pinRawConsumingSnapshotSource(source),coordinateUnit:'UTF-16 code units',edit:deriveRawConsumingSnapshotSourceInterval(previous[index].source,source)};
  });
  const packet={schemaVersion:1,model:'raw consuming snapshot source profile 1',revision:RAW_CONSUMING_SNAPSHOT_REVISION,previous,sources,boundary:RAW_CONSUMING_SNAPSHOT_BOUNDARY};
  verifyRawConsumingSnapshotSourceStructure(packet);
  const path=new URL(helper,root),source=readFileSync(path,'utf8'),pattern=/^const packetSha256='(?:PENDING|[a-f0-9]{64})';$/mu;
  assert.equal(source.split(pattern).length,2);
  if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  const expected=source.replace(pattern,"const packetSha256='"+pinRawConsumingSnapshotSource(readFileSync(output)).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(path,expected);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(argument=>argument==='--check'));
  const packet=generateRawConsumingSnapshotSourceProfile({check:process.argv.includes('--check')});
  console.log(JSON.stringify({revision:packet.revision,sources:packet.sources.length,historicalArtifactsChanged:false,nativeExecution:false}));
}
