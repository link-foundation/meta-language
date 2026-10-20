import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {generateStaticBorrowModuleHistory} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-static-borrow-module-history.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';
import {tokenize} from '../src/translation/lexer.js';

const root=new URL('../../',import.meta.url);
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
function normalizeNativeLiteralSeparators(source) {
  const tokens=tokenize(source,'Rust').tokens.filter(token=>token.kind==='number'&&token.raw.includes('_'));
  for(const token of tokens.reverse())source=source.slice(0,token.start)+source.slice(token.start,token.end).replaceAll('_','')+source.slice(token.end);
  return source;
}

test('one native static rule owner retains the complete original storage body source membership and controls',()=>{
  const history=generateStaticBorrowModuleHistory({check:true});assert.equal(history.sources.length,2);
  for(const row of history.sources){
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:history.revision,path:row.file});
    const current=readFileSync(new URL(row.file,root),'utf8');assert.deepEqual(pin(current),row.current);
    let previous=current;
    for(const action of [...row.actions].reverse()){
      assert.equal(previous.split(action.to).length,2);previous=previous.replace(action.to,action.from);
    }
    assert.equal(previous,row.sourceText);
    assert.throws(()=>verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText+'\n'),{revision:history.revision,path:row.file}));
  }
  for(const row of history.unchanged)assert.deepEqual(pin(readFileSync(new URL(row.file,root))),{bytes:row.bytes,sha256:row.sha256});
});

test('every complete static-borrow test input and assertion remains exact modulo value preserving separators',()=>{
  const history=generateStaticBorrowModuleHistory({check:true}),previous=history.sources.find(row=>row.file.endsWith('grammar_outcome_static_borrows.rs'));
  const current=readFileSync(new URL(previous.file,root),'utf8');
  for(const name of ['generated_static_reference_selection_preserves_all_independent_cases_and_resource_boundaries',
    'native_storage_returns_the_actual_existing_static_reference_without_allocating_a_new_lifetime']){
    assert.equal(normalizeNativeLiteralSeparators(readSourceTextRustBlock(current,'fn',name)),
      readSourceTextRustBlock(previous.sourceText,'fn',name),'complete bodies retain every original assertion and source input');
  }
  assert.ok(current.includes('use storage::rules;'));
  assert.ok(!current.includes('mod rules;'));
  assert.ok(readFileSync(new URL(history.sources[0].file,root),'utf8').includes('pub(super) mod rules;'));
  for(const [original,current]of [['1048576.0','1_048_576.0'],['1048577.0','1_048_577.0'],['1048577','1_048_577']])
    assert.equal(Number(original),Number(current.replaceAll('_','')));
});
