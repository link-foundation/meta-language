// Deferred actual module boundaries, typed consumers and complete codecs.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../src/project-data-files.js';
import {selfTranslate,selfTranslationSignatures} from '../src/self-translation.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-module-values.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.publications)test('actual public Rust module values: '+row.name,async()=>{
 const signatures=selfTranslationSignatures(row.source,'Rust',{semanticTranslation:true}),translation=selfTranslate(row.source,'Rust','JavaScript',{semanticTranslation:true});
 assert.ok(translation.items.length>0);assert.ok(translation.items.every(item=>item.status==='translated'));
 const module=await import('data:text/javascript;base64,'+Buffer.from(translation.code).toString('base64'));
 assert.deepEqual(Object.keys(module).sort(),[...row.exports].sort());assert.equal(String(module.observe()),row.value);
 assert.deepEqual(signatures.map(signature=>signature.name).sort(),[...row.exports].sort());
 for(const check of row.checks){assert.equal(typeof module[check.name],check.kind);const value=check.call===null?module[check.name]:module[check.name](...check.call.map(argument=>BigInt(argument.slice(0,-1))));assert.equal(String(value),check.value);const signature=signatures.find(signature=>signature.name===check.name);assert.equal(signature.k,check.sourceKind);if(check.sourceKind==='const'&&check.kind==='function')assert.equal(signature.type.kind,'function');}
});
for(const row of fixture.consumers)test('actual callable constant consumer: '+row.name,()=>{
 const program=checkProgram(parseJavaScript(row.source,{externals:row.externals})),emitted=emitJavaScript(program),name=emitted.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;
 for(const [code,call] of [[row.source,'result'],[emitted.text,name]]){const actual=runInNewContext("'use strict';\n"+row.setup+'\n'+code.replace(/^export /gmu,'')+'\n[String('+call+'()),JSON.stringify(typeof events==="undefined"?[]:events)];');assert.equal(actual[0],row.value);assert.equal(actual[1],JSON.stringify(row.events));}
 const packet=encodeTypedProgramLinks(program);for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}
});
test('an invalid private body never publishes a valid-looking public interface',()=>assert.throws(()=>selfTranslationSignatures('pub const VALUE:i64=7;fn hidden()->i64{missing_name}','Rust',{semanticTranslation:true})));
test('complete module value policies survive both codecs and all target emissions',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/rust-module-value-program');for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}});
