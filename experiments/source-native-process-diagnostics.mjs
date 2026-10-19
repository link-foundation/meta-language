// Process transport only. Failure output remains bounded evidence, never an observation.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {lstatSync,mkdirSync,mkdtempSync,writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData} from '../js/scripts/project-data.mjs';

const maximumBytes=1024*1024;
const profiles=readProjectData(new URL('./source-native-compiler-profiles.lino',import.meta.url));
for(const arguments_ of Object.values(profiles)){
 assert.ok(Array.isArray(arguments_)&&arguments_.every(value=>typeof value==='string'),'compiler profiles require actual argument lists');
 Object.freeze(arguments_);
}
Object.freeze(profiles);

/** Only an explicitly selected experiment profile changes the compiler command. */
export function readSourceNativeCompilerArguments(profile){
 assert.ok(typeof profile==='string'&&Object.hasOwn(profiles,profile),'unknown source native compiler profile');
 return profiles[profile];
}

/** Mirror retained native-process bytes when a frozen CI job cannot upload files. */
export function mirrorSourceNativeProcessDiagnostics({processMode,label,directory,stdout,stderr},write=bytes=>process.stderr.write(bytes)){
 if(processMode!=='native subprocesses')return;
 for(const[stream,captured]of[['stdout',stdout],['stderr',stderr]]){
  assert.ok(Buffer.isBuffer(captured),'diagnostic mirroring requires actual captured bytes');
  const bytes=captured.subarray(0,maximumBytes);
  const identity=JSON.stringify({label,stream,file:join(directory,stream+'.log'),capturedBytes:captured.length,retainedBytes:bytes.length,
   truncated:captured.length!==bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')});
  write(Buffer.from('\nBEGIN SOURCE NATIVE PROCESS DIAGNOSTICS '+identity+'\n','utf8'));
  write(bytes);
  write(Buffer.from('\nEND SOURCE NATIVE PROCESS DIAGNOSTICS '+identity+'\n','utf8'));
 }
}

/** Retain exactly the available bounded process bytes, including compiler warnings. */
export function retainSourceNativeProcessDiagnostics(result,{command,args,options,label,processMode}){
 const root=options.diagnosticsDirectory??fileURLToPath(new URL('../issue-195-artifacts/source-native-observations/',import.meta.url));
 mkdirSync(root,{recursive:true});
 assert.ok(lstatSync(root).isDirectory()&&!lstatSync(root).isSymbolicLink(),'process diagnostics require an owned actual directory');
 const directory=mkdtempSync(join(root,'failure-'));
 const captured={};
 const retain=(name,value)=>{
  const bytes=Buffer.isBuffer(value)?value:typeof value==='string'?Buffer.from(value,'utf8'):Buffer.alloc(0);
  captured[name]=bytes;
  const kept=bytes.subarray(0,maximumBytes),file=name+'.log';
  writeFileSync(join(directory,file),kept,{flag:'wx',mode:0o600});
  return{file,capturedBytes:bytes.length,retainedBytes:kept.length,truncated:kept.length!==bytes.length,
   sha256:createHash('sha256').update(kept).digest('hex')};
 };
 const stdout=retain('stdout',result.stdout??result.error?.stdout),stderr=retain('stderr',result.stderr??result.error?.stderr);
 const receipt={version:1,processMode,label,command,arguments:[...args],timeout:options.timeout,maximumBufferBytes:maximumBytes,
  status:result.status??null,signal:result.signal??null,
  error:result.error?{name:result.error.name??'Error',code:result.error.code??null,message:String(result.error.message??'').slice(0,4096)}:null,
  stdout,stderr,scope:'actual captured process diagnostics only; no successful native observation'};
 const receiptPath=writeProjectData(join(directory,'receipt.lino'),receipt);
 mirrorSourceNativeProcessDiagnostics({processMode,label,directory,stdout:captured.stdout,stderr:captured.stderr});
 return{directory,receiptPath};
}
