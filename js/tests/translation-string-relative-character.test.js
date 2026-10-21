import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {runInNewContext} from 'node:vm';
import {format} from 'node:util';
import test from 'node:test';
import {readStringRelativeCharacter} from '../src/translation/string-relative-character-rules.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {selfTranslate} from '../src/self-translation.js';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {readStringRelativeCharacterTemplateGeneration} from '../scripts/generate-string-relative-character-template.mjs';

const source=`/** @param {string} text @param {number} position @returns {string|undefined} */
function relativeCharacter(text,position){return text.at(position);}
/** @param {string} text @param {number|null|undefined} position @returns {string|undefined} */
function nullableRelativeCharacter(text,position){return text.at(position);}`;
const texts=['','ABC','\u0000\uffff','A😀B','\ud800x\udfff','\ud800','\udfff'];
const positions=[-Infinity,-9007199254740992,-5,-4,-3,-2,-1.9,-1,-0.9,-Number.MIN_VALUE,-0,0,Number.MIN_VALUE,0.9,1,1.9,2,3,4,9007199254740992,Infinity,NaN];

test('relative character translation preserves every UTF16 unit negative index and undefined result',()=>{
  const program=checkProgram(parseJavaScript(source));
  const compiled=runInNewContext(emitJavaScript(program).text+';({relativeCharacter,nullableRelativeCharacter});');
  for(const text of texts)for(const position of positions){
    const expected=text.at(position);
    assert.equal(readStringRelativeCharacter(text,position),expected);
    assert.equal(compiled.relativeCharacter(text,position),expected);
    assert.equal(compiled.nullableRelativeCharacter(text,position),expected);
  }
  for(const position of [null,undefined])assert.equal(compiled.nullableRelativeCharacter('ABC',position),'A');
  assert.equal(compiled.relativeCharacter('😀',-2),'\ud83d');
  assert.equal(compiled.relativeCharacter('😀',-1),'\ude00');
  assert.equal(compiled.relativeCharacter('ABC',3),undefined);
  assert.equal(compiled.relativeCharacter('ABC',-4),undefined);
});

test('relative character defaults and effects retain caller ownership and explicit conversion refusals',()=>{
  const ordered=`/** @returns {string} */function receiver(){console.log('receiver');return 'ABC';}
/** @returns {number} */function position(){console.log('position');return -1.9;}
console.log(receiver().at(position()));console.log(receiver().at());
console.log('%s %s %s','ABC'.at(undefined),'ABC'.at(null),'ABC'.at(NaN));`;
  for(const text of [ordered,emitJavaScript(checkProgram(parseJavaScript(ordered))).text]){
    const lines=[];runInNewContext(text,{console:{log:(...values)=>lines.push(format(...values))}});
    assert.deepEqual(lines,['receiver','position','C','receiver','A','A A A']);
  }
  const collision=`/** @param {string} text @param {number} position @param {number} Math @param {number} Number @returns {string|undefined} */
function collision(text,position,Math,Number){const readStringRelativeCharacter=999;return text.at(position);}`;
  const translated=runInNewContext(emitJavaScript(checkProgram(parseJavaScript(collision))).text+';collision;');
  assert.equal(translated('ABC',-1,8,9),'C');
  for(const position of ['true','false',"'1'",'{}','[1]'])assert.throws(()=>checkProgram(parseJavaScript("console.log('ABC'.at("+position+'));')));
  assert.throws(()=>checkProgram(parseJavaScript("console.log('ABC'.at(0,1));")),/expects 0 to 1 arguments/u);
  assert.throws(()=>checkProgram(parseJavaScript('/** @returns {string} */function wrong(){return "".at(0);}')));
  const array='/** @param {number[]} values @returns {number|undefined} */function last(values){return values.at(-1);}';
  // Keep the original array input. The complete relative-array source lambda
  // now implements its ECMAScript 2022 behavior; retain the old source in history.
  const arrayRead=runInNewContext(emitJavaScript(checkProgram(parseJavaScript(array))).text+';last;');
  assert.equal(arrayRead([1,2,3]),[1,2,3].at(-1));
  assert.equal(arrayRead([]),[].at(-1));
});

test('relative character complete linked archives recheck and retain source free target behavior',()=>{
  const packet=createTypedProgramLinks(source,'JavaScript');
  const native=emitRust(checkProgram(parseJavaScript(source))).text;
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
    const restored=decodeTypedProgramLinks(network,packet.root);recheckLinkedTypedProgram(restored,{wholeProgram:true});
    assert.equal(JSON.stringify(restored.items).includes('"op":"at"'),false);
    const javascript=emitTypedProgramLinks(network,packet.root,'JavaScript').code;
    assert.equal(emitTypedProgramLinks(network,packet.root,'Rust').code,native);
    const read=runInNewContext(javascript+';relativeCharacter;');
    for(const text of texts)for(const position of positions)assert.equal(read(text,position),text.at(position));
  }
});

test('the relative character policy retains the complete source and self translation restores every byte',()=>{
  const policy=readStringRelativeCharacterTemplateGeneration();
  assert.equal(policy.source,readFileSync(new URL('../src/translation/string-relative-character-rules.js',import.meta.url),'utf8'));
  assert.deepEqual(policy.languageVersions,{language:'ECMAScript',minimum:'2022',maximumExclusive:'2027'});
  for(const file of ['js/src/translation/data/string-relative-character-template.lino','rust/src/data/string-relative-character-template.lino'])assert.deepEqual(readProjectData(new URL('../../'+file,import.meta.url)),policy);
  const translated=selfTranslate(source,'JavaScript','Rust');
  assert.equal(translated.items.filter(row=>row.status==='carried').length,0);
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source);
});

test('the independent relative character table executes every original input and rejects forged return meaning',()=>{
  const fixture=readProjectData(new URL('../../parity/fixtures/string-relative-character-observations.lino',import.meta.url));
  const program=checkProgram(parseJavaScript(fixture.source));
  const read=runInNewContext(emitJavaScript(program).text+';relativeCharacter;');
  assert.equal(fixture.cases.length,fixture.units.length*fixture.positions.length);
  for(const row of fixture.cases){
    const text=String.fromCharCode(...fixture.units[row.text]),position=Number(fixture.positions[row.position]);
    for(const value of [text.at(position),read(text,position)]){
      assert.equal(value===undefined,row.undefined);
      assert.deepEqual(value===undefined?[]:Array.from({length:value.length},(_,index)=>value.charCodeAt(index)),row.units);
    }
  }
  const packet=createTypedProgramLinks(fixture.source,'JavaScript');
  const forged=decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()),packet.root);
  assert.equal(forged.items[0].body.k,'apply');assert.equal(forged.items[0].body.fn.k,'lambda');
  forged.items[0].body.fn.type.ret={kind:'string'};
  assert.throws(()=>recheckLinkedTypedProgram(forged,{wholeProgram:true}));
});

test('relative character source history retains every whole preceding policy module and assertion input',()=>{
  const root=new URL('../../',import.meta.url),revision='9ac3038132953e6a220bba6f06a3e6147d39836f';
  const file='parity/self-translation/translation-source-history.lino';
  const policy=readProjectData(new URL(file,root));
  const {relativeCharacterSourceSuccessors,...previous}=policy;
  const original=execFileSync('git',['show',revision+':'+file],{cwd:root,maxBuffer:2*1024*1024});
  assert.ok(Buffer.from(serializeProjectData(previous,file)).equals(original));
  assert.equal(relativeCharacterSourceSuccessors.length,2);
  for(const row of relativeCharacterSourceSuccessors){
    const bytes=Buffer.from(restoreArrayRelativeElementSource(readFileSync(new URL(row.file,root),'utf8'),row.file));
    assert.equal(bytes.length,row.current.bytes);
    assert.equal(createHash('sha256').update(bytes).digest('hex'),row.current.sha256);
    let restored=bytes,limit=bytes.length;
    for(const edit of row.edits.toReversed()){
      assert.ok(edit.start>=0&&edit.end>=edit.start&&edit.end<=limit);
      assert.equal(restored.subarray(edit.start,edit.end).toString(),edit.currentText);
      restored=Buffer.concat([restored.subarray(0,edit.start),Buffer.from(edit.previousText),restored.subarray(edit.end)]);
      limit=edit.start;
    }
    assert.equal(restored.toString(),row.previous.sourceText);
    assert.equal(restored.length,row.previous.bytes);
    assert.equal(createHash('sha256').update(restored).digest('hex'),row.previous.sha256);
    assert.ok(restored.equals(execFileSync('git',['show',revision+':'+row.file],{cwd:root,maxBuffer:1024*1024})));
  }
});

test('relative character provenance refresh preserves all recorded definitions and four complete predecessor files',()=>{
  const root=new URL('../../',import.meta.url);
  const receipt=readProjectData(new URL('parity/self-translation/relative-character-descriptor-refresh.lino',root));
  const original=(file)=>execFileSync('git',['show',receipt.revision+':'+file],{cwd:root,maxBuffer:4*1024*1024});
  const assertOriginal=(value,descriptor)=>{
    const bytes=Buffer.from(serializeProjectData(value,descriptor.file));
    assert.equal(bytes.length,descriptor.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),descriptor.sha256);
    assert.ok(bytes.equals(original(descriptor.file)));
  };
  assert.equal(receipt.changes.length,6);
  const artifact=restoreArrayRelativeElementProjectData(readProjectData(new URL(receipt.currentArtifact.file,root)),receipt.currentArtifact.file),previous=structuredClone(artifact);
  for(const change of receipt.changes){
    assert.equal(change.path.length>1,true);assert.ok(['bytes','sha256'].includes(change.path.at(-1)));
    let owner=previous;for(const key of change.path.slice(0,-1))owner=owner[key];
    assert.equal(owner[change.path.at(-1)],change.after);owner[change.path.at(-1)]=change.before;
  }
  assertOriginal(previous,receipt.previousArtifact);
  const policy=restoreArrayRelativeElementProjectData(readProjectData(new URL(receipt.previousPolicy.file,root)),receipt.previousPolicy.file),oldPolicy=receipt.previousPolicy.value;
  assertOriginal(oldPolicy,receipt.previousPolicy);
  const inverse=structuredClone(policy);inverse.edge.current=oldPolicy.edge.current;inverse.edge.changes=oldPolicy.edge.changes;
  assert.deepEqual(inverse,oldPolicy);
  assert.deepEqual(policy.edge.changes.map(({kind,path,before})=>({kind,path,before})),oldPolicy.edge.changes.map(({kind,path,before})=>({kind,path,before})));
  const register=restoreArrayRelativeElementProjectData(readProjectData(new URL(receipt.previousRegister.file,root)),receipt.previousRegister.file);
  register.sourceDescriptorSuccessors=[oldPolicy.edge];assertOriginal(register,receipt.previousRegister);
  const loan=restoreArrayRelativeElementProjectData(readProjectData(new URL(receipt.previousLoan.file,root)),receipt.previousLoan.file);
  loan.predecessor=receipt.previousLoan.predecessor;assertOriginal(loan,receipt.previousLoan);
});

import {restoreArrayRelativeElementSource,restoreArrayRelativeElementProjectData} from '../scripts/array-relative-element-history.mjs';
