// Optional recursive source positions are owned graph records, never payloads.
// Source-signature profile validators decide when this edge is permitted.
export function extendSourcePositionSchema(before){
 const after=structuredClone(before);
 if(!after.fields||typeof after.fields!=='object'||Array.isArray(after.fields))throw new TypeError('source positions need the complete schema field vocabulary');
 const field='arrayElementPosition',label='array element source position';
 if(Object.hasOwn(after.fields,field)&&after.fields[field]!==label)throw new TypeError('preserve conflicting recursive source position vocabulary');
 if(Object.entries(after.fields).some(([name,value])=>name!==field&&value===label))throw new TypeError('recursive source position labels must retain one owned field');
 after.fields[field]=label;
 return after;
}
