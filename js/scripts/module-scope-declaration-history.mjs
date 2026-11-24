// Correct incomplete metadata from the exact original declaration, not a new
// implementation. Every prior identity, module and parameter fragment survives.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {tokenize} from '../src/translation/lexer.js';
import {readRequiredTextFact} from '../src/translation/fact-column-rules.js';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';

export const MODULE_SCOPE_DECLARATION_ORIGIN=Object.freeze({
 revision:'df376364c2373a2002f18a51ba8308a474c34688',
 history:'parity/naming/module-scope-declaration-history.lino',
 receipt:'parity/naming/module-scope-bindings-registration.lino',
 codec:'parity/naming/source-binding-codec-profile.lino',
 register:'parity/naming/source-bindings.lino',
 bytes:77175401,sha256:'1fd8d9011d31dca4a27b5e983cc4728acb0731f1535575887d93b95a3423be98',rows:8038,
});
export const MODULE_SCOPE_DECLARATION_CORRECTIONS=Object.freeze([
 Object.freeze({source:'js/scripts/generate-borrowed-javascript-body-source-history.mjs',name:'generateBorrowedJavaScriptBodySourceHistory'}),
 Object.freeze({source:'js/scripts/generate-native-compiler-host-source-history.mjs',name:'generateNativeCompilerHostSourceHistory'}),
]);
const identity=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
let snapshot;

/** Locate the entire formal list before looking for the body. Punctuation in
 * a string, template or regular expression is never a delimiter.
 * @param {string[]} kinds @param {string[]} values @param {number} position
 * @returns {number[]} */
export function findCompleteModuleScopeDeclaration(kinds,values,position){
 if(kinds.length!==values.length||kinds.length>100000||position<0||position%1!==0||position+2>=kinds.length)return [];
 if(readRequiredTextFact(kinds,position)!=='identifier'||readRequiredTextFact(values,position)!=='function'||readRequiredTextFact(kinds,position+1)!=='identifier'||readRequiredTextFact(kinds,position+2)!=='punct'||readRequiredTextFact(values,position+2)!=='(')return [];
 let depth=1,cursor=position+3;
 while(cursor<kinds.length&&depth>0){
  if(readRequiredTextFact(kinds,cursor)==='punct'){
   const value=readRequiredTextFact(values,cursor);if(value==='(')depth++;else if(value===')')depth--;
  }
  cursor++;
 }
 if(depth!==0||cursor>=kinds.length||readRequiredTextFact(kinds,cursor)!=='punct'||readRequiredTextFact(values,cursor)!=='{')return [];
 const opening=cursor;depth=1;cursor++;
 while(cursor<kinds.length&&depth>0){
  if(readRequiredTextFact(kinds,cursor)==='punct'){
   const value=readRequiredTextFact(values,cursor);if(value==='{')depth++;else if(value==='}')depth--;
  }
  cursor++;
 }
 return depth===0?[opening,cursor]:[];
}

export function verifyModuleScopeDeclarationHistory(packet){
 assert.deepEqual(Object.keys(packet).sort(),['schemaVersion','model','revision','register','receipt','codec','sources','declarations','boundary'].sort());
 assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'complete original module scope declarations 1');
 const origin=MODULE_SCOPE_DECLARATION_ORIGIN;assert.equal(packet.revision,origin.revision);
 assert.deepEqual(packet.register,{file:origin.register,bytes:origin.bytes,sha256:origin.sha256,rows:origin.rows});
 for(const row of [packet.receipt,packet.codec,...packet.sources]){
  assert.deepEqual(Object.keys(row).sort(),['file','source','bytes','sha256','certificate'].sort());
  assert.equal(typeof row.source,'string');assert.equal(row.source.isWellFormed(),true);
  assert.deepEqual(identity(row.source),{bytes:row.bytes,sha256:row.sha256});
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:origin.revision,path:row.file});
 }
 assert.equal(packet.receipt.file,origin.receipt);assert.equal(packet.codec.file,origin.codec);
 const receipt=readBootstrapStructuredDataLino(packet.receipt.source),codec=readBootstrapStructuredDataLino(packet.codec.source);
 assert.equal(receipt.file,origin.register);assert.deepEqual(receipt.current,{bytes:origin.bytes,sha256:origin.sha256,rows:origin.rows});
 assert.deepEqual(codec.profiles.at(-1).source,{path:origin.register,bytes:origin.bytes,sha256:origin.sha256});
 assert.deepEqual(packet.sources.map(row=>row.file),MODULE_SCOPE_DECLARATION_CORRECTIONS.map(row=>row.source));
 assert.deepEqual(packet.declarations.map(row=>({source:row.source,name:row.name})),MODULE_SCOPE_DECLARATION_CORRECTIONS);
 for(const row of packet.declarations){
  assert.deepEqual(Object.keys(row).sort(),['id','source','name','previous','current'].sort());
  const prior=receipt.additions.filter(item=>item.source===row.source&&item.name===row.name);assert.equal(prior.length,1);
  assert.equal(row.id,prior[0].id);assert.equal(row.previous,prior[0].sourceText);
  const source=packet.sources.find(item=>item.file===row.source).source,tokens=tokenize(source,'JavaScript').tokens;
  const positions=tokens.flatMap((token,index)=>token.value==='function'&&tokens[index+1]?.value===row.name?[index]:[]);assert.equal(positions.length,1);
  const at=positions[0],range=findCompleteModuleScopeDeclaration(tokens.map(token=>token.kind),tokens.map(token=>token.value),at);assert.equal(range.length,2);
  let declaration=at;if(tokens[declaration-1]?.value==='async')declaration--;if(tokens[declaration-1]?.value==='export')declaration--;
  assert.deepEqual(Object.keys(row.current).sort(),['start','end','source'].sort());
  assert.deepEqual(row.current,{start:tokens[declaration].start,end:tokens[range[1]-1].end,source:source.slice(tokens[declaration].start,tokens[range[1]-1].end)});
  // Reproduce the original faulty selector independently. Its brace is inside
  // the parameters, so this is missing metadata for the SAME original function.
  let oldOpening=at+2;while(tokens[oldOpening]?.value!=='{'){oldOpening++;assert.ok(oldOpening<tokens.length);}
  assert.ok(oldOpening<range[0]);let depth=1,end=oldOpening+1;
  while(end<tokens.length&&depth){if(tokens[end].kind==='punct'){if(tokens[end].value==='{')depth++;else if(tokens[end].value==='}')depth--;}end++;}
  assert.equal(depth,0);assert.equal(source.slice(tokens[declaration].start,tokens[end-1].end),row.previous);
  assert.ok(row.current.source.startsWith(row.previous)&&row.current.source.length>row.previous.length);
 }
 assert.equal(typeof packet.boundary,'string');assert.ok(packet.boundary.length>0);return packet;
}

export function readModuleScopeDeclarationHistory(){
 const text=readFileSync(new URL('../../'+MODULE_SCOPE_DECLARATION_ORIGIN.history,import.meta.url),'utf8');
 if(snapshot?.text!==text)snapshot={text,packet:verifyModuleScopeDeclarationHistory(readBootstrapStructuredDataLino(text))};
 // The original whole module, imports and lexical context still define the
 // operation. A changed current context needs a distinct qualified definition.
 for(const row of snapshot.packet.sources)assert.equal(readFileSync(new URL('../../'+row.file,import.meta.url),'utf8'),row.source,'unknown current declaration context refuses');
 return structuredClone(snapshot.packet);
}

export function restoreModuleScopeDeclarationBindings(rows){
 assert.ok(Array.isArray(rows));const packet=readModuleScopeDeclarationHistory();
 return rows.map(row=>{
  const correction=packet.declarations.find(item=>item.id===row.id);if(!correction)return row;
  assert.equal(row.source,correction.source);assert.equal(row.name,correction.name);
  assert.ok(row.sourceText===correction.previous||row.sourceText===correction.current.source,'unknown original declaration metadata refuses');
  return{...row,sourceText:correction.previous};
 });
}
