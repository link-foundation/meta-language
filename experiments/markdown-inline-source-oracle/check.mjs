#!/usr/bin/env node
// Compare actual C source transitions with actual shared Links execution.
// This is an independent transition oracle, not a parser support declaration.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync,rmSync} from 'node:fs';
import {dirname,join,resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {isDeepStrictEqual} from 'node:util';
import {findMarkdownNativeSourceArchive} from '../../js/scripts/capture-markdown-native-sources.mjs';
import {readMarkdownNativeSourceProfile} from '../../js/scripts/markdown-native-source-profile.mjs';
import {writeProjectData} from '../../js/scripts/project-data.mjs';
import {inlineSourceTokens,inlineSourceProbeCases,probeInlineLinksOperations} from './probe.mjs';

const root=fileURLToPath(new URL('../../',import.meta.url));
const digest=value=>createHash('sha256').update(value).digest('hex');

/** Authenticate and compare each original source callback, only in GitHub CI. */
export function verifyInlineScannerSource({archive,output}) {
  if(process.env.CI!=='true'||process.env.GITHUB_ACTIONS!=='true')throw new Error('native inline scanner compilation runs only in GitHub CI');
  const profile=readMarkdownNativeSourceProfile(),bytes=readFileSync(archive);
  assert.equal(digest(bytes),profile.archive.sha256,'the complete original source archive is authenticated');
  const member=path=>execFileSync('tar',['-xOzf','-',profile.archive.memberPrefix+path],{input:bytes,maxBuffer:1024*1024,timeout:30_000});
  const scanner=member('tree-sitter-markdown-inline/src/scanner.c');
  const header=member('tree-sitter-markdown-inline/src/tree_sitter/parser.h');
  assert.equal(digest(scanner),'6a9647420b7260955e8bf1b8338979726347211177bb5cf3d2a9c18109655ac3');
  assert.equal(digest(header),'180b893c8734778fd32f372dfbc27bd6ad1cd2221f26150b31256ff6716320d2');
  const parent=join(root,'.external-grammars');mkdirSync(parent,{recursive:true});
  const work=mkdtempSync(join(parent,'inline-source-oracle-'));
  try{
    mkdirSync(join(work,'tree_sitter'));
    writeFileSync(join(work,'scanner.c'),scanner);writeFileSync(join(work,'tree_sitter/parser.h'),header);
    const harness=fileURLToPath(new URL('./native-probe.c',import.meta.url)),binary=join(work,'source-probe');
    execFileSync('cc',['-std=c11','-O0','-I',work,harness,'-o',binary],{timeout:30_000,stdio:'pipe'});
    const observations=[];
    for(const fixture of inlineSourceProbeCases()){
      const state=fixture.state??[0,0,0,0];
      const mask=fixture.valid.reduce((value,token)=>value|(1<<inlineSourceTokens.indexOf(token)),0);
      let original=null,linked=null;const diagnostics=[];
      try{
        const fields=execFileSync(binary,[String(mask),...state.map(String),Buffer.from(fixture.input).toString('hex')],{encoding:'utf8',timeout:1000}).trim().split(/\s+/u).map(Number);
        assert.equal(fields.length,8,'the original callback reports every result, cursor, mark and serialized state byte');
        assert.ok(fields.every(Number.isSafeInteger),'the source callback protocol retains exact integers');
        assert.ok(fields[0]===0||fields[0]===1);
        if(fields[0]===1)assert.ok(fields[1]>=0&&fields[1]<inlineSourceTokens.length);
        original={accepted:fields[0]===1,token:fields[0]===1?inlineSourceTokens[fields[1]]:null,cursor:fields[2],mark:fields[3]===-1?null:fields[3],state:fields.slice(4)};
      }catch(error){diagnostics.push({side:'original-source',message:String(error.message),stderr:String(error.stderr??'')});}
      try{linked=probeInlineLinksOperations(fixture);}
      catch(error){diagnostics.push({side:'shared-links-operations',message:String(error.message)});}
      observations.push({input:fixture.input,validSymbols:fixture.valid,initialState:state,original,linked,
        equal:diagnostics.length===0&&isDeepStrictEqual(original,linked),diagnostics});
    }
    const report={schemaVersion:1,scope:'authenticated inline scanner callbacks with explicit simultaneous source-symbol context; no grammar, CST composition or Rust operation execution claim',
      commit:execFileSync('git',['rev-parse','HEAD'],{cwd:root,encoding:'utf8'}).trim(),archiveSha256:profile.archive.sha256,
      sourceApplicability:{package:profile.archive.crate,minimumVersionInclusive:profile.archive.version,maximumVersionInclusive:profile.archive.version},
      compiler:{command:'cc',standard:'c11',version:execFileSync('cc',['--version'],{encoding:'utf8',timeout:1000}).split('\n')[0]},
      scannerSha256:digest(scanner),headerSha256:digest(header),harnessSha256:digest(readFileSync(harness)),
      operationGeneratorSha256:digest(readFileSync(new URL('../../js/scripts/markdown-inline-scanner-family.mjs',import.meta.url))),observations};
    mkdirSync(dirname(resolve(output)),{recursive:true});writeProjectData(resolve(output),report);
    const failures=observations.filter(item=>!item.equal);
    assert.equal(failures.length,0,'source transition mismatches: '+failures.slice(0,3).map(item=>JSON.stringify(item)).join('\n'));
    console.log('Inline scanner source oracle:',observations.length,'actual source callbacks agree with shared Links operations');
    return report;
  }finally{rmSync(work,{recursive:true,force:true});}
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2),index=args.indexOf('--archive'),out=args.indexOf('--out');
  if(out<0||!args[out+1]?.endsWith('.lino')||(index<0&&!args.includes('--cargo-cache')))throw new Error('usage: check.mjs (--archive PINNED.crate | --cargo-cache) --out REPORT.lino');
  // Reject a local native invocation before archive discovery or extraction.
  if(process.env.CI!=='true'||process.env.GITHUB_ACTIONS!=='true')throw new Error('native inline scanner compilation runs only in GitHub CI');
  verifyInlineScannerSource({archive:index>=0?args[index+1]:findMarkdownNativeSourceArchive(),output:args[out+1]});
}
