import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {readStringOrderSchemaPredecessor,verifyStringOrderSchemaSuccessor} from '../scripts/check-typed-program-schema-history.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES,readOriginalLinkRawSchemaSixthContext,
  verifyOriginalLinkRawSchemaSixthExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-sixth-extension-closure.mjs';

const root=new URL('../../',import.meta.url);
const schemaFile='js/src/data/typed-program-links-schema.lino';
const inputs=()=>({input:readOriginalLinkRawSchemaSixthContext(),
  current:readFileSync(new URL(schemaFile,root)),
  sixth:readProjectData(new URL('experiments/original-link-complete-table-projection/data/raw-schema-sixth-extension-closure.lino',root)),
  sources:Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES.map(file=>[file,readFileSync(new URL(file,root))]))});

test('the current string order successor preserves every original sixth source guard and immutable schema byte',()=>{
  const {input,current,sixth,sources}=inputs();
  const previous=readStringOrderSchemaPredecessor();
  assert.deepEqual(previous,execFileSync('git',['show','2b9fd0df45757bf70a69deea6ab97d969cf6a25d:'+schemaFile],{cwd:root,maxBuffer:1024*1024}));
  assert.deepEqual(current,readFileSync(new URL('rust/src/data/typed-program-links-schema.lino',root)));
  assert.throws(()=>verifyOriginalLinkRawSchemaSixthExtensionClosure(input,current,sixth,sources),assert.AssertionError);
  const original=verifyOriginalLinkRawSchemaSixthExtensionClosure(input,previous,sixth,sources);
  const actual=verifyStringOrderSchemaSuccessor(input,current,sixth,sources);
  const {sha256,stringOrderPredecessorSha256,stringOrderSourceRevision,...retained}=actual;
  const {sha256:previousSha256,...originalFields}=original;
  assert.deepEqual(retained,originalFields);
  assert.equal(stringOrderPredecessorSha256,previousSha256);
  assert.equal(stringOrderSourceRevision,'2b9fd0df45757bf70a69deea6ab97d969cf6a25d');
  assert.notEqual(sha256,previousSha256);
});

test('current schema admission refuses altered operands model ordering source owners and original receipts',()=>{
  const {input,current,sixth,sources}=inputs();
  const schema=readProjectData(new URL(schemaFile,root));
  for(const mutate of [
    value=>{value.stringOrderModel.codeUnitWidth=8;},
    value=>{value.stringOrderModel.relations.reverse();},
    value=>{value.expressionOperations.stringTest.pop();},
    value=>{value.fields.k='counterfeit construct kind';},
    value=>{value.unexpected=true;},
  ]){
    const changed=structuredClone(schema);mutate(changed);
    assert.throws(()=>verifyStringOrderSchemaSuccessor(input,Buffer.from(serializeProjectData(changed,schemaFile)),sixth,sources),assert.AssertionError);
  }
  const receipt=structuredClone(sixth);receipt.sourceGuard.fields[0].field='counterfeit';
  assert.throws(()=>verifyStringOrderSchemaSuccessor(input,current,receipt,sources),assert.AssertionError);
  const first=ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES[0];
  assert.throws(()=>verifyStringOrderSchemaSuccessor(input,current,sixth,
    {...sources,[first]:Buffer.concat([sources[first],Buffer.from('\n')])}),assert.AssertionError);
});
