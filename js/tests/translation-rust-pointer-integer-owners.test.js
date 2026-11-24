// Future actual shared decisions; complete products use the common matrix.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readRustPointerIntegerOwnerError,readRustPointerIntegerOwnerJoinError,selectRustPointerIntegerResultOwner,acceptRustPointerIntegerOwnerChild} from '../src/translation/rust-pointer-integer-owner-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-pointer-integer-owners.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.decisions)test('actual integer source contract: '+JSON.stringify(row),()=>assert.equal(readRustPointerIntegerOwnerError(row.actual,row.expected),row.error));
for(const row of fixture.joins)test('actual source owner join: '+JSON.stringify(row),()=>{assert.equal(readRustPointerIntegerOwnerJoinError(row.owners),row.error);assert.equal(selectRustPointerIntegerResultOwner(row.owners),row.selected);});
for(const row of fixture.containers)test('actual standard owner projection: '+JSON.stringify(row),()=>assert.equal(acceptRustPointerIntegerOwnerChild(row.kind,row.path,row.count),row.expected));
