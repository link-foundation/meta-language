// Generated from the complete authenticated actual-source decorator.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readProjectData,serializeProjectData} from '../../../../js/scripts/project-data.mjs';
export const diagnosticOrderDecorator=readProjectData(new URL('./decorator.lino',import.meta.url));
assert.equal(createHash('sha256').update(serializeProjectData(diagnosticOrderDecorator,'decorator.lino')).digest('hex'),'0fcd04c15ea3515a8a4059cfb194c0eacdb3b2d4a3839ad85df3208505168699','complete source-derived decorator serialization');
