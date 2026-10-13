import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {rustCompletionFixture} from '../scripts/generate-rust-completion-fixtures.mjs';
import {parseRust} from '../src/translation/rust.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {acceptRustCompletionModel,acceptRustReturnOwnership,readRustConditionalCompletion,readRustCompletionKind} from '../src/translation/rust-completion-rules.js';

const fixture=readProjectData(new URL('../../parity/fixtures/rust-completions.lino',import.meta.url));
const options=entry=>({completionModel:fixture.completionModel,...(entry.arrayResultModel?{arrayResultModel:entry.arrayResultModel}:{})});
function run(code,extra={}) {
  const output=[],context={console:{log:(...values)=>output.push(values.map(String).join(' '))},...extra};
  vm.runInNewContext(code,context,{timeout:1000});
  return {output,context};
}
function expressions(value,predicate,result=[]) {
  if(!value||typeof value!=='object')return result;
  if(predicate(value))result.push(value);
  for(const child of Object.values(value))expressions(child,predicate,result);
  return result;
}
function field(network,parent,label){return network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label)?.child;}
function exactRefusal(entry,selected) {
  assert.throws(()=>checkProgram(parseRust(entry.source,selected)),error=>{
    assert.equal(error.kind,entry.kind,entry.id);assert.equal(error.reason,entry.reason,entry.id);assert.deepEqual(error.span??null,entry.span,entry.id);return true;
  });
}

test('closed Rust completions preserve ten independent original pairs and source-free target behavior',()=>{
  assert.deepEqual(rustCompletionFixture(),fixture);
  assert.equal(fixture.cases.length,10);
  for(const entry of fixture.cases){
    assert.deepEqual(run(entry.originalJavaScript).output,entry.output,entry.id);
    assert.deepEqual(run(emitJavaScript(checkProgram(parseJavaScript(entry.originalJavaScript))).text).output,entry.output,entry.id);
    assert.deepEqual(JSON.parse(JSON.stringify(parseRust(entry.source,options(entry)))),entry.surface,entry.id);
    const network=LinkNetwork.fromLino(entry.archive);
    assert.deepEqual(run(emitTypedProgramLinks(network,entry.root,'JavaScript').code).output,entry.output,entry.id);
    assert.equal(emitTypedProgramLinks(network,entry.root,'Rust').code,entry.rust,entry.id);
    assert.deepEqual(run(emitJavaScript(checkProgram(parseRust(entry.rustDefinitions+'\n'+entry.main,options(entry)))).text).output,entry.output,entry.id);
    assert.deepEqual(run(emitJavaScript(checkProgram(parseRust(entry.reverseRustDefinitions+'\n'+entry.main,options(entry)))).text).output,entry.output,entry.id);
    assert.ok(network.links().every(link=>!link.metadata().definition?.includes(entry.source)),entry.id);
  }
  assert.throws(()=>parseRust(fixture.cases[0].rust,options(fixture.cases[0])),/function without a result: ml_main/u,'full emitted-main wrapper admission remains a separate source proof');
});

test('default direct-return and return-expression oracles remain exact while selected escaping and unsafe completions refuse',()=>{
  assert.equal(fixture.legacy.length,2);assert.equal(fixture.invalid.length,24);
  for(const entry of fixture.legacy)exactRefusal(entry);
  assert.equal(fixture.legacy[0].id,'frontend-rust/frontend-rust-negativeEarlyReturn');
  assert.equal(fixture.legacy[0].reason,'early return: write the result as the tail expression of the block');
  assert.deepEqual(fixture.legacy[0].span,{start:22,end:28});
  assert.equal(fixture.legacy[1].id,'frontend-rust/frontend-rust-returnExpression');
  assert.equal(fixture.legacy[1].reason,'return expression: outside the portable core');
  assert.deepEqual(fixture.legacy[1].span,{start:32,end:38});
  assert.doesNotThrow(()=>checkProgram(parseRust(fixture.legacy[0].source,options({}))));
  exactRefusal(fixture.legacy[1],options({}));
  for(const entry of fixture.invalid)exactRefusal(entry,options(entry));
});

test('selected Rust models and nested user provenance remain inspectable and independently validated',()=>{
  for(const entry of fixture.cases){
    const archive=createTypedProgramLinks(entry.source,'Rust',{rustCompletionModel:fixture.completionModel,...(entry.arrayResultModel?{rustArrayResultModel:entry.arrayResultModel}:{}),provenance:{project:entry.id}});
    const network=LinkNetwork.fromLino(archive.network.toLino()),provenance=field(network,archive.root,'provenance'),profile=field(network,provenance,'source profile');
    assert.equal(network.link(field(network,profile,'rust completion model')).metadata().definition,fixture.completionModel);
    assert.equal(network.link(field(network,field(network,provenance,'user provenance'),'project')).metadata().definition,entry.id);
    const selectedArray=field(network,profile,'rust array result model');
    if(entry.arrayResultModel)assert.equal(network.link(selectedArray).metadata().definition,entry.arrayResultModel);else assert.equal(selectedArray,undefined);
    assert.deepEqual(run(emitTypedProgramLinks(network,archive.root,'JavaScript').code).output,entry.output);
  }
  assert.throws(()=>parseRust('fn main(){}',{completionModel:'unknown'}),/Rust completion model/u);
  assert.throws(()=>createTypedProgramLinks('fn main(){}','Rust',{rustCompletionModel:'unknown'}),/Rust completion model/u);
  assert.throws(()=>createTypedProgramLinks('fn main(){}','Rust',{rustCompletionModel:fixture.completionModel,rustArrayResultModel:'unknown'}),/Rust array result model/u);
  assert.throws(()=>createTypedProgramLinks('function f(){return true;}','JavaScript',{rustCompletionModel:fixture.completionModel}),/requires Rust source/u);
});

test('guard lowering retains condition order, actual shadowed binding identities and original Unicode spans',()=>{
  const condition=fixture.cases.find(entry=>entry.id==='condition-call-once');
  const checked=decodeTypedProgramLinks(LinkNetwork.fromLino(condition.archive),condition.root);
  const choose=checked.items.find(item=>item.name==='choose');
  assert.equal(expressions(choose.body,node=>node.k==='call'&&node.fn==='condition').length,1);
  const counter={calls:0};
  assert.deepEqual(run(condition.javascript.replace('function condition() {','function condition() { counter.calls += 1;'),{counter}).output,condition.output);
  assert.equal(counter.calls,1);
  const scope=fixture.cases.find(entry=>entry.id==='continuation-outside-shadow-scope');
  const network=LinkNetwork.fromLino(scope.archive),program=decodeTypedProgramLinks(network,scope.root);
  assert.deepEqual(run(emitTypedProgramLinks(network,scope.root,'JavaScript').code).output,['inner','outer']);
  const names=expressions(program.items[0].body,node=>node.k==='var'&&node.name==='word');
  assert.equal(names.length,2);
  assert.deepEqual(names.map(node=>scope.source.slice(node.span.start,node.span.end)),['word','word']);
  const letIds=network.fieldRelations().filter(row=>row.label==='construct kind'&&network.link(row.child).metadata().definition==='let').map(row=>row.parent);
  const wordLets=letIds.filter(id=>network.link(field(network,id,'name')).metadata().definition==='word');
  assert.equal(wordLets.length,2);
  const used=network.fieldRelations().filter(row=>row.label==='binding reference'&&wordLets.some(id=>id.equals(row.child)));
  assert.equal(used.length,2);assert.ok(!used[0].child.equals(used[1].child));
});

test('semantic return-value link edits alter execution after source-free reload',()=>{
  const entry=fixture.cases.find(value=>value.id==='simple-guard'),network=LinkNetwork.fromLino(entry.archive);
  const literal=network.links().find(link=>link.metadata().definition==='early');assert.ok(literal);
  literal.setMetadata(literal.metadata().withDefinition('changed'));
  const changed=LinkNetwork.fromLino(network.toLino());
  assert.deepEqual(run(emitTypedProgramLinks(changed,entry.root,'JavaScript').code).output,['changed','late']);
  assert.match(emitTypedProgramLinks(changed,entry.root,'Rust').code,/changed/u);
  const abort=fixture.cases.find(value=>value.id==='abort-continuation');
  const replay=run(emitTypedProgramLinks(LinkNetwork.fromLino(abort.archive),abort.root,'JavaScript').code);
  assert.equal(replay.context.choose(true),'safe');
  assert.throws(()=>replay.context.choose(false),/selected failure/u);
});

test('shared completion decisions admit only closed guards and proved direct ownership',()=>{
  assert.equal(acceptRustCompletionModel(''),true);assert.equal(acceptRustCompletionModel(fixture.completionModel),true);assert.equal(acceptRustCompletionModel('unknown'),false);
  assert.equal(acceptRustReturnOwnership(true,false),true);
  for(const pair of [[false,false],[true,true],[false,true]])assert.equal(acceptRustReturnOwnership(...pair),false);
  assert.equal(readRustConditionalCompletion('returned','',false,true),'guarded continuation');
  assert.equal(readRustConditionalCompletion('returned','returned',true,false),'returned conditional');
  assert.equal(readRustConditionalCompletion('value','value',true,false),'conditional value');
  assert.equal(readRustConditionalCompletion('returned','returned',true,true),'unreachable continuation');
  assert.equal(readRustConditionalCompletion('value','value',true,true),'discarded conditional value');
  for(const row of [['value','',false,true],['returned','',false,false],['returned','value',true,false],['unknown','returned',true,false]])assert.equal(readRustConditionalCompletion(...row),'unsupported completion');
  assert.equal(readRustConditionalCompletion('value','aborted',true,false),'conditional value');
  assert.equal(readRustConditionalCompletion('aborted','value',true,false),'conditional value');
  assert.equal(readRustConditionalCompletion('aborted','aborted',true,false),'aborted conditional');
  assert.equal(readRustCompletionKind('guarded continuation','returned','aborted'),'returned');
  assert.equal(readRustCompletionKind('returned conditional','aborted','returned'),'returned');
  assert.equal(readRustCompletionKind('aborted conditional','aborted','aborted'),'aborted');
  assert.equal(readRustCompletionKind('guarded continuation','aborted','value'),'value');
  const hiddenReturn='fn f(flag:bool,choice:bool)->bool{if flag{if choice{return true;}panic!("stop");}else{false}}fn main(){}';
  assert.throws(()=>checkProgram(parseRust(hiddenReturn,options({}))),/guarded return completion/u,'a source-return path cannot become an aborted-only branch');
  const onlyAborts='fn choose(flag:bool,choice:bool)->String{if flag{if choice{panic!("first");}else{panic!("second");}}String::from("safe")}fn main(){println!("{}",choose(false,false));}';
  const code=emitJavaScript(checkProgram(parseRust(onlyAborts,options({})))).text;
  const checked=run(code);assert.deepEqual(checked.output,['safe']);
  assert.throws(()=>checked.context.choose(true,true),/first/u);
  assert.throws(()=>checked.context.choose(true,false),/second/u);
});
