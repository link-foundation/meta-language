import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {format} from 'node:util';
import {runInNewContext} from 'node:vm';
import test from 'node:test';
import {readArrayRelativeElement} from '../src/translation/array-relative-element-rules.js';
import {generateArrayRelativeElementTemplate} from '../scripts/generate-array-relative-element-template.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {selfTranslate} from '../src/self-translation.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

const source=`/** @param {number[]} values @param {number|null|undefined} position @returns {number|undefined} */
function relativeElement(values,position){return values.at(position);}
/** @param {string[]} values @param {number} position @returns {string|undefined} */
function relativeText(values,position){return values.at(position);}
/** @param {boolean[]} values @param {number} position @returns {boolean|undefined} */
function relativeBoolean(values,position){return values.at(position);}`;
const arrays=[[],[0],[1,2,3],[-0,NaN,Infinity,-Infinity]];
const positions=[-Infinity,-9007199254740992,-5,-4,-3,-2,-1.9,-1,-0.9,-Number.MIN_VALUE,-0,0,Number.MIN_VALUE,0.9,1,1.9,2,3,4,9007199254740992,Infinity,NaN];

test('relative array selection preserves all numeric boundaries primitive element types and undefined values',()=>{
  const compiled=runInNewContext(emitJavaScript(checkProgram(parseJavaScript(source))).text+';({relativeElement,relativeText,relativeBoolean});');
  for(const values of arrays)for(const position of positions){
    assert.ok(Object.is(readArrayRelativeElement(values,position),values.at(position)));
    assert.ok(Object.is(compiled.relativeElement(values,position),values.at(position)));
  }
  for(const position of positions){
    assert.equal(compiled.relativeText(['A','😀','\ud800'],position),['A','😀','\ud800'].at(position));
    assert.equal(compiled.relativeBoolean([true,false,true],position),[true,false,true].at(position));
  }
  for(const position of [null,undefined])assert.equal(compiled.relativeElement([5,6],position),5);
});

test('relative array defaults and operands execute once in order and unsupported coercions stay explicit',()=>{
  const ordered=`/** @returns {number[]} */function receiver(){console.log('receiver');return [10,20,30];}
/** @returns {number} */function position(){console.log('position');return -1.9;}
console.log(receiver().at(position()));console.log(receiver().at());
console.log('%s %s %s',[1].at(undefined),[2].at(null),[3].at(NaN));`;
  for(const text of [ordered,emitJavaScript(checkProgram(parseJavaScript(ordered))).text]){
    const lines=[];runInNewContext(text,{console:{log:(...values)=>lines.push(format(...values))}});
    assert.deepEqual(lines,['receiver','position','30','receiver','10','1 2 3']);
  }
  for(const argument of ['true',"'1'",'{}','[1]'])assert.throws(()=>checkProgram(parseJavaScript('console.log([1,2].at('+argument+'));')));
  assert.throws(()=>checkProgram(parseJavaScript('console.log([1,2].at(0,1));')),/expects 0 to 1 arguments/u);
});

test('relative array source free typed text and binary graphs recheck and preserve every decision',()=>{
  const packet=createTypedProgramLinks(source,'JavaScript');
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
    recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
    const code=emitTypedProgramLinks(network,packet.root,'JavaScript').code;
    const operation=runInNewContext(code+';relativeElement;');
    for(const values of arrays)for(const position of positions)assert.ok(Object.is(operation(values,position),values.at(position)));
    assert.ok(emitTypedProgramLinks(network,packet.root,'Rust').code.includes('optional_at'));
  }
});

test('the complete original relative array kernel is checked shared data and self translation restores every byte',()=>{
  generateArrayRelativeElementTemplate(true);
  const kernel=readFileSync(new URL('../src/translation/array-relative-element-rules.js',import.meta.url),'utf8');
  for(const input of [source,kernel]){
    const translated=selfTranslate(input,'JavaScript','Rust');
    assert.equal(translated.items.filter(row=>row.status==='carried').length,0);
    assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,input);
  }
});

import {readProjectData} from '../scripts/project-data.mjs';


test('independent complete array programs preserve nullable elements and ordered observations',()=>{
  const fixture=readProjectData(new URL('../../parity/fixtures/array-relative-element-observations.lino',import.meta.url));
  assert.equal(fixture.programs.length,2);assert.equal(fixture.positions.length,22);
  for(const row of fixture.programs){
    for(const source of [row.source,emitJavaScript(checkProgram(parseJavaScript(row.source))).text]){
      const lines=[];runInNewContext(source,{console:{log:(...values)=>lines.push(format(...values))}});
      assert.equal(lines.join('\n')+'\n',row.expectedStdout,row.id);
    }
  }
});
