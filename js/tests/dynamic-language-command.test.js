// Deferred until the complete requirements code draft is in place.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {runDynamicLanguageCommand} from '../src/dynamic-language-cli.js';
import {DynamicMetaLanguage} from '../src/dynamic-meta-language.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {readMetaLanguageArchiveText,readMetaLanguageArchiveBinary} from '../src/meta-language-archive.js';
import {decodeStructuredDataLinks,readStructuredDataLino} from '../src/structured-data-links.js';
import {LinkNetwork} from '../src/network.js';
const fixture=readProjectData(new URL('../../parity/fixtures/dynamic-language-command.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const encoder=new TextEncoder(),decoder=new TextDecoder('utf-8',{fatal:true});
function bytes(tree){
 if(tree.type==='missing')return [];
 if(tree.type==='token'||tree.type==='error')return tree.text===null?[...tree.hex.matchAll(/../gu)].map(([value])=>Number.parseInt(value,16)):[...encoder.encode(tree.text)];
 return (tree.children??[tree.root]).flatMap(bytes);
}
function concreteData(entry){
 const network=new LinkNetwork(),seen=new Set(),pending=[entry.root];
 for(let index=0;index<pending.length;index+=1){const id=pending[index];if(seen.has(id.asU64()))continue;seen.add(id.asU64());
  const link=entry.network.link(id);network.insertLinkWithOptionalId(id,link.references(),link.metadata().clone());pending.push(...link.references());}
 return decodeStructuredDataLinks(network,entry.root);
}
test('one shared command sequence retains concrete trees, recursive definitions and old contexts',()=>{
 const files=new Map(fixture.files.map(file=>[file.path,encoder.encode(file.text)]));
 const host={readFile:path=>decoder.decode(files.get(path)??assert.fail('missing fixture file '+path)),readBytes:path=>files.get(path)??assert.fail('missing fixture file '+path)};
 for(const step of fixture.steps){
  const output=runDynamicLanguageCommand(step.args,host);
  assert.equal(output.exitCode,step.exit,step.name+' '+output.stderr);
  if(step.error)assert.ok(output.stderr.includes(step.error),step.name);
  if(step.rejection)assert.ok(readStructuredDataLino(output.stderr).rejection,step.name);
  if(step.save)files.set(step.save,typeof output.stdout==='string'?encoder.encode(output.stdout):output.stdout);
  if(step.inventory)assert.deepEqual(readStructuredDataLino(output.stdout).map(row=>row.identifier),step.inventory,step.name);
  if(step.grammar)assert.deepEqual(parseGrammarLinks(output.stdout).normalized(),parseGrammarLinks(host.readFile(step.grammar)).normalized(),step.name);
  if(step.archive){
   const archive=step.binary?readMetaLanguageArchiveBinary(output.stdout):readMetaLanguageArchiveText(output.stdout);
   const session=new DynamicMetaLanguage(archive);
   if(step.concrete){
    const entry=session.entries.find(entry=>entry.identifier===step.concrete);
    assert.equal(entry.kind,'concrete syntax',step.name);
    const concrete=concreteData(entry);
    assert.equal(concrete.projection,'concrete syntax',step.name);
    assert.deepEqual(bytes(concrete.outcome.tree),[...encoder.encode(step.text)],step.name);
   }
   if(step.abstract)assert.equal(session.entries.find(entry=>entry.identifier===step.abstract)?.kind,'abstract syntax',step.name);
  }
 }
});
test('source-free reload needs no grammar source callback',()=>{
 const files=new Map(fixture.files.map(file=>[file.path,encoder.encode(file.text)]));
 const host={readFile:path=>decoder.decode(files.get(path)??assert.fail('unexpected source read '+path)),readBytes:path=>files.get(path)??assert.fail('unexpected source read '+path)};
 const prepare=runDynamicLanguageCommand(fixture.steps[0].args,host);
 assert.equal(prepare.exitCode,0,prepare.stderr);
 files.set('prepared.lino',encoder.encode(prepare.stdout));files.delete('first.lino');files.delete('configuration.lino');
 const parsed=runDynamicLanguageCommand(fixture.steps[3].args,host);
 assert.equal(parsed.exitCode,0,parsed.stderr);
 assert.ok(new DynamicMetaLanguage(readMetaLanguageArchiveText(parsed.stdout)).entries.some(entry=>entry.identifier==='original document'));
});
