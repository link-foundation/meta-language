// Isolated JavaScript-first prototype. No native execution or full-module claim.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {rustFunctionDefinitions} from '../js/scripts/generate-self-translation-report.mjs';
import {parseRust} from '../js/src/translation/rust.js';
import {parseJavaScript} from '../js/src/translation/javascript.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {LinkNetwork} from '../js/src/network.js';
import {createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, encodeTypedProgramLinks} from '../js/src/typed-program-links.js';

const model = 'closed primitive scalar loop 1';
const options = {scalarLoopModel: model};
const linkedOptions = {rustScalarLoopModel: model};
const suffix = '\nfn main(){}';
const moduleSource = readFileSync(new URL('../rust/src/link_network_binary_rules/part_1.rs', import.meta.url), 'utf8');
const originalName = 'ml_link_network_binary_integer_byte_loop2';
const authentic = rustFunctionDefinitions(moduleSource).get(originalName);
assert.equal(Buffer.byteLength(authentic), 512);
const cases = [
  {name: 'advance', source: 'fn advance(mut value:f64,limit:f64,mut at:f64)->f64{loop{return if at<limit{(value,at)=(value+1f64,at+1f64);continue;}else{value};}}',
    original: 'function advance(value,limit,at){for(;;){if(at<limit){[value,at]=[value+1,at+1];continue;}return value;}}', observations: [[[3,5,0],8], [[0,20000,0],20000]]},
  {name: 'swap', source: 'fn swap(mut a:f64,mut b:f64,limit:f64,mut at:f64)->f64{loop{return if at<limit{(a,b,at)=(b,a,at+1f64);continue;}else{a};}}',
    original: 'function swap(a,b,limit,at){for(;;){if(at<limit){[a,b,at]=[b,a,at+1];continue;}return a;}}', observations: [[[0,-0,1,0],-0], [[0,-0,2,0],0], [[1,2,1,0],2], [[1,2,2,0],1]]},
  {name: 'toggle', source: 'fn toggle(mut flag:bool,limit:f64,mut at:f64)->bool{loop{return if at<limit{(flag,at)=(!flag,at+1f64);continue;}else{flag};}}',
    original: 'function toggle(flag,limit,at){for(;;){if(at<limit){[flag,at]=[!flag,at+1];continue;}return flag;}}', observations: [[[true,1,0],false], [[true,2,0],true]]},
  {name: originalName, source: authentic,
    original: `function ${originalName}(value,index,at){for(;;){if(at<index){const value_2=Math.floor(value/256);const at_2=at+1;[value,at]=[value_2,at_2];continue;}return value;}}`,
    observations: [[[65536,1,0],256], [[65536,2,0],1], [[-65537,1,0],-257], [[NaN,1,0],NaN], [[Infinity,1,0],Infinity], [[-Infinity,1,0],-Infinity], [[-0,0,0],-0], [[-0,1,0],-0], [[5e-324,1,0],0], [[-5e-324,1,0],-0], [[257,NaN,0],257]]},
];
import {writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
const scalar = value => typeof value === 'boolean' ? value : Object.is(value,-0) ? '-0' : String(value);
const fixture = {model, originalName, actualSource:authentic, sourceOracle:'rustFunctionDefinitions complete CST item', resourcePremise:'No termination or universal resource guarantee; existing iterative tail-call lowering', cases:cases.map(entry=>{
 const surface=parseRust(entry.source+suffix,options),program=checkProgram(surface),javaScriptDefinitions=emitJavaScript(program).definitions.join('\n\n');
 const reverse=checkProgram(parseJavaScript(javaScriptDefinitions,options));
 return {name:entry.name,source:entry.source+suffix,originalJavaScript:entry.original,observations:entry.observations.map(([args,expected])=>({arguments:args.map(scalar),expected:scalar(expected)})),surface,javaScriptDefinitions,rustDefinitions:emitRust(program).definitions.join('\n\n'),javaScriptReverseSurface:parseJavaScript(javaScriptDefinitions,options),reverseRustDefinitions:emitRust(reverse).definitions.join('\n\n')};
})};
(process.argv.includes('--check') ? checkProjectData : writeProjectData)(new URL('../parity/fixtures/rust-scalar-loops.lino',import.meta.url),JSON.parse(JSON.stringify(fixture)));
