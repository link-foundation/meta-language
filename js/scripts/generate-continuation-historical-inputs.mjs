// Retain the complete inputs of historical controls after current CI repairs.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {restoreNativeCompatibilitySourceHistory} from './native-compatibility-source-history.mjs';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {readProjectData,checkProjectData,writeProjectData} from './project-data.mjs';

const root=new URL('../../',import.meta.url),revision='d1232503fb38246f7e28f5d414c882a70944b38b';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const arrayHistory=readProjectData(new URL('parity/self-translation/array-relative-element-history.lino',root));
const rangeHistory=readProjectData(new URL('parity/self-translation/original-number-source-range-history.lino',root));

/** Freeze exact predecessor views; no current artifact or implementation is rewritten. */
export function generateContinuationHistoricalInputs({check=false}={}) {
  const profiles=new Map();
  const include=(file,profile,expected)=>{
    const entry=profiles.get(file)??{profiles:[],expected};
    assert.deepEqual(entry.expected,expected);entry.profiles.push(profile);profiles.set(file,entry);
  };
  for(const row of arrayHistory.sources)include(row.file,'relative array',row.current);
  // This library is an input of the old descriptor generator, not one of its
  // outputs. Its current packaging is verified by the independent current gate.
  const library='parity/self-translation/collection-construction-rules-native.lino';
  include(library,'relative array',pin(captureGitArtifactCertificate(revision,library).blob));
  for(const row of rangeHistory.sources)include(row.file,'original number source ranges',row.current);
  for(const row of rangeHistory.unchanged)include(row.file,'original number source ranges',{bytes:row.bytes,sha256:row.sha256});
  include(rangeHistory.previousReceipt.file,'original number source ranges',rangeHistory.currentReceipt);
  const sources=[];
  for(const [file,{profiles:roles,expected}]of profiles){
    const actual=restoreNativeCompatibilitySourceHistory(file,readFileSync(new URL(file,root),'utf8'));
    if(pin(actual).sha256===expected.sha256)continue;
    const original=captureGitArtifactCertificate(revision,file),sourceText=original.blob.toString('utf8');
    verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
    assert.deepEqual(pin(original.blob),expected,'the complete historical input is independently pinned');
    assert.deepEqual(Buffer.from(sourceText),original.blob);
    let start=0,end=sourceText.length,finish=actual.length;
    while(start<end&&start<finish&&sourceText[start]===actual[start])start++;
    while(end>start&&finish>start&&sourceText[end-1]===actual[finish-1]){end--;finish--;}
    // Common prefixes/suffixes may split a surrogate pair; keep each edit on
    // UTF-8 scalar boundaries so every saved spelling has an exact byte inverse.
    if(start>0&&/[\uDC00-\uDFFF]/u.test(sourceText[start]??''))start--;
    if(end<sourceText.length&&/[\uDC00-\uDFFF]/u.test(sourceText[end]??'')){end++;finish++;}
    const edit={offsetUnit:'UTF-8 bytes',start:Buffer.byteLength(actual.slice(0,start)),end:Buffer.byteLength(actual.slice(0,finish)),
      removed:sourceText.slice(start,end),inserted:actual.slice(start,finish)};
    const bytes=Buffer.from(actual),inverse=Buffer.concat([bytes.subarray(0,edit.start),Buffer.from(edit.removed),bytes.subarray(edit.end)]);
    assert.ok(inverse.equals(original.blob));
    sources.push({file,profiles:roles,commit:revision,...expected,sourceText,certificate:original.certificate,current:pin(actual),edit});
  }
  const value={schemaVersion:1,revision,run:39389240046,sources,
    boundary:'Exact historical input views only. Current implementations and artifacts retain independent current gates. Unknown successors are never authenticated by these views. Every original historical assertion and its complete input remain required. This record proves source preservation, not native execution or semantic equivalence.'};
  const output=new URL('parity/self-translation/ci-39389240046-historical-inputs.lino',root);
  if(check)checkProjectData(output,value);else writeProjectData(output,value);
  const helperFile=new URL('js/scripts/continuation-source-history.mjs',root),source=readFileSync(helperFile,'utf8');
  const expected=source.replace(/const historyHash='(?:[a-f0-9]{64}|PENDING)';/u,"const historyHash='"+pin(readFileSync(output)).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(helperFile,expected);
  return value;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const value=generateContinuationHistoricalInputs({check:process.argv.includes('--check')});
  console.log('Preserved '+value.sources.length+' complete certified historical inputs and exact current guards.');
}
