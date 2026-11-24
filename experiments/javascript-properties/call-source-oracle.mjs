import {observeAllCallSourceControls,observeAllCallSourceControlRefusals} from './call-source-control-controls.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {assertVersions,callSourceFixture,observeCallSource,observeCallSourceRefusal} from './call-source-controls.mjs';
import {sourceExceptionFixture,observeCallSourceException,observeCallSourceExceptionRefusal} from './call-source-exception-controls.mjs';
assertVersions();
for(const row of callSourceFixture.refusals)observeCallSourceRefusal(row);
process.stdout.write(structuredDataToLino({schemaVersion:1,rows:[...callSourceFixture.rows.map(observeCallSource),...sourceExceptionFixture.programs.map(observeCallSourceException),...observeAllCallSourceControls()],sourceRefusals:[...sourceExceptionFixture.refusals.map(observeCallSourceExceptionRefusal),...observeAllCallSourceControlRefusals()]}));
