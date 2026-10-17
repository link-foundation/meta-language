// JavaScript-first evidence; native compilation and runtime tests await CI.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {parseRust} from '../js/src/translation/rust.js';
import {parseJavaScript} from '../js/src/translation/javascript.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {createTypedProgramLinks,encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../js/src/typed-program-links.js';
import {LinkNetwork} from '../js/src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../js/src/link-network-binary.js';
import {rustConstBranchFixture} from './generate-rust-const-branch-fixture.mjs';
import {rustConstBranchPolicy,rustConstBranchRules} from './generate-rust-const-branch-rules.mjs';
import {rustConstFunctionFixture} from './generate-rust-const-function-fixture.mjs';

const fixture = readProjectData(new URL('./rust-const-branch-fixture.lino',import.meta.url));
const oldFixture = readProjectData(new URL('./rust-const-function-fixture.lino',import.meta.url));
const load = entry => decodeTypedProgramLinks(LinkNetwork.fromLino(entry.links),entry.root);
function observeTruthRows(code,entry,expected=entry.expected) {
  const context = {};
  vm.runInNewContext(code,context,{timeout:1000});
  assert.deepEqual(entry.inputs.map(args => context[entry.name](...args)),expected);
  return context;
}
function observe(code,entry,expected=entry.expected) {
  const context = observeTruthRows(code,entry,expected);
  for (const value of [undefined,null,0,1,'false',{},new Boolean(false)]) if(entry.inputs[0].length) {
    assert.throws(() => context[entry.name](value,...entry.inputs[0].slice(1)),/requires boolean/u);
  }
}
function archives(program) {
  const packet = encodeTypedProgramLinks(program);
  return {root:packet.root,networks:[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]};
}

test('the explicit Boolean tree model preserves the original predicate contract and all immutable legacy cases',()=>{
  assert.deepEqual(rustConstFunctionFixture(),oldFixture);
  assert.equal(oldFixture.cases.length,3);assert.equal(oldFixture.invalid.length,27);
  const conditional = oldFixture.invalid.find(entry => entry.name === 'conditional');
  assert.ok(conditional);
  assert.throws(() => checkProgram(parseRust(conditional.source,{constFunctionModel:oldFixture.model})),error => {
    assert.equal(error.reason,conditional.reason);assert.deepEqual(error.span,conditional.span);return true;
  });
  const tree = checkProgram(parseRust(conditional.source,{constFunctionModel:fixture.model}));
  assert.equal(tree.items[0].body.k,'if');
  assert.equal(tree.items[0].sourceConstFunction.model,fixture.model);
  assert.equal(tree.items[0].sourceConstFunction.interpretation.inputDomain,'primitive Boolean values');
  assert.notEqual(tree.items[0].sourceConstFunction.interpretation.inputDomain,oldFixture.scope.inputDomain);
  assert.throws(() => createTypedProgramLinks(conditional.source,'Rust'));
});

test('actual complete qualified Boolean trees execute every truth row after source-free text and binary reload',()=>{
  assert.deepEqual(rustConstBranchFixture(),fixture);
  assert.equal(fixture.cases.length,4);
  for (const entry of fixture.cases) {
    assert.equal(createHash('sha256').update(entry.source).digest('hex'),entry.sourceSha256);
    assert.ok(entry.source.includes(entry.definition));assert.match(entry.definition,/^pub const fn /u);
    const program = load(entry), fn = program.items[0];
    assert.equal(fn.body.k,'if');assert.deepEqual(fn.sourceConstFunction,entry.proof);assert.deepEqual(fn.sourceSignature,entry.signature);
    assert.equal(entry.source.slice(entry.proof.keywordSpan.start,entry.proof.keywordSpan.end),'const');
    // Ordinary translation is compared on its actual primitive Boolean input
    // domain; only the selected const model promises foreign-value guards.
    observeTruthRows(emitJavaScript(checkProgram(parseRust(entry.ordinary+'\nfn main(){}'))).text,entry);
    observe(entry.javascript,entry);
    const packet = archives(program);
    for (const network of packet.networks) {
      assert.deepEqual(decodeTypedProgramLinks(network,packet.root),program);
      observe(emitTypedProgramLinks(network,packet.root,'JavaScript').code,entry);
      const rust = emitTypedProgramLinks(network,packet.root,'Rust');
      assert.ok(rust.code.includes(entry.rustDefinition));
    }
    let definitions = [entry.rustDefinition];
    for (let cycle=0;cycle<3;cycle++) {
      const actualSource = definitions.join('\n\n')+'\nfn main(){}';
      const replay = checkProgram(parseRust(actualSource,{constFunctionModel:fixture.model}));
      observe(emitJavaScript(replay).text,entry);
      const emitted = emitRust(replay);assert.deepEqual(emitted.definitions,definitions);definitions = emitted.definitions;
      assert.throws(() => parseRust(emitted.text,{constFunctionModel:fixture.model}),'the complete generated runtime shell is a separate source interpretation');
    }
  }
});

test('source-free condition and branch edits change the actual Boolean results while owned headers survive',()=>{
  const entry = fixture.cases[0];
  for (const [edit,expected] of [
    [body => {body.cond={k:'unary',op:'not',arg:body.cond,type:{kind:'bool'}};},[false,true,true,false]],
    [body => {body.then={k:'unary',op:'not',arg:body.then,type:{kind:'bool'}};},[true,false,true,false]],
  ]) {
    const program = load(entry), fn = program.items[0], proof = structuredClone(fn.sourceConstFunction), signature = structuredClone(fn.sourceSignature);
    edit(fn.body);
    const packet = archives(program);
    for (const network of packet.networks) {
      const replay = decodeTypedProgramLinks(network,packet.root);
      assert.deepEqual(replay.items[0].sourceConstFunction,proof);assert.deepEqual(replay.items[0].sourceSignature,signature);
      observe(emitTypedProgramLinks(network,packet.root,'JavaScript').code,entry,expected);
      const rust = emitRust(replay);assert.match(rust.definitions[0],/^pub const fn /u);assert.notEqual(rust.definitions[0],entry.rustDefinition);
      observe(emitJavaScript(checkProgram(parseRust(rust.definitions.join('\n\n')+'\nfn main(){}',{constFunctionModel:fixture.model}))).text,entry,expected);
    }
  }
});

test('Boolean tree interfaces, operations, owned fields and both reachable and unreachable branches fail closed',()=>{
  assert.equal(fixture.invalid.length,17);
  for (const entry of fixture.invalid) assert.throws(() => createTypedProgramLinks(entry.source,'Rust',{rustConstFunctionModel:fixture.model}),error => {
    assert.equal(error.kind,entry.kind,entry.name);assert.equal(error.reason??error.message,entry.reason,entry.name);assert.deepEqual(error.span??null,entry.span,entry.name);return true;
  });
  const changes = [
    fn => {fn.body.cond.type={kind:'float'};},fn => {fn.body.then.type={kind:'float'};},fn => {fn.body.else.type={kind:'float'};},
    fn => {fn.body.cond.name='foreign';},fn => {fn.body.borrowEscape=false;},fn => {fn.body.else.opaqueBinding=false;},
    fn => {fn.body.cond={k:'call',fn:fn.fullName,args:[],type:{kind:'bool'}};},
    fn => {fn.body.then={k:'binary',op:'eq',left:fn.body.then,right:fn.body.then,type:{kind:'bool'}};},
    fn => {fn.sourceConstFunction.model='unknown';},fn => {fn.sourceConstFunction.interpretation=structuredClone(oldFixture.scope);},
    fn => {fn.sourceSignature.parameters[0].owner.path=['CustomBool'];},fn => {fn.generated=null;},
    fn => {fn.params[0].optional=false;},fn => {fn.body.span={start:0,end:1};},
  ];
  for (const change of changes) {
    const program = load(fixture.cases[0]);change(program.items[0]);
    for (const action of [()=>encodeTypedProgramLinks(program),()=>emitRust(program),()=>emitJavaScript(program)]) {
      assert.throws(action,/const|primitive|signature|unrepresented|owned|source/u);
    }
  }
  for (const field of ['cond','then','else']) {
    const program = load(fixture.cases[0]);let reads=0;
    Object.defineProperty(program.items[0].body,field,{get(){reads++;return {k:'lit',value:true,type:{kind:'bool'}};},enumerable:true});
    assert.throws(() => encodeTypedProgramLinks(program),/unrepresented/u);assert.equal(reads,0);
  }
});

test('the complete Boolean branch policy and all five primitive decisions are generated without changing the old module count',()=>{
  assert.deepEqual(readProjectData(new URL('../js/src/translation/data/rust-const-branches.lino',import.meta.url)),rustConstBranchPolicy);
  assert.equal(readFileSync(new URL('../js/src/translation/rust-const-branch-rules.js',import.meta.url),'utf8'),rustConstBranchRules());
  const source = rustConstBranchRules();
  assert.equal((source.match(/^export function /gmu)??[]).length,5);
  assert.ok(source.includes('"and"'));assert.ok(source.includes('"or"'));
  assert.ok(!source.includes('"eq"'));assert.ok(!source.includes('"float"'));
  const legacy = readFileSync(new URL('../js/src/translation/rust-const-function-rules.js',import.meta.url),'utf8');
  assert.equal((legacy.match(/^export function /gmu)??[]).length,9);
  const program = checkProgram(parseJavaScript(source));assert.equal(program.items.length,5);
  const packet = archives(program);
  for (const network of packet.networks) {
    const context = {};
    vm.runInNewContext(emitTypedProgramLinks(network,packet.root,'JavaScript').code,context,{timeout:1000});
    for (const model of [fixture.model,oldFixture.model,'','unknown']) assert.equal(context.acceptRustConstBranchModel(model),model===fixture.model);
    for (const kind of ['bool','float','unit','fixed','foreign']) assert.equal(context.acceptRustConstBranchType(kind),kind==='bool');
    for (const kind of ['lit','var','unary','binary','if','call','let','abort']) assert.equal(context.acceptRustConstBranchKind(kind),['lit','var','unary','binary','if'].includes(kind));
    for (const kind of ['unary','binary','if','call']) for (const operation of ['not','and','or','eq','ne','foreign']) {
      assert.equal(context.acceptRustConstBranchOperation(kind,operation),kind==='unary'&&operation==='not'||kind==='binary'&&(operation==='and'||operation==='or'));
    }
    for (const result of ['bool','float']) for (const condition of ['bool','float']) for (const consequent of ['bool','float']) for (const alternative of ['bool','float']) {
      assert.equal(context.acceptRustConstBranchConditional(result,condition,consequent,alternative),[result,condition,consequent,alternative].every(kind=>kind==='bool'));
    }
  }
});

test('raw source-free Boolean tree qualification and construct edits refuse before both target emitters',()=>{
  for (const [label,original,changed] of [['model',fixture.model,'unknown const model'],['construct kind','if','call']]) {
    const entry = fixture.cases[0], network = LinkNetwork.fromLino(entry.links);
    const row = network.fieldRelations().find(row => row.label===label&&network.link(row.child).metadata().definition===original);
    assert.ok(row,'the mutation targets an actual owned '+label+' relation');
    const replacement = network.insertLink([],network.link(row.child).metadata().withDefinition(changed));
    row.link.setReferences([row.parent,row.link.references()[1],replacement]);
    const restored = LinkNetwork.fromLino(network.toLino());
    for (const target of ['Rust','JavaScript']) assert.throws(() => emitTypedProgramLinks(restored,entry.root,target),/const|primitive|unrepresented|construct|expression/u);
  }
});
