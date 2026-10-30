// Deferred source and publication controls. Do not execute during drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readIntrinsicModuleInitializerBinding} from '../src/translation/intrinsic-initializers.js';
import {acceptIntrinsicModuleInitializerScope} from '../src/translation/intrinsic-initializer-rules.js';
import {readInertModuleBindingDeclaration} from '../src/translation/inert-module-bindings.js';
import {selfTranslate} from '../src/self-translation.js';
const fixture=readProjectData(new URL('../../parity/fixtures/intrinsic-module-initializers.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.positives)test('actual intrinsic initializer '+row.source,()=>{
 assert.deepEqual(readIntrinsicModuleInitializerBinding(row.source),[row.name,row.owner]);
 assert.equal(readInertModuleBindingDeclaration(row.source),null,'the independent original classifier retains its refusal');
});
for(const source of fixture.negatives)test('eager or unknown initializer remains unproved '+source,()=>assert.equal(readIntrinsicModuleInitializerBinding(source),null));
for(const row of fixture.scope)test('scope '+row.forms+' '+row.bindings,()=>assert.equal(acceptIntrinsicModuleInitializerScope(row.forms,row.owners,row.bindings),row.accepted));
test('scope proof publishes no initializer signature or replacement body',()=>{
 const output=selfTranslate(fixture.module,'JavaScript','Rust');
 assert.ok(output.items.some(item=>item.status==='carried'&&fixture.module.slice(item.start,item.end).includes('TABLE')));
 assert.ok(output.items.some(item=>item.status==='translated'&&fixture.module.slice(item.start,item.end).includes('function keep')));
 assert.ok(output.code.includes('fn keep'));
 assert.ok(!output.code.includes('const TABLE:'));
});
