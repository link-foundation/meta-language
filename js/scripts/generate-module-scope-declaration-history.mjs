// Automatically recover missing definition text from its exact original Git
// source. No implementation, identity or earlier input is rewritten.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {tokenize} from '../src/translation/lexer.js';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {writeProjectData,checkProjectData} from './project-data.mjs';
import {MODULE_SCOPE_DECLARATION_ORIGIN,MODULE_SCOPE_DECLARATION_CORRECTIONS,findCompleteModuleScopeDeclaration,verifyModuleScopeDeclarationHistory} from './module-scope-declaration-history.mjs';

export function generateModuleScopeDeclarationHistory(options){
 const check=options?.check??false,origin=MODULE_SCOPE_DECLARATION_ORIGIN;
 const capture=file=>{const{blob,certificate}=captureGitArtifactCertificate(origin.revision,file),source=blob.toString('utf8');assert.deepEqual(Buffer.from(source),blob);return{file,source,bytes:blob.length,sha256:createHash('sha256').update(blob).digest('hex'),certificate};};
 const receipt=capture(origin.receipt),codec=capture(origin.codec),previous=readBootstrapStructuredDataLino(receipt.source);
 const sources=MODULE_SCOPE_DECLARATION_CORRECTIONS.map(row=>capture(row.source));
 const declarations=MODULE_SCOPE_DECLARATION_CORRECTIONS.map(row=>{
  const [binding]=previous.additions.filter(item=>item.source===row.source&&item.name===row.name);assert.ok(binding);
  const source=sources.find(item=>item.file===row.source).source,tokens=tokenize(source,'JavaScript').tokens;
  const at=tokens.findIndex((token,index)=>token.value==='function'&&tokens[index+1]?.value===row.name),range=findCompleteModuleScopeDeclaration(tokens.map(token=>token.kind),tokens.map(token=>token.value),at);assert.equal(range.length,2);
  let declaration=at;if(tokens[declaration-1]?.value==='async')declaration--;if(tokens[declaration-1]?.value==='export')declaration--;
  const start=tokens[declaration].start,end=tokens[range[1]-1].end;
  return{id:binding.id,...row,previous:binding.sourceText,current:{start,end,source:source.slice(start,end)}};
 });
 const packet={schemaVersion:1,model:'complete original module scope declarations 1',revision:origin.revision,
  register:{file:origin.register,bytes:origin.bytes,sha256:origin.sha256,rows:origin.rows},receipt,codec,sources,declarations,
  boundary:'Two original metadata fields stopped inside destructured parameters. Complete original Git-certified modules and register/codec receipts recover the SAME declarations, imports and lexical contexts without implementation or identity changes. Every old parameter fragment survives through the exact field inverse. This corrects incomplete source metadata; it is neither program equivalence across changed implementations nor runtime execution evidence.'};
 verifyModuleScopeDeclarationHistory(packet);
 for(const row of sources)assert.equal(readFileSync(new URL('../../'+row.file,import.meta.url),'utf8'),row.source,'unknown current source context refuses');
 const output=new URL('../../'+origin.history,import.meta.url);if(check)checkProjectData(output,packet);else writeProjectData(output,packet);return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){assert.ok(process.argv.slice(2).every(value=>value==='--check'));generateModuleScopeDeclarationHistory({check:process.argv.includes('--check')});}
