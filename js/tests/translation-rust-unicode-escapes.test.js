import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {rustUnicodeEscapeFixture} from '../scripts/build-rust-unicode-escape-fixtures.mjs';
import {decodeRustUnicodeEscape} from '../src/translation/rust-unicode-escape-rules.js';
import {decodeUnicodeEscape} from '../src/translation/frontend-rules.js';
import {tokenize} from '../src/translation/lexer.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks,LinkNetwork,translateProgram} from '../src/index.js';
const root=fileURLToPath(new URL('../../',import.meta.url));
const fixture=readProjectData(new URL('../../parity/fixtures/rust-unicode-escapes.lino',import.meta.url));
const units=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));
const sourceFor=literal=>'// 😀 original UTF-16 offsets\nfn main(){println!("{}",'+literal+');}\n';
function execute(code){const output=[];vm.runInNewContext(code,{console:{log:value=>output.push(String(value))}},{timeout:1000});return output;}

test('the Rust-specific decoder reads all linked scalar cases from original full source at a nonzero start',()=>{
 assert.deepEqual(rustUnicodeEscapeFixture(),fixture);assert.equal(fixture.scalarCases.length,17);assert.equal(fixture.refusalCases.length,16);
 for(const {id,escape,code,value}of fixture.scalarCases){const prefix='😀 prefix ',text=prefix+escape+' untouched suffix',result=decodeRustUnicodeEscape(units(text),prefix.length);assert.deepEqual(result,{$:'scalar',code,end:escape.length},id);assert.equal(String.fromCodePoint(result.code),value,id);const literal='"'+escape+'"',source=sourceFor(literal),token=tokenize(source,'Rust').tokens.find(token=>token.raw===literal);assert.ok(token,id);assert.equal(token.value,value,id);assert.equal(token.start,source.indexOf(literal),id);assert.equal(token.end,token.start+literal.length,id);assert.equal(source.slice(token.start,token.end),literal,id);}
});

test('the Rust decoder covers every code point through the Unicode maximum, retaining nonscalar refusals',()=>{
 let scalars=0,nonscalars=0;
 for(let code=0;code<=0x10FFFF;code++){const hexadecimal=code.toString(16),escape='\\u{'+[...hexadecimal].join('_')+'_}',result=decodeRustUnicodeEscape(units(escape),0);if(code>=0xD800&&code<=0xDFFF){assert.deepEqual(result,{$:'unsupported',end:escape.length,escapeLength:escape.length});nonscalars++;}else{assert.equal(result.$,'scalar');assert.equal(result.code,code);assert.equal(result.end,escape.length);scalars++;}}
 assert.equal(scalars,1112064);assert.equal(nonscalars,2048);
});

test('leading separators, excessive actual digits, missing delimiters and nonscalars preserve result and original diagnostic contracts',()=>{
 for(const row of fixture.refusalCases){const {id,escape,result}=row;const prefix='😀 prefix ',text=prefix+escape+' suffix';assert.deepEqual(decodeRustUnicodeEscape(units(text),prefix.length),result,id);const source=sourceFor('"'+escape+'"'),start=source.indexOf(escape);if(Object.hasOwn(row,'sourceValue')){assert.equal(tokenize(source,'Rust').tokens.find(token=>token.kind==='string'&&token.raw==='"'+escape+'"').value,row.sourceValue,id+' is a separate valid ASCII source escape');continue;}assert.throws(()=>tokenize(source,'Rust'),error=>error.kind===(result.$==='unsupported'?'unsupported':'syntax')&&error.span.start===start&&error.span.end===start+(result.$==='unsupported'?result.end:2)&&(result.$!=='unsupported'||error.message.includes(escape)),id);}
 for(const {escape}of fixture.scalarCases)for(let end=0;end<escape.length;end++)assert.deepEqual(decodeRustUnicodeEscape(units(escape.slice(0,end)),0),{$:'malformed'},escape+' truncated at '+end);
 for(const {escape,result}of fixture.refusalCases)if(result.$==='unsupported')for(let end=0;end<escape.length;end++)assert.deepEqual(decodeRustUnicodeEscape(units(escape.slice(0,end)),0),{$:'malformed'},'unclosed nonscalar '+escape+' at '+end);
 for(const start of [-1,0.5,NaN,Infinity,6])assert.deepEqual(decodeRustUnicodeEscape(units(String.raw`\u{0}`),start),{$:'malformed'},'invalid start '+start);
});

test('arbitrary interior and trailing separators have no escape-width cap and never reprocess a decoded slash',()=>{
 const separators='_'.repeat(16_384),escape='\\u{0'+separators+'0'+separators+'0'+separators+'0'+separators+'4'+separators+'1'+separators+'}';
 assert.deepEqual(decodeRustUnicodeEscape(units('prefix '+escape+' suffix'),7),{$:'scalar',code:65,end:escape.length});assert.equal(tokenize('"'+escape+'"','Rust').tokens[0].value,'A');
 for(const literal of [String.raw`"\\u{41}"`,String.raw`"\u{5_c}u{41}"`])assert.equal(tokenize(literal,'Rust').tokens[0].value,String.raw`\u{41}`);
 assert.deepEqual(decodeRustUnicodeEscape(units('\\u{000000'+separators+'0}'),0),{$:'malformed'});
});

test('public source-free Links reload and actual JavaScript emission retain every scalar value',()=>{
 for(const {id,escape,value}of fixture.scalarCases){const source=sourceFor('"'+escape+'"'),direct=translateProgram(source,'Rust','JavaScript');assert.equal(direct.diagnostic,null,id);assert.deepEqual(execute(direct.code),[value],id);const packet=createTypedProgramLinks(source,'Rust'),network=LinkNetwork.fromLino(packet.network.toLino());assert.deepEqual(decodeTypedProgramLinks(network,packet.root),decodeTypedProgramLinks(packet.network,packet.root),id);assert.deepEqual(execute(emitTypedProgramLinks(network,packet.root,'JavaScript').code),[value],id);const actualRust=emitTypedProgramLinks(network,packet.root,'Rust').code;assert.ok(tokenize(actualRust,'Rust').tokens.some(token=>token.kind==='string'&&token.value===value),id+' actual complete Rust artifact token');}
});

test('generic Unicode decoding and JavaScript, Lean and Rocq retain all prior separator and simple-value interpretations',()=>{
 for(const {escape,code}of fixture.scalarCases){const plain='\\u{'+code.toString(16)+'}';assert.deepEqual(decodeUnicodeEscape(units(plain),false),{$:'scalar',code,end:plain.length});if(escape.includes('_')){for(const allowFixed of [false,true])assert.deepEqual(decodeUnicodeEscape(units(escape),allowFixed),{$:'malformed'});for(const language of ['JavaScript','Lean'])assert.throws(()=>tokenize('"'+escape+'"',language),error=>error.kind==='syntax'&&error.message.includes('malformed unicode escape'));}assert.equal(tokenize('"'+escape+'"','Rocq').tokens[0].value,escape);}
 for(const [literal,value]of [[String.raw`"\u0041"`,'A'],[String.raw`"\ud83d\uDE00"`,'😀'],[String.raw`"\xE9"`,'é'],[String.raw`"\u{10FFFF}"`,String.fromCodePoint(0x10FFFF)]])assert.equal(tokenize(literal,'JavaScript').tokens[0].value,value);
 assert.equal(tokenize(String.raw`"\u{1F600}"`,'Lean').tokens[0].value,'😀');assert.throws(()=>tokenize(String.raw`"\u0041"`,'Lean'),/malformed unicode escape/u);
 for(const {escape}of fixture.scalarCases)if(escape.includes('_'))assert.throws(()=>tokenize('`'+escape+'`','JavaScript'),/malformed unicode escape/u,'JavaScript template separators remain refused');
});

test('many Rust Unicode escapes reuse one lazy full-source encoding rather than copied suffixes or bounded windows',()=>{
 const count=128,escape='\\u{1'+'_'.repeat(32)+'F_6_0_0_}',source=Array.from({length:count},(_,index)=>'let text'+index+' = "'+escape+'";').join('\n'),original=String.prototype.charCodeAt;let calls=0,result;
 try{String.prototype.charCodeAt=function(index){calls++;return original.call(this,index);};result=tokenize(source,'Rust');}finally{String.prototype.charCodeAt=original;}
 assert.ok(calls<=source.length+count,'one full-source pass plus existing per-escape prefix recognition');assert.equal(result.tokens.filter(token=>token.kind==='string').length,count);assert.ok(result.tokens.filter(token=>token.kind==='string').every(token=>token.value==='😀'));
});

test('the Rust Unicode policy, actual shared hexadecimal dependency and linked fixtures regenerate in JavaScript only',()=>{
 for(const script of ['generate-rust-unicode-escape-rules.mjs','build-rust-unicode-escape-fixtures.mjs'])execFileSync(process.execPath,['js/scripts/'+script,'--check'],{cwd:root,timeout:30_000,stdio:'pipe'});
});
