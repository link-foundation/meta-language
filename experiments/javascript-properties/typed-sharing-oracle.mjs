// CI-only original sources and exact value sharing observations.
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {assertVersions} from './controls.mjs';
import {sharingFixture,observeTypedSharing} from './typed-sharing-controls.mjs';
assertVersions();process.stdout.write(structuredDataToLino({...sharingFixture,programs:sharingFixture.programs.map(observeTypedSharing)}));
