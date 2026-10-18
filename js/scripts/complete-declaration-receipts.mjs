// Finite current receipts; historical artifacts and semantic output stay exact.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readProjectData} from './project-data.mjs';
import {structuredDataToLino} from '../src/structured-data-links.js';

export const COMPLETE_DECLARATION_RECEIPTS = new URL('../../parity/self-translation/complete-declaration-receipts.lino',import.meta.url);
const digest=value=>createHash('sha256').update(value).digest('hex');

/** Restore only a recorded source endpoint, archive receipt or source descriptor. */
export function restoreCompleteDeclarationReceipt(artifact,identity){
 const register=readProjectData(COMPLETE_DECLARATION_RECEIPTS);
 assert.equal(register.schemaVersion,1);
 const matches=register.receipts.filter(row=>row.identity===identity);
 assert.equal(matches.length,1,'one complete declaration-range receipt has this identity');
 const receipt=matches[0],bytes=Buffer.from(structuredDataToLino(artifact)),actual=digest(bytes);
 if(actual===receipt.previous.sha256){assert.equal(bytes.length,receipt.previous.bytes);return structuredClone(artifact);}
 assert.equal(actual,receipt.current.sha256,'the entire current artifact must match its reviewed receipt');
 assert.equal(bytes.length,receipt.current.bytes);
 assert.equal(receipt.previous.file,receipt.current.file);
 assert.match(receipt.previous.revision,/^[a-f0-9]{40}$/u);
 const inverse=structuredClone(artifact),paths=new Set();
 for(const change of receipt.changes){
  const path=change.path,key=JSON.stringify(path),last=path.at(-1);
  assert.ok(path.length>1&&!paths.has(key)&&path.every(part=>typeof part==='string'&&!['__proto__','prototype','constructor'].includes(part)),'each recorded path is distinct and owned');
  paths.add(key);
  const endpoint=['span','sourceSpan'].includes(path.at(-2))&&last==='end';
  const archive=path.includes('archive')&&['textBytes','textSha256','binaryBytes','binarySha256'].includes(last);
  const descriptor=identity==='raw consuming'&&['bytes','sha256'].includes(last)
   &&((path.length===3&&path[0]==='decisionSources'&&/^\d+$/u.test(path[1]))
    ||(path.length===6&&path.slice(0,4).join('.')==='predecessor.proof.producerApplicability.sources'&&/^\d+$/u.test(path[4])));
  assert.ok(endpoint||archive||descriptor,'semantic fields cannot be refreshed as source provenance');
  if(endpoint||last.endsWith('Bytes')||last==='bytes')for(const value of[change.before,change.after])assert.ok(Number.isSafeInteger(value)&&value>=0);
  else for(const value of[change.before,change.after])assert.match(value,/^[a-f0-9]{64}$/u);
  assert.notDeepEqual(change.before,change.after);
  let parent=inverse;
  for(const part of path.slice(0,-1)){assert.ok(Object.hasOwn(parent,part));parent=parent[part];}
  assert.ok(Object.hasOwn(parent,last));assert.deepEqual(parent[last],change.after);
  parent[last]=structuredClone(change.before);
 }
 const restored=Buffer.from(structuredDataToLino(inverse));
 assert.equal(restored.length,receipt.previous.bytes,'the complete previous byte count is restored');
 assert.equal(digest(restored),receipt.previous.sha256,'the complete immutable previous artifact is restored');
 return inverse;
}
