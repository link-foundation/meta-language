// Future source construction/publication only. No represented program,
// assertion library, test runner, compiler or freshness command executes.
import {constants,lstatSync,realpathSync,readFileSync,openSync,closeSync,fstatSync,writeFileSync,mkdirSync,renameSync} from 'node:fs';
import {resolve,relative,dirname,sep,basename} from 'node:path';
import {fileURLToPath} from 'node:url';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {serializeProjectData} from './project-data.mjs';
import {extendCurrentTypedProgramSourceSchema as extendClassOwnerSchema} from './class-owner-schema-extension.mjs';
import {classOwnerSourceProfileReceipt,classOwnerSourceProfilePredecessorRevision} from './class-owner-schema-profile.mjs';
const project=realpathSync(fileURLToPath(new URL('../../',import.meta.url)));
const schemas=['js/src/data/typed-program-links-schema.lino','rust/src/data/typed-program-links-schema.lino'];
const types='rust/src/translation/types.rs',adapter='js/scripts/read-program-type-neutral-defaults.mjs',policy='js/scripts/class-owner-schema-profile.mjs';
const digest=value=>createHash('sha256').update(value).digest('hex');
const fail=message=>{throw new TypeError('class owner source author: '+message);};
export const classOwnerSourceAuthorPrerequisites=[
 ['node','js/scripts/draft-class-owner-source-profiles.mjs','--draft'],
 ['node','js/scripts/generate-typed-program-link-rules.mjs','--draft'],
 ['node','js/scripts/draft-rust-baseline-source-policies.mjs','--draft'],
];
function pathFor(file,{create=false}={}){
 if(![...schemas,types,adapter,policy,classOwnerSourceProfileReceipt].includes(file))fail('unknown project source owner');
 const absolute=resolve(project,file),parts=relative(project,absolute).split(sep);let cursor=project;
 for(let at=0;at<parts.length;at+=1){cursor=resolve(cursor,parts[at]);let stat;try{stat=lstatSync(cursor);}catch(error){if(error.code!=='ENOENT')throw error;if(at<parts.length-1){if(!create)fail('missing project-owned parent');mkdirSync(cursor);stat=lstatSync(cursor);}else return absolute;}
  if(stat.isSymbolicLink()||at<parts.length-1&&!stat.isDirectory()||at===parts.length-1&&!stat.isFile())fail('nonordinary or symlink source owner '+file);
 }
 return absolute;
}
function read(file,limit,{optional=false}={}){
 const absolute=pathFor(file);let descriptor;try{descriptor=openSync(absolute,constants.O_RDONLY|constants.O_NOFOLLOW);}catch(error){if(optional&&error.code==='ENOENT')return null;throw error;}
 try{const stat=fstatSync(descriptor);if(!stat.isFile()||stat.size>limit)fail('source resource boundary '+file);return readFileSync(descriptor);}finally{closeSync(descriptor);}
}
function snapshot(file,bytes,revision){if(bytes.length>(file===types?128*1024:file===adapter?64*1024:256*1024))fail('complete source capture exceeds its finite bound');const source=bytes.toString('utf8');if(!Buffer.from(source).equals(bytes))fail('source is not exact UTF-8');return {file,revision,bytes:bytes.length,sha256:digest(bytes),source};}
function predecessor(file){const bytes=execFileSync('git',['show',classOwnerSourceProfilePredecessorRevision+':'+file],{cwd:project,maxBuffer:256*1024});return snapshot(file,bytes,classOwnerSourceProfilePredecessorRevision);}
function verifySnapshot(value,file,limit){if(!value||value.file!==file||typeof value.source!=='string'||!Number.isInteger(value.bytes)||value.bytes<0||value.bytes>limit||Buffer.byteLength(value.source)!==value.bytes||digest(value.source)!==value.sha256)fail('invalid existing owned publication '+file);}
export function prepareClassOwnerSourceProfiles(){
 const previousNativeTypes=predecessor(types),previousAdapter=predecessor(adapter),currentNativeTypes=snapshot(types,read(types,128*1024),'authored complete successor source'),currentAdapter=snapshot(adapter,read(adapter,64*1024),'authored complete successor source');
 if(previousNativeTypes.sha256!=='89d8ca15420493e3e9210a1b0a5b359d55d8a9821a7445806ea82a98fd99f92f')fail('committed predecessor differs from the retained complete version-two Type guard');
 const rows=schemas.map(file=>{const previous=predecessor(file),value=extendClassOwnerSchema(readStructuredDataLino(previous.source)),source=serializeProjectData(value,file);return {file,previous,current:snapshot(file,Buffer.from(source),'authored complete successor schema')};});
 const receipt={version:3,model:'complete class owner schema and native Type source profile 3',predecessorRevision:classOwnerSourceProfilePredecessorRevision,previousNativeTypes,currentNativeTypes,schemas:rows,ownedSources:[{file:adapter,previous:previousAdapter,current:currentAdapter}],scope:'Exact complete committed predecessor and authored successor source/schema interpretation declarations. Original version-one and version-two bool-default assertions remain intact; version three separately retains the optional class-constructor owner. No represented program, policy assertion, compiler, semantic equivalence or parity observation is asserted.',futurePrerequisites:classOwnerSourceAuthorPrerequisites};
 const sourceBytes=[previousNativeTypes,currentNativeTypes,previousAdapter,currentAdapter,...rows.flatMap(row=>[row.previous,row.current])].reduce((sum,row)=>sum+row.bytes,0);if(sourceBytes>1024*1024)fail('aggregate source closure exceeds 1 MiB');
 const receiptSource=serializeProjectData(receipt,classOwnerSourceProfileReceipt);if(Buffer.byteLength(receiptSource)>4*1024*1024)fail('complete source receipt exceeds 4 MiB');
 const policyBytes=read(policy,64*1024),policySource=policyBytes.toString('utf8'),pin=/const classOwnerSourceProfileReceiptSha256='(PENDING|[a-f0-9]{64})';/gu,matches=[...policySource.matchAll(pin)];if(matches.length!==1)fail('one explicit successor receipt pin is required');
 const existingBytes=read(classOwnerSourceProfileReceipt,4*1024*1024,{optional:true});let existing=null;
 if(matches[0][1]!=='PENDING'){
  if(!existingBytes||digest(existingBytes)!==matches[0][1])fail('preserve changed or missing predecessor receipt');existing=readStructuredDataLino(existingBytes.toString('utf8'));
  if(existing.version!==3||existing.model!==receipt.model||existing.predecessorRevision!==receipt.predecessorRevision||existing.previousNativeTypes?.sha256!==previousNativeTypes.sha256||!Array.isArray(existing.schemas)||JSON.stringify(existing.schemas.map(row=>row.file))!==JSON.stringify(schemas))fail('unknown predecessor publication inventory');
  for(const row of existing.schemas)verifySnapshot(row.current,row.file,256*1024);
  verifySnapshot(existing.currentNativeTypes,types,128*1024);if(existing.currentNativeTypes.source!==currentNativeTypes.source)fail('a changed complete native Type source needs an independent successor profile, not a rewritten version-three guard');
  if(!Array.isArray(existing.ownedSources)||existing.ownedSources.length!==1||existing.ownedSources[0].file!==adapter)fail('unknown predecessor source adapter inventory');verifySnapshot(existing.ownedSources[0].current,adapter,64*1024);if(existing.ownedSources[0].current.source!==currentAdapter.source)fail('a changed source adapter needs an independent successor profile');
  for(const row of rows)if(existing.schemas.find(peer=>peer.file===row.file).current.source!==row.current.source)fail('a changed schema extension needs an independent successor profile');
  if(existingBytes.toString('utf8')!==receiptSource)fail('preserve the complete already published version-three receipt');
 }else if(existingBytes&&existingBytes.toString('utf8')!==receiptSource)fail('preserve unexpected unowned receipt');
 const products=rows.map(row=>({file:row.file,source:row.current.source}));products.push({file:classOwnerSourceProfileReceipt,source:receiptSource});
 for(const row of rows){const actual=read(row.file,256*1024);const accepted=[row.previous.source,row.current.source,...(existing?[existing.schemas.find(peer=>peer.file===row.file).current.source]:[])];if(!accepted.includes(actual.toString('utf8')))fail('preserve edited or unexpected schema output '+row.file);}
 products.push({file:policy,source:policySource.replace(pin,"const classOwnerSourceProfileReceiptSha256='"+digest(receiptSource)+"';")});
 if(products.reduce((sum,row)=>sum+Buffer.byteLength(row.source),0)>5*1024*1024)fail('aggregate publication exceeds 5 MiB');
 return {receipt,products,guards:[{file:types,sha256:currentNativeTypes.sha256},{file:adapter,sha256:currentAdapter.sha256},{file:policy,sha256:digest(policyBytes)},...rows.map(row=>({file:row.file,sha256:digest(read(row.file,256*1024))})),{file:classOwnerSourceProfileReceipt,sha256:existingBytes?digest(existingBytes):null}],futurePrerequisites:classOwnerSourceAuthorPrerequisites};
}
export function publishClassOwnerSourceProfiles(prepared){
 // Re-derive every byte in memory; caller-edited plans never authorize writes.
 const actual=prepareClassOwnerSourceProfiles();if(JSON.stringify(actual)!==JSON.stringify(prepared))fail('publication plan changed before writing');
 for(const guard of actual.guards){const bytes=read(guard.file,guard.file===classOwnerSourceProfileReceipt?4*1024*1024:256*1024,{optional:guard.sha256===null});if((bytes?digest(bytes):null)!==guard.sha256)fail('source changed during publication preparation');}
 const staged=[];try{for(const row of actual.products){const output=pathFor(row.file,{create:true}),temporary=resolve(dirname(output),'.'+basename(output)+'.class-owner-'+process.pid+'-'+staged.length);const descriptor=openSync(temporary,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|constants.O_NOFOLLOW,0o600);try{writeFileSync(descriptor,row.source);}finally{closeSync(descriptor);}staged.push({temporary,output});}
  // Pin is last: an interrupted earlier write cannot admit a partial successor.
  for(const row of staged)renameSync(row.temporary,row.output);
 }catch(error){error.message+='; staged files are retained for source inspection';throw error;}
 return actual;
}
export function draftClassOwnerSourceProfiles({publish=false}={}){if(typeof publish!=='boolean')fail('publication needs an explicit Boolean');const prepared=prepareClassOwnerSourceProfiles();return publish?publishClassOwnerSourceProfiles(prepared):prepared;}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 if(process.argv.slice(2).join(' ')!=='--draft')fail('use --draft');const result=draftClassOwnerSourceProfiles({publish:true});process.stdout.write('Authored '+result.products.length+' complete class owner source/schema products; execution evidence deferred.\n');
}
