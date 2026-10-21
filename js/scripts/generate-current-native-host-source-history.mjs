// One source-only normal refresh after the final native source freeze.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {CURRENT_NATIVE_HOST_REVISION,CURRENT_NATIVE_HOST_OUTPUT,CURRENT_NATIVE_HOST_FILES,
  pinCurrentNativeHostSource,deriveCurrentNativeHostSourceIntervals,verifyCurrentNativeHostSourceHistory} from './current-native-host-source-history.mjs';
const root=new URL('../../',import.meta.url),helper='js/scripts/current-native-host-source-history.mjs';

export function generateCurrentNativeHostSourceHistory({check=false}={}){
  const sources=CURRENT_NATIVE_HOST_FILES.map(file=>{
    const{blob,certificate}=captureGitArtifactCertificate(CURRENT_NATIVE_HOST_REVISION,file);
    assert.deepEqual(Buffer.from(blob.toString('utf8')),blob);
    const source=readFileSync(new URL(file,root),'utf8'),previous={source:blob.toString('utf8'),...pinCurrentNativeHostSource(blob),certificate};
    return{file,previous,current:{source,...pinCurrentNativeHostSource(source)},coordinateUnit:'UTF-16 code units',edits:deriveCurrentNativeHostSourceIntervals(previous.source,source)};
  });
  const packet={schemaVersion:1,model:'complete current native historical host sources 1',revision:CURRENT_NATIVE_HOST_REVISION,sources,
    boundary:'Only four complete native host source views compose before existing historical CI-repair readers and the already-closed named required-field/ASCII reader scopes. All previous source packets, descriptor counts, native definitions, fixtures, checks and old oracles remain unchanged. Real8d Git membership and exact complete current byte inverses establish source conservation only; actual current parsing, compilation, runtime behavior and source descriptors require their independent current gates. Unknown source bytes remain visible to original strict validators.'};
  verifyCurrentNativeHostSourceHistory(packet);
  const output=new URL(CURRENT_NATIVE_HOST_OUTPUT,root);
  if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  const path=new URL(helper,root),source=readFileSync(path,'utf8'),pattern=/const packetSha256='(?:PENDING|[a-f0-9]{64})';/u;
  assert.equal(source.split(pattern).length,2);const expected=source.replace(pattern,"const packetSha256='"+pinCurrentNativeHostSource(readFileSync(output)).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(path,expected);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(argument=>argument==='--check'));
  const packet=generateCurrentNativeHostSourceHistory({check:process.argv.includes('--check')});
  console.log(JSON.stringify({revision:packet.revision,sources:packet.sources.length,nativeCompilation:false,nativeExecution:false}));
}
