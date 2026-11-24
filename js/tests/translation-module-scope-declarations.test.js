import assert from 'node:assert/strict';
import test from 'node:test';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {Script} from 'node:vm';
import {tokenize} from '../src/translation/lexer.js';
import {parseNative} from '../src/native-grammar-parser.js';
import {LinkNetwork} from '../src/network.js';
import {encodeStructuredDataLinks,decodeStructuredDataLinks} from '../src/structured-data-links.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {MODULE_SCOPE_DECLARATION_ORIGIN,findCompleteModuleScopeDeclaration,verifyModuleScopeDeclarationHistory,readModuleScopeDeclarationHistory,restoreModuleScopeDeclarationBindings} from '../scripts/module-scope-declaration-history.mjs';
import {generateModuleScopeDeclarationHistory} from '../scripts/generate-module-scope-declaration-history.mjs';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const read=file=>readFileSync(new URL('../../'+file,import.meta.url),'utf8');
const range=(tokens,at)=>findCompleteModuleScopeDeclaration(tokens.map(token=>token.kind),tokens.map(token=>token.value),at);

test('complete definitions include destructured defaults nested calls comments and literal braces',()=>{
 for(const declaration of [
  'function inspect({check=false}={}){return check;}',
  'export async function inspect({value={text:"} )"}}={},select=(x)=>({x})){const expression=/[{})]/u;return {value,select,expression};}',
  'function inspect(value=(()=>({nested:true}))()){/* } ) */const text="{";return value;}',
 ]){
  assert.doesNotThrow(()=>new Script(declaration.replace(/^export /u,'')));
  const source='/* λ */\n'+declaration+'\nconst after=1;',tokens=tokenize(source,'JavaScript').tokens;
  const at=tokens.findIndex(token=>token.kind==='identifier'&&token.value==='function'),selected=range(tokens,at);assert.equal(selected.length,2);
  assert.equal(source.slice(tokens[at].start,tokens[selected[1]-1].end),declaration.slice(declaration.indexOf('function')));
  assert.equal(tokens[selected[0]].value,'{');assert.equal(tokens[selected[1]-1].value,'}');
  assert.equal(parseNative('native-javascript',declaration).hasError,false);
 }
});

test('unfinished formal lists bodies and malformed token columns refuse',()=>{
 for(const source of ['function inspect({check=false}', 'function inspect(){return {value:1};', 'function inspect() return 1;']){
  const tokens=tokenize(source,'JavaScript').tokens;assert.deepEqual(range(tokens,0),[]);
 }
 const tokens=tokenize('function inspect({check=false}={}){return check;}','JavaScript').tokens;
 for(const at of [-1,0.5,NaN,Infinity,tokens.length])assert.deepEqual(range(tokens,at),[]);
 assert.deepEqual(findCompleteModuleScopeDeclaration(tokens.map(token=>token.kind),[],0),[]);
 assert.deepEqual(range(tokens,1),[]);
});

test('complete certified originals prove the two old fields were parameter fragments of the same declarations',()=>{
 const packet=readModuleScopeDeclarationHistory();assert.equal(packet.declarations.length,2);
 assert.deepEqual(generateModuleScopeDeclarationHistory({check:true}),packet);
 const receipt=readBootstrapStructuredDataLino(packet.receipt.source);
 for(const row of packet.declarations){
  assert.equal(row.previous,receipt.additions.find(value=>value.id===row.id).sourceText);
  assert.equal(parseNative('native-javascript',row.current.source).hasError,false);
  assert.equal(parseNative('native-javascript',row.previous).hasError,true);
  assert.equal(row.current.source,read(row.source).slice(row.current.start,row.current.end));
  assert.match(row.current.source,/return packet;/u);
 }
});

test('the exact metadata inverse preserves every identity field original input and caller container',()=>{
 const packet=readModuleScopeDeclarationHistory(),original=readBootstrapStructuredDataLino(packet.receipt.source).additions;
 const current=original.map(row=>{const correction=packet.declarations.find(value=>value.id===row.id);return correction?{...row,sourceText:correction.current.source}:row;});
 const before=structuredClone(current),restored=restoreModuleScopeDeclarationBindings(current);
 assert.deepEqual(restored,original);assert.deepEqual(current,before);assert.equal(restored.length,current.length);
 for(const correction of packet.declarations){
  const row=current.find(value=>value.id===correction.id);
  for(const changed of [{...row,sourceText:row.sourceText+'\n'},{...row,source:'unknown.mjs'},{...row,name:'other'}])assert.throws(()=>restoreModuleScopeDeclarationBindings([changed]));
  const extended={...row,extraContext:{preserved:true}};
  assert.deepEqual(restoreModuleScopeDeclarationBindings([extended])[0],{...extended,sourceText:correction.previous});
 }
});

test('forged coherent repins changed membership altered fragments and missing declarations refuse',()=>{
 const packet=readModuleScopeDeclarationHistory();
 for(const mutate of [value=>{value.revision='0'.repeat(40);},value=>{value.register.rows--;},value=>{value.sources.reverse();},value=>{value.declarations.pop();},value=>{value.declarations[0].id='unknown';},value=>{value.declarations[0].previous+=' ';},value=>{value.declarations[0].current.end--;},value=>{value.declarations[0].current.source+='\n';},value=>{value.sources[0].certificate.path='unknown.mjs';},value=>{const row=value.sources[0];row.source+='\n';row.bytes=Buffer.byteLength(row.source);row.sha256=createHash('sha256').update(row.source).digest('hex');}]){
  const changed=structuredClone(packet);mutate(changed);assert.throws(()=>verifyModuleScopeDeclarationHistory(changed));
 }
 const detached=readModuleScopeDeclarationHistory();detached.declarations[0].current.source='caller mutation';assert.deepEqual(readModuleScopeDeclarationHistory(),packet);
});

test('both Links codecs retain every complete original module certificate fragment and declaration',()=>{
 const packet=readModuleScopeDeclarationHistory(),encoded=encodeStructuredDataLinks(packet),canonical=LinkNetwork.fromLino(encoded.network.toLino());
 for(const network of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
  const restored=decodeStructuredDataLinks(network,encoded.root);assert.deepEqual(restored,packet);verifyModuleScopeDeclarationHistory(restored);
 }
});

test('the actual current registration contains complete original declarations for both corrected fields',()=>{
 const packet=readModuleScopeDeclarationHistory(),current=readBootstrapStructuredDataLino(read(MODULE_SCOPE_DECLARATION_ORIGIN.receipt));
 for(const row of packet.declarations){
  const matches=current.additions.filter(value=>value.id===row.id);assert.equal(matches.length,1);
  assert.equal(matches[0].source,row.source);assert.equal(matches[0].name,row.name);assert.equal(matches[0].sourceText,row.current.source);
 }
});
