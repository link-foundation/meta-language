// CI-only complete frontend/product observations. Never run during source draft.
import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {parseRust} from '../../js/src/translation/rust.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {renderRustFunctionCall,renderRustConstructorFields,acceptRustNamedConstructorFields} from '../../js/src/translation/rust-record-rules.js';
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const fixture=readProjectData(new URL('../../parity/fixtures/rust-source-render-interfaces.lino',import.meta.url));
const calls=fixture.calls.map(row=>{const actual=renderRustFunctionCall(row.head,row.values);assert.equal(actual,row.expected);return {...row,actual};});
const constructors=fixture.constructors.map(row=>{const actual=renderRustConstructorFields(row.language,row.head,row.names,row.values);assert.equal(actual,row.expected);assert.equal(acceptRustNamedConstructorFields(row.language,row.names),row.named);return {...row,actual};});
const rows=fixture.programs.map(row=>{
 const surface=parseRust(row.source),program=checkProgram(surface),javascript=emitJavaScript(program),native=emitRust(program),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
 assert.equal(String(runInNewContext(javascript.text+'\n'+entry+'();')),row.expected,row.name);
 return {...row,surface,javascript,native};
});
process.stdout.write(structuredDataToLino({rows,calls,constructors}));
