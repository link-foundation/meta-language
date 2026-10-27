// CI-only producer. Never invoke during the complete requirements draft phase.
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {selfTranslate} from '../../js/src/self-translation.js';
const [major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-closed-classes.lino',import.meta.url));
const rows=[];
for(const row of fixture.cases){
 const expected=String(runInNewContext('"use strict";\n'+row.source+'\ncompute();',{}, {timeout:1000}));
 if(expected!==row.value)throw new TypeError('independent original source disagrees with retained expectation: '+row.name);
 const surface=parseJavaScript(row.source),program=checkProgram(surface),javascript=emitJavaScript(program),native=emitRust(program);
 const translation=selfTranslate(row.source,'JavaScript','Rust',{semanticTranslation:true});
 if(!translation.items.every(item=>['translated','comment','provenance'].includes(item.status)))throw new TypeError('the complete class source was carried: '+row.name);
 rows.push({name:row.name,source:row.source,resultType:row.resultType,expected,surface,javascript,native,translation});
}
process.stdout.write(structuredDataToLino({rows}));
