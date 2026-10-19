import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readStructuredDataLino,structuredDataToLino} from '../../js/src/structured-data-links.js';
import {extendTypedProgramSchemaFifthStage} from '../../js/scripts/extend-typed-program-schema-fifth-stage.mjs';
import {checkTypedProgramSchemaFifthArchive} from '../../js/scripts/extend-typed-program-schema-sixth-stage.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES,readOriginalLinkRawSchemaSixthContext,
  verifyOriginalLinkRawSchemaSixthExtensionClosure} from './raw-schema-sixth-extension-closure.mjs';
const root=new URL('../../',import.meta.url),input=readOriginalLinkRawSchemaSixthContext();
const sixth=readStructuredDataLino(readFileSync(new URL('./data/raw-schema-sixth-extension-closure.lino',import.meta.url),'utf8'));
const current=readFileSync(new URL('../../js/src/data/typed-program-links-schema.lino',import.meta.url));
const schema=readStructuredDataLino(current.toString('utf8'));
const sources=Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES.map(file=>[file,readFileSync(new URL(file,root))]));
const repin=(changed,receipt)=>{const bytes=Buffer.from(structuredDataToLino(changed));receipt.current.bytes=bytes.length;receipt.current.sha256=createHash('sha256').update(bytes).digest('hex');return bytes;};

test('the sixth closure appends one source-derived false-default table and retains every predecessor byte and model',()=>{
  const result=verifyOriginalLinkRawSchemaSixthExtensionClosure(input,current,sixth,sources);assert.equal(result.sha256,sixth.current.sha256);
  const before=readStructuredDataLino(sixth.previous.source);
  assert.deepEqual(Object.keys(schema),[...Object.keys(before),'neutralTypeFieldDefaults']);
  assert.deepEqual(schema.neutralTypeFieldDefaults,{array:{reference:'false'},record:{open:'false',reference:'false'},map:{reference:'false'},set:{reference:'false'}});
  const inverse=structuredClone(schema);delete inverse.neutralTypeFieldDefaults;
  assert.equal(JSON.stringify(inverse),JSON.stringify(before));assert.equal(sixth.sourceGuard.fields.length,5);
});

test('each default remains a boolean-false omission of its exact existing type field despite coherent fresh hashes',()=>{
  for(const [kind,fields]of Object.entries(schema.neutralTypeFieldDefaults))for(const field of Object.keys(fields)){
    for(const action of ['true-default','null-default','remove-field','unknown-field','wrong-kind']){
      const changed=structuredClone(schema),receipt=structuredClone(sixth);
      if(action==='remove-field')delete changed.neutralTypeFieldDefaults[kind][field];
      else if(action==='unknown-field')changed.neutralTypeFieldDefaults[kind].invented='false';
      else if(action==='wrong-kind')changed.neutralTypeFieldDefaults.float={[field]:'false'};
      else changed.neutralTypeFieldDefaults[kind][field]=action==='true-default'?'true':'null';
      receipt.appendedSchemaProperties.neutralTypeFieldDefaults=structuredClone(changed.neutralTypeFieldDefaults);
      assert.throws(()=>verifyOriginalLinkRawSchemaSixthExtensionClosure(input,repin(changed,receipt),receipt,sources),kind+'/'+field+'/'+action);
    }
  }
});

test('the finite root inverse rejects reordered roots and defaults and any change to old schema behavior',()=>{
  for(const action of ['old-neutral','old-projection','version','root','language','root-order','kind-order','record-field-order']){
    let changed=structuredClone(schema);const receipt=structuredClone(sixth);
    if(action==='old-neutral')changed.neutralFieldDefaults.program.referenceSemantics='true';
    else if(action==='old-projection')changed.expressionProjections.set.fields.reverse();
    else if(action==='version')changed.schemaVersion++;
    else if(action==='root')changed.root+=' changed';
    else if(action==='language')changed.language+=' changed';
    else if(action==='root-order'){const {neutralTypeFieldDefaults,...old}=changed;changed={neutralTypeFieldDefaults,...old};}
    else if(action==='kind-order')changed.neutralTypeFieldDefaults=Object.fromEntries(Object.entries(changed.neutralTypeFieldDefaults).reverse());
    else changed.neutralTypeFieldDefaults.record={reference:'false',open:'false'};
    assert.throws(()=>verifyOriginalLinkRawSchemaSixthExtensionClosure(input,repin(changed,receipt),receipt,sources),action);
  }
});

test('all predecessor source bytes and the independent actual Type-field source closure stay authenticated',()=>{
  for(const action of ['predecessor-receipt','previous-source','source-guard','scope','author-order','author-hash']){
    const receipt=structuredClone(sixth);
    if(action==='predecessor-receipt')receipt.predecessorReceiptSha256='0'.repeat(64);
    else if(action==='previous-source')receipt.previous.source+='\n';
    else if(action==='source-guard')receipt.sourceGuard.fields[0].field='invented';
    else if(action==='scope')receipt.scope+=' changed';
    else if(action==='author-order')receipt.authoring.sourceFiles.reverse();
    else receipt.authoring.sourceFiles[0].sha256='0'.repeat(64);
    assert.throws(()=>verifyOriginalLinkRawSchemaSixthExtensionClosure(input,current,receipt,sources),action);
  }
  const changed={...sources};const first=ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES[0];changed[first]=Buffer.concat([sources[first],Buffer.from('\n')]);
  assert.throws(()=>verifyOriginalLinkRawSchemaSixthExtensionClosure(input,current,sixth,changed));
  assert.throws(()=>verifyOriginalLinkRawSchemaSixthExtensionClosure({...input,fifthBytes:Buffer.concat([input.fifthBytes,Buffer.from('\n')])},current,sixth,sources));
});


test('the immutable fifth CLI refuses live sixth bytes while its authenticated exact archive remains checkable',()=>{
  assert.throws(()=>extendTypedProgramSchemaFifthStage({check:true}),assert.AssertionError);
  const result=checkTypedProgramSchemaFifthArchive();
  assert.equal(result.schemaSha256,input.fifth.current.sha256);
  assert.equal(result.predecessorSchemaSha256,input.fifth.previous.sha256);
  assert.equal(result.predecessorReceiptSha256,input.fifth.predecessorReceiptSha256);
  assert.equal(result.appendedProjectionKeys,2);
  assert.equal(result.preservedProducerBindings,47);
  assert.equal(result.sourceClosureOnly,true);assert.equal(result.check,true);
});
