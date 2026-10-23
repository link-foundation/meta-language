// Independent original-JavaScript execution, transported as Links data for CI.
// This payload is an oracle input, not a requirement-observation record.
import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import {fileURLToPath} from 'node:url';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {selfTranslate,selfTranslationSignatures} from '../../js/src/self-translation.js';
import {createTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {createNumberBitwiseProgramLinks} from '../../js/src/number-bitwise-program.js';

export function readNumberBitwiseOraclePacket(){
 const [major,minor]=process.versions.node.split('.').map(Number);
 assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
 const fixture=readProjectData(new URL('../../parity/fixtures/number-bitwise.lino',import.meta.url));
 assert.equal(fixture.operations.length,7);assert.equal(fixture.valueSources.length,30);
 const functions=runInNewContext(fixture.source.replace(/^export /gmu,'')+'\n['+fixture.operations.map(row=>row.name).join(',')+'];',{}, {timeout:1000});
 const values=fixture.valueSources.map(source=>runInNewContext(source,{}, {timeout:1000}));
 const bits=value=>{const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value,false);return view.getBigUint64(0,false).toString(16).padStart(16,'0');};
 const expected=[];
 for(const observe of functions)for(const left of values)for(const right of values)expected.push(bits(observe(left,right)));
 assert.equal(expected.length,6300,'every declared boundary cell executes before returning a result');
 const packet=createTypedProgramLinks(fixture.source,'JavaScript'),definition=createNumberBitwiseProgramLinks();
 return{model:fixture.model,runtime:{name:'Node.js',version:process.versions.node},source:fixture.source,values:values.map(bits),expected,
  translation:selfTranslate(fixture.source,'JavaScript','Rust'),signatures:selfTranslationSignatures(fixture.source,'JavaScript'),
  program:{links:packet.network.toLino(),root:packet.root.asU64(),emission:emitTypedProgramLinks(packet.network,packet.root,'Rust')},
  definition:{links:definition.network.toLino(),root:definition.root.asU64(),emission:emitTypedProgramLinks(definition.network,definition.root,'Rust')}};
}
if(process.argv[1]===fileURLToPath(import.meta.url))process.stdout.write(structuredDataToLino(readNumberBitwiseOraclePacket()));
