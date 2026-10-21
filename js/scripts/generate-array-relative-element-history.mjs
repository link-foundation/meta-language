// Refresh current source descriptors without regenerating a recorded native body.
// Whole predecessor files retain Git membership and exact byte inverses.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {checkProjectData,serializeProjectData,writeProjectData} from './project-data.mjs';
import {restoreContinuationHistoricalSource} from './continuation-source-history.mjs';
import {restoreRelativeArrayFrontendDescriptorSource} from './relative-array-frontend-source-history.mjs';

const revision='25e6361f2e31e09f98f78b3c973f242bdf88e31c';
const root=new URL('../../',import.meta.url);
const artifactFile='experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino';
const policyFile='parity/self-translation/current-translation-source-descriptors.lino';
const registerFile='parity/self-translation/complete-declaration-receipts.lino';
const loanFile='experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino';
const descriptors=[artifactFile,policyFile,registerFile,loanFile];
const sourceFiles=['js/src/translation/check.js','js/src/translation/primitive-method-rules.js',
  'js/scripts/generate-primitive-method-rules.mjs','rust/Cargo.toml','rust/src/translation/check.rs',
  'rust/src/translation/check/primitive_methods.rs','rust/src/translation/primitive_method_rules.rs',
  'parity/self-translation/primitive-method-rules-decorators.lino','js/scripts/translation-source-history.mjs',
  'js/scripts/current-translation-source-descriptors.mjs','js/tests/translation-string-relative-character.test.js',
  'rust/tests/unit/translation_primitive_methods.rs',
  'js/scripts/self-translation-layout-generation.mjs','rust/src/translation/self_translation_layout_rules/part_1.rs',
  'parity/self-translation/self-translation-layout-rules-decorators.lino','rust/src/self_translation.rs',
  'experiments/grammar-outcome-network/native-transport/inverse/generate-number-kinds.mjs',
  'rust/src/translation/owned_outcome_number_kind_rules/part_1.rs',
  'parity/self-translation/owned-outcome-number-kind-rules-decorators.lino',
  'rust/tests/unit/grammar_outcome_number_kind_rules.rs',...descriptors];
const digest=value=>createHash('sha256').update(value).digest('hex');
const pin=value=>({bytes:Buffer.byteLength(value),sha256:digest(value)});
const historicalSource=file=>restoreRelativeArrayFrontendDescriptorSource(restoreContinuationHistoricalSource(readFileSync(new URL(file,root),'utf8'),file,'relative array'),file,'relative array');

export function generateArrayRelativeElementHistory({check=false}={}) {
  assert.ok(check||historicalSource(artifactFile)===readFileSync(new URL(artifactFile,root),'utf8'),
    'a historical generator cannot overwrite a newer current artifact; use --check');
  const originals=new Map(sourceFiles.map(file=>{const {certificate,blob}=captureGitArtifactCertificate(revision,file);return[file,{certificate,source:blob.toString('utf8')}];}));
  const original=file=>readStructuredDataLino(originals.get(file).source);
  const artifact=original(artifactFile),previous=original(artifactFile),policy=original(policyFile),register=original(registerFile),loan=original(loanFile);
  const changes=[];
  const refresh=(row,path)=>{
    const current=pin(historicalSource(row.file));
    for(const field of ['bytes','sha256'])if(row[field]!==current[field]){changes.push({path:[...path,field],before:row[field],after:current[field]});row[field]=current[field];}
  };
  artifact.decisionSources.forEach((row,index)=>refresh(row,['decisionSources',String(index)]));
  artifact.predecessor.proof.producerApplicability.sources.forEach((row,index)=>refresh(row,['predecessor','proof','producerApplicability','sources',String(index)]));
  assert.equal(changes.length,2,'only the complete current checker descriptor changes');
  const inverse=structuredClone(artifact);
  for(const change of changes){let owner=inverse;for(const key of change.path.slice(0,-1))owner=owner[key];owner[change.path.at(-1)]=change.before;}
  assert.deepEqual(inverse,previous);assert.equal(serializeProjectData(inverse,artifactFile),originals.get(artifactFile).source);
  for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','mappings','preludeHashes','replays','archive'])assert.deepEqual(artifact[field],previous[field],field);
  const artifactText=serializeProjectData(artifact,artifactFile);
  Object.assign(policy.edge.current,pin(artifactText));
  for(const change of policy.edge.changes){let owner=artifact;for(const key of change.path)owner=owner[key];change.after=structuredClone(owner);}
  assert.equal(policy.edge.changes.length,13);
  register.sourceDescriptorSuccessors=[policy.edge];Object.assign(loan.predecessor,pin(artifactText));
  const descriptorValues=new Map([[artifactFile,artifact],[policyFile,policy],[registerFile,register],[loanFile,loan]]);
  for(const [file,value]of descriptorValues){
    if(check)assert.equal(historicalSource(file),serializeProjectData(value,file),'every complete historical descriptor byte remains exact');
    else writeProjectData(new URL(file,root),value);
  }
  const helperFile=new URL('js/scripts/current-translation-source-descriptors.mjs',root),helper=historicalSource('js/scripts/current-translation-source-descriptors.mjs');
  const expected=helper.replace(/const policyHash='[a-f0-9]{64}';/u,"const policyHash='"+digest(historicalSource(policyFile))+"';");
  if(check)assert.equal(helper,expected);else writeFileSync(helperFile,expected);
  const sources=sourceFiles.map(file=>{
    const {source:sourceText,certificate}=originals.get(file),current=historicalSource(file);
    let start=0,end=sourceText.length,finish=current.length;
    while(start<end&&start<finish&&sourceText[start]===current[start])start++;
    while(end>start&&finish>start&&sourceText[end-1]===current[finish-1]){end--;finish--;}
    const edit={start:Buffer.byteLength(current.slice(0,start)),end:Buffer.byteLength(current.slice(0,finish)),currentText:current.slice(start,finish),previousText:sourceText.slice(start,end)};
    const bytes=Buffer.from(current),restored=Buffer.concat([bytes.subarray(0,edit.start),Buffer.from(edit.previousText),bytes.subarray(edit.end)]);
    assert.equal(restored.toString(),sourceText);assert.deepEqual(pin(restored),pin(sourceText));
    return{file,commit:revision,...pin(sourceText),sourceText,certificate,current:pin(current),edit,role:descriptors.includes(file)?'historical descriptor':'historical source'};
  });
  const history={schemaVersion:1,revision,sources,descriptorChanges:changes,
    assertionCorrection:{file:'js/tests/translation-string-relative-character.test.js',input:'/** @param {number[]} values @returns {number|undefined} */function last(values){return values.at(-1);}',
      authority:'https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.at',languageVersions:{minimum:'2022',maximumExclusive:'2027'},
      reason:'The complete relative array lambda now executes the original standard operation. Keep the original input and predecessor source; require its independently observed present and absent results.'},
    boundary:'Whole predecessor sources and descriptors only. Every recorded native definition ABI owner replay archive and original assertion remains. Current execution and descriptor gates read current data; historical views invert only complete authenticated inputs. No native execution or global equivalence claim.'};
  const file=new URL('parity/self-translation/array-relative-element-history.lino',root);
  if(check)checkProjectData(file,history);else writeProjectData(file,history);
  const adapter=new URL('js/scripts/array-relative-element-history.mjs',root),text=readFileSync(adapter,'utf8');
  const pinned=text.replace(/const historyHash='[a-f0-9]{64}';/u,"const historyHash='"+digest(readFileSync(file))+"';");
  if(check)assert.equal(text,pinned);else writeFileSync(adapter,pinned);
  return history;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const history=generateArrayRelativeElementHistory({check:process.argv.includes('--check')});
  console.log('Retained '+history.sources.length+' complete original files and all native definitions; two checker provenance fields refreshed.');
}
