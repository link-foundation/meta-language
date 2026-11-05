// Future product producer; actual source execution remains deferred.
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {createRustSourceOwnerProgramLinks} from '../../js/src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
const [major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/rust-constructed-string-interface.lino',import.meta.url));
const original=createRustSourceOwnerProgramLinks(fixture.roundTripSource),program=decodeTypedProgramLinks(original.network,original.root),emission=emitRust(program,{sourceHeaderModel:fixture.model,arrayConsumption:true});
const source=emission.definitions.join('\n\n')+'\nfn main(){}',restored=createRustSourceOwnerProgramLinks(source),reparsed=decodeTypedProgramLinks(restored.network,restored.root),selected=emitRust(reparsed,{sourceHeaderModel:fixture.model,arrayConsumption:true});
const javascript=emitTypedProgramLinks(restored.network,restored.root,'JavaScript'),name=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;
const value=String(runInNewContext(javascript.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000}));
const header=selected.text.split('\n').find(line=>line.startsWith('#![allow('));
if(!header)throw new TypeError('complete native library attributes are required');
process.stdout.write(structuredDataToLino({source,root:restored.root.asU64(),links:restored.network.toLino(),value,nativeLibrary:[header,...selected.preludes,...selected.definitions].join('\n\n'),emissions:Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(restored.network,restored.root,target)]))}));
