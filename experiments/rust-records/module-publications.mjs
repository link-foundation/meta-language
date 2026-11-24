// Future public API products. Run only in CI after the complete source draft.
import {readProjectData} from '../../js/src/project-data-files.js';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {selfTranslate,selfTranslationSignatures} from '../../js/src/self-translation.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
const [major,minor]=process.versions.node.split('.').map(Number);if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/rust-module-values.lino',import.meta.url));
const publications=fixture.publications.map(row=>{const translation=selfTranslate(row.source,'Rust','JavaScript',{semanticTranslation:true});return {...row,code:translation.code,items:translation.items,signatures:selfTranslationSignatures(row.source,'Rust',{semanticTranslation:true})};});
const consumers=fixture.consumers.map(row=>{const program=checkProgram(parseJavaScript(row.source,{externals:row.externals})),emitted=emitJavaScript(program),packet=encodeTypedProgramLinks(program);return {...row,code:emitted.text,target:emitted.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target,root:packet.root.asU64(),links:packet.network.toLino(),emissions:Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]))};});
process.stdout.write(structuredDataToLino({schemaVersion:1,publications,consumers,versions:fixture.versions,boundary:'Actual generated public API products, no execution observations. Native controls must compare and execute them.'}));
