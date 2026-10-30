// Author literal controls and copy historical metadata; never execute programs.
import {existsSync,readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const historyFile=new URL('../../parity/fixtures/rust-formatted-panic-history.lino',import.meta.url);
if(!existsSync(historyFile)){
 const sourceFile=new URL('../../parity/fixtures/rust-completion-panic.lino',import.meta.url),bytes=readFileSync(sourceFile),original=readProjectData(sourceFile);
 writeProjectData(historyFile,{schemaVersion:1,revision:'a227aec9',path:'parity/fixtures/rust-completion-panic.lino',sha256:createHash('sha256').update(bytes).digest('hex'),legacy:original.legacy,defaultCanonical:original.defaultCanonical,reference:'https://doc.rust-lang.org/std/macro.panic.html#2021-and-later',change:'Keep complete former ordinary-profile refusals as history. Their source inputs now receive positive parse/emission/runtime coverage; the closed profile retains its original refusals.'});
}
const history=readProjectData(historyFile),cases=[],premises=[],refusals=[];
function add(name,body,message,declarations='',value=null){cases.push({name,module:'formatted_panic_case_'+cases.length,source:declarations+'pub fn result()->bool{'+body+'}',value,abort:message});}
const macros=[['panic',''],['unreachable','internal error: entered unreachable code: '],['unimplemented','not implemented: '],['todo','not yet implemented: ']];
for(const [macro,prefix]of macros)for(const [name,argumentsText,locals,message]of [
 ['positional values','"value {} / {}",7u64,true','','value 7 / true'],
 ['repeated indexed values','"{1}-{0}-{1}",2u8,7u8','','7-2-7'],
 ['named values','"left {last} {first}",first=2u8,last=7u8','','left 7 2'],
 ['lexical capture','"value {value}"','let value=7u8;','value 7'],
 ['escaped braces around a capture','"{{{value}}}"','let value=7u8;','{7}'],
 ['non-ASCII text','"{}","é😀"','','é😀'],
 ['a signed integer','"{}",-7i16','','-7'],
 ['a Boolean','"{}",true','','true'],
 ])add(macro+' formats '+name,locals+macro+'!('+argumentsText+')',prefix+message);
add('format operands evaluate before reordered Display fields','panic!("{1} {0}",first(),second())','first operand','fn first()->u8{panic!("first operand")}fn second()->u8{panic!("second operand")}');
add('named format operands retain source evaluation order','panic!("{right} {left}",left=first(),right=second())','first named operand','fn first()->u8{panic!("first named operand")}fn second()->u8{panic!("second named operand")}');
add('a String argument is formatted through its actual scalar contents','let message=String::from("é😀");panic!("text {}",message)','text é😀');
add('a referenced String argument preserves its source owner','let message=String::from("abc");panic!("text {}",&message)','text abc');
add('a formatted panic with a semicolon remains nonreturning','let value=7u8;panic!("value {value}");','value 7');
add('formatted panic in a Unit callback retains its effect','b"A".iter().copied().for_each(|value|panic!("item {value}"));true','item 65');
add('formatted panic in a typed mapper retains the required result type','b"A".iter().copied().map(|value|->u8{panic!("mapped {value}")}).count()==0','mapped 65');
add('an unselected formatted branch performs no formatting','if true{true}else{let value=7u8;panic!("unselected {value}")}',null,'','true');
add('an empty iterator never invokes a formatted panic callback','b"".iter().copied().for_each(|value|panic!("unused {value}"));true',null,'','true');
for(const [key,args,message]of [['legacy','7u64','a 7'],['defaultCanonical','','literal']])cases.push({name:'retained complete former '+key+' source',module:'formatted_panic_case_'+cases.length,source:'mod retained{'+history[key].source+'pub fn result()->bool{f('+args+');true}}pub fn result()->bool{retained::result()}',value:null,abort:message});
for(const [name,ordinary,expected]of [['panic',true,true],['unreachable',true,true],['unimplemented',true,true],['todo',true,true],['format',true,false],['panic',false,false],['',true,false]])premises.push({operation:'acceptRustPanicFormattingMacro',args:[name,ordinary],expected});
for(const [name,ordinary,kind,count,braces,expected]of [['panic',true,'str',2,false,true],['panic',true,'str',1,true,true],['panic',true,'str',1,false,false],['panic',true,'str',0,true,false],['panic',false,'str',2,true,false],['unknown',true,'str',2,true,false],['panic',true,'name',1,true,false],['panic',true,'name',2,false,true]])premises.push({operation:'acceptRustPanicFormattingValue',args:[name,ordinary,kind,count,braces],expected});
for(const [name,expected]of [...macros,['unknown','']])premises.push({operation:'readRustPanicFormattingPrefix',args:[name],expected});
for(const [kind,operation,expected]of [['abort','','abort'],['math','raiseRustPanicValue','abort'],['math','unknown','math'],['let','raiseRustPanicValue','let'],['','','']])premises.push({operation:'readRustPanicCompletionKind',args:[kind,operation],expected});
refusals.push({name:'lexically captured Strings retain their actual moved-owner state',source:'pub fn result()->bool{let message=String::from("a");let moved=message;panic!("{message}")}fn main(){}',kind:'unsupported',reason:'Rust owned binding: a moved source binding cannot be read or consumed again'});
writeProjectData(new URL('../../parity/fixtures/rust-formatted-panics.lino',import.meta.url),{schemaVersion:1,model:'ordinary Rust formatted panic macro values 1',retainSourceTypeOwners:true,cases,premises,refusals,history,sourceDefinitionPrograms:['meta-language/source-program/rust-panic-value-program','meta-language/source-program/abrupt-result-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=32 or 64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/std/macro.panic.html','https://doc.rust-lang.org/std/macro.unreachable.html','https://doc.rust-lang.org/std/macro.unimplemented.html','https://doc.rust-lang.org/std/macro.todo.html'],boundary:'Unexecuted original controls and source decisions. Ordinary formatted panic macros reuse the shared plain Display formatter and evaluated abort effects. Closed completion profiles keep their selected contracts. Debug/width/precision/custom formatting, hooks and locations, non-string payloads, Result/Option propagation and complete source translation remain full obligations.'});
