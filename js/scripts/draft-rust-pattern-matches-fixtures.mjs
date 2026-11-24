// Source authoring only. Preserve independent originals and old assertion inputs.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],premises=[],refusals=[];
function add(name,body,helpers='',abort=null,checks=[]){cases.push({name,module:'pattern_macro_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;',...checks]});}
for(const owner of ['u8','i8','u16','i16','u32','i32','u64','i64','u128','i128','usize','isize'])for(const accepted of [false,true]){
 const value=accepted?'7':'8';
 add(owner+' scalar literal macro '+accepted,'matches!('+value+owner+',7'+owner+')=='+String(accepted));
 add(owner+' bound guard macro '+accepted,'matches!('+value+owner+',selected if selected==7'+owner+')=='+String(accepted));
 add(owner+' actual nested reference macro '+accepted,'let value='+value+owner+';let first=&value;let second=&first;matches!(second,&&7'+owner+')=='+String(accepted));
}
for(const [pattern,first,last] of [['_',true,true],['binding',true,true],['"a" if true',true,false],['"a" | _',true,true],['("a")',true,false]]){
 for(const [value,expected] of [['a',first],['b',last]])add('retained formerly unsupported '+pattern+' on '+value,'selected("'+value+'") == '+String(expected),'pub fn selected(value:&str)->bool{matches!(value,'+pattern+')}');
}
add('tuple binding and guard preserve their actual field owners','matches!((3u8,4u16),(first,last) if u16::from(first)+last==7u16)');
add('tuple wildcard preserves remaining positional inputs','matches!((3u8,4u16),(3u8,_))');
add('unit pattern is not an empty macro invocation','matches!((),())');
add('Boolean alternatives preserve actual exhaustiveness','matches!(false,true|false)');
add('nominal enum patterns retain real declaration and constructor ownership','matches!(Event::Data(7u8),Event::Data(value) if value==7u8)','pub enum Event{Empty,Data(u8)}');
add('nominal enum rejection retains the original empty variant','!matches!(Event::Empty,Event::Data(_))','pub enum Event{Empty,Data(u8)}');
add('record field order and guard use the existing match reader','matches!(Pair{last:4u16,first:3u8},Pair{last,first} if u16::from(first)+last==7u16)','pub struct Pair{pub first:u8,pub last:u16}');
add('all standard macro delimiters use the same expansion','matches![7u8,7u8]&&matches!{7u8,selected if selected==7u8}');
add('leading or marker and trailing macro comma retain pattern meaning','matches!(7u8,|6u8|7u8,)');
add('guard blocks keep their nested delimiters and own comma tokens','matches!(7u8,value if {let pair=(value,1u8);pair.0==7u8},)');
add('nested pattern macro restores its caller token stream','matches!(7u8,value if matches!(value,7u8))&&matches!(8u8,8u8)');
add('receiver failure precedes any guard evaluation','matches!(receiver(),value if guard(value))','pub fn receiver()->u8{panic!("receiver first")}pub fn guard(value:u8)->bool{panic!("guard later")}','receiver first');
add('a failed pattern does not invoke its guard','!matches!(8u8,7u8 if guard())','pub fn guard()->bool{panic!("unreachable guard")}');
add('a selected guard retains its divergence','matches!(7u8,value if guard(value))','pub fn guard(value:u8)->bool{panic!("selected guard")}','selected guard');
add('a false guard selects the ordinary wildcard fallback','!matches!(7u8,value if value==8u8)');
for(const pattern of ['0u8|7u8','value if value==7u8']){
 add('receiver appears once in expanded '+pattern,'matches!(receiver(),'+pattern+')','pub fn receiver()->u8{7u8}');cases.at(-1).callCounts=[{source:'receiver',expected:1}];
}
add('whole-value pattern alias remains local to its actual match','matches!(7u8,selected@7u8 if selected==7u8)');
add('a source alias local to the macro never escapes its scope','let value=99u8;matches!(7u8,value if value==7u8)&&value==99u8');
add('remaining original borrowed string layers remain meaningful','let value="a";let borrowed=&value;matches!(borrowed,"a")');
for(const [owner,ordinary,expected] of [['matches',true,true],['std::matches',true,true],['core::matches',true,true],['foreign::matches',true,false],['matches',false,false]])premises.push({operation:'acceptRustPatternMatchMacroOwner',args:[owner,ordinary],expected});
for(const [code,value,kind] of [[-1,'{','punct'],[-2,'=>','punct'],[-3,'true','identifier'],[-4,',','punct'],[-5,'_','identifier'],[-6,'false','identifier'],[-7,'}','punct'],[0,'','']]){
 premises.push({operation:'readRustPatternMatchTokenValue',args:[code],expected:value},{operation:'readRustPatternMatchTokenKind',args:[code],expected:kind});
}
premises.push({operation:'planRustPatternMatchRows',args:[['identifier','punct'],['_',')'],0,')'],expected:[2,-1,0,-2,-3,-4,-5,-2,-6,-7]},{operation:'planRustPatternMatchRows',args:[['punct','identifier','punct','punct','punct'],['(','value',')',',',']'],0,']'],expected:[5,-1,0,1,2,-2,-3,-4,-5,-2,-6,-7]},{operation:'planRustPatternMatchRows',args:[['punct'],[')'],0,')'],expected:[]},{operation:'planRustPatternMatchRows',args:[['punct','punct'],['(',']'],0,')'],expected:[]});
refusals.push({name:'pattern macro cannot invent an actual foreign macro interpretation',source:'use crate::foreign::matches;pub fn result()->bool{matches!(7u8,_)}fn main(){}',kind:'unsupported',reason:'Rust pattern matches!: the actual standard macro needs its ordinary source interpretation'},{name:'a missing macro pattern remains a source error',source:'pub fn result()->bool{matches!(7u8,)}fn main(){}',kind:'unsupported',reason:'Rust pattern matches!: a complete delimited source pattern and optional guard are required'});
const revision='0c6c40151e4169e001ead836a8a98824176e0a7d',paths=['js/tests/translation-rust-literal-matches.test.js','rust/tests/unit/translation_rust_literal_matches.rs'];
const history=paths.map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-pattern-matches.lino',import.meta.url),{schemaVersion:1,model:'Rust standard general pattern macro 1',retainSourceTypeOwners:true,cases,premises,refusals,history,assertionRevisions:[{scope:'Rust >=1.90.0 <1.100.0; edition 2024',inputs:['_','binding','"a" if true','"a" | _','("a")'],before:'literal-only source admission refused every pattern',after:'standard matches! uses ordinary match patterns and optional guards',reason:'The standard macro definition expands to a match with true/false arms; preserve every original source input in positive controls and retain preceding complete tests in history.'}],sourceDefinitionPrograms:['meta-language/source-program/rust-pattern-match-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/std/macro.matches.html','https://doc.rust-lang.org/reference/patterns.html'],boundary:'Unexecuted originals and source-only expansion policies. Source types, bindings, guards, ownership and unsupported pattern forms retain the same existing match contracts. Complete Rust patterns, ownership, macro definitions/expansion and whole-language coverage remain obligations.'});
