// Deferred complete controls. Never execute during the drafting phase.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../scripts/project-data.mjs';
import {registerRustCallableProfile} from './helpers/rust-callable-profile.js';
import * as rules from '../src/translation/rust-primitive-function-value-rules.js';
const root=new URL('../../',import.meta.url),fixture=readProjectData(new URL('parity/fixtures/rust-contextual-from-function-value.lino',root));
registerRustCallableProfile('rust-contextual-from-function-value');
test('all actual contextual primitive From source facts remain explicit',()=>{
 for(const row of fixture.sourceControls)assert.equal(rules.readRustPrimitiveFromFunctionSource(row.target,row.parameters,row.kind,row.bits,row.signed,row.ownerKind,row.source,row.children,row.proven,row.layers),row.expected);
});
test('corrected function-value refusals preserve all preceding inputs scopes and unaffected assertions',()=>{
 for(const retained of fixture.retained){
  const bytes=readFileSync(new URL(retained.path,root));assert.equal(bytes.length,retained.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),retained.sha256);
  const before=readProjectData(new URL(retained.path,root)),after=readProjectData(new URL('parity/fixtures/'+retained.fixture+'.lino',root));
  for(const [field,value]of Object.entries(before))if(!['refusals','assertionCorrections'].includes(field))assert.deepEqual(after[field],value);
  for(const prior of before.assertionCorrections??[])assert.ok(after.assertionCorrections.some(row=>JSON.stringify(row)===JSON.stringify(prior)));
  for(const prior of before.refusals){const kept=after.refusals.find(row=>row.name===prior.name);if(kept){assert.deepEqual(kept,prior);continue;}
   const positive=fixture.cases.find(row=>row.retainedRefusal?.fixture===retained.fixture&&row.retainedRefusal.source===prior.source);assert.ok(positive);assert.ok(positive.source.startsWith(prior.source));assert.deepEqual(positive.retainedRefusal,{fixture:retained.fixture,source:prior.source,kind:prior.kind,reason:prior.reason});
   assert.ok(after.assertionCorrections.some(row=>row.input===prior.source&&row.priorReason===prior.reason&&row.retainedFixture===retained.path));
  }
 }
});

test('complete preceding generation recipes remain authenticated source history',()=>{
 for(const row of fixture.recipes){const bytes=readFileSync(new URL(row.path,root));assert.equal(bytes.length,row.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),row.sha256);const prior=readProjectData(new URL(row.path,root));assert.ok(prior.sourceClosure.inputs.length>0);for(const source of prior.sourceClosure.inputs){assert.equal(Buffer.byteLength(source.source),source.bytes);assert.equal(createHash('sha256').update(source.source).digest('hex'),source.sourceSha256);}}
});
