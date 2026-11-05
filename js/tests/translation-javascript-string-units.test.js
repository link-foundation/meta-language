// Future controls only. Never execute during the whole-requirements draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {createContext,runInContext,runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {tokenize} from '../src/translation/lexer.js';
import {decodeJavaScriptStringUnicodeEscape} from '../src/translation/javascript-string-unit-rules.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
const units=value=>Array.from({length:value.length},(_,at)=>value.charCodeAt(at));
test('all UTF16 escapes retain their original unit with independent source observations',()=>{
 const context=createContext({});
 for(let unit=0;unit<=65535;unit+=1){
  const escape='\\u'+unit.toString(16).padStart(4,'0');
  assert.deepEqual(decodeJavaScriptStringUnicodeEscape(units(escape),0),{$:'units',units:[unit],end:6});
  const source='"'+escape+'"';
  assert.deepEqual(units(runInContext(source,context)),[unit]);
 }
});
test('literal and template tokens preserve explicit complete unit columns and malformed syntax',()=>{
 const fixture=readProjectData(new URL('../../parity/fixtures/javascript-string-units.lino',import.meta.url));
 for(const row of fixture.unitLexicalCases){
  const tokens=tokenize(row.source,'JavaScript').tokens;
  assert.equal(tokens.length,2);
  const token=tokens[0],part=row.kind==='template'?token.parts[0]:token;
  assert.equal(token.kind,row.kind);
  assert.equal(part.value??part.text,row.value);
  assert.deepEqual(part.stringUnits??units(part.value??part.text),row.units);
  assert.deepEqual(units(runInNewContext(row.source)),row.units);
 }
 for(const escape of fixture.malformedEscapes){
  assert.deepEqual(decodeJavaScriptStringUnicodeEscape(units(escape),0),{$:'malformed'});
  assert.throws(()=>tokenize('"'+escape+'"','JavaScript'),error=>error.kind==='syntax'&&error.reason==='malformed unicode escape');
  assert.throws(()=>runInNewContext('"'+escape+'"'),error=>error.name==='SyntaxError');
 }
});
