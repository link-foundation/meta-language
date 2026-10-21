import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readPortableLibraryRelocation} from '../scripts/record-rule-library.mjs';
import {tokenize} from '../src/translation/lexer.js';
import {DecoratorSet} from '../src/decorators.js';

test('module relocation preserves emitted source literals, comments and complete identifier boundaries',()=>{
  const source='pub mod ml_array {}\npub fn f() { crate :: ml_array::read(); crate::ml_array_extra::read(); let text = "crate::ml_array::map()"; let raw = r##"crate::ml_array"##; /* crate::ml_array */ // crate::ml_array\n}\n';
  const expected=source.replace('crate :: ml_array::read()','self :: ml_array::read()');
  const result=readPortableLibraryRelocation(source);assert.equal(result.source,expected);assert.equal(result.edits.length,1);
  const literals=value=>tokenize(value,'Rust').tokens.filter(token=>token.kind==='string').map(token=>token.raw);
  assert.deepEqual(literals(result.source),literals(source));
  assert.deepEqual(tokenize(result.source,'Rust').comments.map(token=>token.text),tokenize(source,'Rust').comments.map(token=>token.text));
  const decorators=new DecoratorSet([{id:'actual-library-relocation',level:'emitter',order:0,when:[['format','Rust']],actions:result.actions}]);
  assert.equal(DecoratorSet.fromLino(decorators.toLino()).decorate('emitter',{format:'Rust',source}).source,expected);
  let restored=result.source;for(const edit of result.edits){assert.equal(restored.slice(edit.start,edit.start+edit.to.length),edit.to);restored=restored.slice(0,edit.start)+edit.from+restored.slice(edit.start+edit.to.length);}
  assert.equal(restored,source);
});

test('already correct declaration relocation keeps its original compact receipt',()=>{
  const source='pub mod values {}\npub fn f() { crate::values::read(); }';
  assert.deepEqual(readPortableLibraryRelocation(source).actions,[{op:'replace',field:'source',from:'crate::values',to:'self::values'}]);
});

test('source-producing native libraries retain JavaScript crate-qualified array operation text',()=>{
  for(const [file,expected] of [['collection_construction_rules/part_1.rs','crate::ml_array::{}('],
    ['array_from_rules/part_1.rs','crate::ml_array::optional_at(']]){
    const source=readFileSync(new URL('../../rust/src/translation/'+file,import.meta.url),'utf8');
    const values=tokenize(source,'Rust').tokens.filter(token=>token.kind==='string').map(token=>token.value);
    assert.ok(values.some(value=>value.includes(expected)));
    assert.ok(values.every(value=>!value.includes('self::ml_array::')));
  }
});
