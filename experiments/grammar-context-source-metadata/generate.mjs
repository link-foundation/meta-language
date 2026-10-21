// Current source facts retain their complete independently certified predecessors.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';
import {readProjectData,serializeProjectData,writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';
import {restoreGlobalGrammarContextSource} from '../global-grammar-context-reuse/current-source.mjs';

const root=new URL('../../',import.meta.url),revision='abde6ee604b914a895f23525946fab2234109e86';
const output='parity/self-translation/grammar-context-source-metadata.lino';
const helper='js/scripts/grammar-context-metadata-history.mjs';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const sources=['js/tests/support/windows-linker-lookup-history.js',
  'js/scripts/native-compatibility-source-history.mjs','js/scripts/generate-translation-workflow-history.mjs',
  'js/tests/support/translation-workflow-history.js','js/scripts/array-relative-element-history.mjs',
  'js/tests/support/platform-test-deadline-history.js','js/scripts/ci-repair-source-history.mjs',
  'js/scripts/native-graph-source-history.mjs',
  'experiments/grammar-outcome-network/native-transport/inverse/generate-native-graph-source-history.mjs',
  'js/tests/translation-concrete-program-native-inputs.test.js',
  'experiments/grammar-outcome-network/native-transport/inverse/generate-fixtures.mjs',
  'experiments/grammar-outcome-network/native-transport/inverse/plan.mjs',
  'experiments/grammar-outcome-network/native-transport/inverse/generate-original-number-observations.mjs',
  'js/tests/translation-owned-original-numbers.test.js',
  'experiments/grammar-outcome-network/native-transport/inverse/generate-native-compatibility-source-history.mjs',
  'js/tests/translation-native-compatibility-source-history.test.js',
  'js/scripts/continuation-source-history.mjs',
  'experiments/rust-text-certificate-adapter/reviewed-providers.mjs',
  'experiments/rust-text-certificate-adapter/data/provider-revisions.lino',
  'js/tests/translation-reviewed-overlay-history.test.js',
  'js/tests/issue-195-evidence-ci-node-runtime.test.js',
  'js/scripts/scheduling-source-history.mjs',
  'experiments/rust-text-certificate-adapter/data/overlays-current.lino',
  'experiments/rust-raw-binding-certificate-adapter/data/overlays-current.lino',
  'experiments/rust-text-certificate-adapter/overlay-builder.test.mjs',
  'js/scripts/generate-current-ci-repair-source-history.mjs',
  'experiments/grammar-outcome-network/native-transport/inverse/generate-current-ci-repair-history.mjs',
  'js/scripts/current-ci-repair-source-history.mjs',
  'experiments/original-link-complete-table-projection/raw-native-observations.mjs',
  'experiments/original-link-complete-table-projection/generate-successor-source-phases.mjs',
  'experiments/grammar-outcome-network/native-transport/inverse/projection-fixtures.lino'];

/** Validate the complete current metadata and the exact unchanged original payload. */
export function checkGrammarContextMetadataPredecessor(file,current,previous) {
  assert.ok(file==='experiments/grammar-outcome-network/native-transport/inverse/projection-fixtures.lino'
    ||file==='parity/self-translation/private-output-test-model-generation.lino','one explicitly recorded metadata profile is required');
  const inverse=structuredClone(current);
  if(file.endsWith('/projection-fixtures.lino')){
    assert.deepEqual(current.cases,previous.cases,'all fourteen complete parser and boundary expectations remain exact');
    assert.equal(current.cases.length,14);
    for(const [index,row]of inverse.inputs.entries()){
      assert.equal(row.path,previous.inputs[index].path);
      if(row.path!=='js/src/grammar-runtime/grammar-selection.js')continue;
      const source=readFileSync(new URL(row.path,root),'utf8');assert.deepEqual(pin(source),{bytes:row.bytes,sha256:row.sha256});
      const restored=restoreGlobalGrammarContextSource(row.path,source);assert.equal(restored.changed,true);
      assert.deepEqual(pin(restored.predecessor),{bytes:previous.inputs[index].bytes,sha256:previous.inputs[index].sha256});
      Object.assign(row,previous.inputs[index]);
    }
  }else{
    for(const [index,row]of inverse.inputs.entries()){
      assert.equal(row.file,previous.inputs[index].file);
      if(row.file!=='rust/src/grammar/feature_runtime/grammar_selection.rs')continue;
      const source=readFileSync(new URL(row.file,root),'utf8');assert.deepEqual(pin(source),{bytes:row.bytes,sha256:row.sha256});
      const restored=restoreGlobalGrammarContextSource(row.file,source);assert.equal(restored.changed,true);
      assert.deepEqual(pin(restored.predecessor),{bytes:previous.inputs[index].bytes,sha256:previous.inputs[index].sha256});
      Object.assign(row,previous.inputs[index]);
    }
  }
  assert.deepEqual(inverse,previous,'all other complete source facts, selected definitions, outputs, controls and semantics remain exact');
  return inverse;
}

/** Regeneration is performed by the original authors, each in its own bounded
 * process. This author only proves the full metadata inverse and source history. */
export function generateGrammarContextSourceMetadata({check=false}={}) {
  const model=readProjectData(new URL('parity/self-translation/private-output-test-model.lino',root));
  const artifacts=[model.receipt];
  const rows=[...sources,...artifacts].map(file=>{
    const{blob,certificate}=captureGitArtifactCertificate(revision,file),actual=readFileSync(new URL(file,root));
    verifyGitArtifactCertificate(certificate,blob,{revision,path:file});
    assert.deepEqual(Buffer.from(blob.toString()),blob);assert.deepEqual(Buffer.from(actual.toString()),actual);
    if(artifacts.includes(file)){
      const previous=readStructuredDataLino(blob.toString()),current=readProjectData(new URL(file,root));
      const inverse=checkGrammarContextMetadataPredecessor(file,current,previous);
      assert.equal(serializeProjectData(inverse,file),blob.toString(),'every complete predecessor Links byte survives');
    }
    let start=0,end=actual.length,oldEnd=blob.length;
    while(start<end&&start<oldEnd&&actual[start]===blob[start])start++;
    while(start>0&&(actual[start]&192)===128)start--;
    while(end>start&&oldEnd>start&&actual[end-1]===blob[oldEnd-1]){end--;oldEnd--;}
    while(end<actual.length&&((actual[end]&192)===128||(blob[oldEnd]&192)===128)){end++;oldEnd++;}
    const removed=blob.subarray(start,oldEnd),inserted=actual.subarray(start,end);
    assert.deepEqual(Buffer.from(removed.toString()),removed);assert.deepEqual(Buffer.from(inserted.toString()),inserted);
    assert.deepEqual(Buffer.concat([actual.subarray(0,start),removed,actual.subarray(end)]),blob);
    return{file,commit:revision,...pin(blob),sourceText:blob.toString(),certificate,current:pin(actual),
      edit:{offsetUnit:'UTF-8 bytes',start,end,removed:removed.toString(),inserted:inserted.toString()}};
  });
  const packet={schemaVersion:1,revision,sources:rows,
    boundary:'Current metadata records actual current producers. Every complete predecessor retains independent Git membership and an exact byte inverse. Original parser expectations, selected native definitions, constructor bodies, numeric validation, outputs and all refusal assertions remain required. Source conservation supplies no native execution or semantic equivalence evidence.'};
  const target=new URL(output,root);if(check)checkProjectData(target,packet);else writeProjectData(target,packet);
  const path=new URL(helper,root),source=readFileSync(path,'utf8');
  const expected=source.replace(/const archiveSha256='(?:PENDING|[a-f0-9]{64})';/u,"const archiveSha256='"+pin(readFileSync(target)).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(path,expected);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const packet=generateGrammarContextSourceMetadata({check:process.argv.includes('--check')});
  console.log('Retained '+packet.sources.length+' complete original sources and metadata with unchanged expected behavior.');
}
