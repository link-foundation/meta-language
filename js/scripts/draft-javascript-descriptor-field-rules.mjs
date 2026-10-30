// Declarative Links field order into actual ordinary source; no conversion runs.
import {readProjectData,writeProjectData} from './project-data.mjs';
import {writeFileSync,readFileSync,existsSync} from 'node:fs';
import {createHash} from 'node:crypto';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const fields=[['enumerable','boolean'],['configurable','boolean'],['value','value'],['writable','boolean'],['get','accessor'],['set','accessor']].map(([name,kind],position)=>({name,kind,position}));
const file=new URL('parity/translation/javascript-descriptor-fields.lino',root),model='ECMAScript 2025 ordered descriptor source fields 1';
if(!existsSync(file))writeProjectData(file,{schemaVersion:1,model,fields});
const definition=readProjectData(file);
if(definition.schemaVersion!==1||definition.model!==model||!Array.isArray(definition.fields)||definition.fields.length!==fields.length
 ||definition.fields.some((field,index)=>field.name!==fields[index].name||field.kind!==fields[index].kind||field.position!==index))throw new TypeError('a different descriptor field definition needs its separately qualified source profile');
const hash=createHash('sha256').update(readFileSync(file)).digest('hex');
let source='// Generated from parity/translation/javascript-descriptor-fields.lino. Do not edit.\n// Complete Links source sha256 '+hash+'\n/** @param {number} index @returns {{name:string,kind:string,units:number[]}} */\nexport function readJavaScriptDescriptorField(index){\n';
for(const field of definition.fields){const units=Array.from({length:field.name.length},(_,at)=>field.name.charCodeAt(at));source+='  if(index === '+field.position+') return {name:'+JSON.stringify(field.name)+',kind:'+JSON.stringify(field.kind)+',units:['+units.join(',')+']};\n';}
source+='  return {name:"",kind:"invalid",units:[]};\n}\n';
writeFileSync(new URL('js/src/translation/javascript-descriptor-field-rules.js',root),source);
