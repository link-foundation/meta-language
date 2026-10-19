#!/usr/bin/env node
// Exact source publication and reversal, separate from C execution semantics.
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';
import {wellFormedString} from '../../js/src/structured-data-model.js';

const root=new URL('../../',import.meta.url);
const archiveFile=new URL('./data/sizeof-source-publication.lino',import.meta.url);
const paths=['experiments/restricted-c-source-lowering/restricted-c-cst.mjs',
 'experiments/restricted-c-source-lowering/restricted-c-controls.test.mjs',
 'experiments/restricted-c-source-lowering/restricted-c-sizeof.test.mjs'];
const originalHashes=['b77b21b087b5db9b84974e7a1a02c0e9880490939e15343a0e47ebbf6324aa11',
 '453b0466f8d089ef0f81cc4c001c1e4614a30a35ab155792943627dd57a183e1'];
const oldConstructor='  const state = new Lowering(parsed, bytes, options);\n';
const newConstructor='  const state = new SizeQueryLowering(parsed, bytes, options);\n';
const oldAssertions="  assert.ok(unit.obligations.some(item => item.category === 'C expression form'\n"
 +"    && ['sizeof_expression', 'size_of_expression'].includes(item.detail)));\n"
 +"  assert.ok(instructions(unit.functions.find(fn => fn.name === 'width').body)\n"
 +"    .some(item => item.operation === 'unresolved C expression'));\n";

/** Reconstruct the finite source extension and recover both entire originals. */
export function generateRestrictedCSizeQueryPublication(archive){
 if(archive?.schemaVersion!==1||archive.kind!=='restricted C size query source publication'
  ||!Array.isArray(archive.originals)||archive.originals.length!==2
  ||!Array.isArray(archive.edits)||archive.edits.length!==2
  ||!Array.isArray(archive.published)||archive.published.length!==3
  ||typeof archive.moduleAppend!=='string'||!wellFormedString(archive.moduleAppend)||archive.moduleAppend.length===0)throw new TypeError('invalid C size-query source publication');
 const sources=[];
 for(let index=0;index<2;index++){
  const original=archive.originals[index],edit=archive.edits[index];
  if(original?.path!==paths[index]||original.sha256!==originalHashes[index]||typeof original.source!=='string'||!wellFormedString(original.source)
   ||Buffer.byteLength(original.source)!==original.bytes
   ||createHash('sha256').update(original.source).digest('hex')!==originalHashes[index])throw new TypeError('C size-query publication changed an original source');
  if(edit?.path!==paths[index]||edit.before!==(index===0?oldConstructor:oldAssertions)
   ||typeof edit.after!=='string'||!wellFormedString(edit.after)||edit.after.length===0||edit.after===edit.before
   ||(index===0&&edit.after!==newConstructor))throw new TypeError('C size-query publication changed its finite source edit');
  const pieces=original.source.split(edit.before);
  if(pieces.length!==2)throw new TypeError('C size-query publication edit needs one original occurrence');
  sources.push({path:paths[index],source:pieces.join(edit.after)});
 }
 sources[0].source+=archive.moduleAppend;
 const added=archive.addedTest;
 if(added?.path!==paths[2]||typeof added.source!=='string'||!wellFormedString(added.source)||Buffer.byteLength(added.source)!==added.bytes
  ||createHash('sha256').update(added.source).digest('hex')!==added.sha256)throw new TypeError('C size-query publication changed its complete added test');
 sources.push({path:paths[2],source:added.source});
 for(let index=0;index<3;index++){
  const actual=sources[index],published=archive.published[index];
  if(published?.path!==actual.path||published.bytes!==Buffer.byteLength(actual.source)
   ||published.sha256!==createHash('sha256').update(actual.source).digest('hex'))throw new TypeError('C size-query publication differs from its published source bytes');
  if(index<2){
   const forward=index===0?actual.source.slice(0,-archive.moduleAppend.length):actual.source;
   const pieces=forward.split(archive.edits[index].after);
   if(pieces.length!==2||pieces.join(archive.edits[index].before)!==archive.originals[index].source)throw new TypeError('C size-query publication cannot reverse its complete original source');
  }
 }
 return sources;
}

/** Compare actual published bytes using the adjacent normal Links archive. */
export function checkRestrictedCSizeQueryPublication(options={}){
 const archive=options.archive??readProjectData(archiveFile);
 const sources=generateRestrictedCSizeQueryPublication(archive);
 for(const source of sources){
  const actual=options.readSource?options.readSource(source.path):readFileSync(new URL(source.path,root));
  if(!Buffer.from(actual).equals(Buffer.from(source.source)))throw new TypeError(`C size-query source publication is stale: ${source.path}`);
 }
 return sources.map(source=>source.path);
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
 const args=process.argv.slice(2);
 if(args.length>1||args.some(arg=>!['--check','--write'].includes(arg)))throw new Error('usage: sizeof-source-publication.mjs [--check | --write]');
 if(args.includes('--write')){
  const archive=readProjectData(archiveFile),sources=generateRestrictedCSizeQueryPublication(archive);
  writeProjectData(archiveFile,archive,{format:'lino'});
  for(const source of sources)writeFileSync(new URL(source.path,root),source.source);
 }
 console.log('C size-query source publication:',checkRestrictedCSizeQueryPublication().length,'files match their source archive and complete inverses');
}
