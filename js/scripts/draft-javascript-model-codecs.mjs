// Generate the JavaScript side of the same canonical declared model transport.
// Constructors/fields come from actual checked source, never a parallel DTO.
import {writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {modelTextWirePolicy,encodeModelText,decodeModelText} from './model-text-wire.mjs';
import {javaScriptConstructorFieldValueType} from '../src/translation/javascript-constructor-presence.js';
export function draftJavaScriptModelCodecs({selected,overrides,output}){
 if(!/^experiments\/[a-z][a-z0-9-]*\/[a-z][a-z0-9-]*\.mjs$/u.test(output))throw new TypeError('a JavaScript model adapter needs its explicit owned experimental output');
 const name=value=>value.replaceAll('.','_');
 for(const key of selected.keys())if(!/^[A-Za-z_][A-Za-z_0-9.]*$/u.test(key))throw new TypeError('unregistered JavaScript model function identity');
 let source='// Generated from actual checked declarations. Transport only; never semantics.\n';
 source+='const modelTextWirePolicy='+JSON.stringify(modelTextWirePolicy)+';\n'+encodeModelText.toString()+'\n'+decodeModelText.toString()+'\n';
 source+='function scalar(value,kind){if(kind==="unit")return undefined;if(kind==="integer"){if(typeof value!=="string")throw new TypeError("model exact integer");return BigInt(value);}if(typeof value!==kind||kind==="number"&&!Number.isFinite(value))throw new TypeError("model "+kind);return value;}\n';
 source+='function fromBits(value){if(typeof value!=="string"||!/^[0-9a-fA-F]{16}$/u.test(value))throw new TypeError("complete model IEEE bits");const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt("0x"+value));return view.getFloat64(0);}\n';
 source+='function toBits(value){if(typeof value!=="number")throw new TypeError("model Number");const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value);return view.getBigUint64(0).toString(16).padStart(16,"0");}\n';
 function field(type,value,decode){switch(type.kind){
  case 'bool':return 'scalar('+value+',"boolean")';
  case 'float':return 'scalar('+value+',"number")';
  case 'string':return (decode?'decodeModelText':'encodeModelText')+'('+value+')';
  case 'unit':return decode?'scalar('+value+',"unit")':'null';
  case 'int':case 'nat':return decode?'scalar('+value+',"integer")':'String('+value+')';
  case 'data':return (decode?'decode':'encode')+name(type.name)+'('+value+')';
  case 'array':return value+'.map(value=>'+field(type.element,'value',decode)+')';
  case 'tuple':return '['+type.elements.map((entry,index)=>field(entry,value+'['+index+']',decode)).join(',')+']';
  case 'record':{
   const entries=type.fields.map(entry=>{
    const key=JSON.stringify(entry.name),input=value+'['+key+']';
    return {key,optional:entry.optional,converted:field(javaScriptConstructorFieldValueType(entry),input,decode)};
   });
   if(type.ordered){
    orderedRecords=true;
    return 'wireOrderedRecord('+value+', ['+entries.map(({key,converted,optional})=>'['+key+', ()=>'+converted+', '+Boolean(optional)+']').join(',')+'])';
   }
   return '({'+entries.map(({key,converted,optional})=>optional?'...(Object.hasOwn('+value+','+key+')?{['+key+']:'+converted+'}:{})':'['+key+']:'+converted).join(',')+'})';
  }
  case 'option':{
   const present=field(type.element,value,decode);
   const withNull=type.absence==='null'||type.absence==='both'?'('+value+'===null?null:'+present+')':present;
   if(type.absence==='null')return withNull;
   return decode?'(isWireUndefined('+value+')?undefined:'+withNull+')':'('+value+'===undefined?{fieldPresence:"undefined"}:'+withNull+')';
  }
  default:throw new TypeError('unregistered JavaScript model field '+type.kind);
 }}
 let optionalWire=false,orderedRecords=false;
 for(const [owner,declaration]of selected){
  for(const decode of [true,false]){
   source+='export function '+(decode?'decode':'encode')+name(owner)+'(value){switch(value.$){\n';
   for(const ctor of declaration.ctors){
    source+='case '+JSON.stringify(ctor.name)+':return {$:'+JSON.stringify(ctor.name);
    for(const entry of ctor.fields){const override=overrides.find(rule=>rule.owner===owner&&rule.constructor===ctor.name&&rule.field===entry.name),input=decode?(override?.wire??entry.name):entry.name,output=decode?entry.name:(override?.wire??entry.name);
     const inputKey=JSON.stringify(input),outputKey=JSON.stringify(output);
     const converted=override?(decode?'fromBits':'toBits')+'(value['+inputKey+'])':field(javaScriptConstructorFieldValueType(entry),'value['+inputKey+']',decode);
     if(entry.optional){optionalWire=true;source+=',...(Object.hasOwn(value,'+inputKey+')?{'+outputKey+':'+converted+'}:{})';}
     else source+=','+outputKey+':'+converted;
    }
    source+='};\n';
   }
   source+='default:throw new TypeError("unregistered actual model constructor");}}\n';
  }
 }
 if(optionalWire||source.includes('isWireUndefined('))source+='function isWireUndefined(value){return value!==null&&typeof value==="object"&&Object.keys(value).length===1&&Object.hasOwn(value,"fieldPresence")&&value.fieldPresence==="undefined";}\n';
 if(orderedRecords)source+=`function wireOrderedRecord(value, fields) {
 const directory=new Map(fields.map(([key, convert, optional])=>[key,{convert,optional}]));
 for(const [key, {optional}] of directory)if(!optional&&!Object.hasOwn(value,key))throw new TypeError('required ordered record field '+key);
 return Object.fromEntries(Object.keys(value).map(key=>{
  const entry=directory.get(key);
  if(!entry)throw new TypeError('undeclared ordered record field '+key);
  return [key,entry.convert()];
 }));
}
`;
 writeFileSync(new URL('../../'+output,import.meta.url),source);
 return {output,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')};
}
