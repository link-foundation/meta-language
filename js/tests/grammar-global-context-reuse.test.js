import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {generateGlobalGrammarContextReuse} from '../../experiments/global-grammar-context-reuse/generate.mjs';
import {readGlobalGrammarContextProposal} from '../../experiments/global-grammar-context-reuse/proposal.mjs';
import {parseNative} from '../src/native-grammar-parser.js';
const root=new URL('../../',import.meta.url);

test('complete context reuse source proposals retain original producers tests and exact source inverses',()=>{
  const packet=generateGlobalGrammarContextReuse({check:true});assert.equal(packet.sources.length,4);
  for(const row of packet.sources){
    const proposed=readGlobalGrammarContextProposal(row.file);assert.equal(readFileSync(new URL(row.file,root),'utf8'),row.sourceText);
    assert.deepEqual(proposed.proposed,row.proposed);
    const tree=parseNative(row.file.endsWith('.rs')?'native-rust':'native-javascript',proposed.source);
    assert.equal(tree.hasError,false);assert.equal(tree.start,0);assert.equal(tree.end,Buffer.byteLength(proposed.source));
  }
  assert.throws(()=>readGlobalGrammarContextProposal('unknown/source.js'));
});

test('an explicitly loaded private proposal passes all original archive controls and unchanged default priority budgets',()=>{
  const environment={...process.env,CI:'false',GITHUB_ACTIONS:'false',PATH:''};
  delete environment.NODE_TEST_CONTEXT;
  const child=spawnSync(process.execPath,['--max-old-space-size=320','--max-semi-space-size=4','--import',
    new URL('../../experiments/global-grammar-context-reuse/loader.mjs',import.meta.url).pathname,
    '--test','--test-concurrency=1','js/tests/global-concept-archive-parser.test.js'],
  {cwd:root,env:environment,encoding:'utf8',timeout:60000,maxBuffer:262144});
  assert.equal(child.error,undefined);assert.equal(child.signal,null);assert.equal(child.status,0,child.stderr+'\n'+child.stdout);
  assert.match(child.stdout,/all four archived priority grammar samples retain complete concrete rows within default budgets/u);
  assert.match(child.stdout,/(?:pass 11|# pass 11)/u);assert.match(child.stdout,/(?:fail 0|# fail 0)/u);
});
