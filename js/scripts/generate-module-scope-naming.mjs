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
sources.push('js/src/translation/number-bitwise-rules.js', 'js/src/number-bitwise-program.js',
 'js/scripts/generate-number-bitwise-rules.mjs', 'js/scripts/declaration-dependency-branch-styles.mjs',
 'js/scripts/generate-declaration-dependency-branch-fixture.mjs', 'js/scripts/generate-translation-source-layout.mjs');
sources.push('js/src/translation/semantic-input-rules.js', 'js/src/meta-language-archive.js',
 'js/src/meta-language-archive-cli.js', 'js/src/dynamic-meta-language.js', 'js/src/project-translation.js',
 'js/scripts/generate-semantic-input-rules.mjs', 'js/scripts/draft-project-translation.mjs');
sources.push('js/src/meta-language-archive-rules.js', 'js/scripts/draft-source-library.mjs',
 'js/scripts/generate-meta-language-archive-rules.mjs');
sources.push('js/src/dynamic-meta-language-rules.js', 'js/scripts/generate-dynamic-meta-language-rules.mjs');
sources.push('js/src/project-translation-rules.js', 'js/scripts/generate-project-translation-rules.mjs');
sources.push('js/src/syntax-projection.js', 'js/src/syntax-projection-rules.js', 'js/scripts/generate-syntax-projection-rules.mjs');
sources.push('js/src/translation/fixed-integer-cast-rules.js', 'js/src/translation/javascript-fixed-integer-casts.js', 'js/src/fixed-integer-cast-program.js', 'js/scripts/generate-fixed-integer-cast-rules.mjs');
sources.push('js/src/grammar-inference-rules.js', 'js/src/grammar-inference.js', 'js/scripts/generate-grammar-inference-rules.mjs', 'js/scripts/generate-regular-grammar-inference-fixtures.mjs');
sources.push('js/src/translation/reference-method-rules.js', 'js/src/translation/reference-methods.js', 'js/src/translation/reference-callbacks.js', 'js/src/reference-method-program.js', 'js/scripts/generate-reference-method-rules.mjs', 'js/scripts/generate-reference-method-fixtures.mjs');
sources.push('js/src/translation/collection-access-rules.js', 'js/scripts/generate-collection-access-rules.mjs', 'js/scripts/generate-collection-access-fixtures.mjs');
sources.push('js/src/translation/record-write-rules.js', 'js/src/translation/record-writes.js', 'js/scripts/generate-record-write-rules.mjs', 'js/scripts/generate-record-write-fixtures.mjs');
sources.push('js/src/translation/big-integer-bitwise-rules.js', 'js/scripts/generate-big-integer-bitwise-rules.mjs', 'js/scripts/generate-big-integer-bitwise-fixtures.mjs');
sources.push('js/src/translation/rust-primitive-bitwise-rules.js', 'js/src/translation/rust-primitive-bitwise.js', 'js/scripts/generate-rust-primitive-bitwise-rules.mjs', 'js/scripts/generate-rust-primitive-bitwise-fixtures.mjs');
sources.push('js/src/translation/array-state-method-rules.js', 'js/src/translation/array-state-methods.js', 'js/scripts/generate-array-state-method-rules.mjs', 'js/scripts/generate-array-state-method-fixtures.mjs');
sources.push('js/src/translation/rust-shift-rules.js', 'js/src/translation/rust-shifts.js', 'js/scripts/generate-rust-shift-rules.mjs', 'js/scripts/generate-rust-shift-fixtures.mjs');
sources.push('js/src/resource-description-address-rules.js', 'js/src/resource-description-identifiers.js', 'js/scripts/generate-resource-description-address-rules.mjs', 'js/scripts/generate-resource-description-address-fixtures.mjs');
sources.push('js/src/link-cli-archive-topology-rules.js', 'js/src/link-cli-archive-command.js');
sources.push('js/src/translation/javascript-constructor-field-type-rules.js');
sources.push('js/src/grammar-scanner-inference-rules.js', 'js/src/grammar-scanner-inference.js');
sources.push('js/src/grammar-scanner-character-class-rules.js', 'js/src/grammar-scanner-mode-stack-rules.js',
 'js/src/grammar-structural-inference-rules.js', 'js/src/grammar-structural-inference.js');
sources.push('js/src/grammar-attribute-inference-rules.js', 'js/src/grammar-attribute-inference.js');
const archiveOperationNames = {
 readLinkCliArchiveStoreName: 'read link command line interface archive store name',
 readLinkCliArchiveStateName: 'read link command line interface archive state name',
 acceptLinkCliArchiveDirectory: 'accept link command line interface archive directory',
 planLinkCliArchiveDoublets: 'plan link command line interface archive endpoint pairs',
 readLinkCliArchiveDoublets: 'read link command line interface archive endpoint pairs',
 linkCliPacketLimits: 'limit link command line interface packet size',
 metaLanguageArchiveToLinkCliStore: 'encode meta language archive as link command line interface store',
 readMetaLanguageArchiveLinkCliStore: 'read meta language archive from link command line interface store',
 readArchiveCommandEncodings: 'read archive command formats',
 runLinkCliArchiveCommand: 'run link command line interface archive command',
};
const qualifiedOperationNames = {
 'js/src/dynamic-meta-language.js': { configuration: 'configure dynamic meta language' },
 'js/src/grammar-inference.js': { configuration: 'configure grammar inference' },
 'js/src/grammar-scanner-inference.js': {
  inferScanner: 'infer scanner from token observations',
  readScannerListing: 'read inferred scanner grammar listing',
 },
 'js/src/grammar-structural-inference.js': {
  limit: 'limit structural grammar inference resources',
  sourceText: 'validate structural grammar source text',
  indexObservations: 'index structural grammar observations',
  grammarFromProductions: 'build grammar from structural productions',
  replayStructuralRefinementHistory: 'replay structural grammar refinement history',
  buildStructuralGrammarInference: 'build structural grammar inference checkpoint',
  readStructuralCheckpoint: 'read structural grammar inference checkpoint',
 },
 'js/src/grammar-structural-inference-rules.js': {
  testSameGrammarStructuralProduction: 'compare structural grammar productions',
 },
 'js/src/grammar-attribute-inference.js': {
  selectTraceNode: 'select grammar attribute trace node',
  actionGrammar: 'build grammar with inferred attribute action',
 },
};
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const declarationRevisions=[
 ['2e1af9f2a6f31a829981a6bcb4faea95e9d3bf88',[
  ['js/src/grammar-inference.js',['samples','grammarFromTable','inferRegularGrammar','readRegularGrammarInferenceLinks']],
 ]],
 ['e04c5a0648546df65e9eba38a79b59fbfe781e14',[
  ['js/src/grammar-inference.js',['inferRegularGrammar','readRegularGrammarInferenceLinks']],
  ['js/src/grammar-scanner-inference.js',['inferGrammarScanner']],
  ['js/scripts/draft-source-library.mjs',['draftSourceLibrary']],
 ]],
 ['5157d52a189d66f07c45bcf6865a1f7ad6fa3179',[
  ['js/src/grammar-structural-inference.js',['inferStructuralGrammar','readStructuralGrammarInferenceLinks']],
 ]],
 ['6f59fe37b3fb0061a46c0efd5b1a2682582501ef',[
  ['js/scripts/draft-source-library.mjs',['draftSourceLibrary']],
 ]],
 ['31e0006ea8c06484dc30e6a2c529a5166e1bc504',[
  ['js/scripts/draft-source-library.mjs',['draftSourceLibrary']],
  ['js/src/translation/record-write-rules.js',['readRecordCompoundBinaryOperation']],
  ['js/src/translation/record-writes.js',['checkRecordFieldWrite','checkSourceRecordFieldWrite']],
 ]],
];
function retainQualifiedDeclarationRevisions(rows,expected){
 const path='parity/naming/module-scope-bindings-registration.lino';
 const historical=declarationRevisions.flatMap(([revision,sources])=>{
  const captured=captureGitArtifactCertificate(revision,path);
  verifyGitArtifactCertificate(captured.certificate,captured.blob,{revision,path});
  const additions=readBootstrapStructuredDataLino(captured.blob.toString('utf8')).additions;
  return sources.flatMap(([source,names])=>names.map(name=>{
   const row=additions.find(item=>item.source===source&&item.name===name);
   assert.ok(row,'missing certified declaration '+name);
   return row;
  }));
 });
 return rows.map((row,index)=>{
  const earlier=historical.find(value=>value.id===row.id&&value.source===row.source&&value.sourceText===row.sourceText);
  if(!earlier)return row;
  const current=expected[index];
  assert.deepEqual(row,earlier);
  assert.deepEqual({...current,sourceText:earlier.sourceText},earlier);
  return current;
 });
}
function operations(){
 const retainedReaderReceipt='parity/naming/module-scope-bindings-registration.lino';
 const prior=readBootstrapStructuredDataLino(readFileSync(new URL(retainedReaderReceipt,root),'utf8'));
 const previousHistoricalReader=prior.additions.find(row=>row.id==='translation.read-historical-naming-document-text');
 assert.equal(previousHistoricalReader.source,'js/scripts/historical-naming-inventory-storage.mjs');
 assert.equal(previousHistoricalReader.name,'readHistoricalNamingDocumentText');
 assert.equal(pin(previousHistoricalReader.sourceText).sha256,'4ee313872b26368fbb19ebdc134e2b017f9caee9d9e0af86531f025a3f436499');
 return sources.flatMap(source=>{
  const text=readFileSync(new URL(source,root),'utf8'),tokens=tokenize(text,'JavaScript').tokens,kinds=tokens.map(token=>token.kind),values=tokens.map(token=>token.value),rows=[];
  for(let at=0;at<tokens.length;at++){
   if(tokens[at].kind!=='identifier'||tokens[at].value!=='function'||tokens[at+1]?.kind!=='identifier')continue;
   const name=tokens[at+1].value,range=findCompleteModuleScopeDeclaration(kinds,values,at);assert.equal(range.length,2,'every named operation has its complete formal list and body');
   const end=range[1];let declaration=at;if(tokens[declaration-1]?.value==='async')declaration--;if(tokens[declaration-1]?.value==='export')declaration--;const start=tokens[declaration].start;
   const sourceText=text.slice(start,tokens[end-1].end),phrase=qualifiedOperationNames[source]?.[name]??archiveOperationNames[name]??name.replace(/([a-z])([A-Z])/gu,'$1 $2').toLowerCase();
   const selected=source==='experiments/original-link-complete-table-projection/raw-source-archive-workers.mjs'?['runOriginalLinkRawArchivePreparationJobs','prepareOriginalLinkRawSourceArchives'].includes(name):source!==previousHistoricalReader.source||name==='readBoundedHistoricalNamingDocumentText';
   if(selected)rows.push({id:'translation.'+phrase.split(' ').join('-'),source,name,inventoryName:phrase,sourceText});at=end-1;
  }
  if(source===previousHistoricalReader.source)rows.push(previousHistoricalReader);
  return rows;
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
 assert.ok(rows.length>0);
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
  const evolved=section==='bindings'?retainQualifiedDeclarationRevisions(retained,expected):retained;
  assert.deepEqual(evolved,expected,'registered identities and their certified declaration revisions must survive');
  list.splice(first);
 }
 const oldCount=list.length;assert.deepEqual(pin(serializeProjectData(document,file)),original.expected,'every preceding register byte and ordered row stays exact');
 const ids=new Set(list.map(row=>section==='canonical'?row.id:JSON.stringify([row.source,row.name])));
 for(const row of additions){const key=section==='canonical'?row.id:JSON.stringify([row.source,row.name]);assert.ok(!ids.has(key),'duplicate source identity '+key+' in '+(row.source??row.sourceAliases[0].source));ids.add(key);}
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
