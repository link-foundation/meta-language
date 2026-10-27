// Source integration only. Stage every complete generated native file from
// its recipe, without touching other staged paths or running code checks.
import {execFileSync} from 'node:child_process';
import {lstatSync,readFileSync,realpathSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);
export function stageSourceLibraryFiles(recipePath,{directory='translation'}={}){
 if(!['translation',''].includes(directory))throw new TypeError('use an explicit supported generated source directory');
 if(typeof recipePath!=='string'||!/^parity\/self-translation\/[a-z][a-z0-9-]*-native\.lino$/u.test(recipePath))throw new TypeError('a complete task-owned native generation recipe is required');
 const actualRoot=realpathSync(fileURLToPath(root)),file=new URL(recipePath,root),stat=lstatSync(file);if(!stat.isFile()||stat.isSymbolicLink()||realpathSync(file)!==actualRoot+'/'+recipePath||stat.size>32*1024*1024)throw new RangeError('the source recipe must be a regular file within the 32 MiB author bound');
 const data=readProjectData(file),stem=recipePath.split('/').at(-1).slice(0,-12).replaceAll('-','_');
 if(data.schemaVersion!==1||!Array.isArray(data.files)||data.files.length===0||data.files.length>128)throw new TypeError('the complete native recipe file directory is required');
 const paths=[],seen=new Set();
 for(const row of data.files){
  if(typeof row.path!=='string'||!(row.path===stem+'.rs'||new RegExp('^'+stem+'/part_[1-9][0-9]*\\.rs$','u').test(row.path))||seen.has(row.path)||typeof row.source!=='string')throw new TypeError('only distinct actual generated module paths may be staged');
  seen.add(row.path);const path='rust/src/'+(directory?directory+'/':'')+row.path,target=new URL(path,root),stat=lstatSync(target);
  if(!stat.isFile()||stat.isSymbolicLink()||realpathSync(target)!==actualRoot+'/'+path)throw new TypeError('a generated source path must remain a regular project-owned file');
  if(!readFileSync(target).equals(Buffer.from(row.source)))throw new TypeError('preserve edited or stale generated source for review: '+path);
  paths.push(path);
 }
 if(!seen.has(stem+'.rs'))throw new TypeError('the actual generated module root is required');
 execFileSync('git',['add','--',...paths],{cwd:fileURLToPath(root),stdio:'inherit'});
 return paths;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const args=process.argv.slice(2);if(args.length!==3||args[0]!=='--stage'||args[1]!=='--recipe')throw new TypeError('use --stage --recipe parity/self-translation/<module>-native.lino');
 const paths=stageSourceLibraryFiles(args[2]);process.stdout.write('Staged '+paths.length+' complete generated Rust source files.\n');
}
