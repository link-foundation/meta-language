import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData} from './project-data.mjs';

export function readNodeDependencyPlacementLiteral(source) {
  const lines=[...source.matchAll(/^export const NAME_INVENTORIES =[^\n]+$/gmu)];assert.equal(lines.length,1);
  const marker='.map(entry=>{const placement=',at=lines[0][0].indexOf(marker);assert.ok(at>=0);
  const start=at+marker.length,end=lines[0][0].indexOf('.find(row=>row.inventory===entry.inventory',start);assert.ok(end>start);
  const text=lines[0][0].slice(start,end);assert.ok(Array.isArray(JSON.parse(text)));
  return{start:lines[0].index+start,end:lines[0].index+end,text,declaration:lines[0][0]};
}

export function generateNodeDependencyTestPlacement({check=false}={}) {
  const root=new URL('../../',import.meta.url);
  const policy=readProjectData(new URL('parity/naming/node-dependency-test-placement.lino',root));
  assert.equal(policy.schemaVersion,1);
  assert.equal(policy.namingSource,'js/scripts/issue-195-naming.mjs');
  const old=JSON.parse(policy.originalPlacementLiteral);assert.equal(old.length,2);
  assert.equal(JSON.stringify(old),policy.originalPlacementLiteral);
  assert.equal(createHash('sha256').update(policy.originalPlacementLiteral).digest('hex'),policy.sha256);
  const locationBytes=readFileSync(new URL(policy.locationHistory.path,root));
  assert.equal(locationBytes.length,policy.locationHistory.bytes);
  assert.equal(createHash('sha256').update(locationBytes).digest('hex'),policy.locationHistory.sha256);
  const history=readProjectData(new URL(policy.locationHistory.path,root)),before=history.sources[0];
  assert.equal(policy.placement.source,before.file);assert.equal(policy.placement.destination,history.target);
  assert.equal(policy.placement.inventory,'B70 physical field source view names for '+before.file);
  const destination=readFileSync(new URL(policy.placement.destination,root),'utf8');
  assert.equal(restoreNodeDependencyGeneratorTestHistory(destination),before.sourceText+history.additions);
  assert.equal(Buffer.byteLength(destination),policy.placement.bytes);
  assert.equal(createHash('sha256').update(destination).digest('hex'),policy.placement.sha256);
  const file=new URL(policy.namingSource,root),source=readFileSync(file,'utf8');
  const literal=readNodeDependencyPlacementLiteral(source),current=JSON.stringify([...old,policy.placement]);
  assert.equal(createHash('sha256').update(policy.originalInventoryDeclaration).digest('hex'),'f8b6321374beb1a940a89201479e8f1e93ae3aa2efb7f93bc5af09c13ec07eb4');
  assert.equal(literal.declaration.replace(literal.text,policy.originalPlacementLiteral),policy.originalInventoryDeclaration,'the complete original inventory constructor and every gate remain exact');
  if(check)assert.equal(literal.text,current,'all old placement records and the exact new whole-source view remain');
  else if(literal.text!==current){
    assert.equal(literal.text,policy.originalPlacementLiteral,'unknown placement changes cannot be overwritten');
    const updated=source.slice(0,literal.start)+current+source.slice(literal.end);
    assert.equal(updated.slice(0,literal.start)+policy.originalPlacementLiteral+updated.slice(literal.start+current.length),source);
    writeFileSync(file,updated);
  }
  return policy;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.length===3&&['--write','--check'].includes(process.argv[2]));
  generateNodeDependencyTestPlacement({check:process.argv[2]==='--check'});
  console.log('The exact Node test source placement retains both original placements and every naming table row.');
}

import {restoreNodeDependencyGeneratorTestHistory} from './node-dependency-registration-history.mjs';
