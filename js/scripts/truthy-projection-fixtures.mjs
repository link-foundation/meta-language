// Independent inputs/oracles; normal capture does not execute source programs.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {encodeTypedProgramLinks} from '../src/typed-program-links.js';
import {encodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {checkProjectData,writeProjectData} from './project-data.mjs';

const probe=(type,values)=>`/** @param {${type}} value @returns {boolean} */function probe(value){return value ? true : false;}\n`
 +values.map(value=>`console.log(probe(${value}));`).join('\n');
export const TRUTHY_PROJECTION_CASES=Object.freeze([
 {id:'number',source:probe('number',['0','-0','1','NaN','Infinity','-Infinity']),expected:['false','false','true','false','true','true']},
 {id:'bigint',source:probe('bigint',['0n','-1n','2n']),expected:['false','true','true']},
 {id:'string',source:probe('string',["''","'0'","'false'","'\\u0000'","'\\ud800'","'😀'"]),expected:['false','true','true','true','true','true']},
 {id:'optional-number',source:probe('number|null|undefined',['null','undefined','0','-0','NaN','2']),expected:['false','false','false','false','false','true']},
 {id:'optional-array',source:probe('number[]|null|undefined',['null','undefined','[]','[0]']),expected:['false','false','true','true']},
 {id:'ordered-call-effects',source:'/** @returns {number} */function read(){console.log("read");return 1;}console.log(read()?true:false);console.log("after");',expected:['read','true','after']},
]);
// Retain the complete original six-call literal input and all six oracle
// results. Its one unsupported escape is a lexer boundary, not Boolean IR.
export const TRUTHY_PROJECTION_LITERAL_INPUT=TRUTHY_PROJECTION_CASES.find(input=>input.id==='string');
export const TRUTHY_PROJECTION_CHECKED_CASES=Object.freeze(TRUTHY_PROJECTION_CASES.filter(input=>input!==TRUTHY_PROJECTION_LITERAL_INPUT));
export const TRUTHY_PROJECTION_HOST_PARAMETER_SOURCE=TRUTHY_PROJECTION_LITERAL_INPUT.source.split('\n')[0];
export const TRUTHY_PROJECTION_HOST_PARAMETER_VALUES=Object.freeze([
 {units:[],expected:false},{units:[48],expected:true},{units:[102,97,108,115,101],expected:true},
 {units:[0],expected:true},{units:[55296],expected:true},{units:[55357,56832],expected:true},
 {units:[56320],expected:true},{units:[55296,56320],expected:true},
]);
export const TRUTHY_PROJECTION_FIXTURE='parity/fixtures/truthy-projection.lino';
function captureTruthyProjectionProgram(input){
 const program=checkProgram(parseJavaScript(input.source));program.utf16Strings=true;
 const packet=encodeTypedProgramLinks(program);
 return{...input,sourceSha256:createHash('sha256').update(input.source).digest('hex'),
  archive:{root:String(packet.root.asU64()),text:packet.network.toLino(),binaryHex:Buffer.from(encodeLinkNetworkBinary(packet.network)).toString('hex')}};
}
export function generateTruthyProjectionFixtures({check=false}={}) {
 const cases=TRUTHY_PROJECTION_CHECKED_CASES.map(captureTruthyProjectionProgram);
 const literalBoundary={...TRUTHY_PROJECTION_LITERAL_INPUT,
  sourceSha256:createHash('sha256').update(TRUTHY_PROJECTION_LITERAL_INPUT.source).digest('hex'),
  expectedRefusal:{kind:'unsupported',reason:'not a scalar value',start:TRUTHY_PROJECTION_LITERAL_INPUT.source.indexOf('\\ud800')}};
 const hostParameter={...captureTruthyProjectionProgram({source:TRUTHY_PROJECTION_HOST_PARAMETER_SOURCE}),
  values:TRUTHY_PROJECTION_HOST_PARAMETER_VALUES,
  inputRole:'Original complete function declaration; the same six string values plus lone-low and surrogate-pair controls enter through the existing UTF16 host parameter representation.'};
 const fixture={schemaVersion:1,model:'checked source Boolean conversion projection',
  assertionVersions:{Node:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0'},cases,literalBoundary,hostParameter,
  interpretation:'Original source inputs and independent expected results plus normally encoded checked graphs. This capture does not execute JavaScript or native programs and is not execution evidence.'};
 const output=new URL('../../'+TRUTHY_PROJECTION_FIXTURE,import.meta.url);
 if(check)checkProjectData(output,fixture);else writeProjectData(output,fixture);return fixture;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const args=process.argv.slice(2);assert.ok(args.length<=1&&args.every(arg=>arg==='--check'));
 generateTruthyProjectionFixtures({check:args.includes('--check')});
 console.log('Boolean conversion fixture inputs and actual checked archives captured; execution remains separate.');
}
