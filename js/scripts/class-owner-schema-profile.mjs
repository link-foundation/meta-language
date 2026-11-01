// Exact complete-source succession only. Receipts declare source profiles,
// never execution, semantic equivalence or a passing compiler observation.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {extendCurrentTypedProgramSourceSchema as extendClassOwnerSchema} from './class-owner-schema-extension.mjs';
export const classOwnerSourceProfileReceipt='parity/self-translation/class-owner-source-profiles.lino';
export const classOwnerSourceProfilePredecessorRevision='397c2ea47e8531558857dc54caf88358fd51033a';
const classOwnerSourceProfileReceiptSha256='PENDING';
const previousNativeTypeSha256='89d8ca15420493e3e9210a1b0a5b359d55d8a9821a7445806ea82a98fd99f92f';
const root=new URL('../../',import.meta.url),hash=bytes=>createHash('sha256').update(bytes).digest('hex');
const fail=message=>{throw new TypeError('class owner source profile: '+message);};
const schemaFiles=['js/src/data/typed-program-links-schema.lino','rust/src/data/typed-program-links-schema.lino'];
let retained;
function snapshot(value,file,limit){
 if(!value||value.file!==file||typeof value.source!=='string'||!Number.isInteger(value.bytes)||value.bytes<0||value.bytes>limit||Buffer.byteLength(value.source)!==value.bytes||hash(Buffer.from(value.source))!==value.sha256)fail('changed complete source owner '+file);
}
export function readClassOwnerSourceProfile(){
 if(retained)return retained;
 const bytes=readFileSync(new URL(classOwnerSourceProfileReceipt,root));if(bytes.length>4*1024*1024||hash(bytes)!==classOwnerSourceProfileReceiptSha256)fail('unknown complete successor receipt');
 const value=readStructuredDataLino(bytes.toString('utf8'));
 if(value.version!==3||value.model!=='complete class owner schema and native Type source profile 3'||value.predecessorRevision!==classOwnerSourceProfilePredecessorRevision)fail('unknown source interpretation');
 snapshot(value.previousNativeTypes,'rust/src/translation/types.rs',128*1024);snapshot(value.currentNativeTypes,'rust/src/translation/types.rs',128*1024);
 if(value.previousNativeTypes.sha256!==previousNativeTypeSha256)fail('the complete version-two Type source guard was changed');
 if(value.previousNativeTypes.revision!==classOwnerSourceProfilePredecessorRevision)fail('native predecessor lost its exact committed source owner');
 if(!Array.isArray(value.schemas)||JSON.stringify(value.schemas.map(row=>row.file))!==JSON.stringify(schemaFiles))fail('changed schema owner inventory');
 for(const row of value.schemas){snapshot(row.previous,row.file,256*1024);snapshot(row.current,row.file,256*1024);if(row.previous.revision!==classOwnerSourceProfilePredecessorRevision)fail('schema predecessor lost its exact committed source owner');if(JSON.stringify(readStructuredDataLino(row.current.source))!==JSON.stringify(extendClassOwnerSchema(readStructuredDataLino(row.previous.source))))fail('unknown complete class schema extension');}
 if(!Array.isArray(value.ownedSources)||value.ownedSources.length!==1||value.ownedSources[0].file!=='js/scripts/read-program-type-neutral-defaults.mjs')fail('changed versioned source adapter inventory');
 for(const row of value.ownedSources){snapshot(row.previous,row.file,64*1024);snapshot(row.current,row.file,64*1024);if(row.previous.revision!==classOwnerSourceProfilePredecessorRevision)fail('source adapter predecessor lost its committed owner');}
 retained=value;return retained;
}
export function restoreClassOwnerSchema(bytes,file){
 if(!schemaFiles.includes(file)||!Buffer.isBuffer(bytes)||bytes.length>256*1024)fail('unknown schema source boundary');
 const row=readClassOwnerSourceProfile().schemas.find(row=>row.file===file);
 if(hash(bytes)===row.previous.sha256&&bytes.toString('utf8')===row.previous.source)return row.previous.source;
 if(hash(bytes)!==row.current.sha256||bytes.toString('utf8')!==row.current.source)fail('unknown complete class schema source');
 return row.previous.source;
}
export function restoreClassOwnerNativeTypeSource(bytes,file){
 if(file!=='rust/src/translation/types.rs'||!Buffer.isBuffer(bytes)||bytes.length>128*1024)fail('unknown native Type source boundary');
 const row=readClassOwnerSourceProfile();
 if(hash(bytes)===row.previousNativeTypes.sha256&&bytes.toString('utf8')===row.previousNativeTypes.source)return row.previousNativeTypes.source;
 if(hash(bytes)!==row.currentNativeTypes.sha256||bytes.toString('utf8')!==row.currentNativeTypes.source)fail('uncovered complete native Type successor');
 return row.previousNativeTypes.source;
}
export function readClassOwnerNativeTypeSource(bytes,file){
 if(file!=='rust/src/translation/types.rs'||!Buffer.isBuffer(bytes)||bytes.length>128*1024)fail('unknown native Type source boundary');
 const row=readClassOwnerSourceProfile().currentNativeTypes;if(hash(bytes)!==row.sha256||bytes.toString('utf8')!==row.source)fail('version three needs its exact complete active Type source');return row.source;
}
export function restoreClassOwnerOwnedSource(file,bytes){
 if(file!=='js/scripts/read-program-type-neutral-defaults.mjs')return bytes;
 if(!Buffer.isBuffer(bytes)||bytes.length>64*1024)fail('invalid source adapter bytes');
 const row=readClassOwnerSourceProfile().ownedSources[0];
 if(hash(bytes)===row.previous.sha256&&bytes.toString('utf8')===row.previous.source)return bytes;
 if(hash(bytes)!==row.current.sha256||bytes.toString('utf8')!==row.current.source)fail('unknown complete source adapter successor');
 return Buffer.from(row.previous.source);
}
