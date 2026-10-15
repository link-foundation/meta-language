// Diagnose the unchanged complete input; no function extraction or replacement program.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {withRustOwnedLocalAuthenticSource,RUST_OWNED_LOCAL_AUTHENTIC_SOURCE_COMMIT} from './rust-owned-local-authentic-source.mjs';
import {parseRust} from '../js/src/translation/rust.js';
import {checkProgram} from '../js/src/translation/check.js';
import {tokenize} from '../js/src/translation/lexer.js';
import {writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
const files=[0,1,2,3].map(index=>`rust/src/link_network_binary_rules/part_${index}.rs`);
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');
export function completeBinaryFormatterProbe(){
 return withRustOwnedLocalAuthenticSource(files,paths=>{
  const entries=paths.map((path,index)=>{const bytes=readFileSync(path);return{file:files[index],bytes:bytes.length,sha256:digest(bytes),source:bytes.toString('utf8')};});
  const source=entries.map(entry=>entry.source).join('\n');
  const attempts=[{}, {completionModel:'closed guarded function returns 1'}, {localValueStateModel:'closed owned text local state 1'},
   {scalarLoopModel:'closed declaration scalar loop 1'}, {localValueStateModel:'closed owned text local state 1',scalarLoopModel:'closed declaration scalar loop 1'}].map(options=>{
   try{checkProgram(parseRust(source,options));}catch(error){return{options,kind:error.kind,reason:error.reason,span:error.span??null,originalToken:error.span?source.slice(error.span.start,error.span.end):null};}
   throw new Error('the complete formatter unexpectedly passed; review its whole-source observation and proof boundary');
  });
  const tokens=tokenize(source,'Rust').tokens;
  const byteSliceSites=['&exact[digits.len().min(exact.len())..]','exact[..digits.len()]','&other[..1]','&other[1..]'].map(text=>{
   const start=entries[0].source.indexOf(text);
   if(start<0)throw new Error('an actual formatter UTF-8 slice site changed');
   return{file:files[0],start,end:start+text.length,originalText:entries[0].source.slice(start,start+text.length)};
  });
  return{schemaVersion:1,gitCommit:RUST_OWNED_LOCAL_AUTHENTIC_SOURCE_COMMIT,
   scope:'All four complete pinned files are the actual parser input in their recorded order. Diagnostics are refusal observations, not executable formatter or standard-library equivalence evidence.',
   files:entries.map(({source,...receipt})=>receipt),combined:{bytes:Buffer.byteLength(source),codeUnits:source.length,sha256:digest(source),tokens:tokens.length},attempts,byteSliceSites};
 });
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('./complete-binary-formatter-probe.lino',import.meta.url),data=completeBinaryFormatterProbe();
 if(process.argv.includes('--check'))checkProjectData(file,data);else writeProjectData(file,data);
 console.log(`Complete pinned formatter: ${data.files.length} files, ${data.combined.bytes} bytes, ${data.attempts.length} actual source-boundary refusals`);
}
