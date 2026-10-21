// Complete source/input fixtures, not execution observations. The own-module
// bytes are read from their actual production owners by the normal author.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readCallableSourceModule} from './callable-source-module.mjs';
import {createSurfaceReferenceFacts} from '../src/translation/surface-reference-facts.js';

export const ownedRecursiveSources = [
  '/** @param {number} value @returns {boolean} */\nexport function first(value) { let remaining=value; let total=0; while(remaining>0) { remaining-=1; total+=1; } return remaining===0 && total===value && (value<=0 ? true : second(value-1)); }',
  '/** @param {number} value @returns {boolean} */\nfunction second(value) { let remaining=value; let total=0; while(remaining>0) { remaining-=1; total+=1; } return remaining===0 && total===value && (value<=0 ? false : first(value-1)); }',
];
export const ownedRecursiveCalls = ['first(0)','first(1)','first(2)','first(3)','first(4)','first(5)','first(6)','first(7)'];
export const ownedRecursiveExpected = 'true|false|true|false|true|false|true|false';

export function readOwnedCallableStatementFixture() {
  const name = value => ({k:'name',path:[value]});
  const read = value => ({s:'expr',expr:name(value)});
  return {bound:['shadow'],expectedFree:['first','second','hidden','third','condition','left','outer','right','tag','branch','again','loop','after','body','initial','next','element','record','key','field','initializer','captured'],expectedAssigned:['outer'],statements:[
    {s:'const',name:'local',value:name('first')},
    {s:'block',body:[{s:'let',name:'hidden',value:name('second')},read('hidden')]},
    read('hidden'),
    {s:'block',inline:true,body:[{s:'let',name:'inline',value:name('third')}]},read('inline'),
    {s:'if',cond:name('condition'),then:[{s:'assign',name:'outer',value:name('left')}],else:[read('right')]},
    {s:'switch',discriminant:name('tag'),clauses:[{body:[read('branch')]}]},
    {s:'while',cond:name('again'),body:[read('loop')]},
    {s:'doWhile',cond:name('after'),body:[read('body')]},
    {s:'for',init:[{s:'let',name:'at',value:name('initial')}],cond:name('at'),update:[{s:'assign',name:'at',value:name('next')}],body:[read('at'),read('element')]},
    {s:'assignReference',object:name('record'),index:name('key'),value:name('field')},
    {s:'return',expr:{k:'lambda',params:[{name:'shadow'}],body:{k:'let',name:'bound',value:name('initializer'),body:{k:'array',items:[name('bound'),name('shadow'),name('captured')]}}}},
  ]};
}

export function readOwnedRecursiveSourceFixtures() {
  const root = new URL('../../', import.meta.url);
  const paths = ['js/src/translation/fact-column-rules.js','js/src/translation/type-syntax-rules.js','js/src/translation/callable-source-rules.js'];
  const inputs = paths.map(path => {
    const source = readFileSync(new URL(path, root), 'utf8');
    return {path,source,bytes:Buffer.byteLength(source),sourceSha256:createHash('sha256').update(source).digest('hex')};
  });
  const statementFixture=readOwnedCallableStatementFixture();
  return {sources:ownedRecursiveSources,calls:ownedRecursiveCalls,rustCalls:ownedRecursiveCalls.map((_,index)=>'first('+index+'.0)'),expected:ownedRecursiveExpected,
    inputs,callableClosure:readCallableSourceModule(inputs[2].source),statementFixture,
    statementFacts:createSurfaceReferenceFacts(statementFixture.statements),
    versions:{Node:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0'},
    interpretation:'Complete original inputs and independent assertion declarations only. Actual JS/native execution is required before recording evidence.'};
}
