// Future controls only. Do not run during the whole requirements draft.
import assert from'node:assert/strict';
import test from'node:test';
import{runInNewContext}from'node:vm';
import{readProjectData}from'../scripts/project-data.mjs';
import{parseRust}from'../src/translation/rust.js';
import{tokenize,TokenCursor}from'../src/translation/lexer.js';
import{checkProgram}from'../src/translation/check.js';
import{emitJavaScript}from'../src/translation/emit-javascript.js';
import{encodeTypedProgramLinks,emitTypedProgramLinks}from'../src/typed-program-links.js';
import{LinkNetwork}from'../src/network.js';
import{encodeLinkNetworkBinary,decodeLinkNetworkBinary}from'../src/link-network-binary.js';
const[major,minor]=process.versions.node.split('.').map(Number);
for(const filename of['rust-records','rust-record-layout','rust-raw-identifiers','rust-record-bindings','rust-tuple-patterns','rust-product-correction-tuple-struct','rust-product-correction-tuple-match','rust-pattern-conditions','rust-let-else']){
const fixture=readProjectData(new URL('../../parity/fixtures/'+filename+'.lino',import.meta.url));
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
test(filename==='rust-records'?'nominal Rust records retain order, actual type owners and four-target codecs':filename==='rust-record-layout'?'Rust named constructor layout retains complete paired controls':filename==='rust-raw-identifiers'?'Rust raw identifiers retain complete paired controls':'Rust irrefutable nominal bindings retain complete paired controls',()=>{
 const program=checkProgram(parseRust(fixture.source)),js=emitJavaScript(program),packet=encodeTypedProgramLinks(program);
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
for(const row of fixture.refusals)test('nominal record refusal: '+row.name,()=>assert.throws(()=>checkProgram(parseRust(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
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
