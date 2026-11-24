// CI producer. Never imported by source authoring.
import {serializeProjectData} from '../../js/scripts/project-data.mjs';
import {assertVersions,observeNaturalSource} from './call-natural-source-controls.mjs';
assertVersions();
process.stdout.write(serializeProjectData(observeNaturalSource(),'observations.lino'));
