import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {normalizeJavaScriptUnsignedInteger,normalizeJavaScriptSignedInteger,readJavaScriptNumberBitwiseOperationCode,readJavaScriptNumberBitwiseOperation,applyJavaScriptNumberBitwiseOperation} from '../src/translation/number-bitwise-rules.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {runInNewContext} from 'node:vm';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {selfTranslate} from '../src/self-translation.js';
import {createNumberBitwiseProgramLinks} from '../src/number-bitwise-program.js';
import {readProjectData} from '../scripts/project-data.mjs';
import {tokenize} from '../src/translation/lexer.js';
import {readNumberBitwiseOraclePacket,readNumberBitwiseAssignmentOraclePacket} from '../../experiments/number-bitwise/oracle.mjs';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const operations=[['&',(left,right)=>left&right],['|',(left,right)=>left|right],['^',(left,right)=>left^right],['<<',(left,right)=>left<<right],['>>',(left,right)=>left>>right],['>>>',(left,right)=>left>>>right],['~',left=>~left]];
const values=[0,-0,NaN,Infinity,-Infinity,1,-1,1.9,-1.9,31,32,33,-33,0.5,-0.5,2147483647,2147483648,2147483649,4294967295,4294967296,4294967297,-2147483649,-4294967296,-4294967297,9007199254740991,9007199254740992,Number.MAX_VALUE,-Number.MAX_VALUE,Number.MIN_VALUE,-Number.MIN_VALUE];

test('Number integer normalization agrees with actual operators including nonfinite values and signed zero',()=>{
 for(const value of values){
  assert.equal(normalizeJavaScriptUnsignedInteger(value),value>>>0,String(value));
  assert.equal(normalizeJavaScriptSignedInteger(value),value|0,String(value));
 }
});

test('all seven Number operators agree with the active JavaScript runtime on the complete boundary product',()=>{
 for(const [position,[symbol,observe]]of operations.entries()){
  const name=readJavaScriptNumberBitwiseOperation(symbol),code=readJavaScriptNumberBitwiseOperationCode(name);
  assert.equal(code,position);
  for(const left of values)for(const right of values){
   assert.equal(applyJavaScriptNumberBitwiseOperation(code,left,right),observe(left,right),symbol+' '+left+' '+right);
  }
 }
 for(const symbol of ['', '&&','||','**','&=','>>>='])assert.equal(readJavaScriptNumberBitwiseOperation(symbol),'');
 for(const operation of ['', 'numberbitwiseConjunction','bitwiseAnd','unknown'])assert.equal(readJavaScriptNumberBitwiseOperationCode(operation),-1);
 for(const operation of [-1,7,0.5,NaN,Infinity])assert.equal(Number.isNaN(applyJavaScriptNumberBitwiseOperation(operation,1,2)),true);
});

test('the complete Number algorithm checks and emits Rust without a handwritten replacement',()=>{
 const source=readFileSync(new URL('../src/translation/number-bitwise-rules.js',import.meta.url),'utf8');
 const program=checkProgram(parseJavaScript(source)),emitted=emitRust(program,{arrayConsumption:true});
 for(const name of ['normalize_java_script_unsigned_integer','normalize_java_script_signed_integer','apply_java_script_number_bitwise_operation'])assert.ok(emitted.text.includes('pub fn '+name+'('),name);
 assert.ok(emitted.definitions.length>=5);
});

const functionSource=(symbol,name='compute')=>'/** @param {number} left @param {number} right @returns {number} */\nexport function '+name+'(left,right){return '+(symbol==='~'?'~left':'left '+symbol+' right')+';}\n';
const evaluate=program=>runInNewContext(emitJavaScript(program).text.replace(/^export /gmu,'')+'\ncompute;',{}, {timeout:1000});

test('actual syntax and checked JavaScript emission retain every Number operator boundary result',()=>{
 for(const [symbol,observe]of operations){
  const program=checkProgram(parseJavaScript(functionSource(symbol))),execute=evaluate(program);
  for(const left of values)for(const right of values)assert.equal(execute(left,right),observe(left,right),symbol+' '+left+' '+right);
 }
});

test('bitwise precedence associativity and unary binding agree with independent JavaScript execution',()=>{
 for(const expression of ['left | right ^ 3 & 12 + 1 << 2','left << right >> 1 >>> 2','~left << right','(left | right) && (left ^ right)','left & right ? 11 : 17']){
  const source='/** @param {number} left @param {number} right @returns {number} */\nexport function compute(left,right){return '+expression+';}\n';
  const execute=evaluate(checkProgram(parseJavaScript(source))),observe=runInNewContext('(left,right)=>('+expression+')');
  for(const left of values)for(const right of [0,1,2,31,32,-1])assert.equal(execute(left,right),observe(left,right),expression+' '+left+' '+right);
 }
});

test('both Links codecs recheck the complete new operations without changing the base schema',()=>{
 for(const [symbol]of operations){
  const program=checkProgram(parseJavaScript(functionSource(symbol))),original=emitRust(program),packet=encodeTypedProgramLinks(program);
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
   const checked=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}),emitted=emitRust(checked);
   for(const field of ['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(emitted[field],original[field]);
   assert.equal(evaluate(checked)(-1,33),operations.find(row=>row[0]===symbol)[1](-1,33));
  }
 }
});

test('whole module translation shares its generated runtime and restores every original byte',()=>{
 const source=operations.map(([symbol],index)=>functionSource(symbol,'compute'+index)).join('\n'),result=selfTranslate(source,'JavaScript','Rust',{retainDiagnostics:true});
 assert.ok(result.items.every(item=>item.status==='translated'||item.status==='comment'),JSON.stringify(result.items));
 assert.equal(result.code.split('pub mod ml_number_bitwise {').length,2,'one complete generated runtime across the module');
 assert.equal(selfTranslate(result.code,'Rust','JavaScript').code,source);
 const typed='export function mask(value:number):number{return(value>>>0)&255;}\n';
 const translated=selfTranslate(typed,'TypeScript','Rust',{retainDiagnostics:true});
 assert.ok(translated.items.every(item=>item.status==='translated'),JSON.stringify(translated.items));
 assert.equal(selfTranslate(translated.code,'Rust','TypeScript').code,typed);
});

test('other primitive conversions BigInt and forged wire operators stay explicit refusals',()=>{
 for(const expression of ['1n & 2n','1n << 2n','1n >>> 2n','1n | 2','true & 1','"3" ^ 1']){
  const source='export function compute(){return '+expression+';}\n',result=selfTranslate(source,'JavaScript','Rust',{retainDiagnostics:true});
  assert.ok(result.items.some(item=>item.status==='carried'),expression);
  assert.equal(selfTranslate(result.code,'Rust','JavaScript').code,source);
 }
 const source=functionSource('&');
 const body=program=>[...program.declarations.values()].find(row=>row.name==='compute').body;
 for(const mutate of [value=>{value.op='unknownBitwise';},value=>{value.args.pop();},value=>{value.args.push(value.args[0]);},value=>{value.type={kind:'bool'};},value=>{value.args[0]={k:'lit',value:true,type:{kind:'bool'}};}]){
  const program=checkProgram(parseJavaScript(source));mutate(body(program));assert.throws(()=>encodeTypedProgramLinks(program));
 }
});

test('all six original local compound assignment inputs retain independent Number behavior and checked archives',()=>{
 for(const [symbol,observe]of operations.slice(0,6)){
  // Retain the complete original formerly unsupported input, including its docs.
  const source='/** @param {number} left @param {number} right */\nexport function compute(left,right){left '+symbol+'= right;return left;}';
  const program=checkProgram(parseJavaScript(source)),execute=evaluate(program);
  for(const left of values)for(const right of values)assert.equal(execute(left,right),observe(left,right),symbol+'= '+left+' '+right);
  const translation=selfTranslate(source,'JavaScript','Rust');
  assert.ok(translation.items.every(item=>item.status==='translated'||item.status==='comment'));
  assert.equal(selfTranslate(translation.code,'Rust','JavaScript').code,source);
  const packet=encodeTypedProgramLinks(program);
  const original=emitRust(program);
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
   const restored=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
   const emitted=emitRust(restored);
   for(const field of ['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(emitted[field],original[field]);
  }
  for(const invalid of [
   'export function compute(){let left=1n;left '+symbol+'=2n;return left;}',
   'export function compute(){const left=1;left '+symbol+'=2;return left;}',
   'export function compute(){left '+symbol+'=2;let left=1;return left;}',
   'export function compute(){const item={value:1};item.value '+symbol+'=2;return item.value;}',
  ])assert.throws(()=>checkProgram(parseJavaScript(invalid)),invalid);
 }
});

test('the separate compound assignment oracle executes all 5400 declared cells',()=>{
 const packet=readNumberBitwiseAssignmentOraclePacket();
 assert.equal(packet.expected.length,5400);assert.equal(packet.values.length,30);
 assert.ok(packet.translation.items.every(item=>item.status==='translated'||item.status==='comment'));
});

test('the published Links definition contains each named operation and fresh caller-owned graphs',()=>{
 const first=createNumberBitwiseProgramLinks(),second=createNumberBitwiseProgramLinks();
 assert.notEqual(first.network,second.network);assert.equal(first.network.toLino(),second.network.toLino());
 const program=decodeTypedProgramLinks(second.network,second.root);
 for(const[symbol]of operations)assert.ok(program.declarations.has(readJavaScriptNumberBitwiseOperation(symbol)));
 first.network.insertPoint('caller-owned extension');
 assert.notEqual(first.network.toLino(),second.network.toLino());
 assert.equal(createNumberBitwiseProgramLinks().network.toLino(),second.network.toLino());
 const native=readFileSync(new URL('../../rust/src/translation/number_bitwise_program_data.rs',import.meta.url),'utf8');
 const literalText=tokenize(native,'Rust').tokens.filter(token=>token.kind==='string').map(token=>token.value).join('');
 assert.equal(literalText,second.network.toLino());
 assert.ok(native.split('\n').length<=1000);
 assert.match(native,new RegExp('const ROOT: u64 = '+second.root.asU64()+';','u'));
});

test('the native oracle packet independently executes the entire declared boundary fixture before transport',()=>{
 const fixture=readProjectData(new URL('../../parity/fixtures/number-bitwise.lino',import.meta.url));
 assert.deepEqual(fixture.valueSources.map(value=>runInNewContext(value)),values);
 assert.equal(fixture.source,operations.map(([symbol],index)=>functionSource(symbol,'compute'+index)).join('\n'));
 const packet=readNumberBitwiseOraclePacket();
 assert.equal(packet.expected.length,6300);assert.equal(packet.values.length,30);assert.equal(packet.source,fixture.source);
 let index=0;
 const bits=value=>{const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value,false);return view.getBigUint64(0,false).toString(16).padStart(16,'0');};
 for(const[,observe]of operations)for(const left of values)for(const right of values)assert.equal(packet.expected[index++],bits(observe(left,right)));
 assert.equal(packet.definition.links,createNumberBitwiseProgramLinks().network.toLino());
 assert.ok(packet.translation.items.every(item=>item.status==='translated'||item.status==='comment'));
});
