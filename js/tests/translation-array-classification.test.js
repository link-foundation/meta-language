// Every normative source is independently executed before translation evidence.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readJavaScriptArrayClassification,readJavaScriptArrayClassificationPlan} from '../src/translation/array-classification-rules.js';
import {generateJavaScriptIdentifierModels} from '../scripts/generate-javascript-identifier-models.mjs';
const policy=readProjectData(new URL('../src/translation/data/array-classification-rules.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+policy.assertionVersions.Node);
const printed=source=>{const lines=[];vm.runInNewContext(source,{console:{log:value=>lines.push(String(value))}},{timeout:1000});return lines.join('\n')+(lines.length?'\n':'');};
const observedError=source=>{try{printed(source);}catch(error){return{name:error.name,message:error.message};}assert.fail('the retained actual source must fail');};
const archives=packet=>[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];
function checkedNodes(program){const nodes=[],seen=new WeakSet();function walk(value){
  if(!value||typeof value!=='object'||seen.has(value))return;seen.add(value);
  if(typeof value.k==='string'&&value.type)nodes.push(value);
  for(const child of value instanceof Map?value.values():Object.values(value))walk(child);
}walk(program);return nodes;}

test('shared array decisions retain every closed kind optional presence and unsupported domain vector',()=>{
  for(const row of policy.decisionVectors){
    assert.equal(readJavaScriptArrayClassification(row.kind),row.kind==='array'||row.kind==='tuple');
    assert.deepEqual(readJavaScriptArrayClassificationPlan(row.kind,row.element),row.expected,JSON.stringify(row));
  }
});

test('every actual array predicate operand stays single evaluated through both checked Links codecs',()=>{
  for(const row of policy.cases){
    assert.equal(printed(row.source),row.expected,row.name+' independent Node');
    const checked=checkProgram(parseJavaScript(row.source)),packet=encodeTypedProgramLinks(checked);
    const originalRust=emitRust(checked).text;
    for(const program of [checked,...archives(packet).map(network=>recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}))]){
      assert.equal(printed(emitJavaScript(program).text),row.expected,row.name);
      assert.equal(emitRust(program).text,originalRust);
      const lets=checkedNodes(program).filter(node=>node.k==='let'&&node.name.startsWith('ml_array_test'));
      assert.equal(lets.length,[...row.source.matchAll(/Array\.isArray\(/gu)].length,'one existing checked Let per actual predicate argument');
      for(const node of lets){assert.equal(node.type.kind,'bool');assert.ok(node.value&&node.body);
        if(node.body.k==='truthy'){assert.equal(node.body.arg.k,'var');assert.equal(node.body.arg.name,node.name);assert.equal(node.body.arg.type.kind,'option');}}
      assert.equal(checkedNodes(program).some(node=>node.k==='arrayIsArray'),false,'checking uses only existing Let Boolean and Truthy nodes');
    }
  }
});

test('optional arrays retain present true absent false and all original argument effects',()=>{
  const row=policy.cases.find(row=>row.name==='single evaluation and both optional array absence policies');assert.ok(row);
  assert.equal(printed(row.source),row.expected);
  const program=checkProgram(parseJavaScript(row.source));
  assert.equal(printed(emitJavaScript(program).text),row.expected);
  assert.equal(checkedNodes(program).filter(node=>node.k==='truthy'&&node.arg?.type?.kind==='option').length,4);
});

test('an argument failure remains observable before any static predicate result',()=>{
  for(const row of policy.throwing){
    assert.deepEqual(observedError(row.source),{name:row.errorName,message:row.message});
    const checked=checkProgram(parseJavaScript(row.source)),packet=encodeTypedProgramLinks(checked);
    for(const program of [checked,...archives(packet).map(network=>recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}))])
      assert.deepEqual(observedError(emitJavaScript(program).text),{name:row.errorName,message:row.message});
  }
});

test('unrepresented unions proxies arities and shadowed owners retain independent source oracles and explicit refusals',()=>{
  for(const row of policy.refusals){
    if(row.errorName)assert.equal(observedError(row.source).name,row.errorName,row.name);
    else assert.equal(printed(row.source),row.expected,row.name+' independent Node');
    assert.throws(()=>checkProgram(parseJavaScript(row.source)),row.name);
  }
});

test('the exact current module context must prove the intrinsic Array owner',()=>{
  const source='function probe(){return Array.isArray([1]);}console.log(probe());';assert.equal(printed(source),'true\n');
  for(const context of [{moduleBindingNames:['Array'],moduleScopeKnown:true},{moduleBindingNames:[],moduleScopeKnown:false},
    {externals:[{k:'const',name:'Array',type:{kind:'float'}}]},
    {literalBindings:new Map([['Array',{k:'num',value:'0',type:{kind:'float'}}]])}])
    assert.throws(()=>checkProgram(parseJavaScript(source,context)),'an actual local/module/context owner cannot become the Array intrinsic');
});


test('source-free optional array ownership rejects forged result types and bound-variable owners through both codecs',()=>{
  const row=policy.cases.find(row=>row.name==='single evaluation and both optional array absence policies');
  const original=checkProgram(parseJavaScript(row.source)),packet=encodeTypedProgramLinks(original);
  for(const network of archives(packet)){
    assert.ok(network.links().every(link=>!link.metadata().definition?.includes(row.source)),'the replay contains no original program text');
    const restored=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
    assert.equal(printed(emitJavaScript(restored).text),row.expected);
    for(const change of [
      value=>{value.body.type={kind:'string'};},
      value=>{value.body.arg.name='unbound_array_classification_owner';},
      value=>{value.body.arg.type={kind:'string'};},
      value=>{value.body.unrepresentedPolicy=true;},
    ]){
      const changed=structuredClone(restored);
      const bound=checkedNodes(changed).find(node=>node.k==='let'&&node.name.startsWith('ml_array_test')&&node.body.k==='truthy');assert.ok(bound);
      change(bound);assert.throws(()=>recheckLinkedTypedProgram(changed,{wholeProgram:true}));
    }
  }
});

test('the complete emitted JavaScript reparses and retains each original array oracle through both codecs',()=>{
  const options={identifierModel:'ordinary JavaScript binding identifiers 1'};
  for(const row of policy.cases){
    assert.equal(printed(row.source),row.expected,row.name+' independent original Node oracle');
    const original=checkProgram(parseJavaScript(row.source));
    const defaultEmitted=emitJavaScript(original).text;assert.equal(printed(defaultEmitted),row.expected);
    assert.throws(()=>parseJavaScript(defaultEmitted),error=>error.reason?.startsWith('reserved identifier: '),row.name+' unchanged default reserved-name refusal');
    const emitted=emitJavaScript(original,options).text;assert.equal(printed(emitted),row.expected);
    const reverse=checkProgram(parseJavaScript(emitted)),packet=encodeTypedProgramLinks(reverse);
    const rust=emitRust(reverse).text;
    for(const program of [reverse,...archives(packet).map(network=>recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}))]){
      assert.equal(printed(emitJavaScript(program,options).text),row.expected,row.name);
      assert.equal(emitRust(program).text,rust,row.name);
    }
  }
});


test('ordinary binding identifiers keep actual module local parameter and nested capture collisions distinct',()=>{
  const options={identifierModel:'ordinary JavaScript binding identifiers 1'};
  const cases=[
    {source:'function binding_ml_array_test1(){return 19;}function probe(binding_ml_array_test1){const binding_ml_array_test1_2=17;console.log(binding_ml_array_test1);console.log(binding_ml_array_test1_2);return Array.isArray([binding_ml_array_test1]);}console.log(binding_ml_array_test1());console.log(probe(7));',expected:'19\n7\n17\ntrue\n',fixedNumberPrelude:true},
    {source:"function probe(binding_ml_array_test1){return [binding_ml_array_test1].map(binding_ml_array_test1_2=>Array.isArray([binding_ml_array_test1_2])).join('|');}console.log(probe(3));",expected:'true\n',fixedNumberPrelude:false},
  ];
  const reverseOracle=(emitted,expected)=>{
    const reverse=checkProgram(parseJavaScript(emitted)),reversePacket=encodeTypedProgramLinks(reverse),reverseRust=emitRust(reverse).text;
    for(const restored of[reverse,...archives(reversePacket).map(network=>recheckLinkedTypedProgram(decodeTypedProgramLinks(network,reversePacket.root),{wholeProgram:true}))]){
      assert.equal(printed(emitJavaScript(restored,options).text),expected);assert.equal(emitRust(restored).text,reverseRust);
    }
  };
  for(const row of cases){
    assert.equal(printed(row.source),row.expected,'independent original Node oracle');
    const original=checkProgram(parseJavaScript(row.source)),packet=encodeTypedProgramLinks(original),rust=emitRust(original).text;
    for(const program of[original,...archives(packet).map(network=>recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}))]){
      const before=emitRust(program).text,emitted=emitJavaScript(program,options).text;
      assert.equal(printed(emitted),row.expected);assert.equal(emitRust(program).text,before);assert.equal(before,rust,'target naming does not change the actual checked program');
      if(row.fixedNumberPrelude){
        // >=1.0.0 <1.0.1 has a separate fixed Number formatter using Object.is.
        // Preserve this complete original program and all its oracles; the
        // naming profile does not admit that unrepresented prelude operation.
        assert.equal(readProjectData(new URL('../package.json',import.meta.url)).version,'1.0.0','uncovered package version; >=1.0.0 <1.0.1 required for this exact fixed-prelude refusal');
        assert.match(emitted,/function ml_showNumber\(value\)/u);
        assert.throws(()=>parseJavaScript(emitted),error=>error.reason==='Object.is: the JavaScript standard library is outside the portable core'&&error.span.start===195&&error.span.end===216);
      }else reverseOracle(emitted,row.expected);
    }
  }
  const variants=[
    {original:cases[0].source,expected:'true\ntrue\ntrue\ntrue\n',edits:[
      ['function binding_ml_array_test1(){','/** @returns {number} */function binding_ml_array_test1(){'],
      ['function probe(binding_ml_array_test1){','/** @param {number} binding_ml_array_test1 @returns {boolean} */function probe(binding_ml_array_test1){'],
      ['console.log(binding_ml_array_test1);','console.log(binding_ml_array_test1===7);'],
      ['console.log(binding_ml_array_test1_2);','console.log(binding_ml_array_test1_2===17);'],
      ['console.log(binding_ml_array_test1());','console.log(binding_ml_array_test1()===19);'],
    ]},
    {original:cases[1].source,expected:'true\n',edits:[['function probe(binding_ml_array_test1){','/** @param {number} binding_ml_array_test1 @returns {string} */function probe(binding_ml_array_test1){']]},
  ];
  for(const row of variants){
    let source=row.original;for(const[before,after]of row.edits){assert.equal(source.split(before).length,2);source=source.replace(before,after);}
    let inverse=source;for(const[before,after]of row.edits.toReversed()){assert.equal(inverse.split(after).length,2);inverse=inverse.replace(after,before);}assert.equal(inverse,row.original);
    assert.equal(printed(source),row.expected,'independent annotated numeric collision oracle');
    const program=checkProgram(parseJavaScript(source)),emitted=emitJavaScript(program,options).text;assert.equal(printed(emitted),row.expected);
    reverseOracle(emitted,row.expected);
  }
});

test('identifier naming keeps the old default and narrow model while refusing unknown selections and raw reserved source names',()=>{
  const model=generateJavaScriptIdentifierModels({check:true}).model;
  const program=checkProgram(parseJavaScript(policy.cases[0].source)),before=encodeTypedProgramLinks(program).network.toLino();
  const original=emitJavaScript(program);assert.deepEqual(emitJavaScript(program,{identifierModel:undefined}),original);
  emitJavaScript(program,{identifierModel:model});assert.equal(encodeTypedProgramLinks(program).network.toLino(),before,'only target names change');
  for(const identifierModel of['','unregistered identifier model',null,false,1])assert.throws(()=>emitJavaScript(program,{identifierModel}),/JavaScript identifier model/u);
  assert.throws(()=>emitJavaScript(program,{scopedBindingModel:'owned JavaScript scoped bindings 1'}),/JavaScript scoped binding emission/u,'the original closed scope model still refuses this array program');
  assert.throws(()=>emitJavaScript(program,{identifierModel:model,scopedBindingModel:'owned JavaScript scoped bindings 1'}),/identifier model composition/u);
  assert.throws(()=>parseJavaScript('function probe(ml_source_name){return ml_source_name;}'),error=>error.reason?.startsWith('reserved identifier: ml_source_name'));
});
