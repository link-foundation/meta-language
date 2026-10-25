// An authoring compiler phase, never source execution: construct a V8 module
// without linking, resolving imports or evaluating any top-level statement.
import {execFileSync} from 'node:child_process';
const compiler="import {readFileSync} from 'node:fs';import {SourceTextModule} from 'node:vm';new SourceTextModule(readFileSync(0,'utf8'),{identifier:process.argv[1]});";
export function parseAuthoredJavaScriptModule(source,identifier){
 if(typeof source!=='string'||typeof identifier!=='string')throw new TypeError('source module construction needs actual text and identity');
 const [major,minor]=process.versions.node.split('.').map(Number);
 const versionRange='>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0';
 if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered source-author Node '+process.versions.node+'; '+versionRange);
 try{execFileSync(process.execPath,['--no-warnings','--experimental-vm-modules','--max-old-space-size=64','--input-type=module','-e',compiler,identifier],{input:source,encoding:'utf8',maxBuffer:1024*1024,timeout:20000});}
 catch(error){throw new SyntaxError('authored module syntax '+identifier+': '+String(error.stderr??error.message).slice(0,16384));}
 return {language:'ECMAScript module',engine:'Node.js V8',versionRange,phase:'parse without linking, resolving imports or evaluation'};
}
