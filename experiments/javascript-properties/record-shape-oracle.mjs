import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {assertVersions,recordShapeFixture,observeRecordShape,observeRecordShapePredecessors} from './record-shape-controls.mjs';
assertVersions();observeRecordShapePredecessors();
process.stdout.write(structuredDataToLino({schemaVersion:1,model:recordShapeFixture.model,programs:recordShapeFixture.programs.map(observeRecordShape)}));
