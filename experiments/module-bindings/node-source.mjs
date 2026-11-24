// Independent original/renamed module execution. Never invoke during drafting.
import { mkdtempSync,writeFileSync,rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { spawnSync } from 'node:child_process';
export function observeJavaScriptModule(source,provider,observations) {
  const directory=mkdtempSync(join(tmpdir(),'meta-language-module-binding-'));
  try {
    writeFileSync(join(directory,'provider.js'),provider);
    const entry=join(directory,'entry.mjs');writeFileSync(entry,source);
    const script='import * as namespace from '+JSON.stringify(pathToFileURL(entry).href)+';const rows='+JSON.stringify(observations)+';console.log(JSON.stringify(rows.map(row=>{let owner=namespace;const path=row.entry.split(".");for(const key of path.slice(0,-1))owner=owner[key];return String(owner[path.at(-1)](...row.arguments.map(Number)));})));';
    const result=spawnSync(process.execPath,['--experimental-default-type=module','--input-type=module','-e',script],{encoding:'utf8',timeout:5000,maxBuffer:1024*1024});
    if(result.error)throw result.error;
    if(result.status!==0||result.signal)throw new Error('independent module failed: '+result.status+' '+result.signal+' '+result.stderr);
    return {source,values:JSON.parse(result.stdout),status:result.status,signal:result.signal,stdout:result.stdout,stderr:result.stderr};
  } finally { rmSync(directory,{recursive:true,force:true}); }
}
