#!/usr/bin/env node
// CI-only source capture input: an already downloaded, pinned .crate archive.
// No download, build, source extraction to disk or native parser execution.
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {existsSync,readFileSync,readdirSync,statSync} from 'node:fs';
import {homedir} from 'node:os';
import {join,posix,resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {writeProjectData} from './project-data.mjs';
import {readMarkdownNativeSourceProfile} from './markdown-native-source-profile.mjs';

const MAXIMUM_ARCHIVE_BYTES=64*1024*1024;
const MAXIMUM_MEMBER_BYTES=16*1024*1024;
const digest=value=>createHash('sha256').update(value).digest('hex');

/** Locate the archive acquired by the existing locked Cargo fetch, without downloading. */
export function findMarkdownNativeSourceArchive(cargoHome=process.env.CARGO_HOME??join(homedir(),'.cargo'),profile=readMarkdownNativeSourceProfile()){
  const cache=join(cargoHome,'registry/cache');
  const name=`${profile.archive.crate}-${profile.archive.version}.crate`;
  const candidates=existsSync(cache)?readdirSync(cache,{withFileTypes:true})
    .filter(entry=>entry.isDirectory()).map(entry=>join(cache,entry.name,name)).filter(file=>existsSync(file)).sort():[];
  if(candidates.length===0)throw new Error(`Pinned Markdown crate archive is absent from ${cache}; CI must finish cargo fetch --locked first`);
  for(const file of candidates){
    if(statSync(file).size>MAXIMUM_ARCHIVE_BYTES||digest(readFileSync(file))!==profile.archive.sha256)throw new Error('Markdown Cargo cache archive checksum mismatch');
  }
  return candidates[0];
}

/** Record actual pinned archive members without claiming native grammar support. */
export function captureMarkdownNativeSources(archive,profile=readMarkdownNativeSourceProfile()){
  const file=resolve(archive),size=statSync(file).size;
  if(size>MAXIMUM_ARCHIVE_BYTES)throw new Error('Markdown source archive exceeds the capture byte bound');
  const bytes=readFileSync(file);
  if(bytes.length>MAXIMUM_ARCHIVE_BYTES)throw new Error('Markdown source archive exceeds the capture byte bound');
  if(digest(bytes)!==profile.archive.sha256)throw new Error('Markdown source archive checksum mismatch');
  const prefix=profile.archive.memberPrefix;
  const paths=execFileSync('tar',['-tzf','-'],{input:bytes,maxBuffer:MAXIMUM_MEMBER_BYTES,timeout:30000}).toString('utf8').trim().split('\n');
  if(paths.length>10000||new Set(paths).size!==paths.length||paths.some(path=>!path.startsWith(prefix)||path.includes('\0')||path.includes('\\')||path.split('/').includes('..')))throw new Error('Markdown archive member names are ambiguous or outside the pinned source root');
  const present=new Set(paths),members=new Map(),contents=new Map();let capturedBytes=0;
  const member=(relative,role)=>{
    const path=prefix+relative;
    if(!present.has(path))throw new Error(`Pinned Markdown archive lacks ${relative}`);
    if(members.has(relative))return{record:members.get(relative),content:contents.get(relative)};
    const content=execFileSync('tar',['-xOzf','-',path],{input:bytes,maxBuffer:MAXIMUM_MEMBER_BYTES,timeout:30000});
    capturedBytes+=content.length;
    if(capturedBytes>MAXIMUM_ARCHIVE_BYTES)throw new Error('Markdown source members exceed the aggregate capture byte bound');
    const record={path:relative,role,bytes:content.length,sha256:digest(content)};
    members.set(relative,record);
    contents.set(relative,content);
    return{record,content};
  };
  const components=[];
  for(const component of profile.components){
    const grammar=member(component.grammarMember,'grammar');
    const scanner=member(component.scannerMember,'scanner');
    const parser=member(`${component.directory}/src/parser.c`,'generated-oracle-parser');
    if(parser.record.sha256!==component.parserSha256)throw new Error(`Pinned Markdown parser source differs from the existing ${component.id} oracle`);
    const parsed=JSON.parse(grammar.content.toString('utf8'));
    if(!parsed.rules||typeof parsed.rules!=='object')throw new Error(`Invalid pinned ${component.id} grammar`);
    const pending=[component.scannerMember],seen=new Set();
    while(pending.length){
      const path=pending.pop();if(seen.has(path))continue;seen.add(path);
      const source=path===component.scannerMember?scanner:member(path,'scanner-include');
      for(const match of source.content.toString('utf8').matchAll(/^\s*#\s*include\s+"([^"]+)"/gmu)){
        const included=posix.normalize(posix.join(posix.dirname(path),match[1]));
        if(included.startsWith('../')||posix.isAbsolute(included))throw new Error('Markdown scanner include escapes the pinned archive');
        pending.push(included);
      }
    }
    const corpus=paths.filter(path=>path.startsWith(prefix+component.corpusDirectory)&&!path.endsWith('/'))
      .map(path=>member(path.slice(prefix.length),'corpus').record);
    components.push({id:component.id,position:component.position,role:component.role,
      grammar:grammar.record,scanner:scanner.record,parser:parser.record,
      scannerIncludes:[...seen].filter(path=>path!==component.scannerMember).map(path=>members.get(path)),
      ruleNames:Object.keys(parsed.rules),externalTokens:(parsed.externals??[]).map(value=>value.name??value.value),
      corpusMembers:corpus,corpusIncludedInArchive:corpus.length>0});
  }
  for(const path of['Cargo.toml','.cargo_vcs_info.json'])if(present.has(prefix+path))member(path,'package-provenance');
  return{schemaVersion:1,kind:'authenticated-source-capture',behavioralEvidence:false,
    archive:{...profile.archive,bytes:bytes.length},components,members:[...members.values()],
    unresolvedObligations:['native scanner transitions require source audit and shared Links generation','block and inline composition remains required','missing archive corpora require a separately pinned upstream corpus capture','source capture does not prove parsing or translation']};
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2),output=args[args.indexOf('--out')+1],profile=readMarkdownNativeSourceProfile();
  const explicit=args.includes('--archive')?args[args.indexOf('--archive')+1]:null;
  if((args.includes('--archive')===args.includes('--cargo-cache'))||!args.includes('--out')||!output||!output.endsWith('.lino')||(explicit!==null&&(!explicit||explicit.startsWith('--'))))throw new Error('usage: capture-markdown-native-sources.mjs (--archive PINNED.crate | --cargo-cache) --out RECEIPT.lino');
  const receipt=captureMarkdownNativeSources(explicit??findMarkdownNativeSourceArchive(undefined,profile),profile);
  writeProjectData(resolve(output),receipt,{format:'lino'});
  process.stdout.write(`Captured ${receipt.components.length} Markdown components and ${receipt.members.length} pinned source members. No behavioral verification performed.\n`);
}
