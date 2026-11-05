// Package authoring from actual Rust include tokens; no Rust process runs.
import {readFileSync,readdirSync,lstatSync,writeFileSync} from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {tokenize} from '../src/translation/lexer.js';
import {writeProjectData} from './project-data.mjs';

const root=fileURLToPath(new URL('../../',import.meta.url));
const begin='    # BEGIN generated literal source asset closure';
const end='    # END generated literal source asset closure';
const digest=source=>createHash('sha256').update(source).digest('hex');
function files(directory){
  const output=[];
  for(const entry of readdirSync(directory,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:a.name>b.name?1:0)){
    const name=path.join(directory,entry.name);
    if(entry.isSymbolicLink())throw new TypeError('package source assets refuse symbolic links: '+name);
    if(entry.isDirectory())output.push(...files(name));else if(entry.isFile()&&entry.name.endsWith('.rs'))output.push(name);
  }
  return output;
}
/** Read actual literal macro operands. Comment/string contents are not macros. */
export function readRustPackageSourceIncludes(source){
  const {tokens}=tokenize(source,'Rust'),includes=[];
  for(let index=0;index<tokens.length;index+=1){
    const token=tokens[index];
    const name=token.value.split('::').at(-1);
    if(!['include_str','include_bytes'].includes(name))continue;
    const offset=token.kind==='macro'?1:tokens[index+1]?.value==='!'?2:0;
    if(!offset)continue;
    const open=tokens[index+offset]?.value,close=open==='('?')':open==='['?']':open==='{'?'}':'';
    const operand=tokens[index+offset+1],after=tokens[index+offset+2],last=after?.value===','?tokens[index+offset+3]:after;
    if(!close||operand?.kind!=='string'||last?.value!==close)throw new TypeError('package source asset needs an explicit supported include operand at '+token.start);
    if(!operand.value||operand.value.includes('\0')||operand.value.includes('\\')||path.posix.isAbsolute(operand.value))throw new TypeError('package source asset needs a portable relative path');
    includes.push({macro:name,path:operand.value,start:token.start,end:last.end});
  }
  return includes;
}
function matches(pattern,file){
  // Cargo's rooted source patterns in this repository; reject unknown syntax.
  if(/[?\[\]{}!]/u.test(pattern))throw new TypeError('unmodelled package include pattern '+pattern);
  const segments=pattern.split('/');
  const rendered=segments.map((segment,index)=>segment==='**'?(index===segments.length-1?'.*':'(?:[^/]+/)*')
    :segment.split('*').map(part=>part.replace(/[.*+?^${}()|[\]\\]/gu,'\\$&')).join('[^/]*')+(index===segments.length-1?'':'/')).join('');
  return new RegExp('^'+rendered+'$','u').test(file);
}
/** Construct the conservative closure of assets included by every shipped .rs. */
export function draftRustPackageSourceAssets({check=false}={}){
  const crate=path.join(root,'rust'),manifestPath=path.join(crate,'Cargo.toml');
  const manifest=readFileSync(manifestPath,'utf8');
  const selection=manifest.match(/\binclude = \[\n([\s\S]*?)\n\]/u);
  if(!selection)throw new TypeError('the package needs its explicit include allowlist');
  const previous=selection[1];
  const first=previous.indexOf(begin),last=previous.indexOf(end);
  if((first<0)!==(last<0)||first>=0&&(first!==previous.lastIndexOf(begin)||last!==previous.lastIndexOf(end)||last<first))throw new TypeError('ambiguous generated package asset interval');
  const retained=first<0?previous:previous.slice(0,first)+previous.slice(last+end.length);
  const patterns=[...retained.matchAll(/^\s*("[^"\n]+")\s*,?\s*$/gmu)].map(match=>JSON.parse(match[1]));
  if(!patterns.includes('src/**/*.rs'))throw new TypeError('the package needs its complete Rust source glob');
  const inputs=[],owners=new Map();
  for(const file of files(path.join(crate,'src'))){
    const source=readFileSync(file,'utf8');
    if(Buffer.byteLength(source)>4*1024*1024)throw new TypeError('one Rust source exceeds the authoring byte bound');
    const sourcePath=path.relative(root,file).split(path.sep).join('/');
    inputs.push({path:sourcePath,bytes:Buffer.byteLength(source),sha256:digest(source)});
    // Identifiers cannot hide their spelling behind Unicode escapes in Rust.
    // This rejects no actual include and avoids lexing unrelated large modules.
    if(!source.includes('include_str')&&!source.includes('include_bytes'))continue;
    let includes;
    try{includes=readRustPackageSourceIncludes(source);}catch(error){error.message=sourcePath+': '+error.message;throw error;}
    for(const include of includes){
      const absolute=path.resolve(path.dirname(file),include.path);
      const relative=path.relative(crate,absolute).split(path.sep).join('/');
      if(relative==='..'||relative.startsWith('../')||path.isAbsolute(relative))throw new TypeError('a shipped source references an asset outside the crate: '+sourcePath+' -> '+include.path);
      if(!relative.startsWith('src/'))throw new TypeError('a shipped source asset is outside its source directory: '+relative);
      if(!lstatSync(absolute).isFile())throw new TypeError('source include is not a regular file: '+relative);
      const row=owners.get(relative)??{path:relative,owners:[]};
      row.owners.push({source:sourcePath,...include});owners.set(relative,row);
    }
  }
  const assets=[...owners.values()].sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0).map(row=>{
    const bytes=readFileSync(path.join(crate,row.path));return {...row,bytes:bytes.length,sha256:digest(bytes)};
  });
  const additions=assets.filter(asset=>!patterns.some(pattern=>matches(pattern,asset.path))).map(asset=>asset.path);
  const block=additions.length?begin+'\n'+additions.map(file=>'    '+JSON.stringify(file)+',\n').join('')+end+'\n':'';
  const body=retained.replace(/\n*$/u,'')+'\n'+block;
  const current=manifest.slice(0,selection.index)+selection[0].replace(previous,()=>body.trimEnd())+manifest.slice(selection.index+selection[0].length);
  if(check){if(current!==manifest)throw new TypeError('package source asset closure is stale; author the complete closure');return;}
  writeFileSync(manifestPath,current);
  writeProjectData(new URL('../../parity/self-translation/rust-package-source-assets.lino',import.meta.url),{
    schemaVersion:1,model:'literal Rust source asset closure 1',inputs,assets,additions,
    manifest:{source:current,bytes:Buffer.byteLength(current),sha256:digest(current)},
    boundary:'Package source construction only. Literal operands come from Rust tokens; no source program, fixture, Cargo process, native compilation, behavior gate or execution observation runs. Unknown macro operands and outside-crate assets refuse. Published size and clean-consumer execution remain CI obligations.',
  });
}
if(process.argv[1]&&path.resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  if(process.argv.slice(2).join(' ')==='--draft')draftRustPackageSourceAssets();
  else if(process.argv.slice(2).join(' ')==='--check')draftRustPackageSourceAssets({check:true});
  else throw new TypeError('use --draft or --check');
}
