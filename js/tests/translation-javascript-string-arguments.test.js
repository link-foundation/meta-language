// Deferred shared argument-policy controls; source behavior uses the call suite.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readProjectData} from '../scripts/project-data.mjs';
import {planJavaScriptStringArguments} from '../src/translation/javascript-string-argument-rules.js';
import {assertVersions} from '../../experiments/javascript-properties/controls.mjs';
assertVersions();
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-string-arguments.lino',import.meta.url));
for(const row of fixture.rows)test(row.name,()=>assert.deepEqual(planJavaScriptStringArguments(row.count),row.expected));
