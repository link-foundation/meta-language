// Source/data authoring only; no original or translated fixture executes.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,source,body,value,checks){cases.push({name,module:'named_format_case_'+cases.length,source:source+'pub fn result()->String{'+body+'}fn main(){}',expected:value,value,abort:null,resultType:'String',checks});}
add('explicit name overrides a same-named lexical binding','pub fn render(value:&str)->String{format!("{value}",value="override")}','render("original")','override',["const _: for<'value> fn(&'value str)->String=CASE::render;"]);
add('source argument order and field order remain independent','pub fn render(left:i64,right:bool)->String{format!("{second}/{first}",first=left,second=right)}','render(-7i64,true)','true/-7',['const _: fn(i64,bool)->String=CASE::render;']);
add('indexed and implicit fields keep their independent cursor','pub fn render()->String{format!("{name}/{1}/{}/{0}/{name}",7i64,9i64,name="é")}','render()','é/9/7/7/é',['const _: fn()->String=CASE::render;']);
add('implicit lexical captures coexist with explicit names','pub fn render(label:&str,count:usize)->String{format!("{label}:{total}",total=count)}','render("😀",18446744073709551615usize)','😀:18446744073709551615',["const _: for<'label> fn(&'label str,usize)->String=CASE::render;"]);
add('repeated named expression is captured once before Display','pub fn input()->i64{7}pub fn render()->String{format!("{word}/{word}",word=input())}','render()','7/7',['const _: fn()->i64=CASE::input;','const _: fn()->String=CASE::render;']);
add('all raw arguments precede reordered repeated display fields','pub fn first()->i64{1}pub fn second()->i64{2}pub fn render()->String{format!("{two}/{one}/{two}",one=first(),two=second())}','render()','2/1/2',['const _: fn()->i64=CASE::first;','const _: fn()->i64=CASE::second;','const _: fn()->String=CASE::render;']);
add('explicit text fields preserve escaped braces and combining values','pub fn render(value:&str)->String{format!("{{{text}}}",text=value)}','render("é")','{é}',["const _: for<'value> fn(&'value str)->String=CASE::render;"]);
add('a named operand retains an owned nested format result','pub fn render(value:i128)->String{format!("[{text}]",text=format!("{number}",number=value))}','render(-170141183460469231731687303715884105728i128)','[-170141183460469231731687303715884105728]', ['const _: fn(i128)->String=CASE::render;']);
add('Unicode argument identifiers enter after source NFC normalization','pub fn render(value:bool)->String{format!("{café}",café=value)}','render(false)','false',['const _: fn(bool)->String=CASE::render;']);
add('raw keyword argument spelling retains its actual identifier','pub fn render(value:i64)->String{format!("{type}",r#type=value)}','render(-7i64)','-7',['const _: fn(i64)->String=CASE::render;']);
add('generated binding candidates cannot capture explicit fields','pub fn render(ml_product_1:bool)->String{format!("{ml_product_1}/{value}",value=ml_product_1)}','render(true)','true/true',['const _: fn(bool)->String=CASE::render;']);
add('borrowed owned text remains live after named formatting','pub fn render(value:&String)->String{format!("{text}/{text}",text=value)}pub fn call(value:String)->String{format!("{a}/{b}",a=render(&value),b=value.len())}','call(String::from("é"))','é/é/2',["const _: for<'value> fn(&'value String)->String=CASE::render;",'const _: fn(String)->String=CASE::call;']);
add('ordinary positional source remains unchanged beside named sources','pub fn render(value:i64)->String{format!("{0}/{0}",value)}','render(7i64)','7/7',['const _: fn(i64)->String=CASE::render;']);
const printedSources=[
 {name:'println resolves explicit and lexical names',source:'fn main(){let value=7i64;println!("{label}/{value}",label="é");}',lines:['é/7']},
 {name:'function output precedes reordered named fields',source:'pub fn first()->i64{println!("first");1}pub fn second()->i64{println!("second");2}fn main(){println!("{two}/{one}/{two}",one=first(),two=second());}',lines:['first','second','2/1/2']},
 {name:'empty println retains the original empty line',source:'fn main(){println!();}',lines:['']},
];
const refusals=[
 {name:'duplicate named arguments refuse',source:'fn main(){println!("{value}",value=1i64,value=2i64);}',kind:'syntax',reason:'format argument name is repeated'},
 {name:'a positional operand after a named one refuses',source:'fn main(){println!("{value} {}",value=1i64,2i64);}',kind:'syntax',reason:'positional format arguments cannot follow named arguments'},
 {name:'unused explicit names remain errors',source:'fn main(){println!("literal",value=1i64);}',kind:'syntax',reason:'format string has fewer {} than arguments'},
 {name:'unused positional operands remain errors',source:'fn main(){println!("{value}",1i64,value=2i64);}',kind:'syntax',reason:'format string has fewer {} than arguments'},
 {name:'named formatting retains the complete specifier refusal',source:'fn main(){println!("{value:?}",value=1i64);}',kind:'unsupported',reason:'format spec {value:?}: only plain {} Display formatting is portable'},
 {name:'the first argument is an actual positional template',source:'fn main(){println!(value=1i64);}',kind:'syntax',reason:'the format string must be the first positional argument'},
 {name:'a lone underscore cannot name an operand',source:'fn main(){println!("{_}",_=1i64);}',kind:'syntax',reason:'invalid named format argument'},
];
const sequences=[
 {names:[],error:''},{names:['',''],error:''},{names:['','one','two'],error:''},{names:['one','two'],error:''},
 {names:['one',''],error:'positional format arguments cannot follow named arguments'},
 {names:['one','one'],error:'format argument name is repeated'},
 {names:['','one','one'],error:'format argument name is repeated'},
];
// Expected instruction streams are independent source facts, not planner output.
const plans=[
 {plan:[0,2,1,5],names:['word'],fields:['word'],expected:[0,1,0,0]},
 {plan:[0,2,1,5,0,6,7,2,8,12],names:['word'],fields:['word','word'],expected:[0,1,0,0,0,6,7,1,0,0]},
 {plan:[0,2,1,6,2,8,12],names:['word'],fields:['local','word'],expected:[0,2,1,6,1,0,0]},
 {plan:[0,1,1,0,2,1,5],names:['','word'],fields:['word'],expected:[-7,0,1]},
 {plan:[0,1,0,0,2,1,5],names:['','word'],fields:['word'],expected:[0,1,0,0,1,1,0]},
 {plan:[0,0,0,1],names:['word'],fields:[],expected:[-7,0,1]},
 {plan:[0,2,1,5],names:['word','word'],fields:['word'],expected:[-9,0,0]},
 {plan:[0,2,1,5],names:['word',''],fields:['word'],expected:[-9,0,0]},
 {plan:[-3,1,8],names:['word'],fields:[],expected:[-3,1,8]},
 {plan:[],names:[],fields:[],expected:[-8,0,0]},
 {plan:[0,2,1],names:[],fields:[],expected:[-8,0,0]},
 {plan:[0,0,2,1],names:[],fields:[],expected:[-8,0,0]},
 {plan:[0,1,1,0],names:['word'],fields:[],expected:[-8,0,0]},
 {plan:[0,1,0,1],names:['word'],fields:[],expected:[-8,0,0]},
 {plan:[0,2,1,1],names:['word'],fields:['word'],expected:[-8,0,0]},
 {plan:[0,2,1,5],names:['word'],fields:[],expected:[-8,0,0]},
 {plan:[0,2,1,5],names:['word'],fields:[''],expected:[-8,0,0]},
 {plan:[0,0,0,0],names:[],fields:['extra'],expected:[-8,0,0]},
 {plan:[0,7,0,0],names:[],fields:[],expected:[-8,0,0]},
 {plan:[0,1,-1,0],names:['word'],fields:[],expected:[-8,0,0]},
 {plan:[0,1,0.5,0],names:['word'],fields:[],expected:[-8,0,0]},
];
const revision='ee7be581',history=['js/src/translation/rust.js','js/src/translation/rust-formatting-rules.js','rust/src/translation/rust/formatting.rs','rust/src/translation/rust/items.rs','rust/src/translation/rust/expressions/control.rs','rust/src/translation/mod.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return{path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-named-formatting.lino',import.meta.url),{schemaVersion:1,retainSourceTypeOwners:true,model:'retained displayed Rust string function interface 1',cases,printedSources,refusals,sequences,plans,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},references:['https://doc.rust-lang.org/std/fmt/index.html','https://doc.rust-lang.org/reference/identifiers.html'],assertions:['actualNamedOperandsOverrideLexicalCapturesWithoutChangingTheirSourceValues','explicitIndexedImplicitAndLexicalFieldsRetainTheirOriginalSeparateCursorRules','everyOperandIsCapturedOnceInOriginalOrderBeforeRepeatedOrReorderedDisplay','normalUnicodeSourceNamesAndRawKeywordArgumentsKeepTheirActualIdentifiers','unusedRepeatedOutOfOrderMalformedAndUnsupportedSpecifierInputsRetainTheirDiagnostics','ordinaryPositionalFormattingRetainsThePrecedingInterpretation','completeOriginalAndBothGeneratedNativeProductsRetainIndependentTypesAndExactStrings','bothCodecsAndAllFourTargetsRetainCheckedCurrentMeaning'],boundary:'Unexecuted code and source-data draft. Shared source grammar terms and named-resolution decisions are automatically translated; no native replacement formatting algorithm is handwritten. One hundred thousand operands/terms and one million name-comparison steps bound this named profile. The original positional/captured-variable entry point retains its original admission and unused-operand diagnosis. This does not establish custom Display/Debug/precision/width/radix behavior, general macro expansion, public ABI or complete own-codebase translation. All controls remain deferred.'});
