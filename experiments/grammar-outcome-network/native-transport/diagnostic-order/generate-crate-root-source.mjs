// Project one already certified complete source and derive the reviewed finite successor automatically.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readBootstrapStructuredDataLino} from '../../../../scripts/lib/structured-data-bootstrap.mjs';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {OUTCOME_CRATE_ROOT_SOURCE_PROFILE,verifyOutcomeCrateRootSourcePacket} from './crate-root-source-view.mjs';
const root=new URL('../../../../',import.meta.url);
const archive=OUTCOME_CRATE_ROOT_SOURCE_PROFILE.origin.path;
const archiveSha256=OUTCOME_CRATE_ROOT_SOURCE_PROFILE.origin.sha256;
const helper='experiments/grammar-outcome-network/native-transport/diagnostic-order/crate-root-source-view.mjs';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

export function generateOutcomeCrateRootSource({check=false}={}) {
  const profile=OUTCOME_CRATE_ROOT_SOURCE_PROFILE,archiveBytes=readFileSync(new URL(archive,root));
  assert.equal(pin(archiveBytes).sha256,archiveSha256,'the complete independently reviewed source archive is unchanged');
  const history=readBootstrapStructuredDataLino(archiveBytes.toString('utf8'));
  assert.equal(history.schemaVersion,1);assert.equal(history.revision,profile.revision);
  const owners=history.sources.filter(row=>row.file===profile.file);assert.equal(owners.length,1);
  const original=owners[0],before=Buffer.from(original.sourceText),after=readFileSync(new URL(profile.file,root));
  assert.deepEqual(pin(after),profile.current,'only the complete reviewed current crate-root successor is admitted');
  let start=0;while(start<before.length&&start<after.length&&before[start]===after[start])start++;
  let suffix=0;while(suffix<before.length-start&&suffix<after.length-start&&before[before.length-suffix-1]===after[after.length-suffix-1])suffix++;
  const removed=before.subarray(start,before.length-suffix),inserted=after.subarray(start,after.length-suffix);
  assert.equal(Buffer.from(removed.toString('utf8')).equals(removed),true);assert.equal(Buffer.from(inserted.toString('utf8')).equals(inserted),true);
  const packet={schemaVersion:1,revision:profile.revision,
    origin:{path:archive,sha256:archiveSha256},
    source:{...original,current:pin(after),edit:{start,deleteBytes:removed.length,insertText:inserted.toString('utf8')}},
    boundary:'One complete independently committed crate-root source, preserved certificate and automatically derived exact current byte inverse. This is source provenance and CST ownership, never semantic or native execution evidence.'};
  const pair=verifyOutcomeCrateRootSourcePacket(packet);assert.equal(pair.successor.source,after.toString('utf8'));
  const helperPath=new URL(helper,root),source=readFileSync(helperPath,'utf8'),pattern=/^const recipeSha256='(?:PENDING|[a-f0-9]{64})';$/mu;
  assert.equal(source.split(pattern).length,2,'one complete top-level recipe binding');
  const output=new URL(profile.recipe,root);if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  const expected=source.replace(pattern,"const recipeSha256='"+pin(readFileSync(output)).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(helperPath,expected);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.ok(args.length<=1&&args.every(arg=>arg==='--check'));
  generateOutcomeCrateRootSource({check:args.includes('--check')});
  console.log('Complete certified crate-root predecessor and finite current source successor retained; native execution remains CI-only.');
}
