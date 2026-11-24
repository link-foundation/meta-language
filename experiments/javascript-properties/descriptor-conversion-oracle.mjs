// CI-only observations, never execute during the whole requirements draft.
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {assertVersions,conversionFixture,observeDescriptorConversion,observeSemanticBoolean,observeInvalidDescriptorState} from './descriptor-conversion-controls.mjs';
assertVersions();
process.stdout.write(structuredDataToLino({cases:conversionFixture.cases.map(observeDescriptorConversion),booleans:conversionFixture.booleans.map(observeSemanticBoolean),invalid:conversionFixture.invalid.map(observeInvalidDescriptorState),invalidHeap:conversionFixture.invalidHeap}));
