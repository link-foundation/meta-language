// Optional recursive source positions are owned graph records, never payloads.
// Source-signature profile validators decide when this edge is permitted.
export function extendSourcePositionSchema(before){
 const after=structuredClone(before);
 if(!after.fields||typeof after.fields!=='object'||Array.isArray(after.fields))throw new TypeError('source positions need the complete schema field vocabulary');
 for(const[field,label]of [['arrayElementPosition','array element source position'],['sourcePosition','source expression position']]){
  if(Object.hasOwn(after.fields,field)&&after.fields[field]!==label)throw new TypeError('preserve conflicting recursive source position vocabulary');
  if(Object.entries(after.fields).some(([name,value])=>name!==field&&value===label))throw new TypeError('recursive source position labels must retain one owned field');
  after.fields[field]=label;
 }
 if(!after.expressionFields||typeof after.expressionFields!=='object'||Array.isArray(after.expressionFields))throw new TypeError('expression source positions need the complete expression field vocabulary');
 for(const fields of Object.values(after.expressionFields)){
  if(!Array.isArray(fields)||fields.some(field=>typeof field!=='string')||new Set(fields).size!==fields.length)throw new TypeError('expression fields must retain their complete distinct field list');
  if(!fields.includes('sourcePosition'))fields.push('sourcePosition');
 }
 return after;
}
