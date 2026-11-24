// Normal compiler specialization of one opaque field in the actual complete
// JavaScript source. No represented sorter executes during specialization.
import {readFileSync} from 'node:fs';
import {parseJavaScript} from './javascript.js';
import {checkProgram} from './check.js';
import {typeError} from './diagnostics.js';
import {FLOAT,STRING,array,data,sameType} from './types.js';
export const stableVectorSortTemplateModel='complete stable opaque UTF16 keyed vector insertion sort 1';
export function specializeStableVectorSort(element,namespace){
 if(!element||typeof element.kind!=='string'||typeof namespace!=='string'||!/^[$A-Z_a-z][$0-9A-Z_a-z]*$/u.test(namespace))throw typeError('stable vector sorting requires its actual checked payload type and fresh identifier namespace');
 const source=readFileSync(new URL('./stable-vector-sort-template.js',import.meta.url),'utf8'),surface=parseJavaScript(source,{referenceSemantics:false});
 if(surface.language!=='JavaScript'||surface.externals?.length||surface.main?.effects?.length)throw typeError('stable vector sorting requires its complete closed ordinary source');
 const owners=surface.items.filter(item=>item.k==='data'&&item.name==='OpaqueValue'),owner=owners[0];
 if(owners.length!==1||owner.ctors.length!==1||owner.ctors[0].name!=='opaqueValue'||owner.ctors[0].fields.length!==1||owner.ctors[0].fields[0].name!=='value'||owner.ctors[0].fields[0].type.kind!=='float')throw typeError('stable vector sorting retains exactly its actual one-field opaque type parameter');
 // This is the only payload-dependent source edit. All original declarations,
 // comparisons, branches and insertion bodies remain in the ordinary checker.
 owner.ctors[0].fields[0].type=structuredClone(element);
 const library=checkProgram({...surface,items:[{k:'module',name:namespace,items:surface.items}],main:null}),type=data(namespace+'.OpaqueValue'),entryType=data(namespace+'.KeyedValue'),entry=namespace+'.sortWrappedStableVectorValues',keyComparison=namespace+'.compareStableVectorKeys',payload=library.declarations.get(type.name),keyed=library.declarations.get(entryType.name),sorter=library.declarations.get(entry),compare=library.declarations.get(keyComparison);
 if(payload?.k!=='data'||payload.ctors.length!==1||payload.ctors[0].name!=='opaqueValue'||payload.ctors[0].fields.length!==1||payload.ctors[0].fields[0].name!=='value'||!sameType(payload.ctors[0].fields[0].type,element))throw typeError('stable vector specialization preserves its actual opaque payload wrapper');
 if(keyed?.k!=='data'||keyed.ctors.length!==1||keyed.ctors[0].name!=='keyedValue'||keyed.ctors[0].fields.length!==2||keyed.ctors[0].fields[0].name!=='key'||!sameType(keyed.ctors[0].fields[0].type,STRING)||keyed.ctors[0].fields[1].name!=='value'||!sameType(keyed.ctors[0].fields[1].type,type))throw typeError('stable vector specialization retains its actual ordered key and opaque wrapper entry');
 if(sorter?.k!=='fn'||!sorter.body||sorter.params.length!==1||!sameType(sorter.params[0].type,array(entryType))||!sameType(sorter.ret,array(entryType))||compare?.k!=='fn'||!compare.body||compare.params.length!==2||compare.params.some(parameter=>!sameType(parameter.type,STRING))||!sameType(compare.ret,FLOAT))throw typeError('stable vector specialization preserves the actual complete source callable interfaces');
 return {library,type,constructor:'opaqueValue',entryType,entryConstructor:'keyedValue',entry,keyComparison,model:stableVectorSortTemplateModel};
}
/** Checked boundary conversion only. Source owns UTF16 comparison and stable
 * insertion. Root caller authenticates the source comparator and merges every
 * actual checked library declaration/item before publishing the checked graph. */
export function lowerStableVectorSort(node,value,keyLambda,specialization,fresh){
 const keyType=specialization.keyType??STRING;
 if(value?.type?.kind!=='array'||value.type.reference===true||keyLambda?.type?.kind!=='function'||keyLambda.type.params.length!==1||!sameType(keyLambda.type.params[0],value.type.element)||!sameType(keyLambda.type.ret,keyType)||typeof fresh!=='function')throw typeError('stable vector sorting needs its actual owned vector and checked actual key projection',node?.span);
 const element=value.type.element,valuesName=fresh('stableVectorValues'),keyName=fresh('stableVectorKeyFunction'),itemName=fresh('stableVectorValue'),item={k:'var',name:itemName,type:element},wrapped={k:'ctor',data:specialization.type.name,ctor:specialization.constructor,args:[item],type:specialization.type},key={k:'apply',fn:{k:'var',name:keyName,type:keyLambda.type},args:[item],type:keyType},entry={k:'ctor',data:specialization.entryType.name,ctor:specialization.entryConstructor,args:[key,wrapped],type:specialization.entryType},wrap={k:'lambda',params:[{name:itemName,type:element}],body:entry,type:{kind:'function',params:[element],ret:specialization.entryType}},entries={k:'method',op:'map',object:{k:'var',name:valuesName,type:value.type},args:[wrap],type:array(specialization.entryType)},ordered={k:'call',fn:specialization.entry,args:[entries],type:array(specialization.entryType)},entryName=fresh('stableVectorEntry'),entryKey=fresh('stableVectorEntryKey'),entryValue=fresh('stableVectorEntryValue'),payloadName=fresh('stableVectorPayload'),payload={k:'var',name:payloadName,type:element},unwrapped={k:'match',scrutinee:{k:'var',name:entryValue,type:specialization.type},cases:[{pattern:{k:'ctor',data:specialization.type.name,ctor:specialization.constructor,binds:[payloadName]},body:payload}],type:element},unwrap={k:'lambda',params:[{name:entryName,type:specialization.entryType}],body:{k:'match',scrutinee:{k:'var',name:entryName,type:specialization.entryType},cases:[{pattern:{k:'ctor',data:specialization.entryType.name,ctor:specialization.entryConstructor,binds:[entryKey,entryValue]},body:unwrapped}],type:element},type:{kind:'function',params:[specialization.entryType],ret:element}},body={k:'method',op:'map',object:ordered,args:[unwrap],type:value.type};
 return {k:'let',name:valuesName,value,body:{k:'let',name:keyName,value:keyLambda,body,type:value.type},type:value.type,...(node?.span?{span:node.span}:{})};
}
