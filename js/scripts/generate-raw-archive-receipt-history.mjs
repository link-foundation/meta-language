// Metadata-only author. Full graph construction/replays remain mandatory in CI.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {serializeProjectData,writeProjectData,checkProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url),revision='f73a10b22f8d0bcfbf4707621bcc9edd6989533d';
const artifact='experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino';
const output='parity/self-translation/ci-39830836214-raw-archive-history.lino';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
export function generateRawArchiveReceiptHistory({check=false,log}={}){
 const logBytes=readFileSync(log);
 assert.deepEqual(pin(logBytes),{bytes:52776,sha256:'d8e6ac429c9a3457370a371fd369312d94e8eff7e00007ecc8188f836aa565ca'},'the complete retained CI log supplies the archive facts');
 const matches=[...logBytes.toString('utf8').matchAll(/unknown artifacts cannot enter the descriptor chain; actual sha256=([a-f0-9]{64}); bytes=(\d+); diagnostic=(.*)\n/gu)];assert.equal(matches.length,1);
 const diagnostic=JSON.parse(matches[0][3]);
 const rows=[];
 const capture=file=>{const{blob,certificate}=captureGitArtifactCertificate(revision,file);assert.deepEqual(Buffer.from(blob.toString('utf8')),blob);return{blob,certificate,value:readBootstrapStructuredDataLino(blob.toString('utf8'))};};
 const original=capture(artifact),value=original.value;
 assert.deepEqual(diagnostic.source,{bytes:value.sourceBytes,sha256:value.sourceSha256});assert.deepEqual(diagnostic.replays,value.replays);
 const fields=['textBytes','textSha256','binarySha256'],rest={...diagnostic.archive};for(const field of fields)rest[field]=value.archive[field];assert.deepEqual(rest,value.archive,'root, record inventory, byte bounds and binary size stay exact');
 const changes=fields.map(field=>({path:['archive',field],before:value.archive[field],after:diagnostic.archive[field]}));
 for(const change of changes){assert.notDeepEqual(change.before,change.after);value.archive[change.path[1]]=change.after;}
 const source=serializeProjectData(value,artifact),current=pin(source);
 assert.deepEqual(current,{bytes:Number(matches[0][2]),sha256:matches[0][1]},'the entire candidate reproduces the independently logged CI input, not just selected fields');
 rows.push({file:artifact,previous:pin(original.blob),current,certificate:original.certificate,changes});
 assert.equal(readFileSync(new URL(artifact,root),'utf8'),original.blob.toString('utf8'),'the historical physical snapshot remains exact');
 const packet={schemaVersion:1,model:'complete raw archive metadata succession 1',revision,
  diagnosticSource:{run:39830836214,job:120022678167,...pin(logBytes),refusedArtifactSha256:matches[0][1]},rows,
  boundary:'Exactly three generated archive metadata fields distinguish the full current artifact from its historical snapshot. Every complete owner, source, binding, declaration, mapping, assumption, encoding, archive record census, limit and both independent emitted replays stay byte-exact. Complete Git membership recovers the prior file; its physical bytes and borrowed receipt remain unchanged. The actual CI producer must reproduce the entire current artifact and replay both codecs before this exact snapshot view is available. These metadata authoring records are not native execution or requirement observations.'};
 if(check)checkProjectData(new URL(output,root),packet);else writeProjectData(new URL(output,root),packet);
 const path=new URL('js/scripts/raw-archive-receipt-history.mjs',root),helper=readFileSync(path,'utf8'),pattern=/const packetSha256='(?:PENDING|[a-f0-9]{64})';/u;assert.equal(helper.split(pattern).length,2);
 const expected=helper.replace(pattern,"const packetSha256='"+pin(readFileSync(new URL(output,root))).sha256+"';");if(check)assert.equal(helper,expected);else writeFileSync(path,expected);
 return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const args=process.argv.slice(2);const check=args.at(-1)==='--check';if(check)args.pop();assert.ok(args.length===2&&args[0]==='--ci-log');
 generateRawArchiveReceiptHistory({check,log:args[1]});console.log('Authenticated the complete CI archive successor; historical physical artifacts remain exact.');
}
