// Deferred actual shared construction, all declared policy assertions and codecs.
import assert from 'node:assert/strict';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import * as source from '../../js/src/translation/javascript-call-natural-source.js';
import {decodeJavaScriptCallSource,encodeJavaScriptCallSource,encodeJavaScriptCallSourceCompilation} from './call-natural-source-codecs.mjs';
import {assertVersions} from './controls.mjs';
export {assertVersions};
export const naturalSourceFixture=readProjectData(new URL('../../parity/fixtures/javascript-call-natural-source.lino',import.meta.url));
export function observeNaturalSource(){
 for(const row of naturalSourceFixture.guardOrders)assert.deepEqual(source.planJavaScriptNaturalGuardOrder(row.types,row.orders,row.oneRealm),row.expected,row.name);
 for(const row of naturalSourceFixture.casts)assert.equal(source.readJavaScriptNaturalCastPlan(row.from,row.to,row.flavor,row.retainedMessage,row.order,row.oneRealm),row.expected,row.name);
 for(const row of naturalSourceFixture.patterns)assert.deepEqual(source.planJavaScriptNaturalPatternBranches(row.kind,row.patterns),row.expected,row.name);
 for(const row of naturalSourceFixture.bindings)assert.equal(source.readJavaScriptNaturalPatternBinding(row.pattern),row.expected,row.pattern);
 for(const row of naturalSourceFixture.realms)assert.equal(source.testJavaScriptNaturalSourceRealm(row.realms),row.expected);
 const builders=naturalSourceFixture.builders.map(row=>{
  const message=(row.messageUnits??[]).map(unit=>String.fromCharCode(unit)).join('');let actual;
  if(row.operation==='error')actual=source.constructJavaScriptLiteralErrorSource(message);
  else if(row.operation==='guard')actual=source.constructJavaScriptNaturalGuardSource(row.identity,message,decodeJavaScriptCallSource(row.body));
  else if(row.operation==='cast')actual=source.constructJavaScriptNaturalCastSource(row.identity,message,row.clamp);
  else if(row.operation==='difference')actual=source.constructJavaScriptNaturalDifferenceSource(row.left,row.right);
  else if(row.operation==='predecessor')actual=source.constructJavaScriptNaturalPredecessorSource(row.identity);
  else if(row.operation==='match')actual=source.constructJavaScriptNaturalMatchSource(row.identity,decodeJavaScriptCallSource(row.zero),decodeJavaScriptCallSource(row.succ));
  else throw new TypeError('every source builder must have its actual control');
  const wire=encodeJavaScriptCallSource(actual);assert.deepEqual(encodeJavaScriptCallSource(decodeJavaScriptCallSource(wire)),wire,row.name);
  const compilation=source.compileJavaScriptNaturalSource(actual,row.parameters);assert.equal(compilation.error,'',row.name);
  return {...row,source:wire,compilation:encodeJavaScriptCallSourceCompilation(compilation)};
 });
 return {...naturalSourceFixture,builders};
}
