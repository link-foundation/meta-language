// CI-only original programs and complete frontend/emission observations.
import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {readJavaScriptNominalSpreadError,readJavaScriptNominalSpreadGuardArm} from '../../js/src/translation/javascript-nominal-spread-rules.js';
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-nominal-spreads.lino',import.meta.url));
const policies=fixture.policies.map(row=>{const actual=readJavaScriptNominalSpreadError(row.language,row.constructors,row.declared,row.kinds,row.names,row.owners,row.owner,row.prototype);assert.equal(actual,row.expected);return {...row,actual};});
const guards=fixture.guards.map(row=>{const actual=readJavaScriptNominalSpreadGuardArm(row.kind,row.operation,row.absence);assert.equal(actual,row.expected);return {...row,actual};});
const rows=fixture.programs.map(row=>{
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.expected,row.name);
 const surface=parseJavaScript(row.source),program=checkProgram(surface),javascript=emitJavaScript(program),native=emitRust(program),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
 assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),row.expected,row.name);return {...row,surface,javascript,native};
});
for(const row of fixture.refusals)assert.throws(()=>checkProgram(parseJavaScript(row.source)),undefined,row.name);
process.stdout.write(structuredDataToLino({...fixture,policies,guards,rows}));
