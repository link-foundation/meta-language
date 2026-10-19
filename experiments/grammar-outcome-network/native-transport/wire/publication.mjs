// Authenticate authored predecessors and explicit portability/harness changes.
// Source verification does not compile or execute the proposed native transport.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {readProjectData} from '../../../../js/scripts/project-data.mjs';
import {tokenize} from '../../../../js/src/translation/lexer.js';
import {readRustSourceItems} from '../../../../js/scripts/generate-private-output-test-model.mjs';
import {outcomeWirePolicy} from './wire-policy.mjs';

const digest=source=>createHash('sha256').update(source).digest('hex');
const archivePath=fileURLToPath(new URL('./original-authoring.lino',import.meta.url));
const declarationChecks=new Map();
const tokens=source=>tokenize(source,'Rust').tokens.filter(({kind,value})=>kind!=='punct'||!['{','}',','].includes(value))
  .map(({kind,value})=>({kind,value}));

export function replayPublicationEdits(record) {
  assert.equal(Buffer.byteLength(record.original),record.originalBytes);
  assert.equal(digest(record.original),record.originalSha256);
  let source=record.original;
  for(const {from,to,count} of record.edits){
    assert.equal(source.split(from).length-1,count,'one declared portability occurrence count');
    source=source.split(from).join(to);
  }
  return source;
}

/** Fresh bytes on every invocation; cache only immutable parsed source proofs. */
export function verifyPublication({checkNativeCst=false}={}) {
  const archive=readProjectData(archivePath);
  assert.equal(archive.schemaVersion,1);
  assert.equal(archive.baseline,'3780504667f4abe167a270263453e54be2b1cc43');
  assert.equal(digest(archive.originalPolicySource),'6c0e4ee1d490c6b155cfcc9910d7f144ac889f314f16bbe4729c0d2f57a812eb',
    'the complete original policy author remains authenticated');
  assert.equal(digest(readFileSync(new URL('./predecessor-receipt.lino',import.meta.url))),
    'f0b779d66ba81ea6470609575202a716d41d671b8e01f125cf1f2999789a53aa',
    'the original B94-to-B95 source-owner correction receipt remains exact');
  assert.deepEqual(outcomeWirePolicy,archive.policy,'the Links policy retains every original field and pin');
  const files=[];
  for(const record of archive.records){
    const source=readFileSync(new URL('../'+record.path,import.meta.url),'utf8');
    assert.equal(Buffer.byteLength(source),record.publishedBytes,record.path+' byte length');
    assert.equal(digest(source),record.publishedSha256,record.path+' published hash');
    const adapted=replayPublicationEdits(record);
    if(record.formatting){
      assert.equal(record.path,'probe/native-outcome-probe.rs');
      assert.deepEqual(tokens(source),tokens(adapted),'all non-formatting tokens retained after explicit harness edits');
      if(checkNativeCst&&!declarationChecks.has(record.publishedSha256)){
        const original=readRustSourceItems(Buffer.from(adapted));
        const formatted=readRustSourceItems(Buffer.from(source));
        const owners=parsed=>parsed.items.map(({kind,name})=>({kind,name}));
        assert.deepEqual(owners(formatted),owners(original),'complete native CST retains every ordered source owner');
        declarationChecks.set(record.publishedSha256,true);
      }
    }else assert.equal(source,adapted,record.path+' exact original-to-published transformation');
    files.push({path:record.path,bytes:record.publishedBytes,sha256:record.publishedSha256});
  }
  const originalSources=archive.archivalSources.map(({group,file,source,bytes,sha256})=>{
    assert.equal(Buffer.byteLength(source),bytes,'archival byte length '+file);
    assert.equal(digest(source),sha256,'archival complete source '+file);
    return {group,file,bytes,sha256};
  });
  return {schemaVersion:1,baseline:archive.baseline,files,originalSources,
    archive:{bytes:Buffer.byteLength(readFileSync(archivePath)),sha256:digest(readFileSync(archivePath))}};
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  assert.deepEqual(process.argv.slice(2),['--check']);
  const result=verifyPublication();
  console.log('Verified '+result.files.length+' published source transformations and '+result.originalSources.length+' frozen original inputs; no native execution.');
}
