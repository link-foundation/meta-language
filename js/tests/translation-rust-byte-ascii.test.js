// Deferred complete source/codec/product controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {registerRustCallableProfile} from './helpers/rust-callable-profile.js';
import * as rules from '../src/translation/rust-byte-ascii-rules.js';
const data=readProjectData(new URL('../../parity/fixtures/rust-byte-ascii.lino',import.meta.url));
registerRustCallableProfile('rust-byte-ascii');
test('every actual standard byte predicate source premise and term remains explicit',()=>{
 for(const row of data.arities)assert.equal(rules.readRustByteAsciiArity(row.method,row.ordinary),row.expected);
 for(const row of data.receivers)assert.equal(rules.readRustByteAsciiReceiverError(row.kind,row.bits,row.signed,row.ownerKind,row.owner,row.children,row.proven,row.immutable),row.expected);
 for(const [method,terms]of Object.entries(data.terms))assert.deepEqual(rules.planRustByteAscii(method),terms);
});
