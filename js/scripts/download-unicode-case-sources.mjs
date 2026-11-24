// Recreate the prepared snapshot from exact, official source declarations.
// Raw external declarations remain in an ignored, disposable source directory.
import {mkdir,readFile,writeFile,rename,rm} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {spawnSync} from 'node:child_process';
import {readProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const declarations=readProjectData(new URL('../src/data/unicode-case-tables.lino',import.meta.url));
if(declarations.unicode!=='16.0.0')throw new TypeError('uncovered Unicode source profile');
const directory=new URL('../../.cache/unicode-case-source/16.0.0/',import.meta.url);
await mkdir(directory,{recursive:true});
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
for(const source of declarations.sources){
 if(!/^[A-Za-z-]+\.txt$/u.test(source.name)||source.bytes>4194304||!source.url.startsWith('https://www.unicode.org/'))throw new TypeError('uncovered official source declaration');
 const target=new URL(source.name,directory);
 let current;try{current=await readFile(target);}catch(error){if(error.code!=='ENOENT')throw error;}
 if(current&&current.length===source.bytes&&hash(current)===source.sha256)continue;
 if(source.name==='Unicode-License.txt'){
  const license=await readFile(new URL('../src/data/LICENSE-UNICODE.txt',import.meta.url));
  if(license.length!==source.bytes||hash(license)!==source.sha256)throw new TypeError('retained Unicode license differs from its source profile');
  await writeFile(target,license);continue;
 }
 const response=await fetch(source.url,{signal:AbortSignal.timeout(30000)});
 if(!response.ok)throw new Error('official source download failed '+response.status);
 const chunks=[];let bytes=0;
 for await(const chunk of response.body){bytes+=chunk.length;if(bytes>4194304)throw new RangeError('Unicode source resource bound');chunks.push(chunk);}
 const content=Buffer.concat(chunks);
 if(content.length!==source.bytes||hash(content)!==source.sha256)throw new TypeError('official source differs from its recorded complete profile');
 const temporary=new URL(source.name+'.part-'+process.pid,directory);
 try{await writeFile(temporary,content,{flag:'wx'});await rename(temporary,target);}finally{await rm(temporary,{force:true});}
}
const result=spawnSync(process.execPath,['--max-old-space-size=192',fileURLToPath(new URL('./import-unicode-case-tables.mjs',import.meta.url)),fileURLToPath(directory),'--draft'],{stdio:'inherit',timeout:30000});
if(result.error)throw result.error;
if(result.status!==0)throw new Error('Unicode source conversion failed '+result.status);
