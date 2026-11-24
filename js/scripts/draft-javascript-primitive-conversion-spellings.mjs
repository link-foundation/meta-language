// Derive exact UTF16 units from the qualified Links spelling definition.
import {readProjectData,writeProjectData} from './project-data.mjs';
import {existsSync,readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),file=new URL('parity/translation/javascript-primitive-conversion-spellings.lino',root);
const names=['default','number','string','valueOf','toString'],model='ECMAScript 2025 primitive conversion spellings 1';
if(!existsSync(file))writeProjectData(file,{schemaVersion:1,model,names});
const definition=readProjectData(file);
if(definition.schemaVersion!==1||definition.model!==model||definition.names?.join('\n')!==names.join('\n'))throw new TypeError('a changed spelling directory requires a separately qualified profile');
let source='// Generated from parity/translation/javascript-primitive-conversion-spellings.lino. Do not edit.\n// Complete source sha256 '+createHash('sha256').update(readFileSync(file)).digest('hex')+'\n/** @param {string} name @returns {JavaScriptSemanticValue} */\nexport function readJavaScriptPrimitiveConversionSpelling(name){\n';
for(const name of definition.names)source+='  if(name === '+JSON.stringify(name)+')return {$:"text",units:['+Array.from({length:name.length},(_,index)=>name.charCodeAt(index)).join(',')+']};\n';
source+='  throw new TypeError("a primitive conversion spelling needs its actual registered name");\n}\n';
writeFileSync(new URL('js/src/translation/javascript-primitive-conversion-spellings.js',root),source);
