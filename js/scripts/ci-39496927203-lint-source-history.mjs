// Exact reviewed lint successor views; current execution keeps actual source bytes.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const archive='parity/self-translation/ci-39496927203-lint-feedback.lino';
const archiveSha256='df9cc4007df0fb0219110884ac034f47132af14c281804980d953785079469e5';
const revision='24d7cf48778c7e91f15a25d563c225912d6caad0';
const paths=['rust/src/grammar/feature_runtime/grammar_selection.rs',
  'parity/self-translation/typescript-erasure-rules-decorators.lino',
  'rust/src/self_translation/typescript_erasure_rules.rs'];
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let pairs;
function sourcePairs(){
  if(pairs)return pairs;
  const bytes=readFileSync(new URL('../../'+archive,import.meta.url));
  assert.equal(pin(bytes).sha256,archiveSha256,'the complete reviewed lint source recipe');
  const data=readBootstrapStructuredDataLino(bytes.toString('utf8'));
  assert.equal(data.schemaVersion,1);assert.equal(data.revision,revision);
  assert.deepEqual(data.sources.map(row=>row.file),paths);
  const result=new Map();
  for(const row of data.sources){
    const original=Buffer.from(row.source);verifyGitArtifactCertificate(row.certificate,original,{revision,path:row.file});
    assert.deepEqual(pin(original),{bytes:row.bytes,sha256:row.sha256});
    let current=row.source;
    for(const edit of row.edits){
      assert.ok(Number.isSafeInteger(edit.start)&&edit.start>=0&&edit.start<=current.length);
      assert.equal(current.slice(edit.start,edit.start+edit.removed.length),edit.removed);
      current=current.slice(0,edit.start)+edit.inserted+current.slice(edit.start+edit.removed.length);
    }
    assert.equal(pin(current).sha256,row.currentSha256);
    let inverse=current;
    for(const edit of row.edits.toReversed()){
      assert.equal(inverse.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
      inverse=inverse.slice(0,edit.start)+edit.removed+inverse.slice(edit.start+edit.inserted.length);
    }
    assert.equal(inverse,row.source,'every complete predecessor byte remains recoverable');
    result.set(row.file,{file:row.file,predecessor:row.source,current,revision,
      recipe:{path:archive,sha256:archiveSha256},predecessorPin:pin(original),currentPin:pin(current)});
  }
  pairs=result;return pairs;
}

/** Return an independent copy of a complete certified predecessor/successor pair. */
export function readCiLintSourcePair(file){
  assert.equal(typeof file,'string');assert.ok(paths.includes(file),'one registered lint source owner is required');
  return structuredClone(sourcePairs().get(file));
}

/** Only exact current bytes get a historical view; unknown edits stay visible. */
export function restoreCiLintSource(source,file){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');
  if(!paths.includes(file))return source;
  const pair=sourcePairs().get(file);
  return source===pair.current?pair.predecessor:source;
}

/** A current metadata receipt preserves both actual source identity and its inverse. */
export function readCiLintSourceReceipt(source,file){
  const pair=readCiLintSourcePair(file);assert.equal(source,pair.current,'an exact current lint successor is required');
  return{schemaVersion:1,profile:'complete CI lint source successor 1',file,revision,
    recipe:pair.recipe,predecessor:pair.predecessorPin,current:pair.currentPin,inverseVerified:true,
    boundary:'Exact complete source conservation only; no applicability metadata, semantic equivalence, compiler or native execution evidence.'};
}
