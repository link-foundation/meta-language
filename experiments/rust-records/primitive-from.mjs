// Future independent product producer. Do not execute during drafting.
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {createRustSourceOwnerProgramLinks} from '../../js/src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
const [major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/rust-primitive-from.lino',import.meta.url));
const packet=createRustSourceOwnerProgramLinks(fixture.source),program=decodeTypedProgramLinks(packet.network,packet.root),selected=emitRust(program,{sourceHeaderModel:fixture.model,arrayConsumption:true});
const js=emitTypedProgramLinks(packet.network,packet.root,'JavaScript');
const names=fixture.cases.map(row=>js.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.module+'.result').target);
const functions=runInNewContext(js.code.replace(/^export /gmu,'')+'\n['+names.join(',')+'];',{}, {timeout:1000});
const observations=fixture.cases.map((row,index)=>{const value=functions[index]();return row.resultType==='()'?{unit:value===null,value:'()'}:{unit:false,value:String(value)};});
const header=selected.text.split('\n').find(line=>line.startsWith('#![allow('));
if(!header)throw new TypeError('complete native library attributes are required');
const printedProducts=fixture.printedSources.map(row=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),emission=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),lines=[];
 runInNewContext(emission.code.replace(/^export /gmu,''),{console:{log:(...values)=>lines.push(values.map(String).join(' '))}},{timeout:1000});
 return {name:row.name,lines,native:emitTypedProgramLinks(packet.network,packet.root,'Rust')};
});
process.stdout.write(structuredDataToLino({printedProducts,root:packet.root.asU64(),links:packet.network.toLino(),observations,nativeLibrary:[header,...selected.preludes,...selected.definitions].join('\n\n'),emission:emitTypedProgramLinks(packet.network,packet.root,'Rust',{rustSourceHeaderEmissionModel:fixture.model})}));
