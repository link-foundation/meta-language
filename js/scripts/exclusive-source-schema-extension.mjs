// Additive source-authoring vocabulary; generated Links/native schemas are not
// edited here. Every predecessor field and label must retain its exact meaning.
export function extendExclusiveSourceSchema(before){
 const after=structuredClone(before);if(!after.fields||typeof after.fields!=='object'||Array.isArray(after.fields))throw new TypeError('exclusive source signatures require the complete field vocabulary');
 for(const [field,label]of [['sourceFnMutSpecialization','source mutable function specialization'],['rustFnMutSourceModel','Rust source mutable function model'],['sourceExclusiveSignature','exclusive source signature'],['rustExclusiveSourceModel','Rust exclusive source model'],['rustStandardCollectionSourceModel','Rust standard collection source model'],['rustStandardCollectionPointerWidth','Rust standard collection pointer width'],['sourceStandardCollectionCarrier','standard collection carrier'],['sourceStandardCollectionLibraryOwner','standard collection library owner']]){
  if(Object.hasOwn(after.fields,field)&&after.fields[field]!==label)throw new TypeError('preserve conflicting exclusive source field vocabulary');
  if(Object.entries(after.fields).some(([name,value])=>name!==field&&value===label))throw new TypeError('exclusive source field labels retain one canonical owner');
  after.fields[field]=label;
 }
 const install=(table,key,value)=>{if(!table||typeof table!=='object'||Array.isArray(table))throw new TypeError('collection ownership needs complete reference vocabulary');if(Object.hasOwn(table,key)&&JSON.stringify(table[key])!==JSON.stringify(value))throw new TypeError('preserve conflicting collection ownership reference');table[key]=value;};
 const roles=[
  ['standardCollectionLibraryDeclaration','standard collection library declaration reference','Rust standard collection projected library owner 1','declaration'],
  ['standardCollectionKernel','standard collection kernel reference','Rust standard collection kernel library 1','kernelDeclaration'],
  ['standardCollectionPacket','standard collection packet reference','Rust standard collection kernel library 1','packetDeclaration'],
 ];
 for(const [role,label,model,field]of roles){
  install(after.references,role,label);
  install(after.referenceSourceFields,role,[['',model,field]]);
  install(after.referenceTargetFields,role,'fullName');
 }
 return after;
}
