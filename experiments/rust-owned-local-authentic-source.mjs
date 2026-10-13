// Retain the original complete formatter input independently of later style repairs.
import {execFileSync} from 'node:child_process';
import {mkdtempSync,mkdirSync,writeFileSync,readFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join,dirname} from 'node:path';
import {fileURLToPath} from 'node:url';
export const RUST_OWNED_LOCAL_AUTHENTIC_SOURCE_COMMIT='26667280';
export function withRustOwnedLocalAuthenticSource(files,read){
 const root=fileURLToPath(new URL('../',import.meta.url));
 const directory=mkdtempSync(join(tmpdir(),'meta-language-owned-text-source-'));
 try{
  const paths=files.map(file=>{
   if(!/^rust\/src\/link_network_binary_rules\/part_[0-3]\.rs$/u.test(file))throw new Error('the pinned prototype reads only its four actual formatter source files');
   const bytes=execFileSync('git',['show',`${RUST_OWNED_LOCAL_AUTHENTIC_SOURCE_COMMIT}:${file}`],{cwd:root,maxBuffer:4*1024*1024});
   const path=join(directory,file);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,bytes);
   if(!readFileSync(path).equals(bytes))throw new Error('the temporary complete source differs from the pinned Git object');
   return path;
  });
  return read(paths);
 }finally{rmSync(directory,{recursive:true,force:true});}
}
