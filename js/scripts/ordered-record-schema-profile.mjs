// Finite source-profile inverse. Historical guards keep their complete original
// inputs; the current ordered-record profile has its own full source guard.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readStructuredDataLino} from '../src/structured-data-links.js';
const root=new URL('../../',import.meta.url);
const previousSchemaSha256='b1ac91ce373be90d0326f4489dfb9dea8f5b2964c7174322e7d8a532c097ebbf';
const previousNativeTypeSha256='329dfababaff646a3ad870e76abbf026924b8c367cde6b9a5364def05fdd50d0';
const sixthReceiptSha256='d2e37fd76fd37bf72a16f28c20c3a6a0bc50126a42302a77b0a00e3820e18fea';
const currentNativeTypeSha256='89d8ca15420493e3e9210a1b0a5b359d55d8a9821a7445806ea82a98fd99f92f';
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
const fail=message=>{throw new TypeError('ordered record source profile: '+message);};
let retained;
export function readOrderedRecordSchemaPredecessor(){
 if(retained)return retained;
 const value=readStructuredDataLino(readFileSync(new URL('parity/self-translation/ordered-record-schema-predecessor.lino',root),'utf8'));
 if(value.version!==1||value.previousSchema.sha256!==previousSchemaSha256||value.previousNativeTypes.sha256!==previousNativeTypeSha256)fail('unknown predecessor identity');
 if(value.previousSchema.file!=='js/src/data/typed-program-links-schema.lino'||value.previousNativeTypes.file!=='rust/src/translation/types.rs')fail('changed predecessor source owner');
 if(JSON.stringify(value.ownedSources.map(row=>row.file))!==JSON.stringify(['js/scripts/read-program-type-neutral-defaults.mjs','experiments/original-link-complete-table-projection/raw-schema-sixth-extension-closure.mjs']))fail('changed versioned source owner inventory');
 for(const entry of [value.previousSchema,value.previousNativeTypes,...value.ownedSources])if(Buffer.byteLength(entry.source)!==entry.bytes||hash(Buffer.from(entry.source))!==entry.sha256)fail('changed complete predecessor source');
 const receiptBytes=readFileSync(new URL('experiments/original-link-complete-table-projection/data/raw-schema-sixth-extension-closure.lino',root));
 if(hash(receiptBytes)!==sixthReceiptSha256)fail('changed sixth source receipt');
 const sixth=readStructuredDataLino(receiptBytes.toString('utf8'));
 if(sixth.sourceGuard.source.sha256!==previousNativeTypeSha256)fail('native predecessor does not belong to the original source guard');
 for(const entry of value.ownedSources){
  const original=sixth.authoring.sourceFiles.find(row=>row.file===entry.file);
  if(!original||original.bytes!==entry.bytes||original.sha256!==entry.sha256)fail('an original source owner was replaced');
 }
 retained=value;return retained;
}
export function extendOrderedRecordSchema(before){
 const after=structuredClone(before);
 if(after.expressionFields.record.includes('order')||Object.hasOwn(after.neutralTypeFieldDefaults.record,'ordered'))fail('the predecessor already has the new profile');
 after.expressionFields.record.push('order');
 after.expressionOperands.record.push('order');
 after.expressionProjections.record.fields.push(['propertyOrder','optional-expression','order'],['type','copy','type']);
 after.neutralFieldDefaults['expression:record']={order:'null'};
 after.neutralTypeFieldDefaults.record.ordered='false';
 return after;
}
export function restoreOrderedRecordSchema(bytes,file){
 if(!['js/src/data/typed-program-links-schema.lino','rust/src/data/typed-program-links-schema.lino'].includes(file))fail('unknown schema owner');
 if(!Buffer.isBuffer(bytes)||bytes.length>256*1024)fail('invalid schema source boundary');
 if(hash(bytes)===previousSchemaSha256)return bytes.toString('utf8');
 const previous=readOrderedRecordSchemaPredecessor().previousSchema;
 const before=readStructuredDataLino(previous.source),expected=extendOrderedRecordSchema(before),after=readStructuredDataLino(bytes.toString('utf8'));
 if(JSON.stringify(after)!==JSON.stringify(expected))fail('unknown complete current schema edit');
 return previous.source;
}
export function restoreOrderedRecordTypeSource(bytes,file){
 if(file!=='rust/src/translation/types.rs'||!Buffer.isBuffer(bytes)||bytes.length>128*1024)fail('unknown native type owner');
 if(hash(bytes)===previousNativeTypeSha256)return bytes.toString('utf8');
 if(hash(bytes)!==currentNativeTypeSha256)fail('uncovered native type source version');
 return readOrderedRecordSchemaPredecessor().previousNativeTypes.source;
}
function replaceOnce(source,before,after){
 if(source.split(before).length!==2)fail('a bounded source bridge lost its original owner');
 return source.replace(before,after);
}
export function extendOrderedRecordOwnedSource(file,source){
 if(file==='js/scripts/read-program-type-neutral-defaults.mjs'){
  source=replaceOnce(source,"import assert from 'node:assert/strict';","import {restoreOrderedRecordTypeSource} from './ordered-record-schema-profile.mjs';\nimport assert from 'node:assert/strict';");
  source=replaceOnce(source,'export function readProgramTypeNeutralDefaults(){','export function readProgramTypeNeutralDefaults({version=1}={}){\n  if(version!==1&&version!==2)throw new RangeError("uncovered native Type source profile "+version);');
  source=replaceOnce(source,"bytes=readFileSync(new URL(path,root)),source=bytes.toString('utf8');","actualBytes=readFileSync(new URL(path,root)),bytes=version===1?Buffer.from(restoreOrderedRecordTypeSource(actualBytes,path)):actualBytes,source=bytes.toString('utf8');");
  source=replaceOnce(source,"assert.deepEqual(defaults,{array:{reference:'false'},record:{open:'false',reference:'false'},map:{reference:'false'},set:{reference:'false'}});","assert.deepEqual(defaults,version===1?{array:{reference:'false'},record:{open:'false',reference:'false'},map:{reference:'false'},set:{reference:'false'}}:{array:{reference:'false'},record:{ordered:'false',open:'false',reference:'false'},map:{reference:'false'},set:{reference:'false'}});");
  return replaceOnce(source,'assert.equal(evidence.length,5);','assert.equal(evidence.length,version===1?5:6);');
 }
 if(file==='experiments/original-link-complete-table-projection/raw-schema-sixth-extension-closure.mjs'){
  source=replaceOnce(source,"import assert from 'node:assert/strict';","import {restoreOrderedRecordOwnedSource} from '../../js/scripts/ordered-record-schema-profile.mjs';\nimport assert from 'node:assert/strict';");
  return replaceOnce(source,'const bytes=authoredSources[row.file];','const bytes=restoreOrderedRecordOwnedSource(row.file,authoredSources[row.file]);');
 }
 fail('unknown versioned source bridge');
}
export function restoreOrderedRecordOwnedSource(file,bytes){
 if(!Buffer.isBuffer(bytes))fail('source owner requires complete bytes');
 const previous=readOrderedRecordSchemaPredecessor().ownedSources.find(row=>row.file===file);
 if(!previous) return bytes;
 if(hash(bytes)===previous.sha256)return bytes;
 const expected=extendOrderedRecordOwnedSource(file,previous.source);
 if(bytes.toString('utf8')!==expected)fail('unknown complete source bridge edit');
 return Buffer.from(previous.source);
}

/** Deferred source/schema audit, separate from construction and from code
 * execution. Every historical/current interpretation keeps its own inputs. */
export async function readOrderedRecordSourceProfiles(){
 const previous=readOrderedRecordSchemaPredecessor();
 const {readProgramTypeNeutralDefaults}=await import('./read-program-type-neutral-defaults.mjs');
 const legacy=readProgramTypeNeutralDefaults({version:1}),current=readProgramTypeNeutralDefaults({version:2});
 if(legacy.source.sha256!==previousNativeTypeSha256||current.source.sha256!==currentNativeTypeSha256)fail('source profile belongs to another native definition');
 const nativeBytes=readFileSync(new URL(previous.previousNativeTypes.file,root));
 if(restoreOrderedRecordTypeSource(nativeBytes,previous.previousNativeTypes.file)!==previous.previousNativeTypes.source)fail('the complete native source predecessor was lost');
 for(const file of ['js/src/data/typed-program-links-schema.lino','rust/src/data/typed-program-links-schema.lino']){
  const bytes=readFileSync(new URL(file,root));
  if(restoreOrderedRecordSchema(bytes,file)!==previous.previousSchema.source)fail('the complete schema predecessor was lost');
  const schema=readStructuredDataLino(bytes.toString('utf8'));
  const canonical=value=>JSON.stringify(Object.fromEntries(Object.entries(value).map(([key,fields])=>[key,Object.fromEntries(Object.entries(fields).sort())])));
  if(canonical(schema.neutralTypeFieldDefaults)!==canonical(current.defaults))fail('the active schema does not cover every active native Type facet');
 }
 for(const entry of previous.ownedSources){
  const actual=readFileSync(new URL(entry.file,root));
  if(hash(restoreOrderedRecordOwnedSource(entry.file,actual))!==entry.sha256)fail('the complete original source owner was lost');
 }
 return {version:1,legacy,current,sourceClosureOnly:true};
}
