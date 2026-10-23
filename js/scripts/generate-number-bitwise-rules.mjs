// Both native decisions and emitted runtime source derive from this checked
// Number algorithm; no native arithmetic is supplied separately.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {DecoratorSet} from '../src/decorators.js';
import {writeRecordRuleLibrary} from './record-rule-library.mjs';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
import {writeProjectData,checkProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url),pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

export function generateNumberBitwiseRules({check=false}={}){
 const path='js/src/translation/number-bitwise-rules.js',source=readFileSync(new URL(path,root),'utf8');
 const program=checkProgram(parseJavaScript(source,{referenceSemantics:false})),emitted=emitRust(program,{arrayConsumption:true});
 const packet=encodeTypedProgramLinks(program),text=packet.network.toLino(),replays=[];
 for(const[format,network]of[['text',LinkNetwork.fromLino(text)],['binary',decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]]){
  const checked=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}),replay=emitRust(checked,{arrayConsumption:true});
  assert.deepEqual([...checked.declarations.keys()],[...program.declarations.keys()]);
  for(const field of['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(replay[field],emitted[field]);
  replays.push({format,...pin(replay.text),owners:[...checked.declarations.keys()]});
 }
 const files=writeRecordRuleLibrary(emitted,'number_bitwise_rules',{check});
 const header=emitted.text.split('\n').find(line=>line.startsWith('#![allow('));assert.ok(header);
 const raw=[header,...emitted.preludes,...emitted.definitions].join('\n\n');
 const styles=DecoratorSet.fromLino(readFileSync(new URL('parity/self-translation/number-bitwise-rules-decorators.lino',root),'utf8'));
 const flat=formatProductRuleSource(styles.decorate('emitter',{format:'Rust',source:raw}).source);
 const rootHeader=files[0].source.indexOf('#[path =');assert.ok(rootHeader>0,'the native partition owns its complete reviewed library attributes');
 const prelude=formatProductRuleSource('pub mod ml_number_bitwise {\n'+files[0].source.slice(0,rootHeader)+'\n'+flat+'\n}\n');
 assert.equal(text.includes('"####'),false,'the generated Links carrier has an unambiguous Rust string boundary');
 const rootIdentity=packet.root.asU64();
 const carrier='// Generated Links Notation transport for the complete checked Number definition.\n'
  +'pub(crate) const ROOT: u64 = '+rootIdentity+';\n'
  +'pub(crate) const LINKS: &str = r####"'+text+'"####;\n';
 const outputs=[
  ['rust/src/translation/runtime/number-bitwise.rs',prelude],
  ['js/src/translation/number-bitwise-runtime.js','// Generated from the complete checked Number bitwise program. Do not edit.\nexport const NUMBER_BITWISE_PRELUDE = '+JSON.stringify(prelude)+';\nexport const NUMBER_BITWISE_PROGRAM_ROOT = '+rootIdentity+';\nexport const NUMBER_BITWISE_PROGRAM_LINKS = '+JSON.stringify(text)+';\n'],
  ['rust/src/translation/number_bitwise_program_data.rs',carrier],
  ['js/src/data/number-bitwise-program.lino',text],
 ];
 for(const[file,value]of outputs){
  if(check)assert.equal(readFileSync(new URL(file,root),'utf8'),value,'every complete generated Number artifact is fresh');
  else writeFileSync(new URL(file,root),value);
 }
 const symbols=['&','|','^','<<','>>','>>>','~'];
 const operations=symbols.map((symbol,index)=>({symbol,name:'compute'+index}));
 const valueSources=['0','-0','NaN','Infinity','-Infinity','1','-1','1.9','-1.9','31','32','33','-33','0.5','-0.5',
  '2147483647','2147483648','2147483649','4294967295','4294967296','4294967297','-2147483649','-4294967296','-4294967297',
  '9007199254740991','9007199254740992','Number.MAX_VALUE','-Number.MAX_VALUE','Number.MIN_VALUE','-Number.MIN_VALUE'];
 const fixture={schemaVersion:1,model:'complete Number operator boundary product 1',operations,valueSources,
  source:operations.map(({symbol,name})=>'/** @param {number} left @param {number} right @returns {number} */\nexport function '+name+'(left,right){return '+(symbol==='~'?'~left':'left '+symbol+' right')+';}\n').join('\n'),
  assertions:['every actual original operator result','complete JavaScript/native translation parity','one generated runtime','exact source restoration','both typed-program codecs','all declared boundary pairs executed'],
  assertionVersions:{Node:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0'},
  boundary:'Seven original source operators and thirty independent Number input sources declare the complete 6300-cell product. Tests execute the original JavaScript functions to obtain observations, then compare each generated result. This fixture is input and oracle declaration, not execution evidence.'};
 const fixtureFile=new URL('parity/fixtures/number-bitwise.lino',root);
 if(check)checkProjectData(fixtureFile,fixture);else writeProjectData(fixtureFile,fixture);
 const artifact={schemaVersion:1,model:'ECMAScript Number bitwise operations 1',source:{path,source,...pin(source)},
  archive:{file:'js/src/data/number-bitwise-program.lino',root:packet.root.asU64(),...pin(text)},owners:[...program.declarations.keys()],replays,
  emitted:{text:emitted.text,definitions:emitted.definitions,preludes:emitted.preludes,mappings:emitted.mappings,assumptions:emitted.assumptions,encodings:emitted.encodings},
  files:[...files.map(file=>({path:'rust/src/translation/'+(file.path??'number_bitwise_rules.rs'),source:file.source,...pin(file.source)})),...outputs.map(([path,source])=>({path,source,...pin(source)}))],
  boundary:'The complete Number algorithm generates its native library and emitted runtime automatically. Both source-free typed-program codecs recheck every owner and all complete emission fields. This is generation and preservation, not native execution evidence. Syntax admission, checked Number-only arguments, exact arity, operand order and wire validation remain mandatory before translation support is claimed.'};
 const output=new URL('parity/self-translation/number-bitwise-rules-native.lino',root);
 if(check)checkProjectData(output,artifact);else writeProjectData(output,artifact);
 return artifact;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 assert.ok(process.argv.slice(2).every(value=>value==='--check'));
 const value=generateNumberBitwiseRules({check:process.argv.includes('--check')});console.log('Generated '+value.owners.length+' complete checked Number owners and both archive replays.');
}
