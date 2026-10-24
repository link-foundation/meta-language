// Source declarations only; source, native, oracle and assertions run later.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['selected guarded return','pub fn result()->i64{if true{return 7;}9}','7'],
 ['unselected guarded return','pub fn result()->i64{if false{return 7;}9}','9'],
 ['multiple guards retain their ordering','fn choose(first:bool,second:bool)->i64{if first{return 1;}if second{return 2;}3}pub fn result()->i64{choose(true,false)*100+choose(false,true)*10+choose(false,false)}','123'],
 ['guard-local shadow cannot capture continuation','fn choose(flag:bool)->i64{let value=20;if flag{let value=7;return value;}value}pub fn result()->i64{choose(true)+choose(false)}','27'],
 ['both alternatives return','fn choose(flag:bool)->i64{if flag{return 7;}else{return 9;}}pub fn result()->i64{choose(true)*10+choose(false)}','79'],
 ['terminal returned and value alternatives','fn choose(flag:bool)->i64{if flag{return 7;}else{9}}pub fn result()->i64{choose(true)*10+choose(false)}','79'],
 ['terminal value and returned alternatives','fn choose(flag:bool)->i64{if flag{7}else{return 9;}}pub fn result()->i64{choose(true)*10+choose(false)}','79'],
 ['guarded string return has actual owned result','fn choose(flag:bool)->String{if flag{return String::from("short");}String::from("longer")}pub fn result()->usize{choose(true).len()*10+choose(false).len()}','56'],
 ['guarded nominal return retains its constructor','struct Record{value:i64}fn choose(flag:bool)->Record{if flag{return Record{value:3};}Record{value:4}}pub fn result()->i64{choose(true).value*10+choose(false).value}','34'],
 ['nested guards with complete returns','fn choose(first:bool,second:bool)->i64{if first{if second{return 1;}else{return 2;}}3}pub fn result()->i64{choose(true,true)*100+choose(true,false)*10+choose(false,true)}','123'],
 ['an exiting alternative permits its else-if continuation','fn choose(first:bool,second:bool)->i64{if first{return 1;}else if second{return 2;}3}pub fn result()->i64{choose(true,false)*100+choose(false,true)*10+choose(false,false)}','123'],
 ['Unit statements before guarded return retain the call','fn empty(){}pub fn result()->i64{empty();if true{return 18;}19}','18'],
 ['guarded abort is lazy','pub fn result()->i64{if false{panic!("unselected");}7}','7'],
 ['return keyword text is not function control','fn text()->String{String::from("return")}pub fn result()->usize{text().len()}','6'],
 ['raw return identifier stays a value','pub fn result()->i64{let r#return=23;r#return}','23'],
];
const cases=inputs.map(([name,source,value],index)=>({name,module:'ordinary_return_case_'+index,source,value,abort:null}));
cases.push({name:'selected guarded abort preserves its payload',module:'ordinary_return_abort',source:'pub fn result()->i64{if true{panic!("guard failure");}7}',value:null,abort:'guard failure'});
writeProjectData(new URL('../../parity/fixtures/rust-ordinary-returns.lino',import.meta.url),{
 schemaVersion:1,model:'ordinary Rust guarded function completion source values 1',sourceLanguage:'Rust',cases,
 source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n'),
 refusals:[
  {name:'continuing else-if needs a whole joined carrier',source:'fn report(first:bool,second:bool){if first{println!("first");}else if second{return;}println!("after");return;}fn main(){}',kind:'unsupported',reason:'guarded return completion: an else-if after a continuing alternative needs its complete joined completion carrier'},
  {name:'nested partial exits cannot be mistaken for ordinary Unit values',source:'fn report(first:bool,second:bool){if first{if second{return;}();}else{();}println!("after");return;}fn main(){}',kind:'unsupported',reason:'guarded return completion: every guard branch must complete a return or abort, with an enclosing continuation; mixed return/value branches need a separate completion proof'},
 ],
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://doc.rust-lang.org/reference/expressions/return-expr.html ; https://doc.rust-lang.org/reference/expressions/if-expr.html',
 boundary:'Future original Rust/emitted JavaScript/native values and aborts, source parse parity, complete definition/program graphs, codecs and four emissions. No observations or invented selected-profile proof. Existing explicit completion models stay separate. General mixed-exit continuation carriers, pattern/control ownership, loops and every preceding requirement remain.',
});
