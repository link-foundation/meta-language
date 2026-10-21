// Complete independently committed comparator inputs; normal verification is offline.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {readProjectData,checkProjectData,writeProjectData} from './project-data.mjs';

export const GRAMMAR_COMPARISON_PREDECESSOR_REVISION='c33d6e0af666d3b679b5ca420917f5cafb347613';
export const GRAMMAR_COMPARISON_LEGACY_REVISION='ae76b70abaa60d2ec051088da08dcd55fb3f73f4';
export const GRAMMAR_COMPARISON_ASSERTION_VERSIONS=Object.freeze({Node:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0'});
export const GRAMMAR_COMPARISON_PREDECESSOR_SOURCES=Object.freeze([
  {path:'js/src/grammar-runtime/executor.js',bytes:242017,sha256:'2e28ed53ec7ad9f66cbad3941c3c667f20bbd6ade6ad3dba1950f9e7ad81f0c1'},
  {path:'js/src/grammar-runtime/tree-equality.js',bytes:5756,sha256:'0752e5c0d0663d44306f8c80795fd19669e90d3260b94c515240f301ce972d38'},
  {path:'js/src/grammar-runtime.js',bytes:11882,sha256:'680498a6118102d91f039d911a50197dad41778b9ee543be76a65a36e45223c6'},
  {path:'js/src/translation/grammar-selection-rules.js',bytes:2841,sha256:'1905a8a74274b93147f5aae0dc10f8efd17082251a789142245a0d3c78fa49f3'},
]);
const file=new URL('../../parity/fixtures/grammar-lazy-comparison-baseline.lino',import.meta.url);
const boundary='Exact source epochs, without interpolation: ae76 retains its original comparator and every historical input; c33 is the complete actual pre-lazy-change executor, public driver, centralized equality implementation and equality decision source. Git membership and whole-byte pins authenticate inputs, not semantic equivalence or execution. The test executes each original implementation with every retained mutation and compares the new implementation to the actual c33 predecessor. No partial source overlay or native execution is supplied.';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const keys=(value,names)=>assert.deepEqual(Object.keys(value).sort(),[...names].sort());

export function verifyGrammarComparisonPredecessor(value){
  keys(value,['schemaVersion','model','revision','legacyRevision','assertionVersions','modules','boundary']);
  assert.equal(value.schemaVersion,1);assert.equal(value.model,'complete grammar comparator predecessor sources 1');
  assert.equal(value.revision,GRAMMAR_COMPARISON_PREDECESSOR_REVISION);
  assert.equal(value.legacyRevision,GRAMMAR_COMPARISON_LEGACY_REVISION);
  assert.deepEqual(value.assertionVersions,GRAMMAR_COMPARISON_ASSERTION_VERSIONS);assert.equal(value.boundary,boundary);
  assert.deepEqual(value.modules.map(row=>row.path),GRAMMAR_COMPARISON_PREDECESSOR_SOURCES.map(row=>row.path));
  for(const [index,row]of value.modules.entries()){
    keys(row,['path','source','bytes','sha256','certificate']);assert.equal(typeof row.source,'string');
    const expected=GRAMMAR_COMPARISON_PREDECESSOR_SOURCES[index];
    assert.deepEqual({path:row.path,bytes:row.bytes,sha256:row.sha256},expected);
    assert.deepEqual(pin(row.source),{bytes:expected.bytes,sha256:expected.sha256});
    const bytes=Buffer.from(row.source);assert.equal(bytes.toString('utf8'),row.source);
    verifyGitArtifactCertificate(row.certificate,bytes,{revision:value.revision,path:row.path,limits:{maxBlobBytes:512*1024}});
  }
  return value;
}

export function readGrammarComparisonPredecessor(){return verifyGrammarComparisonPredecessor(readProjectData(file));}

export function generateGrammarComparisonPredecessor({check=false}={}){
  if(check){const value=readGrammarComparisonPredecessor();checkProjectData(file,value);return value;}
  const modules=GRAMMAR_COMPARISON_PREDECESSOR_SOURCES.map(expected=>{
    const {blob,certificate}=captureGitArtifactCertificate(GRAMMAR_COMPARISON_PREDECESSOR_REVISION,expected.path);
    const source=blob.toString('utf8');assert.ok(Buffer.from(source).equals(blob));
    assert.deepEqual(pin(blob),{bytes:expected.bytes,sha256:expected.sha256});
    return{...expected,source,certificate};
  });
  const value=verifyGrammarComparisonPredecessor({schemaVersion:1,model:'complete grammar comparator predecessor sources 1',
    revision:GRAMMAR_COMPARISON_PREDECESSOR_REVISION,legacyRevision:GRAMMAR_COMPARISON_LEGACY_REVISION,
    assertionVersions:{...GRAMMAR_COMPARISON_ASSERTION_VERSIONS},modules,boundary});
  writeProjectData(file,value);return value;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateGrammarComparisonPredecessor({check:process.argv.includes('--check')});
  console.log('Verified complete c33 comparator source epoch; capture uses Git once, --check is offline.');
}
