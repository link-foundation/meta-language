// CI-only original source and complete source-machine transition observations.
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {assertVersions,machineFixture,observeCallMachine,observeInvalidCallProgram,observeInvalidCallMachine} from './call-machine-controls.mjs';
assertVersions();
process.stdout.write(structuredDataToLino({...machineFixture,programs:machineFixture.programs.map(observeCallMachine),invalid:machineFixture.invalid.map(observeInvalidCallProgram),invalidStates:machineFixture.invalidStates.map(observeInvalidCallMachine)}));
