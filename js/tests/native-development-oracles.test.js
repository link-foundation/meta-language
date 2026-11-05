import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { LinkNetwork, compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';
import {runAgdaLayoutOracleBuild} from '../scripts/build-agda-layout-oracle.mjs';

const read = (file) => readFileSync(resolveProjectDataPath(new URL(`../../${file}`, import.meta.url)), 'utf8');
const readData = (file) => readProjectData(new URL(`../../${file}`, import.meta.url));
const lock = readData('js/src/vendor/grammars/grammar-lock.json');

test('Agda oracle compiler retries only bounded transient downloads and retains exact build arguments',()=>{
  const args=['--yes','--package','tree-sitter-cli@0.27.0','tree-sitter','build','--wasm','-o','oracle.wasm','grammar'];
  const options={cwd:'/scratch',timeout:300000},calls=[],pauses=[],reports=[];
  const transient=Object.assign(new Error('download'),{stderr:Buffer.from('curl: (22) The requested URL returned error: 502\nError: Failed to download clang')});
  const execute=(command,actual,settings)=>{calls.push({command,actual,settings});if(calls.length<3)throw transient;return Buffer.from('built');};
  assert.deepEqual(runAgdaLayoutOracleBuild(args,options,{execute,pause:value=>pauses.push(value),report:value=>reports.push(value)}),Buffer.from('built'));
  assert.equal(calls.length,3);assert.deepEqual(pauses,[2000,4000]);assert.equal(reports.length,3);
  for(const call of calls){assert.equal(call.command,'npx');assert.deepEqual(call.actual,args);assert.deepEqual(call.settings,{...options,stdio:'pipe',maxBuffer:2*1024*1024});}
  for(const failure of [new Error('source error'),Object.assign(new Error('compiler error'),{stderr:'error: syntax error 502'}),Object.assign(new Error('timeout'),{code:'ETIMEDOUT'}),transient]){
    let count=0;const delays=[];
    assert.throws(()=>runAgdaLayoutOracleBuild(args,options,{execute:()=>{count++;throw failure;},pause:value=>delays.push(value),report:()=>{}}),error=>error===failure);
    assert.equal(count,failure===transient?3:1);assert.deepEqual(delays,failure===transient?[2000,4000]:[]);
  }
});

for (const [id, language, source] of [
  ['lean', 'Lean', 'def answer : Nat := 42\n'],
  ['rocq', 'Rocq', 'Definition answer : nat := 42.\n'],
]) {
  test(`native ${language} retains its independent development oracle`, () => {
    assert.equal(lock.grammars[id].oracle, true);
    const fixture = readData(`parity/fixtures/native-grammars/${id}.json`);
    const parser = compileGrammar(parseGrammarLinks(read(`parity/grammars/native/${id}.lino`)));
    assert.equal(oracleRecovers(source, language), false);
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, language));
    const network = LinkNetwork.parse(source, language);
    assert.equal(network.reconstructText(), source);
    assert.ok(network.parseGrammars().some(({ id: grammar }) => grammar === `native-${id}`));
  });
}
