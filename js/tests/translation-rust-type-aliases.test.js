import {acceptRustTypeAliasOwnedStringMember} from '../src/translation/rust-type-alias-rules.js';
import {readRustTypeAliasPrimitiveMemberOwner} from '../src/translation/rust-type-alias-rules.js';
// Deferred source, declaration-plan and public-interface controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../src/project-data-files.js';
import {selfTranslate,selfTranslationSignatures} from '../src/self-translation.js';
import {tokenize} from '../src/translation/lexer.js';
import {readRustTypeAliasDeclarationPlan} from '../src/translation/rust-type-alias-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-type-aliases.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.plans)test(row.name,()=>{
 const tokens=tokenize(row.source,'Rust').tokens,plan=readRustTypeAliasDeclarationPlan(tokens.map(token=>token.kind==='identifier'&&['mod','type'].includes(token.value)&&token.raw===token.value?'keyword':token.kind),tokens.map(token=>token.value));
 assert.equal(plan[0],'aliases');assert.deepEqual(plan.slice(1).filter((_,index)=>index%5===0),row.keys);
 for(let at=1;at<plan.length;at+=5){assert.equal(tokens[Number(plan[at+1])].raw,'type');assert.equal(tokens[Number(plan[at+3])].value,';');assert.equal(Number(plan[at+4]),Number(plan[at+3])+1);}
});
for(const row of fixture.publications)test(row.name,async()=>{
 const result=selfTranslate(row.source,'Rust','JavaScript',{semanticTranslation:true});assert.ok(result.items.length>0);assert.ok(result.items.every(item=>item.status==='translated'));
 const module=await import('data:text/javascript;base64,'+Buffer.from(result.code).toString('base64'));
 assert.deepEqual(Object.keys(module).sort(),[...row.exports].sort());assert.equal(String(module.observe()),row.value);
 const signatures=selfTranslationSignatures(row.source,'Rust',{semanticTranslation:true});assert.deepEqual(signatures.map(row=>row.name).sort(),[...row.exports].sort());
 for(const check of row.checks){const value=check.call===null?module[check.name]:module[check.name](...check.call.map(argument=>BigInt(argument.slice(0,-1))));assert.equal(String(value),check.value);const signature=signatures.find(row=>row.name===check.name);assert.equal(signature.k,check.name==='read'?'fn':'const');if(check.name==='read'){assert.deepEqual(signature.ret,{kind:'fixed',bits:64,signed:true});assert.deepEqual(signature.params[0].type,signature.ret);}}
});
test('an unused invalid alias cannot publish a valid-looking public function',()=>assert.throws(()=>selfTranslationSignatures('type Hidden=Absent;pub fn observe()->i64{7}','Rust',{semanticTranslation:true})));

for(const [index,row]of fixture.primitiveOwners.entries())test('checked primitive alias owner '+index,()=>assert.equal(readRustTypeAliasPrimitiveMemberOwner(row.kind,row.bits,row.signed,row.ownerKind,row.owner,row.children,row.proven,row.layers,row.suffix),row.expected));

for(const [index,row]of fixture.stringMembers.entries())test('checked String alias member '+index,()=>assert.equal(acceptRustTypeAliasOwnedStringMember(row.kind,row.ownerKind,row.owner,row.children,row.proven,row.layers,row.member),row.expected));
