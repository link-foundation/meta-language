import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {planGrammarWorkOrder,readGrammarWorkPriorities} from '../scripts/grammar-work-order.mjs';
import {readProjectData} from '../scripts/project-data.mjs';

test('grammar work priorities retain the exact user source and four initial languages',()=>{
 const policy=readProjectData(new URL('../../parity/grammar-work-priorities.lino',import.meta.url));
 const bytes=readFileSync(new URL('../../'+policy.source,import.meta.url));
 assert.equal(bytes.length,218);assert.equal(createHash('sha256').update(bytes).digest('hex'),policy.sourceSha256);
 assert.equal(policy.sourceSha256,'aacbb2d28c4afc5900109767df117b9f7584dbb4b5fe3c23f74a19c8b9a10466');
 assert.deepEqual(readGrammarWorkPriorities().slice(0,4),['JavaScript','Markdown','Rust','TypeScript']);
 const catalog=readProjectData(new URL('../../parity/language-grammar-inventory.lino',import.meta.url)).languages.filter(row=>row.grammars?.length>0);
 const order=planGrammarWorkOrder(catalog);
 assert.deepEqual(order.slice(0,4).map(index=>catalog[index].name),['JavaScript','Markdown','Rust','TypeScript']);
 assert.equal(order.length,catalog.length);
});

test('priority work order preserves every catalog occurrence and the original report positions',()=>{
 const languages=Object.freeze(['Python','Rust','JavaScript','Lean','Markdown','JavaScript','TypeScript','Rocq'].map((name,index)=>Object.freeze({name,identity:index})));
 const order=planGrammarWorkOrder(languages);assert.deepEqual(order,[2,5,4,1,6,0,3,7]);
 assert.deepEqual([...order].sort((left,right)=>left-right),[0,1,2,3,4,5,6,7]);
 const rows=new Array(languages.length);for(const index of order)rows[index]=languages[index];
 assert.deepEqual(rows,languages);for(let index=0;index<rows.length;index++)assert.equal(rows[index],languages[index]);
 assert.deepEqual(planGrammarWorkOrder(languages,['Unknown language','Rust']),[1,0,2,3,4,5,6,7]);
 assert.deepEqual(planGrammarWorkOrder(languages,[]),[0,1,2,3,4,5,6,7]);
 assert.deepEqual(planGrammarWorkOrder([]),[]);
});

test('grammar work order refuses invalid priority policies without changing the input inventory',()=>{
 const languages=[{name:'JavaScript'},{name:'Markdown'}],before=structuredClone(languages);
 for(const priorities of [null,{},['Rust','Rust'],[''],[1]])assert.throws(()=>planGrammarWorkOrder(languages,priorities),TypeError);
 assert.throws(()=>planGrammarWorkOrder(null,[]),TypeError);assert.deepEqual(languages,before);
});
