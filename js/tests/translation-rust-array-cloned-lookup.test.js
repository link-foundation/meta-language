// Deferred controls; never execute while the complete requirements are drafted.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../scripts/project-data.mjs';
import {registerRustCallableProfile} from './helpers/rust-callable-profile.js';
import * as rules from '../src/translation/rust-array-lookup-rules.js';
import {projectRustArrayLookupElementOwner} from '../src/translation/rust-array-lookup-source-owners.js';
const root=new URL('../../',import.meta.url),data=readProjectData(new URL('parity/fixtures/rust-array-cloned-lookup.lino',root));
registerRustCallableProfile('rust-array-cloned-lookup');
test('actual standard clone owners defaults and original container projections remain explicit',()=>{
 for(const row of data.clonedElements)assert.equal(rules.readRustArrayLookupClonedElementError(row.kind,row.bits,row.signed,row.ownerKind,row.owner,row.children),row.expected);
 for(const row of data.defaultTerms)assert.equal(rules.readRustArrayLookupDefaultTerm(row.kind,row.standardString),row.expected);
 for(const row of data.leafProjections){assert.equal(row.depth,row.wrappers.length);let owner=structuredClone(row.leaf);for(const wrapper of row.wrappers)owner={...structuredClone(wrapper),arguments:[owner]};assert.deepEqual(projectRustArrayLookupElementOwner(owner),row.expected);}
});
test('every preceding copied scalar input retains its independent cloned expectation',()=>{
 const prior=readProjectData(new URL('parity/fixtures/rust-array-lookup.lino',root));
 for(const row of prior.cases)if(row.source.includes('.copied()')){const current=data.cases.find(value=>value.predecessorCase?.name===row.name);assert.ok(current);assert.equal(current.source,row.source.replaceAll('.copied()','.cloned()'));assert.equal(current.expected,row.expected);assert.deepEqual(current.checks,row.checks);}
});
test('full preceding generation recipes remain authenticated source definitions',()=>{
 for(const row of data.recipes){const bytes=readFileSync(new URL(row.path,root));assert.equal(bytes.length,row.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),row.sha256);const saved=readProjectData(new URL(row.path,root));for(const source of saved.sourceClosure.inputs){assert.equal(Buffer.byteLength(source.source),source.bytes);assert.equal(createHash('sha256').update(source.source).digest('hex'),source.sourceSha256);}}
});
