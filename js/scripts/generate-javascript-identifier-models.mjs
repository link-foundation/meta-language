// Linked naming selection; all rendering semantics reuse the existing shared
// JavaScript decision and its normally translated Rust implementation.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData,checkProjectData} from './project-data.mjs';
import {renderJavaScriptScopedBindingIdentifier} from '../src/translation/javascript-scoped-binding-emission-rules.js';
const root=new URL('../../',import.meta.url),sourceFile='js/src/translation/data/javascript-scoped-binding-emission.lino';
export function generateJavaScriptIdentifierModels({check=false}={}){
  const previous=readFileSync(new URL(sourceFile,root)),source=readProjectData(new URL(sourceFile,root));
  assert.equal(typeof source.model,'string');assert.equal(source.prefix,'binding_');
  for(const name of['ml_array_test1','ml_binding1','binding_ml_array_test1'])
    assert.equal(renderJavaScriptScopedBindingIdentifier(name,false),source.prefix+name);
  const policy={schemaVersion:1,model:'ordinary JavaScript binding identifiers 1',reservedPrefix:'ml_',
    renderer:{operation:'renderJavaScriptScopedBindingIdentifier',sourcePolicy:{file:sourceFile,model:source.model,prefix:source.prefix},preserve:false},
    ownership:'Existing EmitState namespace claims and Scope fresh-name allocation retain actual declarations, binders and bound references. This model changes target identifier spellings only; source namespace admission and checked-program semantics remain unchanged.',
    boundary:'Fixed runtime prelude names and external provider names are outside this owned-identifier projection. This model does not promise that arbitrary emitted JavaScript modules reparse. The original narrow scoped-binding model is unchanged; composition with that model requires a separate proof.',
    assertionVersions:{Node:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0'}};
  for(const file of['js/src/translation/data/javascript-identifier-models.lino','rust/src/data/javascript-identifier-models.lino']){
    const url=new URL(file,root);if(check)checkProjectData(url,policy);else writeProjectData(url,policy);
  }
  assert.ok(readFileSync(new URL(sourceFile,root)).equals(previous),'the complete original narrow policy stays byte exact');
  return policy;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const policy=generateJavaScriptIdentifierModels({check:process.argv.includes('--check')});console.log('Generated '+policy.model+' from the unchanged shared renderer.');
}
