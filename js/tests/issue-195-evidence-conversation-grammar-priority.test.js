import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';

const repositoryFile=path=>new URL('../../'+path,import.meta.url);
const register=readProjectData(repositoryFile('parity/issue-195-sources.lino'));
const manifest=readProjectData(repositoryFile('parity/issue-195-requirements.lino'));

test('the latest recursive syntax language priority and practice messages retain exact captured source bytes',()=>{
 for(const[id,bytes,sha256]of[
  ['recursive-concrete-syntax-and-any-grammar-2026-10-16',535,'ac3e3eef4662dbcd4722c9b8ce72bf40f29254b60b45cfb9a401aacc5ecd0d04'],
  ['own-language-complete-grammar-priority-2026-10-16',218,'aacbb2d28c4afc5900109767df117b9f7584dbb4b5fe3c23f74a19c8b9a10466'],
  ['router-practice-reuse-and-current-recording-2026-10-16',234,'13108798f78cdc33c04b87edc05f1e4ef6e398495c67a0872ee72be51458f114'],
 ]){
  const entry=register.requirementSources.find(row=>row.id===id);assert.ok(entry,id);
  const source=readFileSync(repositoryFile(entry.url));assert.equal(source.length,bytes,id);assert.equal(entry.bytes,bytes,id);
  assert.equal(createHash('sha256').update(source).digest('hex'),sha256,id);assert.equal(entry.sha256,sha256,id);
  assert.equal(entry.kind,'direct-instruction');assert.equal(entry.revisionOrigin,'capture time; the conversation does not expose a source timestamp');
  assert.match(entry.captureProvenance,/Verbatim message/u);
  for(const prefix of entry.ledgerCoverage)assert.ok(manifest.atomicRequirements.some(row=>row.id.startsWith(prefix)&&row.source===entry.url),prefix);
 }
});

test('recursive concrete syntax and complete own language grammar priorities remain critical atomic obligations',()=>{
 const ids=['I195-CST-DEFAULT-EVERY-LANGUAGE','I195-CST-EXPLICIT-AST-SELECTION','I195-CST-EMBEDDED-GLOBAL-CONCEPT-SPACE','I195-CST-ANY-GRAMMAR-RECURSIVE-RULE','I195-CST-DEEPLY-RECURSIVE-MIXED-LANGUAGES','I195-CST-RECURSIVE-BOUNDARIES-LOSSLESS',
  ...['JAVASCRIPT','MARKDOWN','RUST','TYPESCRIPT'].map(name=>'I195-GRAMMAR-PRIORITY-'+name+'-COMPLETE'),'I195-GRAMMAR-PRIORITY-OWN-SOURCE-TRANSLATION','I195-PRACTICE-ROUTER-761-REUSE','I195-CONVERSATION-CURRENT-VISION-REQUIREMENTS'];
 for(const id of ids){
  const row=manifest.atomicRequirements.find(value=>value.id===id);assert.ok(row,id);assert.equal(row.critical,true,id);assert.equal(row.fullScopeReleaseBlocker,true,id);
  assert.ok(row.verifications.length>0,id);for(const cell of row.verifications){assert.ok(cell.assertions.length>0,id);assert.ok(cell.fixtureIds.length>0,id);assert.equal(cell.checkpoint,'pre-merge',id);}
 }
 for(const id of ids.slice(0,11))assert.deepEqual(manifest.atomicRequirements.find(value=>value.id===id).requiredRuntimes,['javascript','rust'],id);
 const vision=readFileSync(repositoryFile('docs/vision.md'),'utf8');
 for(const phrase of ['Concrete syntax is the default for every supported language','Abstract syntax is an explicitly selected','`any-grammar` or `all-grammar`','Markdown →','JavaScript → JSON → Markdown → Python','Complete JavaScript, Markdown, Rust and TypeScript grammars'])assert.ok(vision.includes(phrase),phrase);
 // These are recording checks, not observations that generic recursion or
 // complete executable translation already works in either runtime.
});

test('the Router practice review pins primary sources and keeps project specific policies explicit',()=>{
 const review=readProjectData(repositoryFile('parity/sources/router-pr-761-practice-review.lino'));
 assert.equal(review.pullRequest,'https://github.com/link-assistant/router/pull/761');assert.equal(review.revision,'e2ceac6071d90514fb1a37b3fe5964433d61c402');
 assert.equal(review.sources.length,3);
 for(const source of review.sources){assert.ok(source.url.startsWith('https://github.com/link-assistant/router/blob/'+review.revision+'/'));assert.match(source.sha256,/^[a-f0-9]{64}$/u);assert.match(source.blobSha,/^[a-f0-9]{40}$/u);assert.ok(source.bytes>0&&source.bytes<=131072);}
 assert.ok(review.practices.length>=6);
 for(const practice of review.practices){assert.ok(practice.idea&&practice.application);for(const file of practice.existingEntryPoints)assert.ok(existsSync(repositoryFile(file)),file);}
 for(const word of ['JSON','Node and Bun','cancellation','cloned'])assert.ok(review.applicabilityDecisions.some(value=>value.includes(word)),word);
});
