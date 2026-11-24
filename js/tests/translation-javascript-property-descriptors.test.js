// Deferred source-model/oracle controls. Never execute during the full draft.
import test from 'node:test';
import {assertVersions,fixture,observePropertyCase,observeSameValueCase,observeInvalidValue,observeTextBoundary} from '../../experiments/javascript-properties/controls.mjs';
assertVersions();
for(const row of fixture.cases)test(row.name,()=>observePropertyCase(row));
for(const row of fixture.sameValues)test(row.name,()=>observeSameValueCase(row));
for(const row of fixture.invalidValues)test(row.name,()=>observeInvalidValue(row));
for(const row of fixture.textBoundaries)test('complete UTF16 source budget '+row.units,()=>observeTextBoundary(row));
