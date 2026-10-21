// Capture one real whole-source predecessor once; freshness uses stored proofs.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {RELATIVE_ARRAY_OWNED_RULE_SOURCE,RELATIVE_ARRAY_OWNED_RULE_REVISION,RELATIVE_ARRAY_OWNED_RULE_OUTPUT,
  RELATIVE_ARRAY_OWNED_RULE_BOUNDARY,pinRelativeArrayCurrentSource,deriveRelativeArrayOwnedRuleSourceIntervals,
  verifyRelativeArrayOwnedRuleSourceHistory,readRelativeArrayOwnedRuleSourceHistory} from './relative-array-current-source-views.mjs';
const root=new URL('../../',import.meta.url),helper='js/scripts/relative-array-current-source-views.mjs';

export function generateRelativeArrayOwnedRuleSourceHistory({check=false}={}){
  let previous;
  if(check){previous=readRelativeArrayOwnedRuleSourceHistory().previous;}
  else{
    const{blob,certificate}=captureGitArtifactCertificate(RELATIVE_ARRAY_OWNED_RULE_REVISION,RELATIVE_ARRAY_OWNED_RULE_SOURCE);
    assert.equal(Buffer.from(blob.toString('utf8')).equals(blob),true);
    previous={source:blob.toString('utf8'),...pinRelativeArrayCurrentSource(blob),certificate};
  }
  const source=readFileSync(new URL(RELATIVE_ARRAY_OWNED_RULE_SOURCE,root),'utf8');
  const packet={schemaVersion:1,model:'relative array complete owned rule source history 1',revision:RELATIVE_ARRAY_OWNED_RULE_REVISION,
    source:RELATIVE_ARRAY_OWNED_RULE_SOURCE,previous,current:{source,...pinRelativeArrayCurrentSource(source)},
    coordinateUnit:'UTF-16 code units',edits:deriveRelativeArrayOwnedRuleSourceIntervals(previous.source,source),boundary:RELATIVE_ARRAY_OWNED_RULE_BOUNDARY};
  verifyRelativeArrayOwnedRuleSourceHistory(packet);
  const output=new URL(RELATIVE_ARRAY_OWNED_RULE_OUTPUT,root);
  if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  const path=new URL(helper,root),text=readFileSync(path,'utf8'),pattern=/^const packetSha256='(?:PENDING|[a-f0-9]{64})';$/mu;
  assert.equal(text.split(pattern).length,2);const expected=text.replace(pattern,"const packetSha256='"+pinRelativeArrayCurrentSource(readFileSync(output)).sha256+"';");
  if(check)assert.equal(text,expected);else writeFileSync(path,expected);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(argument=>argument==='--check'));
  const packet=generateRelativeArrayOwnedRuleSourceHistory({check:process.argv.includes('--check')});
  console.log(JSON.stringify({revision:packet.revision,sources:1,currentNativeExecution:false,oldNativeBodiesChanged:false}));
}
