// Translate the complete shared decisions; native host fields remain adapters.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {writeRecordRuleLibrary} from './record-rule-library.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';

export function generateProgramStringRepresentationRules({check=false}={}) {
  const source=readFileSync(new URL('../src/translation/program-string-representation-rules.js',import.meta.url),'utf8');
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false})),emitted=emitRust(program,{arrayConsumption:true});
  assert.equal(emitted.preludes.length,0);assert.equal(emitted.definitions.length,2);
  const encoded=encodeTypedProgramLinks(program),text=encoded.network.toLino(),binary=encodeLinkNetworkBinary(encoded.network);
  for(const network of [LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)]){
    const replay=decodeTypedProgramLinks(network,encoded.root),actual=emitRust(replay,{arrayConsumption:true});
    assert.equal(actual.text,emitted.text,'both complete source-free program archives retain every native definition');
  }
  const policy={schemaVersion:1,model:'explicit program Unicode code unit string representation 1',
    field:'utf16Strings',nativeField:'utf16_strings',presence:'absent or an explicit boolean; false remains present',
    nodeRange:'>=22.0.0 <23.0.0 || >=24.0.0 <25.0.0',source,archive:{root:encoded.root.asU64().toString(),text,binaryHex:Buffer.from(binary).toString('hex')},
    declarations:[...program.declarations.keys()],boundary:'Shared boolean admission and encoding selection only. Existing target emitters provide all string operations. Host adapters preserve field presence and strict booleans; no native execution is inferred.'};
  const path=new URL('../../parity/self-translation/program-string-representation-rules.lino',import.meta.url);
  if(check)checkProjectData(path,policy);else writeProjectData(path,policy);
  const files=writeRecordRuleLibrary(emitted,'program_string_representation_rules',{check});
  return{policy,files};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateProgramStringRepresentationRules({check:process.argv.includes('--check')});
  console.log('Generated both program string-representation decisions from the complete shared checked JavaScript module and both Links replays.');
}
