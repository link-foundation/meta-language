// Source-authoring inventory only. Never parse, import or execute inspected files.
import {execFileSync} from 'node:child_process';
import {readFileSync,lstatSync,realpathSync} from 'node:fs';
import {resolve,extname,sep} from 'node:path';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
import {readSourceConstructSignals} from '../src/translation/source-construct-signal-rules.js';
const declarations=[
 ['JavaScript','class declaration','class ','grammar'],['JavaScript','asynchronous function','async ','effect'],['JavaScript','await candidate','await ','effect'],['JavaScript','module import','import ','project'],['JavaScript','module export','export ','project'],['JavaScript','throw candidate','throw ','effect'],['JavaScript','arrow callable','=>','binding'],['JavaScript','spread or rest','...','binding'],['JavaScript','optional chaining','?.','semantics'],['JavaScript','nullish coalescing','??','semantics'],['JavaScript','Map source use','Map','typing'],['JavaScript','Set source use','Set','typing'],['JavaScript','BigInt source use','BigInt','typing'],['JavaScript','Promise source use','Promise','effect'],['JavaScript','Symbol source use','Symbol','typing'],
 ['Rust','generic type alias candidate','type ','typing'],['Rust','generic argument or comparison','<','typing'],['Rust','generic implementation','impl<','typing'],['Rust','spaced generic implementation','impl <','typing'],['Rust','trait declaration','trait ','typing'],['Rust','dynamic trait object','dyn ','typing'],['Rust','explicit reference lifetime',"&'",'ownership'],['Rust','Result source use','Result<','effect'],['Rust','Option source use','Option<','typing'],['Rust','question mark candidate','?','effect'],['Rust','associated default call','::default(','typing'],['Rust','derive attribute','#[derive(','typing'],['Rust','macro definition','macro_rules!','grammar'],['Rust','vector macro','vec![','ownership'],['Rust','standard library path','std::','project'],['Rust','reference counted type','Rc<','ownership'],['Rust','interior mutable type','RefCell<','ownership'],['Rust','unsafe block or function','unsafe ','ownership'],['Rust','asynchronous source','async ','effect'],['Rust','await candidate','.await','effect']
].map(([language,name,needle,phase],index)=>({id:'source-signal-'+index,language,name,needle,phase}));
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
export function draftOwnSourceConstructInventory(project,{byteLimit=128*1024*1024,fileLimit=10000,sampleLimit=8,roots=['js/src','rust/src']}={}){
 for(const [name,value]of Object.entries({byteLimit,fileLimit,sampleLimit}))if(!Number.isSafeInteger(value)||value<1)throw new RangeError('invalid '+name);
 if(byteLimit>128*1024*1024||fileLimit>10000||sampleLimit>64)throw new RangeError('source inventory authoring budget exceeds supported limits');
 if(!Array.isArray(roots)||roots.length===0||roots.length>32||roots.some(path=>typeof path!=='string'||!path.length||path.startsWith('/')||path.includes('\\')||path.includes(':')||path.split('/').some(part=>!part||part==='.'||part==='..')))throw new TypeError('expected bounded relative tracked source roots');
 const root=realpathSync(project),decode=bytes=>new TextDecoder('utf-8',{fatal:true}).decode(bytes),git=args=>decode(execFileSync('git',['-C',root,...args],{maxBuffer:8*1024*1024}));
 const gitVersion=git(['--version']).trim(),version=/^git version 2\.(\d+)\./u.exec(gitVersion);if(!version||Number(version[1])<29)throw new RangeError('uncovered source-inventory Git version: '+gitVersion+'; require >=2.29.0 <3.0.0');
 const objectFormat=git(['rev-parse','--show-object-format']).trim();if(!['sha1','sha256'].includes(objectFormat))throw new RangeError('unsupported source-inventory Git object format');
 const objectLength=objectFormat==='sha1'?40:64,objectIdentity=new RegExp('^[0-9a-f]{'+objectLength+'}$','u');
 const revision=git(['rev-parse','HEAD']).trim();if(!objectIdentity.test(revision))throw new TypeError('invalid source inventory revision identity');
 const entries=git(['ls-files','--stage','-z','--',...roots]).split('\0').filter(Boolean);
 const files=[],signals=declarations.map(row=>({...row,totalOccurrences:0,matchingFiles:0,unmarkedOccurrences:0,generatorMarkedOccurrences:0,samples:[]}));let bytes=0;
 for(const entry of entries){
  const split=entry.indexOf('\t'),header=entry.slice(0,split).split(' '),path=entry.slice(split+1),extension=extname(path);
  if(!['.js','.mjs','.cjs','.jsx','.ts','.tsx','.mts','.cts','.rs'].includes(extension))continue;
  if(split<0||header.length!==3||header[2]!=='0'||!['100644','100755'].includes(header[0])||!objectIdentity.test(header[1]))throw new TypeError('expected an ordinary tracked source index entry');
  if(files.length>=fileLimit)throw new RangeError('source inventory file limit exceeded');
  const full=resolve(root,path),stat=lstatSync(full);if(!full.startsWith(root+sep)||!stat.isFile()||stat.isSymbolicLink()||realpathSync(full)!==full)throw new TypeError('source must remain an ordinary project-owned file');
  if(stat.size>byteLimit-bytes)throw new RangeError('source inventory byte limit exceeded');
  const raw=readFileSync(full);bytes+=raw.length;if(bytes>byteLimit)throw new RangeError('source inventory byte limit exceeded');
  const source=new TextDecoder('utf-8',{fatal:true}).decode(raw),language=extension==='.rs'?'Rust':['.ts','.tsx','.mts','.cts'].includes(extension)?'TypeScript':'JavaScript',firstLine=source.slice(0,source.indexOf('\n')<0?source.length:source.indexOf('\n'));
  const generatorMarked=firstLine.startsWith('// Generated from complete ')||firstLine.startsWith('// Generated from ');
  const selected=signals.filter(row=>row.language===(language==='TypeScript'?'JavaScript':language)),values=readSourceConstructSignals(source,selected.map(row=>row.needle));
  let occurrences=0;const fileSignals=[];
  for(let index=0;index<selected.length;index+=1){const row=selected[index],count=values[index*3];if(!count)continue;occurrences+=count;row.totalOccurrences+=count;row.matchingFiles+=1;if(generatorMarked)row.generatorMarkedOccurrences+=count;else row.unmarkedOccurrences+=count;
   const coordinates={count,firstOffset:values[index*3+1],firstLfLine:values[index*3+2]};fileSignals.push({id:row.id,...coordinates});
   const sample={path,...coordinates};row.samples.push(sample);row.samples.sort((a,b)=>b.count-a.count||(a.path<b.path?-1:a.path>b.path?1:0));if(row.samples.length>sampleLimit)row.samples.length=sampleLimit;
  }
  files.push({path,language,mode:header[0],indexObject:header[1],bytes:raw.length,sha256:hash(raw),generatorMarked,literalSignalOccurrences:occurrences,signals:fileSignals});
 }
 return {schemaVersion:2,model:'tracked own-production-source lexical candidate inventory 2',sourceState:'working-tree bytes with separate Git index object identities; revision identifies HEAD only',revision,gitVersion,gitVersionRange:'>=2.29.0 <3.0.0',objectFormat,roots:[...roots],byteLimit,fileLimit,totalBytes:bytes,files,signals,
  interpretation:'Literal substring candidates only. Includes strings/comments, overlapping signals and source examples; excludes no tracked production module on the basis of a candidate. Offsets are UTF16 units and lines count LF separators. Generator headers are hints, not recipe certificates. No parser, typechecker, translation, product, test, compiler or completion observation is performed on inspected sources.'};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
 const value=draftOwnSourceConstructInventory(fileURLToPath(new URL('../../',import.meta.url)));
 writeProjectData(new URL('../../parity/inventories/own-source-construct-candidates.lino',import.meta.url),value);
 process.stdout.write('Authored static inventory: '+value.files.length+' tracked production files, '+value.totalBytes+' source bytes; '+value.signals.length+' lexical signals. Not semantic coverage.\n');
}
