// CI-only original source and complete source-machine transition observations.
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {assertVersions,machineFixture,observeCallMachine,observeInvalidCallProgram,observeInvalidCallMachine,observeSourceErrorResumptionRefusal} from './call-machine-controls.mjs';
import {sourceExceptionFixture,observeCallSourceException} from './call-source-exception-controls.mjs';
assertVersions();
process.stdout.write(structuredDataToLino({...machineFixture,programs:[...machineFixture.programs.map(observeCallMachine),...sourceExceptionFixture.programs.map(observeCallSourceException)],invalid:machineFixture.invalid.map(observeInvalidCallProgram),invalidStates:machineFixture.invalidStates.map(observeInvalidCallMachine),errorResumptionRefusals:machineFixture.errorResumptionRefusals.map(observeSourceErrorResumptionRefusal)}));
