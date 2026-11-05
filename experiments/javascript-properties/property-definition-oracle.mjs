// CI-only original definitions and all modeled intermediate observations.
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {assertVersions,definitionFixture,observePropertyDefinition,observeInvalidPropertyDefinition} from './property-definition-controls.mjs';
assertVersions();
process.stdout.write(structuredDataToLino({...definitionFixture,cases:definitionFixture.cases.map(observePropertyDefinition),invalid:definitionFixture.invalid.map(observeInvalidPropertyDefinition)}));
