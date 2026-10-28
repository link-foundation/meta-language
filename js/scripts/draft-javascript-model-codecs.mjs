// Generate the JavaScript side of the same canonical declared model transport.
// Constructors/fields come from actual checked source, never a parallel DTO.
import {writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
export function draftJavaScriptModelCodecs({selected,overrides,output}){
 if(!/^experiments\/[a-z][a-z0-9-]*\/[a-z][a-z0-9-]*\.mjs$/u.test(output))throw new TypeError('a JavaScript model adapter needs its explicit owned experimental output');
 const name=value=>value.replaceAll('.','_');
 for(const key of selected.keys())if(!/^[A-Za-z_][A-Za-z_0-9.]*$/u.test(key))throw new TypeError('unregistered JavaScript model function identity');
 let source='// Generated from actual checked declarations. Transport only; never semantics.\n';
 source+='function scalar(value,kind){if(kind==="unit")return undefined;if(kind==="integer"){if(typeof value!=="string")throw new TypeError("model exact integer");return BigInt(value);}if(typeof value!==kind||kind==="number"&&!Number.isFinite(value))throw new TypeError("model "+kind);return value;}\n';
 source+='function fromBits(value){if(typeof value!=="string"||!/^[0-9a-fA-F]{16}$/u.test(value))throw new TypeError("complete model IEEE bits");const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt("0x"+value));return view.getFloat64(0);}\n';
 source+='function toBits(value){if(typeof value!=="number")throw new TypeError("model Number");const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value);return view.getBigUint64(0).toString(16).padStart(16,"0");}\n';
 function field(type,value,decode){switch(type.kind){
  case 'bool':return 'scalar('+value+',"boolean")';
  case 'float':return 'scalar('+value+',"number")';
  case 'string':return 'scalar('+value+',"string")';
  case 'unit':return decode?'scalar('+value+',"unit")':'null';
  case 'int':case 'nat':return decode?'scalar('+value+',"integer")':'String('+value+')';
  case 'data':return (decode?'decode':'encode')+name(type.name)+'('+value+')';
  case 'array':return value+'.map(value=>'+field(type.element,'value',decode)+')';
  case 'record':return '({'+type.fields.map(entry=>JSON.stringify(entry.name)+':'+field(entry.type,value+'['+JSON.stringify(entry.name)+']',decode)).join(',')+'})';
  default:throw new TypeError('unregistered JavaScript model field '+type.kind);
 }}
 for(const [owner,declaration]of selected){
  for(const decode of [true,false]){
   source+='export function '+(decode?'decode':'encode')+name(owner)+'(value){switch(value.$){\n';
   for(const ctor of declaration.ctors){
    source+='case '+JSON.stringify(ctor.name)+':return {$:'+JSON.stringify(ctor.name);
    for(const entry of ctor.fields){const override=overrides.find(rule=>rule.owner===owner&&rule.constructor===ctor.name&&rule.field===entry.name),input=decode?(override?.wire??entry.name):entry.name,output=decode?entry.name:(override?.wire??entry.name);
     source+=','+JSON.stringify(output)+':'+(override?(decode?'fromBits':'toBits')+'(value['+JSON.stringify(input)+'])':field(entry.type,'value['+JSON.stringify(input)+']',decode));
    }
    source+='};\n';
   }
   source+='default:throw new TypeError("unregistered actual model constructor");}}\n';
  }
 }
 writeFileSync(new URL('../../'+output,import.meta.url),source);
 return {output,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')};
}
