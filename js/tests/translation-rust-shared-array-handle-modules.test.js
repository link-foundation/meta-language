import assert from 'node:assert/strict';
import test from 'node:test';
import {selfTranslate, selfTranslationSignatures} from '../src/self-translation.js';
import {rustSharedArrayHandleRulesPolicy} from '../scripts/generate-rust-shared-array-handle-rules.mjs';
const model = 'standard Rust shared array identity handles 1';
const options = {rustSharedArrayHandleSourceModel: model};
const handle = 'std::rc::Rc<std::cell::RefCell<Vec<bool>>>';
const make = 'std::rc::Rc::new(std::cell::RefCell::new(vec![true]))';
const policy = rustSharedArrayHandleRulesPolicy();
const [major,minor] = process.versions.node.split('.').map(Number);
assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node + '; ' + policy.assertionVersions.Node);
let serial = 0;
const load = code => import('data:text/javascript,' + encodeURIComponent(code) + '#' + serial++);

test('whole shared-handle module publication binds real forward siblings and retains private visibility', async () => {
  const source = `// λ: complete original source\r\npub fn probe()->bool{let value=fresh();let alias=value.clone();std::rc::Rc::ptr_eq(&value,&alias)}\r\nfn fresh()->${handle}{${make}}`;
  const translated = selfTranslate(source, 'Rust', 'JavaScript', options);
  assert.ok(translated.items.length > 2);
  assert.ok(translated.items.every(item => item.status === 'translated'));
  const module = await load(translated.code);
  assert.deepEqual(Object.keys(module), ['probe']);
  assert.equal(module.probe(), true);
  assert.equal(selfTranslate(translated.code, 'JavaScript', 'Rust').code, source);
  assert.deepEqual(selfTranslationSignatures(source, 'Rust', options), [{k:'fn',name:'probe',params:[],ret:{kind:'bool'}}]);
  assert.equal(selfTranslate(source,'Rust','Rust',options).code,source);
  assert.throws(() => selfTranslationSignatures(source,'Rust'));
});

test('complete actual JavaScript publications translate semantically back through all original identity cases', async () => {
  for (const row of policy.cases) {
    const rust = selfTranslate(row.source,'JavaScript','Rust',{referenceSemantics:true});
    assert.ok(rust.items.every(item => item.status === 'translated' || item.status === 'comment'));
    const reverse = selfTranslate(rust.code,'Rust','JavaScript',options);
    assert.ok(reverse.items.every(item => item.status === 'translated'));
    const module = await load(reverse.code);
    assert.equal(module[row.call](),row.expected,row.name);
    assert.equal(selfTranslate(reverse.code,'JavaScript','Rust').code,rust.code);
  }
});

test('shared-handle signatures retain array domains and only actual public owners', async () => {
  const source=`pub fn identity(value:${handle})->${handle}{private_identity(value.clone())} pub(crate) fn private_identity(value:${handle})->${handle}{value.clone()}`;
  const signatures=selfTranslationSignatures(source,'Rust',options);
  assert.equal(signatures.length,1);
  assert.equal(signatures[0].name,'identity');
  assert.deepEqual(signatures[0].ret,{kind:'array',element:{kind:'bool'},reference:true});
  assert.deepEqual(signatures[0].params[0].type,signatures[0].ret);
  const module=await load(selfTranslate(source,'Rust','JavaScript',options).code);
  assert.deepEqual(Object.keys(module),['identity']);
  const value=[true];assert.equal(module.identity(value),value);
});

test('whole source refusals prevent publishing a supported fragment of an unsupported Rust module', () => {
  const supported=`pub fn identity(value:${handle})->${handle}{value.clone()}`;
  for (const source of [
    supported+' use custom::Owner;',
    supported+' mod std{}',
    supported+` fn invalid(value:${handle})->${handle}{value}`,
    supported+' fn main(){}',
    supported+' fn duplicate()->bool{true}fn duplicate()->bool{false}',
    supported+' fn broken(',
  ]) {
    assert.throws(()=>selfTranslate(source,'Rust','JavaScript',options),source);
    assert.throws(()=>selfTranslationSignatures(source,'Rust',options),source);
  }
  assert.throws(()=>selfTranslate(supported,'Rust','JavaScript',{rustSharedArrayHandleSourceModel:'unknown'}));
  assert.throws(()=>selfTranslate('function probe(){return true;}','JavaScript','Rust',options));
  assert.throws(()=>selfTranslate(' '.repeat(65537),'Rust','JavaScript',options));
  for (const extra of [{moduleDirectory:['nested']},{imports:{'./outside.js':[]}},{utf16Strings:true},{referenceSemantics:true}]) {
    assert.throws(()=>selfTranslate(supported,'Rust','JavaScript',{...options,...extra}));
  }
});

test('whole checked source owns behavior even when old provenance describes a different body', async () => {
  const original=selfTranslate('function probe(){return true;}','JavaScript','Rust');
  assert.ok(original.code.includes('true'));
  const changed=original.code.replace(/(pub fn probe\(\) -> bool \{\s*)true/u,'$1false');
  assert.notEqual(changed,original.code);
  const reverse=selfTranslate(changed,'Rust','JavaScript',options);
  const module=await load(reverse.code);
  assert.equal(module.probe(),false,'actual complete Rust code wins over source comments');
  assert.equal(selfTranslate(reverse.code,'JavaScript','Rust').code,changed);
});
