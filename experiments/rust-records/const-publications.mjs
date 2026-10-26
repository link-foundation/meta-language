// Future public API products only; run after complete drafting, in CI.
import {readProjectData} from '../../js/src/project-data-files.js';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {selfTranslate,selfTranslationSignatures} from '../../js/src/self-translation.js';
const [major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/rust-function-qualifiers.lino',import.meta.url));
const publications=fixture.publications.map(row=>{const result=selfTranslate(row.source,'Rust','JavaScript',{semanticTranslation:true});return {...row,...result,signatures:selfTranslationSignatures(row.source,'Rust',{semanticTranslation:true})};});
process.stdout.write(structuredDataToLino({publications,versions:fixture.versions,boundary:'Actual translated products; no execution observations.'}));
