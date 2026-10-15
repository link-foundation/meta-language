// Generic exact-source overlay builder. It changes no production file and
// executes only transformations recorded in a checked decorator document.
import {readFileSync,writeFileSync,mkdtempSync,mkdirSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join,dirname} from 'node:path';
import {pathToFileURL,fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {DecoratorSet} from '../../js/src/decorators.js';
import {tokenize} from '../../js/src/translation/lexer.js';
const hash=source=>createHash('sha256').update(source).digest('hex');
export function relocateCheckedModuleImports(source,origin,resolve){
 const tokens=tokenize(source,'JavaScript').tokens,edits=[];
 const relocate=specifier=>{if(specifier.value.startsWith('.')){const original=new URL(specifier.value,origin).href;edits.push({start:specifier.start,end:specifier.end,text:JSON.stringify(resolve(original))});}};
 for(let index=0;index<tokens.length;index++){
  const token=tokens[index],next=tokens[index+1],value=token.value;
  if(tokens[index-1]?.value==='.'||tokens[index-1]?.value==='?.')continue;
  if(value==='import'&&next?.value==='.'&&tokens[index+2]?.value==='meta'&&tokens[index+3]?.value==='.'&&tokens[index+4]?.value==='url')edits.push({start:token.start,end:tokens[index+4].end,text:JSON.stringify(origin)});
  if(value==='import'&&next?.kind==='string'){relocate(next);continue;}
  if(value==='import'&&next?.value==='('&&tokens[index+2]?.kind==='string'){relocate(tokens[index+2]);continue;}
  // A bare identifier named `from` is not a module specifier. Search only an
  // actual static import or export-from declaration, outside its named list.
  const declaration=value==='import'&&(next?.kind==='identifier'||next?.value==='{'||next?.value==='*')||value==='export'&&(next?.value==='{'||next?.value==='*');
  if(!declaration)continue;
  let depth=0;
  for(let cursor=index+1;cursor<tokens.length;cursor++){
   const current=tokens[cursor];if(current.value==='{')depth++;else if(current.value==='}')depth--;
   if(depth===0&&current.value==='from'&&tokens[cursor+1]?.kind==='string'){relocate(tokens[cursor+1]);break;}
   if(depth===0&&current.value===';')break;
   if(depth===0&&cursor>index+1&&(current.value==='import'||current.value==='export'))break;
  }
 }
 let result=source;for(const edit of edits.sort((a,b)=>b.start-a.start))result=result.slice(0,edit.start)+edit.text+result.slice(edit.end);return result;
}
export async function createCheckedModuleOverlay({records,sourceRoot=new URL('../../js/src/',import.meta.url),virtualModules=new Map()}){
 const directory=mkdtempSync(join(tmpdir(),'meta-language-checked-source-overlay-')),destinations=new Map(),origins=new Map();
 try{
  for(const record of records){if(!/^[a-z0-9/-]+\.js$/u.test(record.module)||record.module.split('/').includes('..'))throw new TypeError('an overlay module path is invalid');const origin=new URL(record.module,sourceRoot).href,destination=pathToFileURL(join(directory,record.module)).href;origins.set(record.module,origin);destinations.set(origin,destination);}
  const receipts=[];
  for(const record of records){const origin=origins.get(record.module),original=readFileSync(new URL(origin),'utf8');if(hash(original)!==record.sourceSha256)throw new TypeError('the exact overlay provider changed: '+record.module);
   const decorated=DecoratorSet.fromLino(record.decorators).decorate('translation-rule',{module:record.module,source:original}).source;if(hash(decorated)!==record.overlaySha256)throw new TypeError('the checked overlay transformation differs: '+record.module);
   const linked=relocateCheckedModuleImports(decorated,origin,url=>virtualModules.get(url)??destinations.get(url)??url),destination=new URL(destinations.get(origin));mkdirSync(dirname(fileURLToPath(destination)),{recursive:true});writeFileSync(destination,linked);receipts.push({module:record.module,sourceSha256:record.sourceSha256,overlaySha256:record.overlaySha256});
  }
  return{receipts,async importModule(module){const origin=origins.get(module);if(!origin)throw new TypeError('the overlay module is not registered');return import(destinations.get(origin));},dispose(){rmSync(directory,{recursive:true,force:true});}};
 }catch(error){rmSync(directory,{recursive:true,force:true});throw error;}
}
