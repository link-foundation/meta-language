import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {gzipSync} from 'node:zlib';
import test from 'node:test';
import {readMarkdownNativeSourceProfile,readMarkdownInlineRanges,projectMarkdownInlineChildren} from '../scripts/markdown-native-source-profile.mjs';
import {captureMarkdownNativeSources,findMarkdownNativeSourceArchive} from '../scripts/capture-markdown-native-sources.mjs';
import {structuredDataToLino,readStructuredDataLino} from '../src/structured-data-links.js';

const digest=value=>createHash('sha256').update(value).digest('hex');

// Creates only tiny synthetic regular-file members, never upstream sources.
function createSourceArchive(files){
  const records=[];
  for(const[path,text]of Object.entries(files)){
    const content=Buffer.from(text),header=Buffer.alloc(512);
    assert.ok(Buffer.byteLength(path)<100);
    header.write(path,0,100);header.write('0000644\0',100,8);
    header.write('0000000\0',108,8);header.write('0000000\0',116,8);
    header.write(content.length.toString(8).padStart(11,'0')+'\0',124,12);
    header.write('00000000000\0',136,12);header.fill(32,148,156);
    header.write('0',156);header.write('ustar\0',257,6);header.write('00',263,2);
    const checksum=header.reduce((sum,byte)=>sum+byte,0);
    header.write(checksum.toString(8).padStart(6,'0')+'\0 ',148,8);
    records.push(header,content,Buffer.alloc((512-content.length%512)%512));
  }
  return gzipSync(Buffer.concat([...records,Buffer.alloc(1024)]));
}

test('Markdown source plan retains both real pins, every block scanner token and explicit composition gaps',()=>{
  const profile=readMarkdownNativeSourceProfile();
  assert.equal(profile.behavioralEvidence,false);
  assert.equal(profile.archive.version,'0.5.6');
  assert.equal(profile.archive.sha256,'b2d54d63bc7fdf0c1c38237611d966548fbccf8f0923a7de5d525b87c9957a96');
  assert.deepEqual(profile.components.map(({id,position})=>[id,position]),[['markdown',0],['markdown_inline',1]]);
  assert.deepEqual(profile.components.map(({grammarMember})=>grammarMember),['tree-sitter-markdown/src/grammar.json','tree-sitter-markdown-inline/src/grammar.json']);
  const tokens=profile.scannerFamilies.flatMap(({sourceTokens})=>sourceTokens);
  assert.equal(tokens.length,47);assert.equal(new Set(tokens).size,47);
  for(const token of['_error','_eof','block_continuation','_pipe_table_start','_html_block_7_start','_fenced_code_block_end_tilde'])assert.ok(tokens.includes(token));
  assert.equal(profile.composition.target,'markdown_inline');
  assert.deepEqual(profile.composition.sourceQualifiedHostKinds,['inline','pipe_table_cell']);
  assert.equal(profile.composition.ranges.exclude.firstRawChildIndex,1);
  assert.equal(profile.composition.spanUnit,'absolute-utf8-bytes');
  assert.equal(profile.composition.resourceBudgets,'shared and unchanged');
  assert.equal(profile.composition.implementationGaps.length,4);
});

test('source capture authenticates synthetic archive members, include closure and corpus metadata without storing grammar payloads',()=>{
  const profile=readMarkdownNativeSourceProfile(),files={'fixture/common.h':'/* shared include */\n','fixture/Cargo.toml':'[package]\nname="synthetic-control"\n'};
  profile.archive={crate:'synthetic-control',version:'1',memberPrefix:'fixture/',license:'MIT',repository:'synthetic-test-input'};
  for(const component of profile.components){
    files[`fixture/${component.grammarMember}`]=JSON.stringify({name:component.id,rules:{document:{type:'STRING',value:'x'}},externals:[{type:'SYMBOL',name:'control_token'}]});
    files[`fixture/${component.scannerMember}`]='#include "../../common.h"\n/* source capture only */\n';
    const parser=`/* synthetic parser ${component.id} */\n`;
    files[`fixture/${component.directory}/src/parser.c`]=parser;
    component.parserSha256=digest(parser);
    files[`fixture/${component.corpusDirectory}tiny.txt`]='===\nsynthetic\n===\nx\n---\n(document)\n';
  }
  const archive=createSourceArchive(files);profile.archive.sha256=digest(archive);
  const directory=mkdtempSync(join(tmpdir(),'meta-language-markdown-source-control-'));
  const file=join(directory,'synthetic.crate');writeFileSync(file,archive);
  try{
    assert.throws(()=>findMarkdownNativeSourceArchive(directory,profile),/archive is absent/u);
    const cache=join(directory,'registry/cache/synthetic-index');mkdirSync(cache,{recursive:true});
    const cached=join(cache,'synthetic-control-1.crate');writeFileSync(cached,archive);
    assert.equal(findMarkdownNativeSourceArchive(directory,profile),cached);
    const receipt=captureMarkdownNativeSources(file,profile);
    assert.equal(receipt.behavioralEvidence,false);
    assert.equal(receipt.archive.sha256,digest(archive));
    assert.deepEqual(receipt.components.map(({id})=>id),['markdown','markdown_inline']);
    for(const component of receipt.components){
      assert.deepEqual(component.ruleNames,['document']);
      assert.deepEqual(component.externalTokens,['control_token']);
      assert.deepEqual(component.scannerIncludes.map(({path})=>path),['common.h']);
      assert.equal(component.corpusMembers.length,1);assert.equal(component.corpusIncludedInArchive,true);
    }
    for(const member of receipt.members){
      const original=files['fixture/'+member.path];
      assert.equal(member.sha256,digest(original));assert.equal(member.bytes,Buffer.byteLength(original));
      assert.equal(Object.hasOwn(member,'content'),false,'only provenance and hashes enter the receipt');
    }
    assert.throws(()=>captureMarkdownNativeSources(file,{...profile,archive:{...profile.archive,sha256:'0'.repeat(64)}}),/checksum mismatch/u);
    const bad=structuredClone(profile);bad.components[1].parserSha256='0'.repeat(64);
    assert.throws(()=>captureMarkdownNativeSources(file,bad),/parser source differs/u);
    const missing=structuredClone(profile);missing.components[0].grammarMember='absent/grammar.json';
    assert.throws(()=>captureMarkdownNativeSources(file,missing),/archive lacks/u);
    writeFileSync(cached,'corrupt');
    assert.throws(()=>findMarkdownNativeSourceArchive(directory,profile),/cache archive checksum mismatch/u);
  }finally{rmSync(directory,{recursive:true,force:true});}
});

test('CI captures source receipts after the existing locked acquisition without replacing the whole grammar run',()=>{
  const workflow=readFileSync(new URL('../../.github/workflows/ci.yml',import.meta.url),'utf8');
  const job=workflow.slice(workflow.indexOf('  grammar-bulk-pipeline:'),workflow.indexOf('  candidates:'));
  assert.match(job,/timeout-minutes: 60/u);
  assert.match(job,/run: cargo fetch --locked/u);
  assert.match(job,/run: node js\/scripts\/run-grammar-bulk-pipeline\.mjs --out-dir grammar-bulk --jobs 4 --timeout 240/u);
  const capture=job.indexOf('name: Capture the pinned Markdown block and inline source closure');
  assert.ok(capture>job.indexOf('run: cargo fetch --locked'));
  assert.ok(capture<job.indexOf('name: Run the bulk grammar pipeline'),'source feedback is retained before the whole-registry workload');
  assert.match(job,/name: Run the bulk grammar pipeline\n\s+if: \$\{\{ !cancelled\(\) \}\}/u,'the complete prior workload still runs if source capture fails');
  assert.match(job,/--cargo-cache --out grammar-bulk\/markdown-native-source-capture\.lino/u);
  assert.match(job,/name: markdown-native-source-capture-\$\{\{ github\.sha \}\}/u);
  assert.match(job,/path: grammar-bulk\/markdown-native-source-capture\.lino\n\s+if-no-files-found: error/u);
  assert.match(job,/name: grammar-bulk-matrix\n\s+path: grammar-bulk\//u);
});

test('linked Markdown composition policy preserves excluded ranges, raw child indices and UTF-8 source ownership',()=>{
  const source=Buffer.from('> *é\n> β*\n'),policy=readMarkdownNativeSourceProfile().composition;
  const host={component:'markdown',sourceKind:'inline',start:2,end:11,children:[
    {kind:'first_named_child',named:true,start:2,end:3},
    {kind:'anonymous_text',named:false,start:3,end:6},
    {kind:'block_continuation',named:true,start:6,end:8,field:'continuation',sourceAlias:'block_continuation'},
  ]};
  const restored=readStructuredDataLino(structuredDataToLino({policy,host}));
  assert.deepEqual(restored,{policy,host});
  const range=readMarkdownInlineRanges(restored.policy,restored.host);
  assert.deepEqual(range.includedRanges,[{start:2,end:6},{start:8,end:11}]);
  assert.deepEqual(range.excludedChildren,[host.children[2]]);
  assert.equal(Buffer.concat(range.includedRanges.map(({start,end})=>source.subarray(start,end))).toString('utf8'),'*é\nβ*');
  const occurrences=[...range.includedRanges,...range.excludedChildren].sort((left,right)=>left.start-right.start);
  assert.equal(Buffer.concat(occurrences.map(({start,end})=>source.subarray(start,end))).toString('utf8'),source.subarray(host.start,host.end).toString('utf8'));
  const offset=Buffer.byteLength('前\n'),shifted={...host,start:host.start+offset,end:host.end+offset,
    children:host.children.map(child=>({...child,start:child.start+offset,end:child.end+offset}))};
  assert.deepEqual(readMarkdownInlineRanges(policy,shifted).includedRanges,range.includedRanges.map(({start,end})=>({start:start+offset,end:end+offset})));
  assert.equal(readMarkdownInlineRanges(policy,{...host,sourceKind:'paragraph'}),null);
  assert.throws(()=>readMarkdownInlineRanges(policy,{...host,component:'unknown'}),/Unknown Markdown host component/u);
  assert.throws(()=>readMarkdownInlineRanges({...policy,ranges:{...policy.ranges,exclude:{...policy.ranges.exclude,children:'descendants'}}},host),/Unsupported/u);
});

test('linked Markdown projection preserves fields, deepest owners, stable ties and zero-width boundaries',()=>{
  const policy=readMarkdownNativeSourceProfile().composition;
  const inline=[{kind:'emphasis',start:2,end:11,field:'body',children:[{kind:'text',start:3,end:10,children:[]}]}];
  const excluded=[{kind:'continuation',start:6,end:8,named:true,field:'continuation',children:[]}];
  const input=readStructuredDataLino(structuredDataToLino({policy,inline,excluded}));
  const result=projectMarkdownInlineChildren(input.policy,input.inline,input.excluded);
  assert.equal(result[0].field,'body');
  assert.deepEqual(result[0].children[0].children,excluded);
  assert.deepEqual(input.inline,inline,'projection does not mutate the archived component tree');
  const children=[{kind:'inline',start:2,end:5,children:[]}],nodes=[
    {kind:'at_start',start:2,end:2},{kind:'inside',start:3,end:3},{kind:'at_end',start:5,end:5},
  ];
  const projected=projectMarkdownInlineChildren(policy,children,nodes);
  assert.deepEqual(projected.map(({kind})=>kind),['at_start','inline','at_end']);
  assert.deepEqual(projected[1].children,[nodes[1]]);
  const cycle={kind:'cycle',start:0,end:1,children:[]};cycle.children.push(cycle);
  assert.throws(()=>projectMarkdownInlineChildren(policy,[cycle],[]),/Cyclic Markdown CST ownership/u);
  assert.throws(()=>projectMarkdownInlineChildren(policy,[{type:'token',kind:'text',start:2,end:11,text:'opaque spanning text'}],excluded),/native range view is required/u);
});
