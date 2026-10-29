// Future controls only. Do not run during the whole requirements draft.
import assert from'node:assert/strict';
import test from'node:test';
import{runInNewContext}from'node:vm';
import{readProjectData}from'../scripts/project-data.mjs';
import{parseRust}from'../src/translation/rust.js';
import {assertRustFreshVectorCallResultCorrection} from './support/rust-fresh-vector-call-result-corrections.js';
import{tokenize,TokenCursor}from'../src/translation/lexer.js';
import{checkProgram}from'../src/translation/check.js';
import{emitJavaScript}from'../src/translation/emit-javascript.js';
import{encodeTypedProgramLinks,emitTypedProgramLinks}from'../src/typed-program-links.js';
import{LinkNetwork}from'../src/network.js';
import{encodeLinkNetworkBinary,decodeLinkNetworkBinary}from'../src/link-network-binary.js';
const[major,minor]=process.versions.node.split('.').map(Number);
for(const filename of['rust-statement-values','rust-completion-carriers','rust-ordinary-returns','rust-terminal-returns','rust-unit-values','rust-records','rust-record-layout','rust-raw-identifiers','rust-record-bindings','rust-tuple-patterns','rust-product-correction-tuple-struct','rust-product-correction-tuple-match','rust-pattern-conditions','rust-conditional-chains','rust-match-guards','rust-let-else','rust-let-else-completions','rust-copy-closures','rust-numeric-patterns','rust-byte-patterns','rust-string-patterns','rust-canonical-discovery','rust-array-lookup-lazy','rust-byte-ascii','rust-array-cloned-lookup','rust-array-lookup','rust-contextual-from-function-value','rust-primitive-function-value','rust-callable-owner','rust-primitive-owner-path','rust-integer-magnitude','rust-wrapping-shift','rust-checked-shift','rust-integer-bit','rust-primitive-default','rust-checked-integer','rust-primitive-try-from','rust-primitive-expression','rust-integer-float-plan','rust-primitive-integer-constant','rust-primitive-from','rust-primitive-inherent-copy','rust-primitive-associated-call','rust-wrapping-integer','rust-saturating-multiplication','rust-saturating-integer','rust-signed-integer-property','rust-selection-imports','rust-primitive-selection','rust-named-formatting','rust-displayed-string-interface','rust-string-addition','rust-constructed-string-interface','rust-string-predicate-interface','rust-standard-string-views','rust-standard-owned-calls','rust-data-type-field-signatures','rust-scalar-vector-copies','rust-fresh-vector-call-results','rust-source-owner-retention','rust-pointer-integer-owners','rust-usize-index-expressions','rust-array-indices','rust-fresh-vector-results','rust-fresh-vector-arguments','rust-type-aliases','rust-function-qualifiers','rust-module-values','rust-source-constants','rust-pointer-results','rust-contextual-calls','rust-contextual-closures','rust-string-byte-lengths','rust-array-lengths','rust-function-pointers','unicode-rust-identifiers','rust-unicode-lifetimes']){
const fixture=readProjectData(new URL('../../parity/fixtures/'+filename+'.lino',import.meta.url));
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
test(filename==='rust-records'?'nominal Rust records retain order, actual type owners and four-target codecs':filename==='rust-record-layout'?'Rust named constructor layout retains complete paired controls':filename==='rust-raw-identifiers'?'Rust raw identifiers retain complete paired controls':'Rust irrefutable nominal bindings retain complete paired controls',()=>{
 const program=checkProgram(parseRust(fixture.source,{retainSourceTypeOwners:fixture.retainSourceTypeOwners===true})),js=emitJavaScript(program),packet=encodeTypedProgramLinks(program);
 for(const target of['JavaScript','Rust','Lean','Rocq']){
  const original=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const network of[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),original);
 }
 if(fixture.layout){
  const rust=emitTypedProgramLinks(packet.network,packet.root,'Rust');
  for(const piece of fixture.layout.pieces)assert.ok(rust.code.includes(piece),piece);
  assert.ok(rust.code.split('\n').some(line=>line.includes(fixture.layout.patternPrefix)&&line.includes(' } => ')));
  assert.equal(rust.encodings.find(encoding=>encoding.id==='data').statement,fixture.layout.encoding);
 }
 const names=fixture.cases.map(row=>js.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===(row.module?row.module+'.result':'result')).target);
 const functions=runInNewContext(js.text.replace(/^export /gmu,'')+'\n['+names.join(',')+'];');
 for(const[index,row]of fixture.cases.entries()){
  if(row.previousExpectation){
   assert.ok(fixture.source.startsWith(row.source));
   const previous=row.correctionKind==='tuple-struct'?{kind:'unsupported',messagePattern:'Rust struct item'}:row.correctionKind==='tuple-match'?{kind:'unsupported',messagePattern:'tuple match pattern'}:{kind:'unsupported',reason:'Rust record update: update syntax needs complete move and field provenance'};
   assert.deepEqual(row.previousExpectation,previous);
  }
  if(row.abort!==null)assert.throws(()=>functions[index](),error=>error.message===row.abort,row.name);
  else assert.equal(String(functions[index]()),row.value,row.name);
 }
});
for(const row of fixture.originalRefusals??[])test('same original completion input now admits complete graphs: '+row.name,()=>{
 assert.equal(row.kind,'unsupported');assert.match(row.reason,/guarded return completion/u);
 const program=checkProgram(parseRust(row.source)),packet=encodeTypedProgramLinks(program);
 for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
 }
});
for(const row of fixture.refusals)test('nominal record refusal: '+row.name,()=>{if(filename==='rust-fresh-vector-results'&&row.name==='unchecked interprocedural result tail')assertRustFreshVectorCallResultCorrection(row);else assert.throws(()=>checkProgram(parseRust(row.source)),error=>error.kind===row.kind&&error.reason===row.reason);});
for(const row of fixture.lexicalCases??[])test('raw identifier token source: '+row.source,()=>{
 const tokens=tokenize(row.source,'Rust').tokens;
 assert.equal(tokens.length,row.tokens.length+1);
 const cursor=new TokenCursor(tokens,'Rust');
 for(const expected of row.tokens){
  const token=cursor.peek();
  assert.deepEqual({raw:token.raw,kind:token.kind,value:token.value,start:token.start,end:token.end},
   {raw:expected.raw,kind:expected.kind,value:expected.value,start:expected.start,end:expected.end});
  assert.equal(cursor.is(expected.value),expected.matchesValue);cursor.next();
 }
 assert.equal(cursor.peek().kind,'eof');
});
for(const row of fixture.lexicalErrors??[])test('raw identifier lexical refusal: '+row.source,()=>assert.throws(()=>tokenize(row.source,'Rust'),error=>{
 assert.equal(error.kind,row.kind);assert.equal(error.reason,row.reason);assert.deepEqual(error.span,row.span);return true;
}));
}

for(const source of [
 'fn bad(){if true{7i64}else{8i64}println!("after");}fn main(){}',
 'fn bad(){if true{true}else{false}println!("after");}fn main(){}',
 'fn bad(flag:bool)->i64{{7i64}if flag{return 1;}2}fn main(){}',
])test('implicit block statements retain their actual Unit constraint: '+source,()=>assert.throws(()=>checkProgram(parseRust(source))));
