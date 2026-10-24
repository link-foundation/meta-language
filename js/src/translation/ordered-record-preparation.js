// Constructor admission becomes ordinary source calls before effect analysis.
// Hosts transport actual checked values/binders; the complete source owns order.
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../structured-data-links.js';
import {checkProgram} from './check.js';
import {constructRecordPropertyNameRows} from './ordered-record-emission.js';
import {collectSourceIdentifierTexts,readSourceIdentifierSupply} from './source-identifiers.js';
import {typeError,unsupported} from './diagnostics.js';
import {FLOAT,array,sameType} from './types.js';
const excluded=new Set(['type','from','to','domain','span']);
function containsOrderedRecord(value,seen=new Set()){
 if(!value||typeof value!=='object'||seen.has(value))return false;
 seen.add(value);
 if(value.k==='record'&&value.order)return true;
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
 let ordinal=0;
 const memo=new WeakMap();
 function lower(value){
  if(!value||typeof value!=='object')return value;
  if(memo.has(value))return memo.get(value);
  if(Array.isArray(value)){const result=value.map(lower);memo.set(value,result);return result;}
  let result=Object.fromEntries(Object.entries(value).map(([key,child])=>[key,excluded.has(key)||key.startsWith('source')?child:lower(child)]));
  if(value.k==='record'&&value.order){
   if(value.type.kind!=='record'||value.type.ordered!==true)throw typeError('an ordered record needs matching payload and order types',value.span);
   const bindings=result.entries.map(entry=>({name:prefix+'recordField'+String(++ordinal),value:entry.value}));
   result={...result,entries:result.entries.map((entry,index)=>({...entry,value:{k:'var',name:bindings[index].name,type:entry.value.type}})),order:{k:'call',fn:namespace+'.createJavaScriptRecordPropertyOrder',args:[constructRecordPropertyNameRows(result.entries),result.order],type:array(FLOAT)}};
   for(let index=bindings.length-1;index>=0;index-=1)result={k:'let',name:bindings[index].name,value:bindings[index].value,body:result,type:value.type,...(value.span?{span:value.span}:{})};
  }
  memo.set(value,result);return result;
 }
 const declarations=new Map([...program.declarations].map(([name,entry])=>[name,entry.k==='fn'?{...entry,body:lower(entry.body)}:entry.k==='theorem'?{...entry,prop:lower(entry.prop)}:entry]));
 const items=values=>values.map(entry=>entry.k==='module'?{...entry,items:items(entry.items)}:declarations.get(entry.fullName)??entry);
 const originalItems=items(program.items),main=program.main?{...program.main,effects:lower(program.main.effects)}:program.main;
 for(const [name,entry]of library.declarations)declarations.set(name,entry);
 return {...program,items:originalItems.concat(library.items),declarations,main};
}
