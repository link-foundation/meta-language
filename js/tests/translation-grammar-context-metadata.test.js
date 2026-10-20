import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {generateGrammarContextSourceMetadata,checkGrammarContextMetadataPredecessor} from '../../experiments/grammar-context-source-metadata/generate.mjs';
import {restoreGrammarContextMetadataHistory} from '../scripts/grammar-context-metadata-history.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
const root=new URL('../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

test('current metadata retains complete certified predecessors and every original oracle fixture byte',()=>{
  const packet=generateGrammarContextSourceMetadata({check:true});
  assert.equal(packet.sources.length,32);
  for(const row of packet.sources){
    const actual=readFileSync(new URL(row.file,root),'utf8');assert.deepEqual(pin(actual),row.current);
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:packet.revision,path:row.file});
    assert.equal(restoreGrammarContextMetadataHistory(actual,row.file),row.sourceText);
    assert.equal(restoreGrammarContextMetadataHistory(actual+'\n# changed\n',row.file),actual+'\n# changed\n');
    assert.equal(restoreGrammarContextMetadataHistory(row.sourceText,row.file),row.sourceText);
  }
  const oracle=packet.sources.find(row=>row.file.endsWith('/projection-fixtures.lino'));assert.ok(oracle);
  assert.deepEqual(oracle.current,{bytes:oracle.bytes,sha256:oracle.sha256});
  assert.equal(readProjectData(new URL(oracle.file,root)).cases.length,14);
});

test('private source metadata changes only its certified complete producer fingerprint',()=>{
  const packet=readProjectData(new URL('parity/self-translation/grammar-context-source-metadata.lino',root));
  const row=packet.sources.find(row=>row.file.endsWith('/private-output-test-model-generation.lino'));assert.ok(row);
  const previous=readStructuredDataLino(row.sourceText),current=readProjectData(new URL(row.file,root));
  assert.deepEqual(checkGrammarContextMetadataPredecessor(row.file,current,previous),previous);
  for(const mutate of [value=>{value.compileTimeBindings++;},value=>{value.outputs[0].sha256='0'.repeat(64);},
    value=>{value.inputs.pop();},value=>{value.sections[0].sha256='0'.repeat(64);},
    value=>{value.inputs.find(item=>item.file.endsWith('/grammar_selection.rs')).sha256='0'.repeat(64);}]){
    const changed=structuredClone(current);mutate(changed);
    assert.throws(()=>checkGrammarContextMetadataPredecessor(row.file,changed,previous));
  }
});
