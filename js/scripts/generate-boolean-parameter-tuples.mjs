// Preserve complete generated decisions and all source argument expressions.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {DecoratorSet} from '../src/decorators.js';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readBooleanParameterTupleStyle,readBooleanParameterTupleCallStyle} from './boolean-parameter-tuple-style.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';

const root=new URL('../../',import.meta.url),revision='25e6361f2e31e09f98f78b3c973f242bdf88e31c';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
export function generateBooleanParameterTuples({check=false}={}) {
  const models=[];
  for(const [name,styleFile,callerFile,qualified]of [
    ['require_self_translation_layout_envelope','parity/self-translation/self-translation-layout-rules-decorators.lino','rust/src/self_translation.rs','crate::translation::self_translation_layout_rules::require_self_translation_layout_envelope'],
    ['read_owned_outcome_native_number_kind','parity/self-translation/owned-outcome-number-kind-rules-decorators.lino','rust/tests/unit/grammar_outcome_number_kind_rules.rs','rules::read_owned_outcome_native_number_kind'],
  ]){
    const styles=DecoratorSet.fromLino(readFileSync(new URL(styleFile,root),'utf8'));
    const actions=styles.decorators.flatMap(row=>row.actions),selected=actions.filter(row=>row.from.includes('fn '+name+'(')&&row.to.includes('ml_boolean_parameters'));
    assert.equal(selected.length,1);const action=selected[0],packed=readBooleanParameterTupleStyle(action.from,name);
    assert.equal(packed.source,action.to);assert.equal(packed.groups.length,1);
    const {certificate,blob}=captureGitArtifactCertificate(revision,callerFile),original=blob.toString('utf8');
    const edited=readBooleanParameterTupleCallStyle(original,qualified,packed.groups);
    const current=execFileSync('rustfmt',['--edition','2024','--config','skip_children=true','--emit','stdout'],{input:edited.source,encoding:'utf8',maxBuffer:1024*1024,timeout:15000});
    const file=new URL(callerFile,root);if(check)assert.equal(readFileSync(file,'utf8'),current);else writeFileSync(file,current);
    models.push({name,styleFile,groups:packed.groups,originalDefinition:action.from,packedDefinition:action.to,
      caller:{file:callerFile,certificate,sourceText:original,...pin(original),current:pin(current),insertions:edited.edits},
      boundary:'Contiguous primitive operands retain their exact order and expressions; only a tuple carrier and local destructuring bind the same original variables. The entire decision body remains source generated.'});
  }
  const policy={schemaVersion:1,revision,models,
    boundary:'Explicit generated library ABI correspondence with exact predecessor source membership. No lint is disabled and no decision body is supplied by hand. Native compilation and behavior validation remain CI requirements.'};
  const file=new URL('parity/self-translation/boolean-parameter-tuples.lino',root);
  if(check)checkProjectData(file,policy);else writeProjectData(file,policy);
  return policy;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateBooleanParameterTuples({check:process.argv.includes('--check')});
  console.log('Recorded both complete Boolean tuple carriers and original ordered argument expressions.');
}
