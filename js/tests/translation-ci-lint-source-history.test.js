import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateCiLintRepairs} from '../scripts/generate-ci-39496927203-lint-repairs.mjs';
import {generateCiLintDependencyHosts} from '../scripts/generate-ci-39496927203-lint-dependency-hosts.mjs';
import {readCiLintSourcePair,restoreCiLintSource,readCiLintSourceReceipt} from '../scripts/ci-39496927203-lint-source-history.mjs';
import {generateTypeScriptReceiverParameters} from '../scripts/generate-typescript-receiver-parameters.mjs';
import {readGlobalGrammarContextSourcePair,restoreGlobalGrammarContextSource,checkGlobalGrammarContextSourceReceipt} from '../../experiments/global-grammar-context-reuse/current-source.mjs';
const root=new URL('../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const feedbackFile='parity/self-translation/ci-39496927203-lint-feedback.lino';
const feedback=readProjectData(new URL(feedbackFile,root));

test('the four retained CI diagnostics keep separate complete-stream and bounded-excerpt identities',()=>{
  generateCiLintRepairs({check:true});generateCiLintDependencyHosts({check:true});
  assert.equal(feedback.feedback.wholeRawStream.bytes,201049);
  assert.equal(feedback.feedback.wholeRawStream.sha256,'d4ce865bd4f861c8db94835e2dcc92002f41c419c3c68fc16568ba659d19d756');
  assert.equal(feedback.feedback.wholeRawStream.retainedOnDisk,false);
  assert.deepEqual(pin(feedback.feedback.excerpt.text),{bytes:12767,sha256:'5d7b56a856e89a9da9d5788ff9d8784d1b9865de81878d4424d88a3d8927889a'});
  assert.equal(feedback.corrections.length,4);
  assert.equal(feedback.corrections.filter(row=>row.lint==='clippy::missing_const_for_fn').length,3);
  assert.equal(feedback.corrections.filter(row=>row.lint==='clippy::single_match_else').length,1);
  assert.equal(Object.hasOwn(feedback,'diagnostics'),false,'rendered suggestions are not fabricated machine-applicable JSON');
});

test('exact lint source successors invert every complete predecessor while unknown bytes retain no authority',()=>{
  for(const row of feedback.sources){
    const current=readFileSync(new URL(row.file,root),'utf8');
    const pair=readCiLintSourcePair(row.file);
    assert.equal(pair.current,current);assert.equal(pair.predecessor,row.source);
    assert.equal(restoreCiLintSource(current,row.file),row.source);
    assert.equal(restoreCiLintSource(row.source,row.file),row.source);
    const changed=current+'\n// altered source\n';
    assert.equal(restoreCiLintSource(changed,row.file),changed);
    assert.throws(()=>readCiLintSourceReceipt(changed,row.file));
    const receipt=readCiLintSourceReceipt(current,row.file);
    assert.deepEqual(receipt.current,pin(current));assert.deepEqual(receipt.predecessor,pin(row.source));
    pair.current+='changed';assert.equal(readCiLintSourcePair(row.file).current,current);
  }
  assert.equal(restoreCiLintSource('unrelated','js/src/unregistered.js'),'unrelated');
  assert.throws(()=>readCiLintSourcePair('js/src/unregistered.js'));
});

test('all eight original receiver hosts still replay while the additive const decorator remains current',()=>{
  const archiveFile='parity/self-translation/typescript-receiver-parameter-hosts.lino';
  const bytes=readFileSync(new URL(archiveFile,root));
  assert.equal(pin(bytes).sha256,'cfac585600d50f9ab4caf59ca15aba1916aea8509f6486f8e5cc11d71f0f28a1');
  const original=readProjectData(new URL(archiveFile,root));assert.equal(original.hosts.length,8);
  const before=original.hosts.map(row=>readFileSync(new URL(row.path,root)));
  generateTypeScriptReceiverParameters({check:true});
  for(const [index,row]of original.hosts.entries())assert.deepEqual(readFileSync(new URL(row.path,root)),before[index]);
  assert.deepEqual(readFileSync(new URL(archiveFile,root)),bytes,'the entire original host archive remains byte exact');
  const style=feedback.sources.find(row=>row.file.endsWith('typescript-erasure-rules-decorators.lino'));
  assert.ok(readFileSync(new URL(style.file,root),'utf8').startsWith(style.source),'every old decorator byte is retained');
});

test('grammar context receipts preserve actual current source pins through the second certified source edge',()=>{
  const file='rust/src/grammar/feature_runtime/grammar_selection.rs';
  const actual=readFileSync(new URL(file,root),'utf8'),pair=readGlobalGrammarContextSourcePair(file);
  const current=restoreGlobalGrammarContextSource(file,actual);
  assert.equal(current.current,actual);assert.equal(current.predecessor,pair.predecessor.source);
  assert.deepEqual(current.receipt.current,pin(actual));
  assert.deepEqual(current.receipt.sourceSuccessor,readCiLintSourceReceipt(actual,file));
  assert.deepEqual(checkGlobalGrammarContextSourceReceipt(file,actual,current.receipt),current.receipt);
  const old=restoreGlobalGrammarContextSource(file,pair.successor.source);
  assert.equal(old.current,pair.successor.source);assert.equal(old.predecessor,pair.predecessor.source);
  assert.equal(Object.hasOwn(old.receipt,'sourceSuccessor'),false,'every old receipt keeps its original shape');
  assert.throws(()=>checkGlobalGrammarContextSourceReceipt(file,actual,old.receipt));
  assert.throws(()=>restoreGlobalGrammarContextSource(file,actual+'\n// unknown edit\n'));
});
