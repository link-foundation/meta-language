// CI-only original computed keys and complete modeled transition observations.
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {assertVersions,keyFixture,observePropertyKeyConversion,observeInvalidPropertyKeyState} from './property-key-conversion-controls.mjs';
assertVersions();
process.stdout.write(structuredDataToLino({...keyFixture,cases:keyFixture.cases.map(observePropertyKeyConversion),invalid:keyFixture.invalid.map(observeInvalidPropertyKeyState)}));
