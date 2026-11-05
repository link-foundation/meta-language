// Future executable producer. Never invoke during the complete source draft.
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {parseRust} from '../../js/src/translation/rust.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {createRustFormattingProgramLinks} from '../../js/src/rust-formatting-program.js';
const [major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/rust-function-output.lino',import.meta.url));
const surface=parseRust(fixture.source),program=checkProgram(surface),packet=encodeTypedProgramLinks(program),definition=createRustFormattingProgramLinks();
const output=emitJavaScript(program),observedLines=[];
runInNewContext(output.text,{console:{log:(...values)=>observedLines.push(values.map(String).join(' '))}},{timeout:1000});
process.stdout.write(structuredDataToLino({...fixture,surface,observedLines,root:packet.root.asU64(),links:packet.network.toLino(),definitionRoot:definition.root.asU64(),definitionLinks:definition.network.toLino(),emissions:Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)])),definitionEmissions:Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(definition.network,definition.root,target)]))}));
