// Deferred owner/profile and foreign-boundary controls; no local draft execution.
import assert from 'node:assert/strict';
import test from 'node:test';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {prepareJavaScriptRecordOrderProfile} from '../src/translation/javascript-record-order-profile.js';
test('actual builtin calls select all source record types while retaining each value order',()=>{
 const surface=parseJavaScript('function pass(value){return Object.keys(value).join("|");}function make(flag){return flag?{z:1,a:2}:{a:2,z:1};}console.log("%s",pass(make(true)));');
 const original=structuredClone(surface),profile=prepareJavaScriptRecordOrderProfile(surface);
 assert.equal(profile.selected,true);assert.deepEqual(surface,original);
 const found=[];const seen=new Set();
 function visit(value){if(!value||typeof value!=='object'||seen.has(value))return;seen.add(value);if(value.kind==='record')found.push(value);for(const child of Object.values(value))visit(child);}
 visit(profile.surface);assert.ok(found.length);assert.ok(found.every(type=>type.ordered===true));
 const program=checkProgram(surface);const records=[];
 function inspect(value){if(!value||typeof value!=='object')return;if(value.k==='record'&&value.type.ordered)records.push(value);for(const child of value instanceof Map?value.values():Object.values(value))inspect(child);}
 inspect(program);assert.ok(records.length);assert.ok(records.every(record=>record.order));
});
test('shadowed Object calls do not activate a builtin source profile',()=>{
 const surface=parseJavaScript('function compute(){const Object={keys:value=>["local"]};return Object.keys(3).join("|");}');
 assert.equal(prepareJavaScriptRecordOrderProfile(surface).selected,false);
});
test('foreign structural records cannot silently acquire the ordered ABI',()=>{
 const surface=parseJavaScript('function compute(){return Object.keys({z:1,a:2}).join("|");}');
 const record={kind:'record',fields:[{name:'value',type:{kind:'float'}}]};
 const external={k:'fn',name:'foreign',params:[{name:'record',type:record}],ret:{kind:'unit'}};
 assert.throws(()=>prepareJavaScriptRecordOrderProfile({...surface,externals:[external]}),/actual foreign record signature/u);
 assert.equal(prepareJavaScriptRecordOrderProfile({...surface,externals:[{...external,params:[{name:'record',type:{...record,ordered:true}}]}]}).selected,true);
});
