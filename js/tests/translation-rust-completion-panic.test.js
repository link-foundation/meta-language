import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {rustCompletionPanicFixture} from '../scripts/generate-rust-completion-panic-fixtures.mjs';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {LinkNetwork} from '../src/network.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {acceptRustCompletionPanicArguments} from '../src/translation/rust-completion-rules.js';

const fixture=readProjectData(new URL('../../parity/fixtures/rust-completion-panic.lino',import.meta.url));
function observe(code){const output=[],context={console:{log:(value)=>output.push(String(value))}};vm.runInNewContext(code,context,{timeout:1000});return{output,context};}
function refused(entry,selected){assert.throws(()=>checkProgram(parseRust(entry.source,selected?{completionModel:fixture.completionModel}:{})),error=>{assert.equal(error.kind,entry.kind);assert.equal(error.reason,entry.reason);assert.deepEqual(error.span??null,entry.span);return true;});}
test('the exact canonical panic form keeps braces, escapes and empty Unicode messages as literal data',()=>{
 assert.deepEqual(rustCompletionPanicFixture(),fixture);assert.equal(fixture.cases.length,4);
 for(const entry of fixture.cases){
  for(const code of [entry.originalJavaScript,emitTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root,'JavaScript').code]){
   const run=observe(code);assert.deepEqual(run.output,entry.output,entry.id);
   assert.equal(run.context.fail(true),true);
   assert.throws(()=>run.context.fail(false),error=>{assert.equal(error.message,entry.message,entry.id);return true;});
  }
  assert.equal(emitTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root,'Rust').code,entry.rust);
  assert.deepEqual(JSON.parse(JSON.stringify(parseRust(entry.source,{completionModel:fixture.completionModel}))),entry.surface);
 }
});
test('dynamic Display, other templates and legacy formatted panic keep their precise original refusals',()=>{
 assert.equal(fixture.invalid.length,10);
 for(const entry of fixture.invalid)refused(entry,true);
 refused(fixture.defaultCanonical,false);refused(fixture.legacy,false);refused(fixture.legacy,true);
 assert.equal(fixture.legacy.id,'frontend-rust/frontend-rust-panicArguments');
 assert.equal(fixture.legacy.reason,'panic! arguments: only a literal message is portable');
 assert.deepEqual(fixture.legacy.span,{start:24,end:42});
});
test('shared literal panic admission and source-free edits require the exact selected semantics',()=>{
 const valid=[fixture.completionModel,'panic','str','{}','str','2'];
 assert.equal(acceptRustCompletionPanicArguments(...valid),true);
 for(const [index,value] of [[0,''],[0,'unknown'],[1,'unreachable'],[2,'name'],[3,'message {}'],[4,'from'],[5,'3']]){const wrong=[...valid];wrong[index]=value;assert.equal(acceptRustCompletionPanicArguments(...wrong),false);}
 const entry=fixture.cases[0],network=LinkNetwork.fromLino(entry.archive),message=network.links().find(link=>link.metadata().definition===entry.message);assert.ok(message);
 message.setMetadata(message.metadata().withDefinition('edited {data}'));
 const edited=LinkNetwork.fromLino(network.toLino()),observed=observe(emitTypedProgramLinks(edited,entry.root,'JavaScript').code);
 assert.deepEqual(observed.output,entry.output);
 assert.throws(()=>observed.context.fail(false),error=>{assert.equal(error.message,'edited {data}');return true;});
 assert.match(emitTypedProgramLinks(edited,entry.root,'Rust').code,/edited \{data\}/u);
});
