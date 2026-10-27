// Deferred original/frontend observations. Never execute during source drafting.
import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-data-fields.lino',import.meta.url));
const rows=fixture.cases.map(row=>{
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.value,row.name);
 const surface=parseJavaScript(row.source),program=checkProgram(surface);
 return {...row,surface,javascript:emitJavaScript(program),native:emitRust(program)};
});
process.stdout.write(structuredDataToLino({rows,refusals:fixture.refusals,plans:fixture.plans}));
