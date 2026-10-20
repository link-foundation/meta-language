import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {stripTypeScriptTypes} from 'node:module';
import test from 'node:test';
import {eraseTypeScript,requiresTypeScriptErasure} from '../src/typescript-erasure.js';
import {parseProgrammingLanguage} from '../src/programming-language-parser.js';
import {selfTranslate} from '../src/self-translation.js';
import {tokenize} from '../src/translation/lexer.js';
import {readProjectData} from '../scripts/project-data.mjs';
import {serializeProjectData} from '../scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';

const oracleRanges=['>=22.15.0 <23.0.0','>=24.0.0 <25.0.0'];
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,
  'Uncovered Node type-stripping oracle version '+process.versions.node+'; supported: '+oracleRanges.join(' or '));
const significantTokens=source=>tokenize(source,'JavaScript').tokens.map(({kind,value,raw})=>({kind,value,raw}));
const fixture=readProjectData(new URL('../../parity/fixtures/typescript-erasure.lino',import.meta.url));
const names=new Set(['mixed-type-import','mixed-type-import-last','mixed-type-import-middle',
  'mixed-type-import-default','mixed-type-import-all-types','mixed-type-import-comments',
  'mixed-type-import-unicode-lines','runtime-import-named-type','ordinary-named-import','type-only-import']);
const expectedLines={
  'mixed-type-import':['entry loaded'],
  'mixed-type-import-last':['entry loaded','3'],
  'mixed-type-import-middle':['entry loaded','8'],
  'mixed-type-import-default':['entry loaded','10'],
  'mixed-type-import-all-types':['entry loaded'],
  'mixed-type-import-comments':['entry loaded','3'],
  'mixed-type-import-unicode-lines':['entry loaded','3'],
  'runtime-import-named-type':['entry loaded','11'],
  'ordinary-named-import':['entry loaded','8'],
  'type-only-import':[],
};

function executeImports(code) {
  // Separate processes give the oracle and implementation independent module
  // caches and the same actual module-evaluation side effects. No files or
  // external packages are created, resolved or downloaded.
  const dependency='console.log("entry loaded"); export default 7; export const value=3,other=5,type=11;';
  const script=`import {registerHooks} from 'node:module';
    registerHooks({resolve(name,context,next){
      if(name==='./entry.js')return{url:${JSON.stringify('data:text/javascript,'+encodeURIComponent(dependency))},shortCircuit:true};
      return next(name,context);
    }});
    await import(${JSON.stringify('data:text/javascript,'+encodeURIComponent(code))});`;
  const child=spawnSync(process.execPath,['--max-old-space-size=96','--input-type=module','--eval',script],
    {encoding:'utf8',timeout:5000,env:{...process.env,NODE_OPTIONS:'--max-old-space-size=96'}});
  assert.equal(child.status,0,child.stderr);assert.equal(child.signal,null);
  return child.stdout.trim().split('\n').filter(Boolean);
}

test('mixed import erasure matches the active bounded Node oracle and actual module evaluation',()=>{
  const cases=fixture.cases.filter(({id})=>names.has(id));assert.equal(cases.length,names.size);
  for(const entry of cases){
    const actual=eraseTypeScript(entry.source),oracle=stripTypeScriptTypes(entry.source,{mode:'strip'});
    assert.deepEqual(actual,entry.erased,entry.id);
    assert.deepEqual(significantTokens(actual.code),significantTokens(oracle),entry.id);
    const observed=executeImports(actual.code);
    assert.deepEqual(observed,executeImports(oracle),entry.id);
    assert.deepEqual(observed,expectedLines[entry.id],entry.id);
    assert.equal(observed.includes('entry loaded'),entry.id!=='type-only-import',entry.id);
    for(const erasure of actual.erasures){
      assert.ok(erasure.start>=0&&erasure.end<=Buffer.byteLength(entry.source));
      assert.ok(Buffer.from(entry.source).subarray(erasure.start,erasure.end).toString().length>0);
    }
    const translated=selfTranslate(entry.source,'TypeScript','JavaScript');
    const expectedStatus=actual.erasures.length===0?'kept':'translated';
    assert.ok(translated.items.every(({status})=>status===expectedStatus),entry.id);
    assert.equal(selfTranslate(translated.code,'JavaScript','TypeScript').code,entry.source,entry.id);
  }
});

test('ordinary named imports including a runtime binding spelled type remain unchanged',()=>{
  for(const id of ['runtime-import-named-type','ordinary-named-import']){
    const entry=fixture.cases.find(row=>row.id===id);
    assert.equal(requiresTypeScriptErasure(parseProgrammingLanguage(entry.source,'TypeScript')),false);
    assert.deepEqual(eraseTypeScript(entry.source),{code:entry.source,erasures:[]});
    assert.equal(selfTranslate(entry.source,'TypeScript','JavaScript').code,entry.source);
  }
});

test('type-only specifier erasure never hides malformed import descendants',()=>{
  for(const source of ['import { type Entry as } from "./entry.js";',
    'import { type Entry,, value } from "./entry.js";']){
    assert.throws(()=>stripTypeScriptTypes(source,{mode:'strip'}));
    assert.throws(()=>eraseTypeScript(source));
  }
});

test('the complete preceding fixture and every original input remain certified after the implemented extension',()=>{
  const history=readProjectData(new URL('../../parity/fixtures/typescript-named-import-expectation-history.lino',import.meta.url));
  assert.equal(history.originalCases.length,13);assert.equal(history.appended.length,8);
  for(const old of history.originalCases)assert.equal(fixture.cases.find(row=>row.id===old.id)?.source,old.source);
  assert.deepEqual(fixture.cases.slice(13).map(row=>row.id),history.appended);
  assert.equal(history.previousCase.error.reason,'TypeScript import_specifier: mixed type imports need a linked separator reconstruction rule');
  const inverse={...fixture,cases:fixture.cases.slice(0,13).map(row=>row.id===history.previousCase.id?history.previousCase:row)};
  verifyGitArtifactCertificate(history.certificate,Buffer.from(serializeProjectData(inverse,history.path)),{
    revision:'15745ffcd322c48bdd537ccb48a5f7c9deb96917',path:'parity/fixtures/typescript-erasure.lino'});
});
