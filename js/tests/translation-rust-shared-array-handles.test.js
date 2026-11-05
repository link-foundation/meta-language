import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readFileSync} from 'node:fs';
import {generateRustSharedArrayHandleRules,rustSharedArrayHandleRulesPolicy} from '../scripts/generate-rust-shared-array-handle-rules.mjs';
import {parseRust,parseRustTokenStream} from '../src/translation/rust.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {tokenize} from '../src/translation/lexer.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {selfTranslate} from '../src/self-translation.js';
import {LinkNetwork} from '../src/network.js';
import {ParseConfiguration} from '../src/primitives.js';
import {parseProgrammingLanguage} from '../src/programming-language-parser.js';
import {createTypedProgramLinks,encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {validateLinkedTypedProgram} from '../src/typed-program-links-validation.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {acceptRustSharedArrayHandleSourceSize,acceptRustSharedArrayHandleLayer,acceptRustSharedArrayHandleTransfer,readRustSharedArrayHandleSourceForm,readRustSharedArrayHandleComparisonProjection} from '../src/translation/rust-shared-array-handle-rules.js';
const model='standard Rust shared array identity handles 1',options={sharedArrayHandleSourceModel:model};
const policy=rustSharedArrayHandleRulesPolicy(),[major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+policy.assertionVersions.Node);
const rc='std::rc::Rc<std::cell::RefCell<Vec<bool>>>',make='std::rc::Rc::new(std::cell::RefCell::new(vec![true]))';
const reloaded=packet=>[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];
function context(source){const result={};vm.runInNewContext(source,result,{timeout:1000});return result;}
function programs(program){const packet=encodeTypedProgramLinks(program);return[program,...reloaded(packet).map(network=>recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}))];}
function actualModule(source){const translated=selfTranslate(source,'JavaScript','Rust',{referenceSemantics:true});assert.ok(translated.items.filter(item=>item.term!=='comment').every(item=>['translated','comment'].includes(item.status)),JSON.stringify(translated.items));return translated.code;}
function sourceGraph(source){const parsed=parseProgrammingLanguage(source,'Rust');assert.equal(parsed.tree.flags.hasError,false);const network=LinkNetwork._parseWithLinks(source,'Rust',ParseConfiguration.default(),parsed).network;assert.ok(network.links().every(link=>!link.metadata().flags.hasError&&!link.metadata().flags.isMissing));assert.equal(network.reconstructText(),source);assert.equal(LinkNetwork.fromLino(network.toLino()).reconstructText(),source);assert.equal(decodeLinkNetworkBinary(encodeLinkNetworkBinary(network)).reconstructText(),source);}
function checkedNodes(program){const result=[],seen=new WeakSet();function visit(value){if(!value||typeof value!=='object'||seen.has(value))return;seen.add(value);if(value.k&&value.type)result.push(value);for(const child of value instanceof Map?value.values():Object.values(value))visit(child);}visit(program);return result;}

test('shared array source policy and actual automatically translated native library are fresh',async()=>{await generateRustSharedArrayHandleRules({check:true,native:true});});

test('the complete unchanged retained identity module reparses without cached checked input or invented main',async()=>{
  const fixture=readFileSync(new URL('./translation-module-reference.test.js',import.meta.url),'utf8');assert.equal(fixture,policy.retainedIdentity.source);
  const matches=[...fixture.matchAll(/const identity = `([^`]+)`;/gu)];assert.equal(matches.length,1);const source=matches[0][1];
  const independent=await import('data:text/javascript,'+encodeURIComponent(source));
  const module=actualModule(source);sourceGraph(module);const parsed=parseRust(module,options);assert.equal(parsed.main,null,'a library source gains no fake entry');
  const checked=checkProgram(parsed);assert.equal(checked.declarations.get('identity').params[0].type.reference,true);assert.equal(checked.declarations.get('identity').ret.reference,true);
  const original=emitRust(checked);
  for(const program of programs(checked)){
    const output=context(emitJavaScript(program).text);
    for(const value of [[],[1],[-0,NaN,Infinity,3]]){assert.equal(independent.identity(value),value);assert.equal(output.identity(value),value);}
    const replay=emitRust(program);for(const field of ['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(replay[field],original[field]);
  }
  const packet=createTypedProgramLinks(module,'Rust',{rustSharedArrayHandleSourceModel:model});
  for(const network of reloaded(packet)){const program=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});const value=[1];assert.equal(context(emitJavaScript(program).text).identity(value),value);}
  assert.throws(()=>parseRust(module),'the original legacy source interpretation remains unchanged');
});

test('actual whole emitted modules keep independent alias distinct and fresh scalar array observations',()=>{
  for(const row of policy.cases){
    assert.equal(context(row.source)[row.call](),row.expected,row.name+' independent original JS');
    const source=actualModule(row.source);sourceGraph(source);const checked=checkProgram(parseRust(source,options));
    assert.ok(checkedNodes(checked).some(node=>node.k==='referenceAllocate'));
    assert.ok(checkedNodes(checked).some(node=>node.k==='binary'&&node.domain?.reference===true));
    const original=emitRust(checked);
    for(const program of programs(checked)){assert.equal(context(emitJavaScript(program).text)[row.call](),row.expected);const replay=emitRust(program);for(const field of ['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(replay[field],original[field]);}
  }
});

test('fresh shared handles preserve every scalar payload after Rust parsing and both Links reloads',()=>{
  const fixtures=policy.cases.filter(row=>row.source.includes('function fresh()'));
  assert.equal(fixtures.length,3,'Boolean, numeric and Unicode string payload domains each retain a fresh allocation fixture');
  for(const row of fixtures){
    const independent=context(row.source),expected=[...independent.fresh()];
    assert.equal(expected.length>0,true);
    const source=actualModule(row.source),checked=checkProgram(parseRust(source,options));
    for(const program of programs(checked)){
      const emitted=context(emitJavaScript(program).text),actual=[...emitted.fresh()];
      assert.deepEqual(actual,expected,row.name+' keeps every value, including signed zero and Unicode');
      assert.notStrictEqual(emitted.fresh(),emitted.fresh(),'each call still owns a distinct allocation');
    }
  }
});

test('two actual borrowed call operands keep one evaluation each and original source order',()=>{
  const source=`fn left()->${rc}{${make}} fn right()->${rc}{${make}} fn probe()->bool{std::rc::Rc::ptr_eq(&left(),&right())}`;
  const independent=context('function left(){return [true];}function right(){return [true];}function probe(){return left()===right();}');
  function observed(ctx){const calls=[];for(const name of ['left','right']){const original=ctx[name];ctx[name]=()=>{calls.push(name);return original();};}return{result:ctx.probe(),calls};}
  assert.deepEqual(observed(independent),{result:false,calls:['left','right']});
  const checked=checkProgram(parseRust(source,options));
  for(const program of programs(checked)){assert.deepEqual(observed(context(emitJavaScript(program).text)),{result:false,calls:['left','right']});const calls=checkedNodes(program).filter(node=>node.k==='call'&&['left','right'].includes(node.fn));assert.deepEqual(calls.map(node=>node.fn),['left','right']);}
});

test('the shared decisions refuse forged owners domains source sizes and payload transfers',()=>{
  for(const size of [-1,1.5,65537,NaN,Infinity])assert.equal(acceptRustSharedArrayHandleSourceSize(size),false);
  for(const size of [0,1,65536])assert.equal(acceptRustSharedArrayHandleSourceSize(size),true);
  assert.equal(acceptRustSharedArrayHandleLayer('handle','std::rc::Rc',false),true);
  for(const owner of ['Rc','std::sync::Arc','alloc::rc::Rc','custom::Rc'])assert.equal(acceptRustSharedArrayHandleLayer('handle',owner,false),false);
  assert.equal(acceptRustSharedArrayHandleLayer('handle','std::rc::Rc',true),false);
  assert.equal(acceptRustSharedArrayHandleTransfer('owned vector','standard shared array handle',0,0),false);
  assert.equal(acceptRustSharedArrayHandleTransfer('standard shared array handle','standard shared array handle',1,0),false);
  assert.equal(readRustSharedArrayHandleSourceForm('pointer equality',2,true,true,true,true,true,false),'identity equality');
  for(const args of [[2,false,true,true,true,true],[2,true,true,false,true,true],[2,true,true,true,true,false],[1,true,true,true,true,true]])assert.equal(readRustSharedArrayHandleSourceForm('pointer equality',...args,false),'');
  for(const operation of ['eq','ne'])assert.equal(readRustSharedArrayHandleComparisonProjection('Rust',operation,'array',true),'referenceIdentity');
  for(const [language,operation,kind,reference]of [['JavaScript','eq','array',true],['Rust','lt','array',true],['Rust','eq','array',false],['Rust','eq','tuple',true],['Rust','eq','map',true]])assert.equal(readRustSharedArrayHandleComparisonProjection(language,operation,kind,reference),'');
});

test('payload equality moves escaping borrows and unrepresented Rust observations remain explicit refusals',()=>{
  const sources=[
    `fn value(input:Vec<bool>)->Vec<bool>{input.clone()}`,
    `fn value(input:&${rc})->bool{true}`,
    `fn value(input:std::sync::Arc<std::cell::RefCell<Vec<bool>>>)->bool{true}`,
    `fn value(input:std::rc::Rc<Vec<bool>>)->bool{true}`,
    `fn value(input:${rc})->${rc}{input}`,
    `fn value(input:${rc})->${rc}{let alias=input;alias.clone()}`,
    `fn value(input:${rc})->bool{let alias=&input;std::rc::Rc::ptr_eq(alias,&input)}`,
    `fn value(input:${rc})->${rc}{input.borrow().clone()}`,
    `fn value(input:${rc})->f64{input.len() as f64}`,
    `fn value(left:${rc},right:${rc})->bool{left.clone()==right.clone()}`,
    `fn value(input:${rc})->usize{std::rc::Rc::strong_count(&input)}`,
    `fn value(input:${rc})->bool{input.borrow_mut().push(true);true}`,
    `fn value()->${rc}{std::rc::Rc::new(std::cell::RefCell::new(vec![]))}`,
    `fn value(input:${rc})->${rc}{std::rc::Rc::new(std::cell::RefCell::new(input.borrow().clone()))}`,
    `fn value()->bool{let bare=vec![true];true}`,
    `fn value()->bool{let bare=Vec::<bool>::new();true}`,
    `fn value(input:${rc})->String{format!("{input}")}`,
    `fn value()->${rc}{std::rc::Rc::new(std::cell::RefCell::new(vec![true]),1)}`,
    `use std::rc::Rc;fn value(input:Rc<std::cell::RefCell<Vec<bool>>>)->bool{true}`,
    `mod std{} fn value(input:${rc})->bool{true}`,
  ];
  for(const source of sources)assert.throws(()=>checkProgram(parseRust(source,options)),source);
  assert.throws(()=>parseRust('fn main(){}',{sharedArrayHandleSourceModel:'unknown'}));
  assert.throws(()=>parseRust(' '.repeat(65537),options));
  for(const key of ['arrayResultModel','completionModel','sourceSignatureModel','scalarLoopModel','primitiveOwnerModel','emptyOptionSourceModel','typeNamespaceModel','pureUnitCallModel','localValueStateModel','constFunctionModel','runtimeShellModel'])assert.throws(()=>parseRust('fn main(){}',{...options,[key]:'nonempty'}));
  assert.throws(()=>createTypedProgramLinks('function value(){return true;}','JavaScript',{rustSharedArrayHandleSourceModel:model}),/requires Rust source/u);
});

test('actual token ownership and source-free comparison domains cannot be forged',()=>{
  const source=`fn value(input:${rc})->${rc}{input.clone()}`;const tokens=tokenize(source,'Rust').tokens;
  const changed=structuredClone(tokens);changed.find(token=>token.value==='std').value='custom';
  assert.throws(()=>parseRustTokenStream(changed,options,source));
  assert.throws(()=>parseRustTokenStream(tokens,options,null));
  const checked=checkProgram(parseRust(`fn value(left:${rc},right:${rc})->bool{std::rc::Rc::ptr_eq(&left,&right)}`,options));
  const bad=structuredClone(checked),binary=checkedNodes(bad).find(node=>node.k==='binary'&&node.domain?.reference===true);assert.ok(binary);binary.domain={...binary.domain,reference:false};
  assert.throws(()=>validateLinkedTypedProgram(bad));assert.throws(()=>recheckLinkedTypedProgram(bad,{wholeProgram:true}));
});

test('explicit Rust identity survives both codecs while ordinary Rust value equality keeps its original refusal',()=>{
  const source=`fn value(left:${rc},right:${rc})->bool{std::rc::Rc::ptr_eq(&left,&right)}`;
  const surface=parseRust(source,options);assert.equal(surface.items[0].body.k,'referenceIdentity');
  assert.equal(source.slice(surface.items[0].body.span.start,surface.items[0].body.span.end),'std::rc::Rc::ptr_eq(&left,&right)');
  const checked=checkProgram(surface);
  for(const program of programs(checked)){
    const functions=context(emitJavaScript(program).text),value=[true];
    assert.equal(functions.value(value,value),true);assert.equal(functions.value([true],[true]),false);
    const binary=checkedNodes(program).find(node=>node.k==='binary');assert.equal(binary.domain.reference,true);
  }
  // The same typed operands under ordinary Rust source equality still refuse:
  // the existing JavaScript-only branch of Checker.binary is unchanged.
  const ordinary=structuredClone(surface);ordinary.items[0].body.k='binary';
  assert.throws(()=>checkProgram(ordinary),/comparison of arrays/u);
  const disabled=structuredClone(surface);disabled.referenceSemantics=false;
  assert.throws(()=>checkProgram(disabled),/explicit Rust array identity/u);
  const legacy='fn same(left:Vec<bool>,right:Vec<bool>)->bool{left==right}fn main(){}';
  assert.throws(()=>checkProgram(parseRust(legacy)),/comparison of arrays/u);
  const extra=structuredClone(surface);extra.items[0].body.domain={kind:'array',element:{kind:'bool'},reference:true};
  assert.throws(()=>checkProgram(extra),/closed owned operands/u);
});

test('the complete executable runner stays an explicit separate source composition obligation',()=>{
  const actual=emitRust(checkProgram(parseJavaScript(policy.cases[0].source,{referenceSemantics:true})));
  assert.ok(actual.text.includes('std::thread::Builder::new()'));assert.ok(actual.text.includes('fn ml_main()'));
  assert.throws(()=>parseRust(actual.text,options),'retain the WHOLE actual executable; no runner or prelude is removed');
});
