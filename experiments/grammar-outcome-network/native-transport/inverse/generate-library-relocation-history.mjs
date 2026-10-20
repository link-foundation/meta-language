// A finite packaging successor; original algorithms and receipt gates stay exact.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../../../js/scripts/git-object-certificate.mjs';
import {readProjectData,writeProjectData,checkProjectData,serializeProjectData} from '../../../../js/scripts/project-data.mjs';
import {tokenize} from '../../../../js/src/translation/lexer.js';

const root=new URL('../../../../',import.meta.url);
const sourceRevision='6f32c2924d6284e0a43130f225ed817450f1f2f8';
const libraryRevision='d1232503fb38246f7e28f5d414c882a70944b38b';
const libraryFile='parity/self-translation/collection-construction-rules-native.lino';
const consumingFile='experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino';
const historyFile='parity/self-translation/library-relocation-receipt-history.lino';
const helperFile='js/scripts/native-library-receipt-history.mjs';
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

function original(file,commit) {
  const captured=captureGitArtifactCertificate(commit,file);
  verifyGitArtifactCertificate(captured.certificate,captured.blob,{revision:commit,path:file});
  return{file,commit,...pin(captured.blob),sourceText:captured.blob.toString('utf8'),certificate:captured.certificate};
}

const helper=String.raw`// Generated from the exact library-relocation receipt history. Do not edit.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData,serializeProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';

const root=new URL('../../',import.meta.url),historyFile='parity/self-translation/library-relocation-receipt-history.lino';
const policySha256='__POLICY_HASH__';
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
let policy;
function readNativeLibraryHistory() {
  if(!policy){
    const bytes=readFileSync(new URL(historyFile,root));assert.equal(pin(bytes).sha256,policySha256,'the complete reviewed native packaging edge remains exact');
    policy=readProjectData(new URL(historyFile,root));assert.equal(policy.schemaVersion,1);assert.equal(policy.sources.length,2);
  }
  return policy;
}

/** Report the actual current artifact descriptor separately from its predecessor. */
export function readNativeLibraryReceiptProfile(identity) {
  const row=readNativeLibraryHistory().sources.find(row=>row.identity===identity);assert.ok(row,'one exact native packaging identity');
  return{file:row.file,...row.current};
}

/** Restore a complete reviewed source adapter before its older source checks. */
export function restoreNativeLibrarySource(source,file) {
  if(!['js/scripts/complete-declaration-receipts.mjs','js/scripts/current-translation-source-descriptors.mjs'].includes(file))return source;
  const row=readNativeLibraryHistory().sourceAdapters.find(row=>row.file===file);assert.ok(row);
  if(pin(source).sha256!==row.current.sha256)return source;
  assert.equal(Buffer.byteLength(source),row.current.bytes);
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:file});
  assert.equal(row.edit.offsetUnit,'UTF-16 code units');
  assert.equal(source.slice(row.edit.start,row.edit.start+row.edit.inserted.length),row.edit.inserted);
  const restored=source.slice(0,row.edit.start)+row.edit.removed+source.slice(row.edit.start+row.edit.inserted.length);
  assert.equal(restored,row.sourceText);assert.deepEqual(pin(restored),{bytes:row.bytes,sha256:row.sha256});return restored;
}

/** Invert only reviewed packaging descriptors before the existing complete gate. */
export function restoreNativeLibraryReceipt(artifact,identity) {
  if(!['collection construction','raw consuming'].includes(identity))return artifact;
  const history=readNativeLibraryHistory(),row=history.sources.find(row=>row.identity===identity);assert.ok(row);
  const bytes=Buffer.from(serializeProjectData(artifact,row.file));
  if(pin(bytes).sha256!==row.current.sha256)return artifact;
  assert.equal(bytes.length,row.current.bytes);
  for(const module of history.modules){
    verifyGitArtifactCertificate(module.certificate,Buffer.from(module.sourceText),{revision:module.commit,path:module.file});
    assert.deepEqual(pin(readFileSync(new URL(module.file,root))),module.current,'the actual corrected native module remains exact');
  }
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
  const restored=structuredClone(artifact);
  for(const change of row.changes){
    assert.ok(change.path.every(key=>typeof key==='string'&&!['__proto__','constructor','prototype'].includes(key)));
    assert.ok(change.path[0]==='library'&&change.path[1]==='files'&&change.path.length===4
      ||change.path.slice(0,4).join('.')==='predecessor.proof.producerApplicability.sources'&&change.path.length===6);
    assert.ok(['bytes','sourceSha256','sha256'].includes(change.path.at(-1)));
    let parent=restored;for(const key of change.path.slice(0,-1)){assert.ok(Object.hasOwn(parent,key));parent=parent[key];}
    const key=change.path.at(-1);assert.deepEqual(parent[key],change.after);parent[key]=change.before;
  }
  const original=Buffer.from(serializeProjectData(restored,row.file));
  assert.deepEqual(pin(original),{bytes:row.bytes,sha256:row.sha256});
  assert.equal(original.toString('utf8'),row.sourceText,'every complete predecessor receipt byte is restored');
  return restored;
}
`;

/** Preserve all emitted algorithms while recording the corrected library bytes. */
export function generateLibraryRelocationHistory({check=false}={}) {
  const before=original(libraryFile,libraryRevision),old=readProjectDataFromSource(before.sourceText);
  const current=readProjectData(new URL(libraryFile,root)),changes=[],modules=[];
  assert.equal(old.library.files.length,current.library.files.length);
  const inverse=structuredClone(current);
  for(const [index,entry]of current.library.files.entries()){
    const previous=old.library.files[index];assert.equal(entry.path,previous.path);
    const file='rust/src/translation/'+entry.path;
    assert.deepEqual(pin(readFileSync(new URL(file,root))),{bytes:entry.bytes,sha256:entry.sourceSha256});
    if(JSON.stringify(entry)===JSON.stringify(previous))continue;
    const module=original(file,libraryRevision),actual=readFileSync(new URL(file,root),'utf8');
    assert.deepEqual({bytes:module.bytes,sha256:module.sha256},{bytes:previous.bytes,sha256:previous.sourceSha256});
    const edits=tokenize(module.sourceText,'Rust').tokens.filter(token=>token.kind==='string'&&token.raw.includes('self::ml_array::'))
      .map(token=>({start:token.start,end:token.end,from:token.raw,to:token.raw.replaceAll('self::ml_array::','crate::ml_array::')}));
    assert.equal(edits.length,1,'the one known source-producing library literal is corrected');
    let derived=module.sourceText;for(const edit of [...edits].reverse())derived=derived.slice(0,edit.start)+edit.to+derived.slice(edit.end);
    assert.equal(derived,actual,'every other complete native module byte remains unchanged');
    modules.push({...module,current:pin(actual),offsetUnit:'UTF-16 code units',edits});
    for(const field of ['bytes','sourceSha256'])if(entry[field]!==previous[field]){
      changes.push({path:['library','files',String(index),field],before:previous[field],after:entry[field]});
      inverse.library.files[index][field]=previous[field];
    }
  }
  assert.equal(modules.length,1);assert.equal(changes.length,2);assert.deepEqual(inverse,old,'all checked algorithms archives inventories and mappings stay exact');
  assert.equal(serializeProjectData(inverse,libraryFile),before.sourceText);
  const library={...before,identity:'collection construction',current:pin(readFileSync(new URL(libraryFile,root))),changes};
  const originalConsuming=original(consumingFile,sourceRevision),consuming=readProjectDataFromSource(originalConsuming.sourceText);
  const descriptions=consuming.predecessor.proof.producerApplicability.sources;
  const positions=descriptions.flatMap((row,index)=>row.file===libraryFile?[index]:[]);assert.equal(positions.length,1);
  const index=positions[0];assert.deepEqual(descriptions[index],{file:libraryFile,bytes:before.bytes,sha256:before.sha256});
  const consumingChanges=[];
  for(const field of ['bytes','sha256'])if(descriptions[index][field]!==library.current[field]){
    consumingChanges.push({path:['predecessor','proof','producerApplicability','sources',String(index),field],
      before:descriptions[index][field],after:library.current[field]});
    descriptions[index][field]=library.current[field];
  }
  assert.ok(consumingChanges.length>0);const consumingText=serializeProjectData(consuming,consumingFile);
  const rows=[library,{...originalConsuming,identity:'raw consuming',current:pin(consumingText),changes:consumingChanges}];
  const sourceAdapters=['js/scripts/complete-declaration-receipts.mjs','js/scripts/current-translation-source-descriptors.mjs'].map(file=>{
    const row=original(file,sourceRevision),actual=readFileSync(new URL(file,root),'utf8');
    let start=0,end=row.sourceText.length,finish=actual.length;
    while(start<end&&start<finish&&row.sourceText[start]===actual[start])start++;
    while(end>start&&finish>start&&row.sourceText[end-1]===actual[finish-1]){end--;finish--;}
    const edit={offsetUnit:'UTF-16 code units',start,end,removed:row.sourceText.slice(start,end),inserted:actual.slice(start,finish)};
    assert.equal(actual.slice(0,start)+edit.removed+actual.slice(start+edit.inserted.length),row.sourceText);
    return{...row,current:pin(actual),edit};
  });
  const policy={schemaVersion:1,sourceRevision,sources:rows,modules,sourceAdapters,
    boundary:'An exact native library string-literal correction and separately identified packaging provenance. Complete JavaScript algorithms checked definitions numeric controls archives and ABI remain unchanged. Source history is not native execution or a proof of general concept equivalence.'};
  const text=serializeProjectData(policy,historyFile),helperText=helper.replace('__POLICY_HASH__',pin(text).sha256);
  const actual=readFileSync(new URL(consumingFile,root),'utf8');
  if(check)assert.equal(actual,consumingText);else assert.ok(actual===originalConsuming.sourceText||actual===consumingText,'refuse unrelated consuming artifact edits');
  if(check){checkProjectData(new URL(historyFile,root),policy);assert.equal(readFileSync(new URL(helperFile,root),'utf8'),helperText);}
  else{writeFileSync(new URL(consumingFile,root),consumingText);writeProjectData(new URL(historyFile,root),policy);writeFileSync(new URL(helperFile,root),helperText);}
  return policy;
}

// The same public data reader handles authenticated predecessor snapshots.
function readProjectDataFromSource(source) {
  return readStructuredDataLino(source);
}
import {readStructuredDataLino} from '../../../../js/src/structured-data-links.js';

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const policy=generateLibraryRelocationHistory({check:process.argv.includes('--check')});
  console.log('Preserved both complete original receipts; '+policy.modules.length+' actual native literal correction and exact metadata successors.');
}
