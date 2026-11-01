// Deferred source assertions; no target compiler/kernel execution happened.
import assert from 'node:assert/strict';
import test from 'node:test';
import * as rules from '../src/translation/formal-datatype-alias-rules.js';
import {checkProgram} from '../src/translation/check.js';
import {emitLean} from '../src/translation/emit-lean.js';
import {emitRocq} from '../src/translation/emit-rocq.js';
import {EmitState,orderDeclarations} from '../src/translation/emit-common.js';
import {planFormalDatatypeModules,formalDatatypeModuleDependencies} from '../src/translation/formal-datatype-modules.js';
import {captureFormalDatatypeAliases,FormalDatatypeAliases} from '../src/translation/formal-datatype-aliases.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {formalDatatypeAliasDecisions as decisions,formalDatatypeAliasSurfaces as surfaces} from './fixtures/formal-datatype-aliases.mjs';
for(const [index,row]of decisions.entries())test('independent formal public alias policy '+index,()=>assert.deepEqual(rules[row.operation](...row.arguments),row.expected));
for(const row of surfaces)test(row.name,()=>{
 const checked=checkProgram(row.surface),before=JSON.stringify([...checked.declarations]),plan=planFormalDatatypeModules(checked);
 assert.ok(plan.aliases.length>0);assert.throws(()=>orderDeclarations(checked),/mutually recursive in different modules/);
 assert.doesNotThrow(()=>orderDeclarations(plan.program,{requireContiguousModules:true,extraDependencies:formalDatatypeModuleDependencies(plan)}));
 const lean=emitLean(checked).text,rocq=emitRocq(checked).text;
 assert.match(lean,/mutual\ninductive/u);assert.match(rocq,/\nwith /u);
 for(const alias of plan.aliases){assert.match(lean,new RegExp('abbrev '+alias.sourceName+' := _root_\\.','u'));assert.match(rocq,new RegExp('Definition '+alias.sourceName+' : Type := ','u'));}
 assert.match(lean,/@\[match_pattern\] abbrev/u);assert.match(rocq,/Notation /u);
 if(row===surfaces[0])assert.equal(rocq.match(/Module nodes\./gu)?.length,1);
 assert.equal(JSON.stringify([...checked.declarations]),before);
 const packet=encodeTypedProgramLinks(checked);for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  assert.deepEqual(decodeTypedProgramLinks(network,packet.root),checked);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
});
test('an alias scope refuses early opening and refuses reopening after its complete publication',()=>{
 const source=checkProgram(surfaces[0].surface),plan=planFormalDatatypeModules(source);
 const make=()=>{const state=new EmitState(source,'Rocq',name=>name,new Set(),{ctorStyle:'module'}),rows=captureFormalDatatypeAliases(plan.aliases,state);state.relocateDatatypes(plan.program,plan.aliases);return {state,aliases:new FormalDatatypeAliases(rows,'Rocq',state)};};
 const early=make();assert.throws(()=>early.aliases.open(['ir','nodes']),/before its complete lifted datatype block/);
 const ready=make();ready.aliases.complete([...plan.program.declarations.values()]);ready.aliases.open([]);ready.aliases.open(['ir']);const lines=ready.aliases.close(['ir']);assert.match(lines.join('\n'),/Module nodes\./u);ready.aliases.finish();assert.throws(()=>ready.aliases.open(['ir','nodes']),/would have to reopen/);
});
