// Constructor admission becomes ordinary source calls before effect analysis.
// Hosts transport actual checked values/binders; the complete source owns order.
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../structured-data-links.js';
import {checkProgram} from './check.js';
import {constructRecordPropertyNameRows} from './ordered-record-emission.js';
import {collectSourceIdentifierTexts,readSourceIdentifierSupply} from './source-identifiers.js';
import {typeError,unsupported} from './diagnostics.js';
import {FLOAT,array,sameType,typeKey} from './types.js';
import {specializeRecordValueOrder,lowerRecordValueOrder,lowerObjectEntryValues} from './record-value-order-specialization.js';
import {readJavaScriptRecordOrderKernelArity,readJavaScriptRecordOrderKernelParameter,readJavaScriptRecordOrderKernelResult,readJavaScriptObjectEntryPairingOperation} from './javascript-enumeration-rules.js';
import {readRecordOrderRoleType} from './record-order-operations.js';
const excluded=new Set(['type','from','to','domain','span']);
function containsOrderedRecord(value,seen=new Set()){
 if(!value||typeof value!=='object'||seen.has(value))return false;
 seen.add(value);
 if(value.k==='record'&&value.order)return true;
 if(value.k==='math'&&readJavaScriptRecordOrderKernelArity(value.op??''))return true;
 return (Array.isArray(value)?value:Object.entries(value).filter(([key])=>!excluded.has(key)&&!key.startsWith('source')).map(([,child])=>child)).some(child=>containsOrderedRecord(child,seen));
}
export function prepareOrderedRecordValues(program){
 const roots=[...program.declarations.values()].flatMap(entry=>entry.k==='fn'?[entry.body]:entry.k==='theorem'?[entry.prop]:[]).concat(program.main?.effects??[]);
 if(!roots.some(value=>containsOrderedRecord(value)))return program;
 if(program.sourceLanguage!=='JavaScript')throw unsupported('ordered record source library','this source profile requires JavaScript Number and exception semantics');
 if(program.outputThreaded||program.abortsThreaded)throw typeError('record constructor admission must precede effect threading');
 const source=readStructuredDataLino(readFileSync(new URL('../data/record-order-surface-library.lino',import.meta.url),'utf8'));
 if(source.version!==1||source.model!=='complete ordinary record order source library 1'||source.surface.language!=='JavaScript'||source.surface.externals?.length||source.surface.main?.effects?.length)throw typeError('an ordered record needs its complete closed source library');
 const prefix=readSourceIdentifierSupply(collectSourceIdentifierTexts(program)),namespace=prefix+'recordOrder';
 const library=checkProgram({...source.surface,items:[{k:'module',name:namespace,items:source.surface.items}],main:null});
 for(const name of library.declarations.keys())if(program.declarations.has(name)||(program.externals??[]).some(entry=>entry.name===name))throw typeError('an ordered record library namespace collides with an actual declaration');
 const constructor=library.declarations.get(namespace+'.createJavaScriptRecordPropertyOrder');
 if(constructor?.k!=='fn'||constructor.params.length!==2||!sameType(constructor.params[0].type,array(array(FLOAT)))||!sameType(constructor.params[1].type,array(FLOAT))||!sameType(constructor.ret,array(FLOAT)))throw typeError('an ordered record constructor needs its complete owned signature');
 for(const entry of library.declarations.values()){
  const arity=readJavaScriptRecordOrderKernelArity(entry.name??'');if(!arity)continue;
  if(entry.k!=='fn'||entry.params.length!==arity||entry.params.some((parameter,index)=>!sameType(parameter.type,readRecordOrderRoleType(readJavaScriptRecordOrderKernelParameter(entry.name,index))))||!sameType(entry.ret,readRecordOrderRoleType(readJavaScriptRecordOrderKernelResult(entry.name))))throw typeError('an ordinary record order library operation needs its complete owned signature');
 }
 let ordinal=0;
 const specializations=new Map();
 const fresh=role=>prefix+role+String(++ordinal);
 // Each source occurrence has its own binding scope, including shared graph
 // records. The native owned IR expands these same occurrences.
 function lower(value){
  if(!value||typeof value!=='object')return value;
  if(Array.isArray(value))return value.map(lower);
  let result=Object.fromEntries(Object.entries(value).map(([key,child])=>[key,excluded.has(key)||key.startsWith('source')?child:lower(child)]));
  if(value.k==='math'&&(value.op==='reorderJavaScriptRecordPropertyValues'||value.op===readJavaScriptObjectEntryPairingOperation())){
   const pairing=value.op===readJavaScriptObjectEntryPairingOperation(),element=value.args[pairing?1:0].type.element,key=typeKey(element);
   let specialization=specializations.get(key);
   if(!specialization){specialization=specializeRecordValueOrder(element,prefix+'recordValues'+String(specializations.size+1));specializations.set(key,specialization);}
   result=pairing?lowerObjectEntryValues(value,result.args,specialization,fresh):lowerRecordValueOrder(value,result.args,specialization,fresh);
  }else if(value.k==='math'&&readJavaScriptRecordOrderKernelArity(value.op??'')){
   const name=namespace+'.'+value.op;
   if(!library.declarations.has(name))throw typeError('an ordinary record order operation has no complete source definition');
   result={k:'call',fn:name,args:result.args,type:result.type,...(value.span?{span:value.span}:{})};
  }
  if(value.k==='record'&&value.order){
   if(value.type.kind!=='record'||value.type.ordered!==true)throw typeError('an ordered record needs matching payload and order types',value.span);
   const bindings=result.entries.map(entry=>({name:fresh('recordField'),value:entry.value}));
   result={...result,entries:result.entries.map((entry,index)=>({...entry,value:{k:'var',name:bindings[index].name,type:entry.value.type}})),order:{k:'call',fn:namespace+'.createJavaScriptRecordPropertyOrder',args:[constructRecordPropertyNameRows(result.entries),result.order],type:array(FLOAT)}};
   for(let index=bindings.length-1;index>=0;index-=1)result={k:'let',name:bindings[index].name,value:bindings[index].value,body:result,type:value.type,...(value.span?{span:value.span}:{})};
  }
  return result;
 }
 const declarations=new Map([...program.declarations].map(([name,entry])=>[name,entry.k==='fn'?{...entry,body:lower(entry.body)}:entry.k==='theorem'?{...entry,prop:lower(entry.prop)}:entry]));
 const items=values=>values.map(entry=>entry.k==='module'?{...entry,items:items(entry.items)}:declarations.get(entry.fullName)??entry);
 const originalItems=items(program.items),main=program.main?{...program.main,effects:lower(program.main.effects)}:program.main;
 for(const [name,entry]of library.declarations)declarations.set(name,entry);
 const specializedItems=[];
 for(const specialization of specializations.values()){
  for(const [name,entry]of specialization.library.declarations){if(declarations.has(name)||(program.externals??[]).some(entry=>entry.name===name))throw typeError('a record value specialization collides with an actual declaration');declarations.set(name,entry);}
  specializedItems.push(...specialization.library.items);
 }
 return {...program,items:originalItems.concat(library.items,specializedItems),declarations,main};
}
