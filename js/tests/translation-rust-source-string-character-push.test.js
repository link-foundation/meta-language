// Deferred actual-source obligations; no represented program runs on import.
import test from 'node:test';import assert from 'node:assert/strict';import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {rustSourceStringCharacterPushOriginals as originals,rustSourceStringCharacterPushRefusals as refusals,rustSourceStringCharacterPushVersions as versions} from './fixtures/rust-source-string-character-push.mjs';
const options={sourceVersion:'1.90.0',exclusiveSourceModel:'Rust exclusive source loans 1'};
function stdout(text){const lines=[];vm.runInNewContext(text,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:10000});return lines.length?lines.join('\n')+'\n':'';}
test('canonical character literals append through actual mutable String and field places',()=>{
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node+'; '+versions.node);
 for(const sourceVersion of ['1.90.0','1.99.0'])for(const row of originals){
  const program=checkProgram(parseRust(row.source,{...options,sourceVersion}));assert.equal(stdout(emitJavaScript(program).text),row.stdout,row.name);
  const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion,rustExclusiveSourceModel:options.exclusiveSourceModel});
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.equal(stdout(emitTypedProgramLinks(network,packet.root,'JavaScript').code),row.stdout,row.name);
 }
});
test('literal-only admission preserves character identity and actual mutable source authority',()=>{
 for(const row of refusals)assert.throws(()=>checkProgram(parseRust(row.source,options)),error=>['syntax','unsupported','type'].includes(error.kind),row.name);
 assert.throws(()=>parseRust("fn main(){let mut value=String::from(\"\");value.push('a');}"),/character literal|mutable binding|mutation/);
});
