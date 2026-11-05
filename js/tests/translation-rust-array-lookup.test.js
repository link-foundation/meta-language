// Deferred complete source/transport/native controls; never run while drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {registerRustCallableProfile} from './helpers/rust-callable-profile.js';
import * as rules from '../src/translation/rust-array-lookup-rules.js';
const data=readProjectData(new URL('../../parity/fixtures/rust-array-lookup.lino',import.meta.url));
registerRustCallableProfile('rust-array-lookup');
test('each array lookup source arity consumer receiver owner and lazy plan remains explicit',()=>{
 for(const row of data.arities)assert.equal(rules.readRustArrayLookupArity(row.method,row.ordinary),row.expected);
 for(const row of data.consumers){assert.equal(rules.acceptRustArrayLookupPresenceConsumer(row.consumer),['is_some','is_none'].includes(row.consumer));assert.equal(rules.readRustArrayLookupConsumerArity(row.consumer,row.copied),row.expected);}
 for(const row of data.receivers)assert.equal(rules.readRustArrayLookupReceiverError(row.kind,row.element,row.model,row.proven,row.immutable,row.copy,row.copied),row.expected);
 for(const row of data.elementOwners)assert.equal(rules.readRustArrayLookupElementOwnerError(row.kind,row.bits,row.signed,row.owner),row.expected);
 for(const row of data.plans){assert.deepEqual(rules.planRustArrayLookupCondition(row.method),row.condition);assert.deepEqual(rules.planRustArrayLookupIndex(row.method),row.index);}
 for(const row of data.consumerPlans)assert.deepEqual(rules.planRustArrayLookupConsumer(row.consumer),row.expected);
});
