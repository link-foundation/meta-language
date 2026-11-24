// Specialize only actual declared scalar key/payload positions. The complete
// source comparator and insertion bodies pass through the ordinary checker.
import {readFileSync} from 'node:fs';
import {parseJavaScript} from './javascript.js';
import {checkProgram} from './check.js';
import {BOOL,FLOAT,STRING,array,data,sameType} from './types.js';
import {typeError} from './diagnostics.js';
import {lowerStableVectorSort} from './stable-vector-sort-specialization.js';
export const primitiveVectorSortTemplateModel='complete stable primitive Rust Ord vector insertion sort 1';
export function specializePrimitiveVectorSort(element,namespace){
 if(!element||!['fixed','bool','string'].includes(element.kind)||element.kind==='fixed'&&(![8,16,32,64,128].includes(element.bits)||typeof element.signed!=='boolean')||typeof namespace!=='string'||!/^[$A-Z_a-z][$0-9A-Z_a-z]*$/u.test(namespace))throw typeError('primitive vector sort requires its actual closed primitive checked element and fresh namespace');
 const source=readFileSync(new URL('./primitive-vector-sort-template.js',import.meta.url),'utf8'),surface=parseJavaScript(source,{referenceSemantics:false});
 if(surface.language!=='JavaScript'||surface.externals?.length||surface.main?.effects?.length)throw typeError('primitive vector sorting retains its complete closed ordinary source');
 const declaration=(kind,name)=>{const rows=surface.items.filter(item=>item.k===kind&&item.name===name);if(rows.length!==1)throw typeError('primitive vector sorting retains every actual complete declared interface');return rows[0];};
 const opaque=declaration('data','OpaqueValue'),integer=declaration('data','IntegerSortEntry'),compare=declaration('fn','integerVectorKeyPrecedes');
 if(opaque.ctors.length!==1||opaque.ctors[0].name!=='opaqueValue'||opaque.ctors[0].fields.length!==1||opaque.ctors[0].fields[0].name!=='value'||!sameType(opaque.ctors[0].fields[0].type,FLOAT)||integer.ctors.length!==1||integer.ctors[0].name!=='integerSortEntry'||integer.ctors[0].fields.length!==2||integer.ctors[0].fields[0].name!=='key'||!sameType(integer.ctors[0].fields[0].type,FLOAT)||compare.params.length!==2||compare.params.some(parameter=>!sameType(parameter.type,FLOAT))||!sameType(compare.ret,BOOL))throw typeError('primitive vector source key and opaque slots retain their exact original declarations');
 opaque.ctors[0].fields[0].type=structuredClone(element);
 if(element.kind==='fixed'){integer.ctors[0].fields[0].type=structuredClone(element);compare.params.forEach(parameter=>{parameter.type=structuredClone(element);});}
 const family=element.kind==='fixed'?'Integer':element.kind==='bool'?'Boolean':'String',library=checkProgram({...surface,items:[{k:'module',name:namespace,items:surface.items}],main:null}),type=data(namespace+'.OpaqueValue'),entryType=data(namespace+'.'+family+'SortEntry'),entry=namespace+'.sort'+family+'VectorEntries',entryConstructor=family.toLowerCase()+'SortEntry',keyComparison=namespace+'.'+family.toLowerCase()+'VectorKeyPrecedes',payload=library.declarations.get(type.name),keyed=library.declarations.get(entryType.name),sorter=library.declarations.get(entry),comparison=library.declarations.get(keyComparison),keyType=element.kind==='fixed'?element:element.kind==='bool'?BOOL:STRING;
 if(payload?.k!=='data'||payload.ctors.length!==1||payload.ctors[0].name!=='opaqueValue'||payload.ctors[0].fields.length!==1||!sameType(payload.ctors[0].fields[0].type,element)||keyed?.k!=='data'||keyed.ctors.length!==1||keyed.ctors[0].name!==entryConstructor||keyed.ctors[0].fields.length!==2||keyed.ctors[0].fields[0].name!=='key'||!sameType(keyed.ctors[0].fields[0].type,keyType)||keyed.ctors[0].fields[1].name!=='value'||!sameType(keyed.ctors[0].fields[1].type,type))throw typeError('primitive vector specialization preserves its genuine selected key and payload constructor positions');
 if(sorter?.k!=='fn'||!sorter.body||sorter.params.length!==1||!sameType(sorter.params[0].type,array(entryType))||!sameType(sorter.ret,array(entryType))||comparison?.k!=='fn'||!comparison.body||comparison.params.length!==2||comparison.params.some(parameter=>!sameType(parameter.type,keyType))||!sameType(comparison.ret,BOOL))throw typeError('primitive vector specialization retains its complete source callable interfaces');
 return {library,type,constructor:'opaqueValue',entryType,entryConstructor,entry,keyComparison,keyType,model:primitiveVectorSortTemplateModel};
}
export function lowerPrimitiveVectorSort(node,value,specialization,fresh){
 if(value?.type?.kind!=='array'||!sameType(value.type.element,specialization.keyType))throw typeError('primitive sorting keeps its actual element as the source key',node?.span);
 const binder=fresh('primitiveVectorKey'),element=value.type.element,key={k:'lambda',params:[{name:binder,type:element}],body:{k:'var',name:binder,type:element},type:{kind:'function',params:[element],ret:element}};
 return lowerStableVectorSort(node,value,key,specialization,fresh);
}
