// Deferred controls only; never execute during the complete source draft.
import test from 'node:test';
import assert from 'node:assert/strict';
import {compileFunction} from 'node:vm';
import {readJavaScriptStringCaseProfileError,executeJavaScriptStringMethodPrimitive} from '../src/translation/javascript-string-method-rules.js';
import {compileJavaScriptStringMethodFunction} from '../src/translation/javascript-string-method-call-source.js';
import {captureJavaScriptOrdinaryClassModule,checkJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-source.js';
import {prepareJavaScriptOrdinaryClassModule} from '../src/translation/javascript-ordinary-class-module.js';
import {runJavaScriptSourceTransition} from '../src/node-source-modules.js';
import {javascriptStringCaseProfileVersions as versions,javascriptStringCaseProfileOriginals as originals} from './fixtures/javascript-string-case-profile.mjs';
const text=value=>({$:'text',units:Array.from({length:value.length},(_,at)=>value.charCodeAt(at))});
function requireActiveProfile(){
 assert.equal(versions.node,'>=24.0.0 <25.0.0');assert.equal(process.versions.node.split('.')[0],'24','uncovered native String casing Node range; select Node >=24 <25');assert.equal(process.versions.unicode,versions.unicode,'uncovered native String casing Unicode table profile');
}
test('String case profile selector accepts only the actual authored Unicode release',()=>{
 assert.equal(readJavaScriptStringCaseProfileError('16.0'),'');
 for(const profile of ['','15.0','15.1','16','17.0']){assert.match(readJavaScriptStringCaseProfileError(profile),/no authored table for active Unicode profile/);const result=executeJavaScriptStringMethodPrimitive('map lower String case',[text('I'),text(profile)]);assert.equal(result.$,'primitiveInvalid');assert.equal(result.reason,readJavaScriptStringCaseProfileError(profile));}
});
test('protected String case bodies retain explicit profile and string conversion order',()=>{
 for(const operation of ['String.toLowerCase','String.toUpperCase']){
  assert.equal(compileJavaScriptStringMethodFunction(1n,2n,operation,undefined,undefined),undefined);
  const fn=compileJavaScriptStringMethodFunction(1n,2n,operation,undefined,'16.0');assert.equal(fn.instructions[1].operation,'require String receiver');assert.equal(fn.instructions[2].operation,'convert primitive text');assert.deepEqual(fn.instructions[3].value,text('16.0'));assert.equal(fn.instructions[4].operation,operation==='String.toLowerCase'?'map lower String case':'map upper String case');assert.deepEqual(fn.instructions[4].arguments,[1,8]);
 }
});
test('String case primitive uses full expansion original sigma context supplementary and isolated units',()=>{
 for(const [operation,input,expected] of [['map upper String case','straße ﬃ','STRASSE FFI'],['map lower String case','İ','i\u0307'],['map lower String case','ΣΣ','σς'],['map lower String case','\u0345Σ','\u0345σ'],['map lower String case','ΑΣ\u0345Α','ασ\u0345α'],['map lower String case','\u1c89','\u1c8a'],['map upper String case','\u1c8a','\u1c89'],['map lower String case','\ud800\u{10400}\udc00','\ud800\u{10428}\udc00']]){const result=executeJavaScriptStringMethodPrimitive(operation,[text(input),text('16.0')]);assert.equal(result.$,'primitiveValue');assert.deepEqual(result.value,text(expected));}
 assert.equal(executeJavaScriptStringMethodPrimitive('map lower String case',[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'A'.repeat(1048577)},text('16.0')]).$,'primitiveResourceLimit');
 assert.equal(executeJavaScriptStringMethodPrimitive('map upper String case',[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'ß'.repeat(524289)},text('16.0')]).$,'primitiveResourceLimit');
});
for(const row of originals)test(row.name,()=>{
 requireActiveProfile();
 const capture=captureJavaScriptOrdinaryClassModule(row.source,{identifier:'literal-string-case-profile.js',entries:row.entries});assert.deepEqual(checkJavaScriptOrdinaryClassModule(capture),capture);
 const instance=prepareJavaScriptOrdinaryClassModule(capture,{realmIdentity:97n}),transition=runJavaScriptSourceTransition(instance.embedding,instance.begin(row.entry,row.arguments.map(instance.embedding.encode)),{steps:65536}).transition;
 assert.equal(transition.$,'machineCompleted',transition.reason);assert.deepEqual(instance.embedding.materializeData(transition.value),row.expected);
 const declarations=capture.origins.filter(origin=>origin.classBinding===undefined||origin.constructor===true).map(origin=>capture.source.slice(origin.start,origin.end)).join('\n'),original=compileFunction('"use strict";'+declarations+'\nreturn read;',[])();
 assert.deepEqual(original(...row.arguments),row.expected);
});
