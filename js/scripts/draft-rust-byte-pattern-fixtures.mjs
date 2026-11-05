// Literal controls only. No represented original is parsed or executed here.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,declarations,body,value='7',ret='u8',abort=null){cases.push({name,module:'byte_pattern_case_'+cases.length,source:declarations+'pub fn result()->'+ret+'{'+body+'}',value,abort});}
for(let byte=0;byte<256;byte+=1){
 const literal="b'\\x"+byte.toString(16).toUpperCase().padStart(2,'0')+"'",other=(byte+1)%256;
 add('every byte keeps both selected and fallback outcomes '+byte,'pub fn choose(x:u8)->u8{match x{'+literal+'=>1,_=>0}}','choose('+byte+'u8)*3+choose('+other+'u8)','3');
}
add('ordinary byte alternatives reuse one arm','pub fn selected(x:u8)->u8{match x{b\'a\'|b\'b\'=>7,_=>0}}','selected(b\'b\')');
add('a false byte guard reaches the next actual literal arm','pub fn selected(x:u8,flag:bool)->u8{match x{b\'a\' if flag=>3,b\'a\'=>7,_=>0}}','selected(b\'a\',false)');
add('an unmatched byte guard remains lazy','fn fail()->bool{panic!("unmatched byte guard")}pub fn selected(x:u8)->u8{match x{b\'a\' if fail()=>3,_=>7}}','selected(b\'b\')');
add('a matched byte guard preserves its actual abort','fn fail()->bool{panic!("selected byte guard")}pub fn selected(x:u8)->u8{match x{b\'a\' if fail()=>3,_=>7}}','selected(b\'a\')',null,'u8','selected byte guard');
add('byte scrutinee failure precedes every arm','fn fail()->u8{panic!("byte scrutinee")}','match fail(){b\'a\'=>3,_=>7}',null,'u8','byte scrutinee');
add('direct byte aliases retain their actual u8 value','pub fn selected(x:u8)->u8{match x{label@b\'a\'=>label,_=>0}}','selected(b\'a\')','97');
add('borrowed byte aliases retain their actual reference layer','pub fn selected(x:&u8)->u8{match x{label@b\'a\'=>*label,_=>0}}','selected(&b\'a\')','97');
add('byte alias guards keep the original arm scope','pub fn selected(x:u8)->u8{match x{label@b\'a\' if label==97=>7,_=>0}}','selected(b\'a\')');
add('conditional byte patterns retain the selected branch','pub fn selected(x:u8)->u8{if let b\'a\'=x{7}else{0}}','selected(b\'a\')');
add('conditional byte alternatives retain the fallback','pub fn selected(x:u8)->u8{if let b\'a\'|b\'b\'=x{3}else{7}}','selected(b\'c\')');
add('nested byte patterns retain actual constructor fields','enum Byte{Value(u8),None}','match Byte::Value(97){Byte::Value(b\'a\')=>7,_=>0}');
add('nested byte aliases retain the matched field','enum Byte{Value(u8),None}','match Byte::Value(97){Byte::Value(label@b\'a\')=>label,_=>0}','97');
for(const [literal,value]of [["b'}'",125],[String.raw`b'\xFF'`,255]]){
 const original='fn choose(x:u8)->u8{match x{'+literal+'=>1,_=>0}}';
 add('exact former rejected pattern '+literal,original,'choose('+value+'u8)','1');
}
add('a direct unsuffixed scrutinee infers its actual byte pattern type','','match 97{b\'a\'=>7,_=>0}');
const typeError='a byte character pattern requires an unsigned 8-bit scrutinee';
const refusals=[['u16','97u16'],['i8','97i8'],['i64','97i64'],['bool','true'],['f64','97.0']].map(([type,value])=>({name:'byte patterns cannot become '+type+' patterns',source:'pub fn selected(x:'+type+')->u8{match x{b\'a\'=>7,_=>0}}pub fn result()->u8{selected('+value+')}fn main(){}',kind:'type',reason:typeError}));
// Independent declared u8 domain; every other Cartesian product cell refuses.
const acceptedTypes=new Set(['fixed:8:false']),types=[];
for(const kind of ['fixed','int','nat','bool','string','float'])for(const bits of [0,8,16,64])for(const signed of [false,true])types.push({kind,bits,signed,expected:acceptedTypes.has(kind+':'+bits+':'+signed)?'':typeError});
const inference=[{language:'Rust',kind:'num',typed:false,patterns:['wild','byteLit'],expected:true},{language:'Rust',kind:'num',typed:false,patterns:['byteLit','numLit'],expected:true},{language:'Rust',kind:'num',typed:false,patterns:[],expected:false},{language:'Rust',kind:'num',typed:false,patterns:['numLit'],expected:false},{language:'Rust',kind:'num',typed:true,patterns:['byteLit'],expected:false},{language:'JavaScript',kind:'num',typed:false,patterns:['byteLit'],expected:false},{language:'Rust',kind:'name',typed:false,patterns:['byteLit'],expected:false},{language:'Rust',kind:'str',typed:false,patterns:['byteLit'],expected:false}];
const revision='f7ea017b',history=['js/tests/translation-rust-character-literals.test.js','rust/tests/unit/translation_rust_raw_string_literals.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return{path,revision,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};});
writeProjectData(new URL('../../parity/fixtures/rust-byte-patterns.lino',import.meta.url),{schemaVersion:1,model:'ordinary Rust unsigned byte literal patterns 1',retainSourceTypeOwners:true,cases,refusals,types,inference,tokens:[{token:'byteChar',expected:true},{token:'char',expected:false},{token:'number',expected:false},{token:'string',expected:false}],aliases:[{form:'strLit',expected:true},{form:'byteLit',expected:true},{form:'numLit',expected:false},{form:'ctor',expected:false}],history,formerDraftExpectation:'character pattern: characters are outside the portable core',source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=32 or 64'},references:['https://doc.rust-lang.org/reference/patterns.html#literal-patterns','https://doc.rust-lang.org/reference/expressions/literal-expr.html#byte-literal-expressions'],assertions:['everyByteSelectedAndFallbackOutcomes','actualUnsignedByteTypePreservedInsteadOfNumericCoercion','orderedArmsGuardsAliasesAndAbruptCompletions','actualBorrowedAndNestedFieldAliasOrigins','bothGraphCodecsAndFourTargetEmissions','independentOriginalAndGeneratedNativeProducts','exactFormerRejectedInputsAndCompletePredecessorAssertionsRetained'],boundary:'Unexecuted literal controls. Complete historical test sources retain predecessor assertions; only obsolete byte-pattern rejection expectations change, under explicit runtime/source ranges, to the existing required-main boundary. Unicode character patterns remain refused. General Rust scrutinee constraint propagation, range/slice patterns, full match ergonomics and complete Rust translation remain requirements. No construction receipt establishes behavioral evidence.'});
