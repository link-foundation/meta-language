// Append source-qualified operations without changing preceding identities.
import assert from 'node:assert/strict';
import {existsSync,readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {tokenize} from '../src/translation/lexer.js';
import {readProjectData,writeProjectData,checkProjectData,serializeProjectData} from './project-data.mjs';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {NAMING_SOURCE_BINDING_CODEC_POLICY} from './naming-source-binding-codec-profile.mjs';
import {inspectBootstrapStructuredDataLino,readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {findCompleteModuleScopeDeclaration,restoreModuleScopeDeclarationBindings} from './module-scope-declaration-history.mjs';

const root=new URL('../../',import.meta.url);
const revision='d3e4076667add3e19e4cf5f10a3cea60255dd2d7';
const sources=['js/src/translation/inert-module-binding-rules.js','js/src/translation/inert-module-bindings.js','js/src/translation/rust-shared-array-handle-rules.js','js/src/translation/empty-module-collection-rules.js','js/src/translation/empty-module-collections.js','js/scripts/native-compiler-source-styles.mjs','js/src/translation/numeric-module-binding-rules.js','js/src/translation/numeric-module-bindings.js','js/scripts/borrowed-javascript-body-source-history.mjs','experiments/original-link-complete-table-projection/raw-source-archive-workers.mjs','js/scripts/native-compiler-host-source-history.mjs','js/scripts/numeric-module-binding-source-styles.mjs','js/scripts/generate-borrowed-javascript-body-source-history.mjs','js/scripts/generate-native-compiler-host-source-history.mjs','js/scripts/historical-naming-inventory-storage.mjs','js/scripts/crate-fixture-package-source-history.mjs','js/scripts/module-scope-declaration-history.mjs','js/scripts/generate-module-scope-declaration-history.mjs','experiments/original-link-complete-table-projection/raw-source-archive-validation.mjs','js/scripts/historical-naming-source-preflight.mjs'];
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
function operations(){
 return sources.flatMap(source=>{
  const text=readFileSync(new URL(source,root),'utf8'),tokens=tokenize(text,'JavaScript').tokens,kinds=tokens.map(token=>token.kind),values=tokens.map(token=>token.value),rows=[];
  for(let at=0;at<tokens.length;at++){
   if(tokens[at].kind!=='identifier'||tokens[at].value!=='function'||tokens[at+1]?.kind!=='identifier')continue;
   const name=tokens[at+1].value,range=findCompleteModuleScopeDeclaration(kinds,values,at);assert.equal(range.length,2,'every named operation has its complete formal list and body');
   const end=range[1];let declaration=at;if(tokens[declaration-1]?.value==='async')declaration--;if(tokens[declaration-1]?.value==='export')declaration--;const start=tokens[declaration].start;
   const sourceText=text.slice(start,tokens[end-1].end),phrase=name.replace(/([a-z])([A-Z])/gu,'$1 $2').toLowerCase();
   const selected=source==='experiments/original-link-complete-table-projection/raw-source-archive-workers.mjs'?['runOriginalLinkRawArchivePreparationJobs','prepareOriginalLinkRawSourceArchives'].includes(name):source!=='js/scripts/historical-naming-inventory-storage.mjs'||name==='readHistoricalNamingDocumentText';
   if(selected)rows.push({id:'translation.'+phrase.split(' ').join('-'),source,name,inventoryName:phrase,sourceText});at=end-1;
  }
  assert.ok(rows.length>0);return rows;
 });
}
function records(rows){return rows.map(row=>({id:row.id,phrase:row.inventoryName,role:'operation',
 definition:'Apply the complete source-qualified '+row.inventoryName+' operation declared in '+row.source+' with its actual imports and explicit refusal domain.',
 constraints:['The complete declaration and source module remain the definition; equal names alone do not prove equivalence.',
 'The source-derived checked Links program and generated native rules retain their separate compilation and runtime parity obligations.'],
 sourceAliases:[{source:row.source,name:row.name}],formerNames:[]}));}

/** Each section runs independently to keep large register documents out of the
 * same process. Complete original bytes and ordered rows survive the inverse. */
export function generateModuleScopeNaming({section,check=false}={}){
 assert.ok(['canonical','bindings'].includes(section));const rows=operations();
 const file=section==='canonical'?'parity/naming/canonical-concepts.lino':'parity/naming/source-bindings.lino';
 const receiptFile='parity/naming/module-scope-'+section+'-registration.lino';
 // A new declaration in an existing module must append after all registered
 // identities, rather than move later modules' already published operations.
 if(existsSync(new URL(receiptFile,root))){
  const previous=readBootstrapStructuredDataLino(readFileSync(new URL(receiptFile,root),'utf8'));
  const positions=new Map(previous.additions.map((row,index)=>[row.id,index]));
  assert.equal(positions.size,previous.additions.length);
  for(const id of positions.keys())assert.ok(rows.some(row=>row.id===id),'every registered operation remains present');
  rows.sort((left,right)=>(positions.get(left.id)??positions.size)-(positions.get(right.id)??positions.size));
 }
 let original;
 if(section==='bindings'){
  // Reuse the small, independently certified source profile rather than copying
  // this 77 MB data document through Git's generic 64 MB blob transport.
  const path=NAMING_SOURCE_BINDING_CODEC_POLICY.profile,captured=captureGitArtifactCertificate(revision,path);
  const profile=readBootstrapStructuredDataLino(captured.blob.toString('utf8'));
  const epoch=profile.profiles.at(-1);assert.equal(epoch.source.path,file);
  original={expected:{bytes:epoch.source.bytes,sha256:epoch.source.sha256},
   authority:{path,source:captured.blob.toString('utf8'),certificate:captured.certificate}};
 }else{
  const captured=captureGitArtifactCertificate(revision,file);
  verifyGitArtifactCertificate(captured.certificate,captured.blob,{revision,path:file});
  original={expected:pin(captured.blob),certificate:captured.certificate};
 }
 const text=readFileSync(new URL(file,root),'utf8');
 let document;
 if(section==='bindings'){
  const policy=NAMING_SOURCE_BINDING_CODEC_POLICY;
  document=inspectBootstrapStructuredDataLino(text,{nodeLimit:policy.nodeLimit,depthLimit:policy.depthLimit,byteLimit:policy.expandedByteLimit}).value;
 }else {global.gc?.();document=readBootstrapStructuredDataLino(text);}
 const list=section==='canonical'?document.concepts:document,additions=section==='canonical'?records(rows):rows;
 const first=list.findIndex(row=>row.id===additions[0].id&& (section==='canonical'||row.source===additions[0].source));
 if(first>=0){
  const preceding=list.slice(first);assert.ok(preceding.length<=additions.length);
  const retained=section==='bindings'?restoreModuleScopeDeclarationBindings(preceding):preceding;
  const expected=section==='bindings'?restoreModuleScopeDeclarationBindings(additions.slice(0,preceding.length)):additions.slice(0,preceding.length);
  assert.deepEqual(retained,expected,'every preceding identity and definition survives; only the certified original parameter fragments gain their missing declaration text');
  list.splice(first);
 }
 const oldCount=list.length;assert.deepEqual(pin(serializeProjectData(document,file)),original.expected,'every preceding register byte and ordered row stays exact');
 const ids=new Set(list.map(row=>section==='canonical'?row.id:JSON.stringify([row.source,row.name])));
 for(const row of additions){const key=section==='canonical'?row.id:JSON.stringify([row.source,row.name]);assert.ok(!ids.has(key),'one distinct source identity');ids.add(key);}
 list.push(...additions);global.gc?.();
 if(check)assert.deepEqual(pin(serializeProjectData(document,file)),pin(text),'the complete current register must equal the normal author output');else writeProjectData(new URL(file,root),document);
 const receipt={schemaVersion:1,model:'complete source operation registration 1',revision,file,
  previous:{...original.expected,...(original.certificate?{certificate:original.certificate}:{authority:original.authority}),rows:oldCount},
  current:{...pin(readFileSync(new URL(file,root))),rows:list.length},
  sources:sources.map(path=>({path,...pin(readFileSync(new URL(path,root)))})),additions,
  boundary:'Append-only source-qualified operation records. Removing exactly these additions reconstructs every complete original register byte, row, alias and order. No earlier identity is reassigned and no equal-name equivalence is asserted. Full WordNet naming, native execution and all requirement checks remain mandatory.'};
 if(check)checkProjectData(new URL(receiptFile,root),receipt);else writeProjectData(new URL(receiptFile,root),receipt);
 return{section,previousRows:oldCount,rows:list.length,added:additions.length};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const args=process.argv.slice(2),check=args.at(-1)==='--check';if(check)args.pop();assert.ok(args.length===2&&args[0]==='--section');
 console.log(JSON.stringify(generateModuleScopeNaming({section:args[1],check})));
}
