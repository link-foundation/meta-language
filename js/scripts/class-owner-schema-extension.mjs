import {extendExclusiveSourceSchema} from './exclusive-source-schema-extension.mjs';
// Source-authored vocabulary succession. No graph, policy or fixture executes.
import {extendPropositionValueSchema} from './proposition-value-schema-extension.mjs';
import {extendSourcePositionSchema} from './source-position-schema-extension.mjs';
export function extendClassOwnerSchema(before){
 const after=structuredClone(before);
 if(!after.references||!after.referenceSourceFields||!after.referenceTargetFields||!after.fields)throw new TypeError('class owners require the complete typed program schema');
 const install=(table,key,value)=>{
  if(Object.hasOwn(table,key)&&JSON.stringify(table[key])!==JSON.stringify(value))throw new TypeError('preserve conflicting class owner schema vocabulary: '+key);
  table[key]=value;
 };
 install(after.references,'classConstructor','source class constructor reference');
 install(after.referenceSourceFields,'classConstructor',[['','record','sourceClassOwner']]);
 install(after.referenceTargetFields,'classConstructor','fullName');
 install(after.fields,'sourceClassOwner','source class owner');
 install(after.fields,'sourceClassOwners','source class owners');
 if(after.optionalReferenceRoles!==undefined&&(!Array.isArray(after.optionalReferenceRoles)||after.optionalReferenceRoles.some(role=>typeof role!=='string'||!Object.hasOwn(after.references,role))))throw new TypeError('optional references must name actual registered roles');
 after.optionalReferenceRoles=[...new Set([...(after.optionalReferenceRoles??[]),'classConstructor'])];
 return after;
}

// Publish one complete pending successor while preserving the independent
// input contracts of the individual additive extensions.
export function extendCurrentTypedProgramSourceSchema(before){
 return extendExclusiveSourceSchema(extendSourcePositionSchema(extendPropositionValueSchema(extendClassOwnerSchema(before))));
}
