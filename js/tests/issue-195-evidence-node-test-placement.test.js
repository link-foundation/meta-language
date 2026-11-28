import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {restoreAutomatedTranslationPauseSource} from '../scripts/automated-translation-pause-source-history.mjs';
import {generateNodeDependencyTestPlacement,readNodeDependencyPlacementLiteral} from '../scripts/generate-node-dependency-test-placement.mjs';
import {NAME_INVENTORIES} from '../scripts/issue-195-naming.mjs';

test('the Node test source placement preserves original namespaces and every preceding inventory placement',()=>{
  const policy=generateNodeDependencyTestPlacement({check:true});
  const source=readFileSync(new URL('../../'+policy.namingSource,import.meta.url),'utf8');
  const literal=readNodeDependencyPlacementLiteral(source);
  assert.deepEqual(JSON.parse(literal.text).slice(0,-1),JSON.parse(policy.originalPlacementLiteral));
  const previous=NAME_INVENTORIES.find(row=>row.inventory===policy.placement.inventory);assert.ok(previous);
  assert.deepEqual(previous.files,[policy.placement.destination]);
  const destination=readFileSync(new URL('../../'+policy.placement.destination,import.meta.url),'utf8');
  const oldNames=previous.extract(destination,policy.placement.destination);assert.equal(oldNames.length,3);
  const placement=NAME_INVENTORIES.find(row=>row.inventory==='B70 physical field source view names for '+policy.placement.destination);assert.ok(placement);
  const precedingDestination=restoreAutomatedTranslationPauseSource(destination,policy.placement.destination);
  const placedNames=placement.extract(precedingDestination,policy.placement.destination);assert.equal(placedNames.length,4);
  const historical=readNodeDependencyGeneratorRegistrationHistory().legacyNames;
  assert.equal(historical.length,2);
  assert.equal(new Set([...historical,...placedNames.slice(0,3).map(name=>placement.recordOf(name))]).size,5,'the two original and three preceding current qualified meanings remain distinct');
  for(const change of [destination+' ',destination.replace('const node=','const other=')])assert.throws(()=>previous.extract(change,policy.placement.destination));
  assert.throws(()=>previous.extract(destination,policy.placement.source));
});

test('placement literal extraction refuses missing ambiguous or altered owner expressions',()=>{
  assert.throws(()=>readNodeDependencyPlacementLiteral(''));
  const source=readFileSync(new URL('../scripts/issue-195-naming.mjs',import.meta.url),'utf8');
  const line=[...source.matchAll(/^export const NAME_INVENTORIES =[^\n]+$/gmu)][0][0];
  assert.throws(()=>readNodeDependencyPlacementLiteral(source+'\n'+line));
  assert.throws(()=>readNodeDependencyPlacementLiteral(source.replace(line,line.replace('.map(entry=>{const placement=','.map(entry=>{const other='))));
});

import {readNodeDependencyGeneratorRegistrationHistory} from '../scripts/node-dependency-registration-history.mjs';
