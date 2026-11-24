// Future actual IEEE values and product controls, serialized as Links data.
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {createRustSourceOwnerProgramLinks} from '../../js/src/rust-source-owner-program.js';
import {emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
const [major,minor]=process.versions.node.split('.').map(Number);if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/rust-scalar-vector-copies.lino',import.meta.url)),controls=fixture.controls,packet=createRustSourceOwnerProgramLinks(controls.source);
const emissions=Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
const names=controls.names.map(name=>emissions.JavaScript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===name).target);
const functions=runInNewContext(emissions.JavaScript.code.replace(/^export /gmu,'')+'\n['+names.join(',')+'];',{}, {timeout:1000});
const tag=value=>Number.isNaN(value)?'NaN':Object.is(value,-0)?'-0':String(value);
const observations=functions.map(operation=>{const input=Object.freeze(controls.values.map(Number)),first=operation(input),second=operation(input);return{fromInput:first!==input,separate:first!==second,input:Array.from(input,tag),first:Array.from(first,tag),second:Array.from(second,tag)};});
process.stdout.write(structuredDataToLino({root:packet.root.asU64(),links:packet.network.toLino(),emissions,observations}));
