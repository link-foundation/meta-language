// CI-only observations; never execute during the full requirements draft.
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {assertVersions,fixture,observePropertyCase,observeSameValueCase,observeInvalidValue,observeTextBoundary} from './controls.mjs';
assertVersions();
process.stdout.write(structuredDataToLino({cases:fixture.cases.map(observePropertyCase),sameValues:fixture.sameValues.map(observeSameValueCase),invalidValues:fixture.invalidValues.map(observeInvalidValue),textBoundaries:fixture.textBoundaries.map(observeTextBoundary)}));
