// Select a whole source program, never infer a dynamic order from its type.
import {readJavaScriptEnumerationForm} from './javascript-enumeration-rules.js';
import {readJavaScriptObjectRestOperation} from './object-rest-rules.js';
import {unsupported} from './diagnostics.js';
export function prepareJavaScriptRecordOrderProfile(surface){
 if(surface.language!=='JavaScript')return {surface,selected:false};
 const seen=new Set();let selected=false;
 function scan(value){
  if(!value||typeof value!=='object'||seen.has(value))return;seen.add(value);
  if(value.k==='math'&&(readJavaScriptEnumerationForm(value.op??'')||value.op===readJavaScriptObjectRestOperation()))selected=true;
  for(const [key,child]of Object.entries(value))if(key!=='span'&&!key.startsWith('source'))scan(child);
 }
 scan(surface.items);scan(surface.main);
 if(!selected)return {surface,selected:false};
 function external(value){
  if(!value||typeof value!=='object')return;
  if(value.kind==='record'&&value.ordered!==true)throw unsupported('ordered record external interface','an actual foreign record signature must already retain its own-field order');
  for(const child of Object.values(value))external(child);
 }
 external(surface.externals??[]);
 const copies=new WeakMap();
 function mark(value){
  if(!value||typeof value!=='object')return value;
  if(copies.has(value))return copies.get(value);
  const result=Array.isArray(value)?[]:{};copies.set(value,result);
  for(const [key,child]of Object.entries(value))Object.defineProperty(result,key,{value:mark(child),enumerable:true,writable:true,configurable:true});
  if(value.kind==='record')result.ordered=true;
  return result;
 }
 return {surface:mark(surface),selected:true};
}
