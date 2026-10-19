import assert from 'node:assert/strict';
import { test } from 'node:test';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { format } from 'node:util';
import { runInNewContext } from 'node:vm';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { array } from '../src/translation/types.js';
import { typeError, unsupported } from '../src/translation/diagnostics.js';
import { acceptOptionalPrimitiveKind } from '../src/translation/optional-value-rules.js';
import { checkCollectionConstruction, inferCollectionConstruction, readCollectionConstructionContext } from '../src/translation/collection-construction.js';
import * as rules from '../src/translation/collection-construction-rules.js';
import { readCollectionConstructionSourceModule } from '../scripts/generate-collection-construction-rules.mjs';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';

// Exact former production branches, captured before extraction. These are
// independent retained oracles; no historical Git objects are needed by CI.
const originalBranches={
  "infer": "      case 'set': {\n        const element = expected?.kind === 'set' ? expected.element : this.fresh();\n        if (node.items) this.unify(this.expr(node.items, env, { kind: 'array', element }), { kind: 'array', element }, node.span);\n        const term = { kind: 'set', element };\n        this.collections.set(node, term);\n        return term;\n      }\n      case 'map': {\n        const key = expected?.kind === 'map' ? expected.key : this.fresh();\n        const value = expected?.kind === 'map' ? expected.value : this.fresh();\n        const pair = { kind: 'tuple', elements: [key, value] };\n        if (node.entries) this.unify(this.expr(node.entries, env, { kind: 'array', element: pair }), { kind: 'array', element: pair }, node.span);\n        const term = { kind: 'map', key, value };\n        this.collections.set(node, term);\n        return term;\n      }\n",
  "check": "      case 'set': {\n        const want = node.type ? this.resolveType(node.type, path, node.span) : expected;\n        const items = this.expr(node.items, env, path, want?.kind === 'set' ? array(want.element) : undefined);\n        if (items.type.kind !== 'array') throw typeError('Set constructor needs an iterable array', node.span);\n        if (!primitiveKey(items.type.element)) throw unsupported('Set object identity', 'set keys need explicit reference identities for structured values', node.span);\n        return { k: 'set', items, type: { kind: 'set', element: items.type.element } };\n      }\n      case 'map': {\n        const want = node.type ? this.resolveType(node.type, path, node.span) : expected;\n        const pair = want?.kind === 'map' ? { kind: 'tuple', elements: [want.key, want.value] } : undefined;\n        const entries = this.expr(node.entries, env, path, pair ? array(pair) : undefined);\n        if (entries.type.kind !== 'array' || entries.type.element.kind !== 'tuple' || entries.type.element.elements.length !== 2) throw typeError('Map constructor needs an iterable of key/value pairs', node.span);\n        const [key, value] = entries.type.element.elements;\n        if (!primitiveKey(key)) throw unsupported('Map object identity', 'map keys need explicit reference identities for structured values', node.span);\n        return { k: 'map', entries, type: { kind: 'map', key, value } };\n      }\n",
  "Rust": "      case 'set':\n      case 'map': {\n        this.usesArray = true;\n        this.usesStandardArray = true;\n        const key = e.k === 'set' ? e.type.element : e.type.key;\n        const comparator = key.kind === 'float' ? '|a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())' : '|a, b| a == b';\n        let values = this.expr(e.k === 'set' ? e.items : e.entries);\n        if (key.kind === 'float') values = `(${values}).into_iter().map(|ml_value| ${e.k === 'set' ? 'if ml_value == 0f64 { 0f64 } else { ml_value }' : '(if ml_value.0 == 0f64 { 0f64 } else { ml_value.0 }, ml_value.1)'}).collect::<Vec<_>>()`;\n        return `crate::ml_array::${e.k}(${values}, ${comparator})`;\n      }\n",
  "JavaScript": "      case 'set':\n        return `new Set(${this.expr(e.items)})`;\n      case 'map':\n        return `new Map(${this.expr(e.entries)})`;\n",
  "Lean": "      case 'set':\n      case 'map': {\n        const set = e.k === 'set';\n        const keyType = set ? e.type.element : e.type.key;\n        const value = set ? 'ml_value' : 'ml_value.1';\n        const previous = set ? 'ml_previous' : 'ml_previous.1';\n        const equal = keyType.kind === 'float' ? `(${previous} == ${value} || (${previous}.isNaN && ${value}.isNaN))` : `(${previous} == ${value})`;\n        const normalized = keyType.kind === 'float' ? set ? '(if ml_value == (0 : Float) then (0 : Float) else ml_value)' : '((if ml_value.1 == (0 : Float) then (0 : Float) else ml_value.1), ml_value.2)' : 'ml_value';\n        const values = this.expr(set ? e.items : e.entries, depth);\n        const existing = set ? 'ml_accumulator' : `(ml_accumulator.map (fun ml_previous => if ${equal} then (ml_previous.1, ml_value.2) else ml_previous))`;\n        return `(${values}.foldl (fun ml_accumulator ml_original => let ml_value := ${normalized.replaceAll('ml_value', 'ml_original')}; if ml_accumulator.any (fun ml_previous => ${equal}) then ${existing} else ml_accumulator.push ml_value) (#[] : ${this.type(e.type)}))`;\n      }\n",
  "Rocq": "      case 'set':\n      case 'map': {\n        const set = e.k === 'set';\n        const keyType = set ? e.type.element : e.type.key;\n        const value = set ? 'ml_value' : '(fst ml_value)';\n        const previous = set ? 'ml_previous' : '(fst ml_previous)';\n        let equal = this.comparison('eq', keyType, previous, value);\n        if (keyType.kind === 'float') equal = `(orb ${equal} (andb (PrimFloat.is_nan ${previous}) (PrimFloat.is_nan ${value})))`;\n        const normalized = keyType.kind === 'float' ? set ? '(if PrimFloat.eqb ml_original 0%float then 0%float else ml_original)' : '((if PrimFloat.eqb (fst ml_original) 0%float then 0%float else fst ml_original), snd ml_original)' : 'ml_original';\n        const values = this.expr(set ? e.items : e.entries);\n        const existing = set ? 'ml_accumulator' : `(List.map (fun ml_previous => if ${equal} then (fst ml_previous, snd ml_value) else ml_previous) ml_accumulator)`;\n        const element = set ? this.type(e.type.element) : `(${this.type(e.type.key)} * ${this.type(e.type.value)})%type`;\n        return `(List.fold_left (fun ml_accumulator ml_original => let ml_value := ${normalized} in if List.existsb (fun ml_previous => ${equal}) ml_accumulator then ${existing} else List.app ml_accumulator (ml_value :: nil)) ${values} (@nil ${element}))`;\n      }\n",
  "LeanType": "      case 'set':\n        return `(Array ${this.type(type.element)})`;\n      case 'map':\n        return `(Array (${this.type(type.key)} \u00d7 ${this.type(type.value)}))`;\n",
  "RocqType": "      case 'set':\n        return `(list ${this.type(type.element)})`;\n      case 'map':\n        return `(list (${this.type(type.key)} * ${this.type(type.value)}))%type`;\n"
};
const originalSources=[
  "/** @param {Map<string,number>|null|undefined} value @returns {string} */function inspect(value){return typeof value;}console.log('%s',inspect(null),inspect(undefined),inspect(new Map([['kept',2]])));",
  "/** @param {{item:number}|null|undefined} value @returns {string} */function recordType(value){return typeof value;}\n/** @param {[number,string]|null|undefined} value @returns {string} */function tupleType(value){return typeof value;}\n/** @param {Map<string,number>|null|undefined} value @returns {string} */function mapType(value){return typeof value;}\n/** @param {Set<number>|null|undefined} value @returns {string} */function setType(value){return typeof value;}\nconsole.log('%s',recordType(null),recordType(undefined),recordType({item:2}));\nconsole.log('%s',tupleType(null),tupleType(undefined),tupleType([2,'kept']));\nconsole.log('%s',mapType(null),mapType(undefined),mapType(new Map([['kept',2]])));\nconsole.log('%s',setType(null),setType(undefined),setType(new Set([2])));\nfunction read(value){console.log('read');const n=value?.item??0;return value;}function describe(value){return typeof read(value);}console.log('%s',describe(null));console.log('%s',describe(undefined));console.log('%s',describe({item:2}));",
  "/** @param {{item:number}} value @returns {string} */\nfunction recordType(value) { return typeof value; }\n/** @param {[number,string]} value @returns {string} */\nfunction tupleType(value) { return typeof value; }\n/** @param {Map<string,number>} value @returns {string} */\nfunction mapType(value) { return typeof value; }\n/** @param {Set<number>} value @returns {string} */\nfunction setType(value) { return typeof value; }\nconsole.log('%s', recordType({item:2}), tupleType([2,'kept']), mapType(new Map([['kept',2]])), setType(new Set([2])));"
];
const plain = value => JSON.parse(JSON.stringify(value));
const number = { kind: 'float' }, text = { kind: 'string' };
function observe(source) {
  const lines=[];
  runInNewContext(source, {console:{log(...values){lines.push(format(...values));}}}, {timeout:1000});
  return lines.join('\n')+'\n';
}
function oldBranch(name, parameters) {
  return runInNewContext('(function('+parameters+'){switch('+(name==='infer'||name==='check'?'node.k':name.endsWith('Type')?'type.kind':'e.k')+'){'+originalBranches[name]+'}})',
    {array,typeError,unsupported,primitiveKey:type=>acceptOptionalPrimitiveKind(type.kind)});
}

test('collection inference retains the complete original constructor constraints and actual expected owners', () => {
  const original=oldBranch('infer','node,env,expected');
  for(const kind of ['set','map']) for(const expected of [undefined,{kind:'option',element:number,absence:'both'},
    {kind:'set',element:text},{kind:'map',key:text,value:number}]) for(const present of [false,true]) {
    const child={k:'fixture'},node={k:kind,span:{start:2,end:8},...(present?{[kind==='set'?'items':'entries']:child}:{})};
    const make=()=>({trace:[],next:0,collections:new Map(),fresh(){const term={kind:'var',id:this.next++};this.trace.push(['fresh',term]);return term;},
      expr(value,env,want){this.trace.push(['expr',value,want]);return want;},unify(a,b,where){this.trace.push(['unify',a,b,where]);}});
    const before=make(),after=make();
    const a=original.call(before,node,new Map(),expected),b=inferCollectionConstruction(after,node,new Map(),expected);
    assert.deepEqual(plain([b,after.trace]),plain([a,before.trace]));
    assert.equal(after.collections.get(node),b);
    if(expected?.kind===kind){assert.equal(kind==='set'?b.element:b.key,kind==='set'?expected.element:expected.key);if(kind==='map')assert.equal(b.value,expected.value);}
  }
  const expected={kind:'map',key:text,value:number};
  const selected=readCollectionConstructionContext('map',expected);
  assert.equal(selected.first,text);assert.equal(selected.second,number);
});

test('collection checked admission preserves all original shapes diagnostics and child evaluation order', () => {
  const original=oldBranch('check','node,env,path,expected');
  for(const kind of ['set','map']) for(const type of [number,array(number),array({kind:'record',fields:[]}),
    array({kind:'tuple',elements:[text,number]}),array({kind:'tuple',elements:[text]}),array({kind:'tuple',elements:[text,number,text]}),
    array({kind:'tuple',elements:[{kind:'record',fields:[]},number]})]) for(const declared of [false,true]) {
    const node={k:kind,[kind==='set'?'items':'entries']:{k:'fixture',type},span:{start:3,end:7},
      ...(declared?{type:kind==='set'?{kind,element:number}:{kind,key:text,value:number}}:{})};
    const make=()=>({trace:[],resolveType(value){this.trace.push(['resolve',value]);return value;},expr(value,env,path,want){this.trace.push(['expr',value,want]);return value;}});
    const run=(fn,host)=>{try{return {value:plain(fn(host)),trace:plain(host.trace)};}catch(error){return {error:{kind:error.kind,message:error.message,details:error.details,span:error.span},trace:plain(host.trace)};}};
    const a=run(host=>original.call(host,node,new Map(),[],undefined),make());
    const b=run(host=>checkCollectionConstruction(host,node,new Map(),[],undefined),make());
    assert.deepEqual(b,a);
  }
  const messages=['','Set constructor needs an iterable array','Map constructor needs an iterable of key/value pairs',
    'set keys need explicit reference identities for structured values','map keys need explicit reference identities for structured values'];
  for(const code of [-1,-0,0,1,2,3,4,5,NaN,Infinity])for(const part of ['message','construct','', 'future']) {
    const message=Number.isInteger(code)&&code>=1&&code<=4?messages[code]:'';
    const construct=code===3?'Set object identity':code===4?'Map object identity':'';
    assert.equal(rules.readCollectionConstructionDiagnostic(code,part),part==='message'?message:part==='construct'?construct:'');
  }
});

test('all four collection templates and formal type strings equal the frozen original branches', () => {
  for(const target of ['JavaScript','Rust','Lean','Rocq']) for(const kind of ['set','map']) for(const keyKind of ['float','string','bool','int','fixed']) {
    const key={kind:keyKind,code:'KEY'},value={kind:'float',code:'VALUE'},type=kind==='set'?{kind,element:key}:{kind,key,value};
    const e={k:kind,type,items:{code:'values()'},entries:{code:'values()'}};
    const receiver={expr:item=>item.code,type:item=>item===type?'COLLECTION':item.code,comparison:()=> 'original_equal'};
    const expected=oldBranch(target,'e,depth').call(receiver,e,0);
    const element=kind==='set'?'KEY':'(KEY * VALUE)%type';
    const actual=rules.renderCollectionConstruction(target,kind,keyKind,'values()',target==='Lean'?'COLLECTION':'',target==='Rocq'?element:'',target==='Rocq'?'original_equal':'');
    assert.equal(actual,expected,`${target}/${kind}/${keyKind}`);
    assert.equal(actual.split('values()').length,2,'constructor input is evaluated once');
    if(target==='Lean'||target==='Rocq')assert.equal(rules.renderCollectionConstructionType(target,kind,'KEY','VALUE'),oldBranch(target+'Type','type').call(receiver,type));
  }
  const javascript=rules.renderCollectionConstruction('JavaScript','set','float','[NaN,NaN,-0,0,2]','','','');
  const set=runInNewContext(javascript);assert.equal(set.size,3);assert.equal(Object.is([...set][1],0),true);assert.equal(Number.isNaN([...set][0]),true);
  const map=runInNewContext(rules.renderCollectionConstruction('JavaScript','map','string',"[['first',1],['second',2],['first',3]]",'','',''));
  assert.deepEqual(plain([...map]),[['first',3],['second',2]]);
});

test('the three complete native failure inputs keep actual values effects and both full checked archives', () => {
  const originals=readFileSync(new URL('../../rust/tests/unit/translation_optional_values.rs',import.meta.url),'utf8');
  const defaults=readFileSync(new URL('../../rust/tests/unit/translation_defaults.rs',import.meta.url),'utf8');
  const expected=['object undefined object\n','object undefined object\n'.repeat(4)+'read\nobject\nread\nundefined\nread\nobject\n','object object object object\n'];
  for(const[index,source]of originalSources.entries()) {
    assert.ok((index===2?defaults:originals).includes(JSON.stringify(source)),'the complete original native input is retained verbatim');
    assert.equal(observe(source),expected[index]);
    const checked=checkProgram(parseJavaScript(source));
    const emitted=emitJavaScript(checked).text;assert.equal(observe(emitted),expected[index]);
    for(const[target,emit]of [['Rust',emitRust],['Lean',emitLean],['Rocq',emitRocq]])assert.ok(emit(checked).text.length>0,target);
    const packet=encodeTypedProgramLinks(checked);
    for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
      const restored=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
      assert.equal(emitJavaScript(restored).text,emitted);assert.equal(observe(emitJavaScript(restored).text),expected[index]);
      assert.equal(emitRust(restored).text,emitRust(checked).text);
    }
  }
  const effects="function values(){console.log('build');return [2,2];}function inspect(value){return typeof value;}console.log(inspect(new Set(values())));";
  assert.equal(observe(effects),'build\nobject\n');
  assert.equal(observe(emitJavaScript(checkProgram(parseJavaScript(effects))).text),'build\nobject\n');
});

test('the complete construction source closure self-translates without dropping any required reader or owner', () => {
  const source=readCollectionConstructionSourceModule();
  const surface=parseJavaScript(source,{referenceSemantics:false}),checked=checkProgram(surface);
  const owners=[...checked.declarations.values()];
  assert.equal(surface.items.filter(item=>item.k==='fn'&&!item.generated).length,16,'all ten original construction definitions and six required readers remain');
  assert.equal(owners.length,surface.items.length,'every original and generated owner remains in the checked module');
  for(const name of Object.keys(rules))assert.ok(checked.declarations.get(name),'the actual complete export remains: '+name);
  for(const[target,emit]of [['JavaScript',emitJavaScript],['Rust',emitRust],['Lean',emitLean],['Rocq',emitRocq]]) {
    const emitted=emit(checked);
    for(const owner of owners)assert.equal(emitted.mappings.filter(mapping=>mapping.kind===(owner.k==='fn'?'function':owner.k)&&mapping.source===owner.fullName).length,1,`${target}/${owner.fullName}`);
    assert.ok(emitted.text.length>source.length/2);
  }
  const emitted=emitJavaScript(checked),javascript=emitted.text;
  const owner=checked.declarations.get('renderCollectionConstruction');
  const target=emitted.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===owner.fullName).target;
  const result=runInNewContext(javascript+'\n'+target+"('JavaScript','map','string','[]','','','');",{}, {timeout:1000});
  assert.equal(result,'new Map([])');
  const packet=encodeTypedProgramLinks(checked),expected=emitRust(checked).text;
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
    const restored=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
    assert.equal(emitRust(restored).text,expected);
  }
  const pins={infer:'d876d44e6eed0597a88a6345bc1c96162c0c004b5842f6faa3546b6d38505967',check:'15846257b1f91cf8094c2e51322505611cc7adcc159c11a23c5535c9907334f8',
    Rust:'9d7d395b1fac905a4455c472eeb81939b33e4d1781a457744913425498dd22e1',JavaScript:'7b0bd63e09ce0aa72bb077aa0cdce0b9daaab036e2c0d1e739c59b830b59df5b',
    Lean:'59d6c5001f117178abc22962f9063fdb4ec22fefe8512452a013cbc3064e4294',Rocq:'c03707727d898497d151101c8ff4e748f6bef2e6ee5b3afbc1ffa8eae58dc267',
    LeanType:'9486a33aa80386139e9a424d30c7f012b50abea95805fac0f1ec51e2b07dc2ec',RocqType:'dee5165d99ba906167dec639e892f5fae7c419d5dfb25e69dd8a32f0beacd294'};
  for(const[name,pin]of Object.entries(pins))assert.equal(createHash('sha256').update(originalBranches[name]).digest('hex'),pin,name+': complete frozen branch bytes');
});
