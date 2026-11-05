// Complete source authoring/publication only. No rule, assertion, represented
// program, compiler, test runner or validation command executes here.
import {constants,lstatSync,realpathSync,openSync,closeSync,fstatSync,readFileSync,writeFileSync,mkdirSync,linkSync,renameSync} from 'node:fs';
import {resolve,dirname,relative,sep,basename} from 'node:path';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {structuredDataToLino,readStructuredDataLino} from '../src/structured-data-links.js';
import {draftNativePolicyAssertionControls,draftNativePolicyAssertionBatch} from './draft-native-policy-assertion-controls.mjs';
import {rustBaselineNativePolicyControlGroups} from './rust-baseline-native-policy-control-groups.mjs';

const project=realpathSync(fileURLToPath(new URL('../../',import.meta.url)));
const namespace='parity/self-translation/rust-baseline-native-policy-controls/';
export const rustBaselineNativePolicyPublicationManifest=namespace+'publication.lino';
export const rustBaselineNativePolicyRegistration='rust/tests/generated/rust_baseline_native_policy_assertions.rs';
const digest=value=>createHash('sha256').update(value).digest('hex');
const model='owned complete native policy assertion publication 1';
const cap=(value,bound,message)=>{if(value>bound)throw new RangeError(message);};
const coordinate=row=>row.fixture+'\0'+row.exportName+'\0'+row.index;
const artifactPath=path=>typeof path==='string'&&(/^rust\/tests\/generated\/[a-z][a-z_0-9]*\.rs$/u.test(path)||/^parity\/self-translation\/rust-baseline-native-policy-controls\/[a-z][a-z_0-9]*\.lino$/u.test(path));
const pin=file=>({path:file.path,bytes:Buffer.byteLength(file.source),sha256:digest(file.source)});

// The original mixed fixture remains intact; neither owner invents imports.
export const rustBaselineMixedPredicateControlGroups=[
 {sourcePath:'js/src/translation/rust-owned-standard-sum-predicate-rules.js',moduleName:'authored_owned_sum_predicate_admission_controls',testName:'all_literal_owned_sum_predicate_admission_controls',fixtureSelections:[{path:'js/tests/fixtures/rust-owned-standard-sum-predicates.mjs',exportName:'rustOwnedStandardSumPredicateDecisions',operations:['readRustOwnedStandardSumPredicateResultError']}]},
 {sourcePath:'js/src/translation/rust-standard-sum-predicate-rules.js',moduleName:'authored_owned_sum_predicate_branch_controls',testName:'all_literal_owned_sum_predicate_branch_controls',fixtureSelections:[{path:'js/tests/fixtures/rust-owned-standard-sum-predicates.mjs',exportName:'rustOwnedStandardSumPredicateDecisions',operations:['readRustStandardSumPredicateArity','readRustStandardSumPredicateCaptureRoles','readRustStandardSumPredicateCallbackArgument','readRustStandardSumPredicateBranch','readRustStandardSumPredicateVersionError']}]},
];

export function prepareRustBaselineNativePolicyControls(){
 const products=[],files=[],expected=new Map(),assigned=new Map(),moduleNames=new Set(),inputs=new Map();let rustBytes=0,closureBytes=0,receiptBytes=0;
 function append(product){
  const moduleName=basename(product.output,'.rs');if(moduleNames.has(moduleName))throw new TypeError('duplicate baseline policy module registration');moduleNames.add(moduleName);
  rustBytes+=Buffer.byteLength(product.rust);closureBytes+=product.receipt.sourceClosure.bytes;receiptBytes+=Buffer.byteLength(product.receiptLino);cap(rustBytes,16*1024*1024,'baseline native policy Rust outputs exceed 16 MiB');cap(closureBytes,16*1024*1024,'baseline native policy source closures exceed 16 MiB');cap(receiptBytes,32*1024*1024,'baseline native policy module receipts exceed 32 MiB');
  for(const input of [...product.receipt.sourceClosure.inputs,...product.receipt.fixtures]){const row={path:input.path,bytes:input.bytes,sha256:input.sourceSha256},previous=inputs.get(row.path);if(previous&&(previous.bytes!==row.bytes||previous.sha256!==row.sha256))throw new TypeError('baseline native policy source input changed between groups');inputs.set(row.path,row);cap(inputs.size,256,'baseline native policy publication exceeds 256 source inputs');}
  for(const fixture of product.receipt.fixtures)for(const row of fixture.controls){const key=coordinate(row),previous=expected.get(key);if(previous&&JSON.stringify(previous)!==JSON.stringify(row))throw new TypeError('original policy fixture coordinate has conflicting literal source');expected.set(key,row);}
  for(const row of product.receipt.controls){const key=coordinate(row);if(assigned.has(key))throw new TypeError('baseline native policy coordinate is assigned more than once');assigned.set(key,{row,owner:product.receipt.sourceRoot,moduleName,fixtureSha256:product.receipt.fixtures.find(fixture=>fixture.path===row.fixture&&fixture.exportName===row.exportName).sourceSha256});}
  products.push(product);files.push({path:product.output,source:product.rust},{path:namespace+moduleName+'.lino',source:product.receiptLino});
 }
 for(const group of rustBaselineNativePolicyControlGroups)append(draftNativePolicyAssertionControls(group));
 const mixed=draftNativePolicyAssertionBatch({groups:rustBaselineMixedPredicateControlGroups});for(const product of mixed.modules)append(product);receiptBytes+=Buffer.byteLength(mixed.receiptLino);cap(receiptBytes,32*1024*1024,'baseline native policy receipts exceed 32 MiB');files.push({path:namespace+'predicate_assignment.lino',source:mixed.receiptLino});
 for(const key of expected.keys())if(!assigned.has(key))throw new TypeError('baseline native policy coordinate has no generated assertion assignment: '+key);
 for(const key of assigned.keys())if(!expected.has(key))throw new TypeError('baseline native policy assignment has no original literal coordinate');
 const registration='// Generated complete policy assertion modules. Do not edit by hand.\n'+products.map(product=>'#[path = "'+basename(product.output)+'"]\nmod '+basename(product.output,'.rs')+';').join('\n')+'\n';rustBytes+=Buffer.byteLength(registration);cap(rustBytes,16*1024*1024,'baseline native policy Rust outputs including registration exceed 16 MiB');files.push({path:rustBaselineNativePolicyRegistration,source:registration});
 const receipt={schemaVersion:1,model:'complete baseline native policy literal assignment authoring 1',groups:rustBaselineNativePolicyControlGroups,mixedGroups:rustBaselineMixedPredicateControlGroups,inputs:[...inputs.values()],controls:[...assigned.values()].map(({row,owner,moduleName,fixtureSha256})=>({fixture:row.fixture,exportName:row.exportName,index:row.index,operation:row.operation,args:row.args,expected:row.expected,owner,moduleName,fixtureSha256})),files:files.map(pin)};
 const receiptLino=structuredDataToLino(receipt);receiptBytes+=Buffer.byteLength(receiptLino);cap(receiptBytes,32*1024*1024,'baseline native policy author receipts exceed 32 MiB');files.push({path:namespace+'author.lino',source:receiptLino});
 cap(files.length,128,'baseline native policy publication exceeds 128 artifacts');cap(files.reduce((bytes,file)=>bytes+Buffer.byteLength(file.source),0),64*1024*1024,'baseline native policy publication exceeds 64 MiB');
 return {files,products,receipt,receiptLino,registration,inputs:[...inputs.values()],resources:{rustBytes,closureBytes,receiptBytes},manifestPath:rustBaselineNativePolicyPublicationManifest};
}

/** Pure complete ownership preflight; current entries are filesystem facts,
 * never a caller's assertion that an existing path is safe to overwrite. */
export function planNativePolicyControlPublication({files,previous=null,current}){
 if(!Array.isArray(files)||files.length===0||files.length>128||!(current instanceof Map))throw new TypeError('complete bounded publication files and current path facts are required');
 const desired=new Map(),owned=new Map();let bytes=0,rustBytes=0,receiptBytes=0;
 for(const file of files){if(!artifactPath(file.path)||file.path===rustBaselineNativePolicyPublicationManifest||typeof file.source!=='string'||desired.has(file.path))throw new TypeError('publication needs unique task-owned artifact paths');const size=Buffer.byteLength(file.source);cap(size,(file.path.endsWith('.rs')?8:32)*1024*1024,'native policy artifact exceeds its per-file budget');bytes+=size;if(file.path.endsWith('.rs'))rustBytes+=size;else receiptBytes+=size;cap(rustBytes,16*1024*1024,'native policy Rust publication exceeds 16 MiB');cap(receiptBytes,32*1024*1024,'native policy receipt publication exceeds 32 MiB');cap(bytes,64*1024*1024,'baseline native policy publication exceeds 64 MiB');desired.set(file.path,pin(file));}
 if(previous!==null){
  if(previous.schemaVersion!==1||previous.model!==model||!Array.isArray(previous.files)||previous.files.length>128)throw new TypeError('preserve an unrecognized native policy publication manifest');
  let previousBytes=0;for(const row of previous.files){if(!artifactPath(row.path)||row.path===rustBaselineNativePolicyPublicationManifest||owned.has(row.path)||!Number.isSafeInteger(row.bytes)||row.bytes<0||typeof row.sha256!=='string'||!/^[a-f0-9]{64}$/u.test(row.sha256))throw new TypeError('preserve an invalid native policy artifact ownership row');previousBytes+=row.bytes;cap(previousBytes,64*1024*1024,'native policy predecessor ownership exceeds 64 MiB');owned.set(row.path,row);}
 }
 const writes=[];
 for(const path of new Set([...owned.keys(),...desired.keys()])){
  const state=current.get(path),prior=owned.get(path),next=desired.get(path);
  if(!state||!['missing','file'].includes(state.kind))throw new TypeError('preserve a symlink, nonregular or unclassified publication path: '+path);
  if(prior){if(state.kind!=='file'||state.bytes!==prior.bytes||state.sha256!==prior.sha256)throw new TypeError('preserve an edited or missing predecessor policy artifact: '+path);}
  else if(state.kind!=='missing')throw new TypeError('preserve an unexpected unowned policy artifact: '+path);
  if(next){if(state.kind==='missing'||state.sha256!==next.sha256||state.bytes!==next.bytes)writes.push(path);owned.set(path,next);}
 }
 cap(owned.size,128,'retained native policy ownership exceeds 128 artifacts');
 return {writes,manifest:{schemaVersion:1,model,files:[...owned.values()].sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0)}};
}

function stat(path){try{return lstatSync(path);}catch(error){if(error.code==='ENOENT')return null;throw error;}}
function ancestors(path,{create=false}={}){
 const parts=path.split('/');let current=project;
 for(const part of parts.slice(0,-1)){current=resolve(current,part);let entry=stat(current);if(!entry&&create){mkdirSync(current);entry=lstatSync(current);}if(!entry){if(create)throw new TypeError('failed to create task-owned policy output directory');return false;}if(entry.isSymbolicLink()||!entry.isDirectory()||realpathSync(current)!==current)throw new TypeError('preserve a non-directory or symlinked native policy output ancestor');}
 return true;
}
function readState(path,maxBytes=32*1024*1024,{probe=false}={}){
 if(!ancestors(path))return {kind:'missing'};const absolute=resolve(project,path),entry=stat(absolute);if(!entry)return {kind:'missing'};
 if(entry.isSymbolicLink()||!entry.isFile()||realpathSync(absolute)!==absolute)return {kind:'unsafe'};cap(entry.size,maxBytes,'native policy publication input exceeds its byte budget');
 if(probe)return {kind:'file',bytes:entry.size};
 if(typeof constants.O_NOFOLLOW!=='number')throw new TypeError('native policy publication requires non-following file opens');
 const fd=openSync(absolute,constants.O_RDONLY|constants.O_NOFOLLOW);try{const opened=fstatSync(fd);if(!opened.isFile()||opened.ino!==entry.ino||opened.dev!==entry.dev)throw new TypeError('preserve a concurrently replaced native policy publication path');const source=readFileSync(fd,'utf8');return {kind:'file',source,bytes:Buffer.byteLength(source),sha256:digest(source)};}finally{closeSync(fd);}
}
function sameState(left,right){return left.kind===right.kind&&(left.kind!=='file'||left.bytes===right.bytes&&left.sha256===right.sha256);}

export function publishRustBaselineNativePolicyControls(prepared){
 if(prepared.manifestPath!==rustBaselineNativePolicyPublicationManifest)throw new TypeError('publication must retain its exact owned manifest path');
 if(!Array.isArray(prepared.inputs)||prepared.inputs.length>256)throw new TypeError('native policy publication needs its complete bounded source pins');
 for(const input of prepared.inputs){if(typeof input.path!=='string'||input.path.length>1024||input.path.startsWith('/')||input.path.includes('\\')||input.path.split('/').some(part=>!part||part==='.'||part==='..'))throw new TypeError('native policy source pins need normalized project paths');const state=readState(input.path,16*1024*1024);if(state.kind!=='file'||state.bytes!==input.bytes||state.sha256!==input.sha256)throw new TypeError('preserve source inputs changed since native policy authoring');}
 const manifestState=readState(rustBaselineNativePolicyPublicationManifest,1024*1024);if(manifestState.kind==='unsafe')throw new TypeError('preserve a symlinked or nonregular policy publication manifest');
 const previous=manifestState.kind==='file'?readStructuredDataLino(manifestState.source):null,current=new Map();
 if(previous!==null&&(!Array.isArray(previous.files)||previous.files.length>128))throw new TypeError('preserve an unrecognized native policy publication manifest');
 const paths=new Set([...prepared.files.map(file=>file.path),...(previous?.files??[]).map(file=>file.path)]);cap(paths.size,128,'native policy current paths exceed 128 artifacts');let currentBytes=0;
 for(const path of paths){if(!artifactPath(path))throw new TypeError('preserve an artifact outside the native policy publication namespace');const state=readState(path,32*1024*1024,{probe:true});currentBytes+=state.bytes??0;cap(currentBytes,64*1024*1024,'native policy current artifact inputs exceed 64 MiB');}
 for(const path of paths){const {source,...state}=readState(path);current.set(path,state);}
 const publication=planNativePolicyControlPublication({files:prepared.files,previous,current}),manifestSource=structuredDataToLino(publication.manifest);cap(Buffer.byteLength(manifestSource),1024*1024,'native policy publication manifest exceeds 1 MiB');
 cap(prepared.resources.receiptBytes+Buffer.byteLength(manifestSource),32*1024*1024,'native policy receipts including publication manifest exceed 32 MiB');
 const files=new Map(prepared.files.map(file=>[file.path,file.source]));if(manifestState.kind==='missing'||manifestState.source!==manifestSource)files.set(rustBaselineNativePolicyPublicationManifest,manifestSource);
 const pending=publication.writes.map(path=>({path,source:files.get(path)}));if(files.has(rustBaselineNativePolicyPublicationManifest))pending.push({path:rustBaselineNativePolicyPublicationManifest,source:manifestSource});
 // All path ownership checks finish before staging or replacing any artifact.
 for(const path of current.keys())if(!sameState(current.get(path),readState(path)))throw new TypeError('preserve a policy artifact changed during publication preflight');if(!sameState(manifestState,readState(rustBaselineNativePolicyPublicationManifest,1024*1024)))throw new TypeError('preserve a policy manifest changed during publication preflight');
 const transaction=digest(manifestSource),staged=[];
 for(const file of pending){
  const stage=namespace+'.staging/'+transaction+'/'+digest(file.path+'\0'+digest(file.source))+'.stage';ancestors(stage,{create:true});const stageState=readState(stage);if(stageState.kind==='file'){if(stageState.source!==file.source)throw new TypeError('preserve an edited native policy staging artifact');}else if(stageState.kind==='missing'){const fd=openSync(resolve(project,stage),constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|constants.O_NOFOLLOW,0o600);try{writeFileSync(fd,file.source);}finally{closeSync(fd);}}else throw new TypeError('preserve an unsafe native policy staging artifact');staged.push({...file,stage});
 }
 for(const file of staged){
  ancestors(file.path,{create:true});const before=file.path===rustBaselineNativePolicyPublicationManifest?manifestState:current.get(file.path),now=readState(file.path);if(!sameState(before,now))throw new TypeError('preserve a concurrently changed native policy publication target');
  // Exclusive links cannot replace an unexpected new file. Existing owned
  // replacements use atomic rename after the complete content recheck.
  if(now.kind==='missing')linkSync(resolve(project,file.stage),resolve(project,file.path));else renameSync(resolve(project,file.stage),resolve(project,file.path));
 }
 return {manifest:publication.manifest,manifestPath:rustBaselineNativePolicyPublicationManifest,published:pending.map(file=>file.path),retained:publication.manifest.files.filter(file=>!publication.writes.includes(file.path)).map(file=>file.path)};
}
export function draftRustBaselineNativePolicyControls({publish=false}={}){if(typeof publish!=='boolean')throw new TypeError('native policy publication selection must be explicit Boolean');const prepared=prepareRustBaselineNativePolicyControls();return {...prepared,...(publish?{publication:publishRustBaselineNativePolicyControls(prepared)}:{})};}
if(process.argv[1]===fileURLToPath(import.meta.url)){if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');const result=draftRustBaselineNativePolicyControls({publish:true});process.stdout.write('Authored '+result.products.length+' complete native policy assertion modules; no assertions or represented programs executed.\n');}
