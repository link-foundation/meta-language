// Deferred controls; source drafting never executes this module.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../src/project-data-files.js';
import {selfTranslate,selfTranslationSignatures} from '../src/self-translation.js';
import {parseRust} from '../src/translation/rust.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-function-qualifiers.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.publications)test(row.name,async()=>{
 const result=selfTranslate(row.source,'Rust','JavaScript',{semanticTranslation:true});
 assert.ok(result.items.length>0);assert.ok(result.items.every(item=>item.status==='translated'));
 const module=await import('data:text/javascript;base64,'+Buffer.from(result.code).toString('base64'));
 assert.deepEqual(Object.keys(module).sort(),[...row.exports].sort());assert.equal(String(module.observe()),row.value);
 const signatures=selfTranslationSignatures(row.source,'Rust',{semanticTranslation:true});
 assert.deepEqual(signatures.map(row=>row.name).sort(),[...row.exports].sort());
 for(const check of row.checks){const value=check.call===null?module[check.name]:module[check.name](...check.call.map(argument=>BigInt(argument.slice(0,-1))));assert.equal(String(value),check.value);assert.equal(signatures.find(row=>row.name===check.name).k,check.name==='read'?'fn':'const');}
});
test('ordinary runtime admission never assigns a closed const-predicate proof',()=>{
 const program=parseRust('const fn read()->i64{7}fn main(){}');
 assert.equal(program.items[0].sourceConstFunction,undefined);
});
