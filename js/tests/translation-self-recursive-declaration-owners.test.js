import assert from 'node:assert/strict';
import {test} from 'node:test';
import {runInNewContext} from 'node:vm';
import {readFileSync} from 'node:fs';
import {readRecursiveDeclarationSourceOwners as owners,acceptRecursiveDeclarationAuxiliaryClosure as closure} from '../src/translation/declaration-dependency-rules.js';
import {selfTranslate,selfTranslationSignatures} from '../src/self-translation.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {planOwnedRecursiveDeclarationPublication} from '../src/translation/recursive-declaration-ownership.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {createSurfaceReferenceFacts} from '../src/translation/surface-reference-facts.js';
import {readCallableStatementUses} from '../src/translation/callable-source-rules.js';
import {statementUses} from '../src/translation/javascript-lower.js';
import {readOwnedRecursiveSourceFixtures,ownedRecursiveSources,ownedRecursiveCalls,ownedRecursiveExpected} from '../scripts/declaration-owned-auxiliary-fixtures.mjs';
import {acceptRecursiveDeclarationPublication,acceptRecursiveDeclarationOwnedPublication,
  acceptRecursiveDeclarationOwnedEmission,acceptRecursiveDeclarationAuxiliaryCorrespondence} from '../src/translation/declaration-dependency-rules.js';

function versions() {
  const [major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
}
function output(source,calls) {
  return runInNewContext(source.replace(/^export /gmu,'')+'\n['+calls.join(',')+'].map(String).join("|");',{}, {timeout:1000});
}
function signatures(program,names) {
  return names.map(name=>{
    const entry=program.declarations.get(name);assert.equal(entry?.k,'fn');
    return {k:'fn',name,params:entry.params.map(({name,type})=>({name,type})),ret:entry.ret};
  });
}
function capture(source,names,externals=[]) {
  const program=checkProgram(parseJavaScript(source,{externals,referenceSemantics:false}));
  return {source,sourceLanguage:'JavaScript',program,
    emitted:emitRust(program,{arrayConsumption:true,recordDefinitionOwnership:true}),
    signatures:signatures(program,names),undecorated:true};
}
function captures(sources=ownedRecursiveSources,names=['first','second']) {
  const whole=capture(sources.join('\n\n'),names);
  const members=sources.map((source,index)=>capture(source,[names[index]],whole.signatures.filter(row=>row.name!==names[index])));
  return {whole,members,names,sources};
}
function plan(value) {return planOwnedRecursiveDeclarationPublication(value.names,value.sources,value.whole,value.members,true);}
function copy(value) {
  const cloneCapture=record=>({...record,program:structuredClone(record.program),
    emitted:{...structuredClone({...record.emitted}),definitions:[...record.emitted.definitions],preludes:[...record.emitted.preludes],
      definitionOwnership:structuredClone(record.emitted.definitionOwnership)}});
  return {...value,whole:cloneCapture(value.whole),members:value.members.map(cloneCapture)};
}
function archives(network) {return [LinkNetwork.fromLino(network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))];}

test('owned auxiliary publication preserves the original flat oracle and actual optional emitter contract',()=>{
  versions();
  assert.equal(acceptRecursiveDeclarationPublication(2,3,2,true,true,true),false);
  assert.equal(acceptRecursiveDeclarationOwnedPublication(2,3,2,2,[1,1,1,1,1,1,1]),true);
  for(const proofs of [[],[1,1,1,1,1,1],[1,1,1,1,1,1,0],[1,1,1,1,1,1,2]])
    assert.equal(acceptRecursiveDeclarationOwnedPublication(2,3,2,2,proofs),false);
  assert.equal(acceptRecursiveDeclarationOwnedPublication(2,2,2,2,[1,1,1,1,1,1,1]),false);
  assert.equal(acceptRecursiveDeclarationAuxiliaryCorrespondence([1,2,3],[1,2,3]),true);
  assert.equal(acceptRecursiveDeclarationAuxiliaryCorrespondence([1,2,3],[1,3,2]),false);
  assert.equal(acceptRecursiveDeclarationOwnedEmission([0,1,2],[0,0,1],
    [0,0,1,0,1,1,1,1,2,2,2,2],[0,0,0,1,0,1,2,0,2,2,1,3],[]),true);
  assert.equal(acceptRecursiveDeclarationOwnedEmission([0,1,2],[0,0,1],
    [0,0,1,0,1,1,1,1,2,2,2,2],[0,0,0,1,0,1,2,0,2,2,1,3],[3]),false);
  const value=captures();
  const plain=emitRust(value.whole.program,{arrayConsumption:true});
  assert.equal(Object.hasOwn(plain,'definitionOwnership'),false);
  assert.equal(Object.keys(value.whole.emitted).includes('definitionOwnership'),false);
  assert.equal(JSON.stringify(plain),JSON.stringify(value.whole.emitted));
  assert.deepEqual(plain.definitions,value.whole.emitted.definitions);
  const selected=plan(value);assert.ok(selected);
  assert.ok(value.whole.program.declarations.size>2);
  assert.ok([...value.whole.program.declarations.values()].some(entry=>entry.k==='data'&&entry.generated));
  assert.deepEqual(selected.indices.flat().sort((left,right)=>left-right),value.whole.emitted.definitions.map((_,index)=>index));
  assert.ok(selected.indices.every(row=>row.length>1),'each original owns real complete function/data blocks');
  const unicode=captures(['// café 🙂\r\n'+ownedRecursiveSources[0].replaceAll('\n','\r\n'),ownedRecursiveSources[1]]);
  assert.ok(plan(unicode),'actual UTF16 source owners do not reuse UTF8 group offsets');
});

test('actual whole/member helper spans layouts mappings targets and closure are mandatory',()=>{
  versions();const original=captures();assert.ok(plan(original));
  const mutate=change=>{const altered=copy(original);change(altered);assert.equal(plan(altered),null);};
  mutate(value=>{value.whole.emitted.definitions[2]+='\n';});
  mutate(value=>{value.whole.emitted.definitionOwnership.blocks[2].sourceSpan.start-=1;});
  mutate(value=>{value.whole.emitted.definitionOwnership.blocks[2].source='missing';});
  mutate(value=>{value.whole.emitted.definitionOwnership.blocks[2].mappingStart+=1;});
  mutate(value=>{value.whole.emitted.mappings.pop();});
  mutate(value=>{value.whole.emitted.definitionOwnership.blocks[2].definitionIndex=0;});
  mutate(value=>{value.whole.signatures[0].ret={kind:'string'};});
  mutate(value=>{value.members[0].source+=' ';});
  mutate(value=>{value.members[0].emitted.mappings[0].target+='2';});
  mutate(value=>{
    const block=value.whole.emitted.definitionOwnership.blocks[2];
    const target=value.whole.emitted.mappings[0].target;
    value.whole.emitted.mappings[block.mappingStart].target=target;block.targets[0]=target;
  });
  mutate(value=>{
    const helper=[...value.whole.program.declarations.values()].find(entry=>entry.generated&&entry.k==='fn');
    helper.span={start:0,end:1};
  });
  mutate(value=>{
    const datatype=[...value.members[0].program.declarations.values()].find(entry=>entry.generated&&entry.k==='data');
    datatype.ctors[0].fields.push({name:'extra',type:{kind:'bool'}});
  });
  mutate(value=>{
    const first=value.whole.program.declarations.get('first');
    first.body={k:'lit',value:true,type:{kind:'bool'}};
  });
  mutate(value=>{
    const helper=[...value.whole.program.declarations.values()].find(entry=>entry.generated&&entry.k==='fn');
    value.whole.program.declarations.get('first').body={k:'lit',value:helper.fullName,type:{kind:'string'}};
  });
  mutate(value=>{
    const declarations=[...value.whole.program.declarations.values()];
    const first=declarations.find(entry=>entry.generated&&entry.k==='fn'&&entry.span.start>value.whole.program.declarations.get('first').span.start&&entry.span.end<value.whole.program.declarations.get('first').span.end);
    const second=declarations.find(entry=>entry.generated&&entry.k==='fn'&&entry.span.start>value.whole.program.declarations.get('second').span.start&&entry.span.end<value.whole.program.declarations.get('second').span.end);
    assert.ok(first&&second);
    first.body={k:'call',fn:second.fullName,args:[],type:first.ret};
  });
  assert.equal(planOwnedRecursiveDeclarationPublication(original.names,original.sources,original.whole,original.members,false),null);
});

test('auxiliary external reservations retain the actual normal emitter targets',()=>{
  versions();
  const provider=checkProgram(parseJavaScript('/** @returns {number} */ export function externalSeed(){return 0;}'));
  const actualSignatures=signatures(provider,['externalSeed']);
  const names=['first','second'],sources=ownedRecursiveSources.map(source=>source.replace('let total=0;','let total=externalSeed();'));
  const whole=capture(sources.join('\n\n'),names,actualSignatures);
  const members=sources.map((source,index)=>capture(source,[names[index]],actualSignatures.concat(whole.signatures.filter(row=>row.name!==names[index]))));
  const value={whole,members,names,sources};assert.ok(plan(value));
  const changed=copy(value);changed.whole.emitted.definitionOwnership.externalTargets[0].target+='changed';
  assert.equal(plan(changed),null,'an external target cannot become a different forbidden identity');
});

test('complete cyclic loops publish coherent actual blocks and retain independent output and both semantic codecs',()=>{
  versions();const source=ownedRecursiveSources.join('\n\n');
  assert.equal(output(source,ownedRecursiveCalls),ownedRecursiveExpected);
  const translated=selfTranslate(source,'JavaScript','Rust');
  assert.deepEqual(translated.items.filter(item=>['function_declaration','export_statement'].includes(item.term)).map(item=>item.status),['translated','translated']);
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source);
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const saved=encodeTypedProgramLinks(program);
  for(const network of archives(saved.network)) {
    const restored=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,saved.root),{wholeProgram:true});
    assert.equal(output(emitJavaScript(restored).text,ownedRecursiveCalls),ownedRecursiveExpected);
    assert.equal(emitRust(restored,{arrayConsumption:true}).text,emitRust(program,{arrayConsumption:true}).text);
    const helper=[...restored.declarations.values()].find(entry=>entry.generated&&entry.k==='fn');
    helper.body.type={kind:'string'};
    assert.throws(()=>recheckLinkedTypedProgram(restored,{wholeProgram:true}));
  }
  for(const second of ['if(value<=0)return new Date();return first(value-1);','if(value<=0)return "wrong";return first(value-1);']) {
    const unavailable=ownedRecursiveSources[0]+'\n\nfunction second(value){'+second+'}\nexport function independent(){return 9;}';
    const result=selfTranslate(unavailable,'JavaScript','Rust');
    assert.deepEqual(result.items.filter(item=>['function_declaration','export_statement'].includes(item.term)).map(item=>item.status),['carried','carried','translated']);
    assert.deepEqual(selfTranslationSignatures(unavailable,'JavaScript').map(row=>row.name),['independent']);
  }
});

test('real callable-source module uses actual import signatures and executes complete original statement traversal',()=>{
  versions();const fixture=readOwnedRecursiveSourceFixtures();
  const [facts,types,callable]=fixture.inputs.map(row=>row.source);
  const factSignatures=selfTranslationSignatures(facts,'JavaScript',{utf16Strings:true});
  for(const name of ['readRequiredNumberFact','readRequiredTextFact','readRequiredBooleanFact']) assert.ok(factSignatures.some(row=>row.name===name),name);
  const typeSignatures=selfTranslationSignatures(types,'JavaScript',{utf16Strings:true,imports:{'./fact-column-rules.js':factSignatures}});
  assert.ok(typeSignatures.some(row=>row.name==='readTypeSeparatorPositions'));
  const options={utf16Strings:true,imports:{'./fact-column-rules.js':factSignatures,'./type-syntax-rules.js':typeSignatures}};
  const translated=selfTranslate(callable,'JavaScript','Rust',options);
  const bytes=Buffer.from(callable,'utf8');
  for(const name of ['readCallableStatementUses','callableStatementUses']) {
    const original=translated.items.find(item=>bytes.subarray(item.start,item.end).toString('utf8').includes('function '+name+'('));
    assert.equal(original?.status,'translated',name+': '+original?.reason);
  }
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,callable);
  const input=fixture.statementFixture;
  const factsOfStatements=createSurfaceReferenceFacts(input.statements,{retainValues:true});
  const events=readCallableStatementUses(factsOfStatements.forms,factsOfStatements.texts,factsOfStatements.starts,factsOfStatements.counts,
    factsOfStatements.labels,factsOfStatements.targets,factsOfStatements.root,input.bound);
  const names={free:new Set(),assigned:new Set()};
  for(let at=0;at<events.length;at+=2) {
    if(events[at]===0) names.free.add(factsOfStatements.texts[events[at+1]]);
    if(events[at]===1) names.assigned.add(factsOfStatements.texts[events[at+1]]);
  }
  const independent=statementUses(input.statements,new Set(input.bound));
  assert.deepEqual([...independent.free],input.expectedFree);
  assert.deepEqual([...independent.assigned],input.expectedAssigned);
  assert.deepEqual([...names.free],[...independent.free]);assert.deepEqual([...names.assigned],[...independent.assigned]);
  // Semantic codec scope is the two COMPLETE actual owners with actual
  // signatures of the independently checked original closure. This avoids
  // building another whole-kernel graph merely to inflate coverage claims.
  const checked=(()=>{
    const complete=checkProgram(parseJavaScript(fixture.callableClosure,{referenceSemantics:false}));
    const names=['readCallableStatementUses','callableStatementUses'];
    const sources=names.map(name=>{
      const owner=complete.declarations.get(name);assert.ok(owner?.span);
      const source=fixture.callableClosure.slice(owner.span.start,owner.span.end);
      assert.ok(source.includes('function '+name+'('),'the exact actual complete owner is retained');
      return source;
    });
    const externals=signatures(complete,[...complete.declarations.values()]
      .filter(entry=>entry.k==='fn'&&!entry.generated&&!names.includes(entry.name)).map(entry=>entry.name));
    return checkProgram(parseJavaScript(sources.join('\n\n'),{externals,referenceSemantics:false}));
  })();
  const saved=encodeTypedProgramLinks(checked);
  const call='readCallableStatementUses('+[factsOfStatements.forms,factsOfStatements.texts,factsOfStatements.starts,factsOfStatements.counts,
    factsOfStatements.labels,factsOfStatements.targets,factsOfStatements.root,input.bound].map(value=>JSON.stringify(value)).join(',')+')';
  for(const network of archives(saved.network)) {
    const restored=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,saved.root),{wholeProgram:true});
    const originalContext=fixture.callableClosure.replace(/^export /gmu,'');
    assert.deepEqual(Array.from(runInNewContext(originalContext+'\n'+emitJavaScript(restored).text+'\n'+call,{}, {timeout:1000})),events);
    assert.equal(emitRust(restored,{arrayConsumption:true}).text,emitRust(checked,{arrayConsumption:true}).text);
  }
  // Full immutable source, every body and actual imports remain test inputs.
  assert.equal(callable,readFileSync(new URL('../src/translation/callable-source-rules.js',import.meta.url),'utf8'));
});

// Retained complete inputs/assertions from the first unexecuted private planner draft.
test('auxiliary ownership uses real disjoint original spans with one exact root each', () => {
  const input = [[0, 50], [40, 90], [0, 50, 5, 5, 55], [40, 90, 30, 30, 80], [0, 0, 2, 1, 1]];
  assert.deepEqual(owners(...input), [0, 1, 0, 0, 1]);
  // Coincident loop fn/data intervals remain distinct, ordered actual declarations.
  for (const [column, index, value] of [[0,1,30], [1,0,0], [2,2,-1], [3,2,41],
    [2,2,0], [3,2,40], [2,0,1], [3,0,39], [4,2,-1], [4,2,3],
    [2,2,NaN], [3,2,Infinity], [2,2,5.5]]) {
    const altered = input.map(values => [...values]); altered[column][index] = value;
    assert.deepEqual(owners(...altered), [], `${column}:${index}:${value}`);
  }
  assert.deepEqual(owners([0],[40],[0],[40],[0]), [0], 'the same ownership rule checks an actual single-member capture');
  assert.deepEqual(owners([],[],[],[],[]), []);
  assert.deepEqual(owners([0,50],[40,90],[0,50,1],[40,90,39],[0,0,0]), []);
  assert.deepEqual(owners([0,50],[40,90],[0,55],[40,80],[0,1]), []);
  const sparse = [...input[2]]; delete sparse[2];
  assert.throws(() => owners(input[0], input[1], sparse, input[3], input[4]), /missing required Number fact/u);
});

test('auxiliary closure retains peer roots but rejects orphan and foreign helper edges', () => {
  // root0→root1/helperfn0/datatype0; helperfn0→datatype0/self;
  // root1→root0/helperfn1; helperfn1→self. Declaration order is real order.
  const actualOwners = [0,1,0,0,1], roles=[0,0,2,1,1];
  const starts=[0,3,5,5,7], counts=[3,2,0,2,1], targets=[1,3,2,0,4,2,3,4];
  assert.equal(closure(actualOwners, roles, starts, counts, targets), true);
  assert.equal(closure([0,0],[0,1],[0,1],[1,1],[1,1]), true, 'actual member capture root and its recursive loop helper');
  assert.equal(closure([],[],[],[],[]), false);
  const foreign = [...targets]; foreign[2]=4;
  assert.equal(closure(actualOwners, roles, starts, counts, foreign), false);
  const orphan = [...targets]; orphan[1]=0;
  assert.equal(closure(actualOwners, roles, starts, counts, orphan), false);
  assert.equal(closure([0,1,0,0,2], roles, starts, counts, targets), false);
  assert.equal(closure(actualOwners, [0,0,2,0,1], starts, counts, targets), false);
  assert.equal(closure(actualOwners, roles, starts, counts, [...targets,0]), false);
  assert.equal(closure(actualOwners, roles, starts, counts, [-1,...targets.slice(1)]), false);
});
