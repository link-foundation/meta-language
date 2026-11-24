// CI-only original coercions and complete modeled transition observations.
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {assertVersions,primitiveFixture,observePrimitiveConversion,observeInvalidPrimitiveState} from './primitive-conversion-controls.mjs';
assertVersions();
process.stdout.write(structuredDataToLino({...primitiveFixture,cases:primitiveFixture.cases.map(observePrimitiveConversion),invalid:primitiveFixture.invalid.map(observeInvalidPrimitiveState)}));
