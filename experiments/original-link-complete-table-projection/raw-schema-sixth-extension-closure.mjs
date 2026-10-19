// Authenticate the complete fifth schema before admitting one new root table.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';
import {readProgramTypeNeutralDefaults} from '../../js/scripts/read-program-type-neutral-defaults.mjs';
import {compareOriginalLinkRawSchemaExtension} from './raw-schema-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES} from './raw-schema-source-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES,readOriginalLinkRawSchemaFifthContext,
  verifyOriginalLinkRawSchemaFifthExtensionClosure} from './raw-schema-fifth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_SIXTH_EXTENSIONS,ORIGINAL_LINK_RAW_SCHEMA_SIXTH_EXTENSION_SCOPE} from './raw-schema-sixth-extension-policy.mjs';

export const ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSION_RECEIPT='9c556b21e47479d9c918a8a50043a1a89f129196bfda03ff403ce04e48c703e5';
export const ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES=Object.freeze([
  'experiments/original-link-complete-table-projection/raw-schema-sixth-extension-policy.mjs',
  'experiments/original-link-complete-table-projection/raw-schema-sixth-extension-closure.mjs',
  'js/scripts/extend-typed-program-schema-sixth-stage.mjs',
  'js/scripts/read-program-type-neutral-defaults.mjs',
]);
export const ORIGINAL_LINK_RAW_SCHEMA_SIXTH_COMMAND='node js/scripts/extend-typed-program-schema-sixth-stage.mjs';
const root=new URL('../../',import.meta.url);
const sha256=bytes=>createHash('sha256').update(bytes).digest('hex');

export function readOriginalLinkRawSchemaSixthContext(read=readFileSync){
  const fifthBytes=read(new URL('./data/raw-schema-fifth-extension-closure.lino',import.meta.url));
  return {...readOriginalLinkRawSchemaFifthContext(read),fifthBytes,
    fifth:readStructuredDataLino(fifthBytes.toString('utf8')),
    fifthSources:Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES.map(file=>[file,read(new URL(file,root))]))};
}

export function verifyOriginalLinkRawSchemaSixthExtensionClosure(input,currentBytes,sixth,authoredSources){
  assert.deepEqual(Object.keys(sixth),['version','predecessorReceiptSha256','previous','current','consumedSchemaProperties',
    'appendedSchemaProperties','sourceGuard','authoring','scope']);
  assert.equal(sixth.version,1);assert.equal(sixth.predecessorReceiptSha256,ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSION_RECEIPT);
  assert.ok(Buffer.isBuffer(input.fifthBytes));assert.equal(sha256(input.fifthBytes),ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSION_RECEIPT);
  assert.deepEqual(readStructuredDataLino(input.fifthBytes.toString('utf8')),input.fifth);
  assert.deepEqual(Object.keys(sixth.previous),['file','bytes','sha256','source']);
  assert.equal(typeof sixth.previous.source,'string');assert.ok(sixth.previous.source.isWellFormed());
  const previousBytes=Buffer.from(sixth.previous.source);
  assert.equal(sixth.previous.file,input.fifth.current.file);assert.equal(sixth.previous.bytes,input.fifth.current.bytes);
  assert.equal(sixth.previous.bytes,previousBytes.length);assert.equal(sixth.previous.sha256,input.fifth.current.sha256);
  assert.equal(sixth.previous.sha256,sha256(previousBytes));
  const previous=verifyOriginalLinkRawSchemaFifthExtensionClosure(input,previousBytes,input.fifth,input.fifthSources);
  assert.deepEqual(Object.keys(sixth.current),['file','bytes','sha256']);assert.equal(sixth.current.file,input.fifth.current.file);
  assert.ok(Buffer.isBuffer(currentBytes));assert.equal(sixth.current.bytes,currentBytes.length);assert.equal(sixth.current.sha256,sha256(currentBytes));
  assert.deepEqual(sixth.consumedSchemaProperties,[...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES]);
  assert.deepEqual(sixth.appendedSchemaProperties,ORIGINAL_LINK_RAW_SCHEMA_SIXTH_EXTENSIONS);
  const before=readStructuredDataLino(sixth.previous.source),after=readStructuredDataLino(currentBytes.toString('utf8'));
  assert.equal(Object.hasOwn(before,'neutralTypeFieldDefaults'),false);
  assert.deepEqual(Object.keys(after),[...Object.keys(before),'neutralTypeFieldDefaults']);
  assert.equal(JSON.stringify(after.neutralTypeFieldDefaults),JSON.stringify(ORIGINAL_LINK_RAW_SCHEMA_SIXTH_EXTENSIONS.neutralTypeFieldDefaults));
  const inverse=readStructuredDataLino(currentBytes.toString('utf8'));assert.equal(delete inverse.neutralTypeFieldDefaults,true);
  compareOriginalLinkRawSchemaExtension(before,inverse,{});assert.equal(JSON.stringify(inverse),JSON.stringify(before),
    'the finite new-root inverse retains every old value, alias, assertion, model and field order');
  const guarded=readProgramTypeNeutralDefaults();assert.deepEqual(sixth.sourceGuard,guarded);
  assert.deepEqual(guarded.defaults,after.neutralTypeFieldDefaults);
  assert.deepEqual(Object.keys(sixth.authoring),['command','sourceFiles']);assert.equal(sixth.authoring.command,ORIGINAL_LINK_RAW_SCHEMA_SIXTH_COMMAND);
  assert.deepEqual(Object.keys(authoredSources),[...ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES]);
  assert.deepEqual(sixth.authoring.sourceFiles.map(row=>row.file),[...ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES]);
  for(const row of sixth.authoring.sourceFiles){
    assert.deepEqual(Object.keys(row),['file','bytes','sha256']);const bytes=authoredSources[row.file];assert.ok(Buffer.isBuffer(bytes));
    assert.equal(row.bytes,bytes.length);assert.equal(row.sha256,sha256(bytes));
  }
  assert.equal(sixth.scope,ORIGINAL_LINK_RAW_SCHEMA_SIXTH_EXTENSION_SCOPE);
  return {...previous,sha256:sixth.current.sha256,sixthPredecessorSha256:sixth.previous.sha256,
    fifthExtensionReceiptSha256:sixth.predecessorReceiptSha256,appendedSixthSchemaProperties:sixth.appendedSchemaProperties};
}
