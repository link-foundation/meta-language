// Source and expected trace declarations only; no represented call is executed.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['ordinary function output','fn report(value:i64){println!("value {}",value);}fn main(){report(7);report(8);}', ['value 7','value 8']],
 ['repeated positional value evaluates its operand once','fn value()->i64{println!("argument");7}fn report(){println!("{0}:{0}",value());}fn main(){report();}', ['argument','7:7']],
 ['reordered placeholders preserve raw operand order','fn first()->i64{println!("first");1}fn second()->i64{println!("second");2}fn main(){println!("{1}:{0}",first(),second());}', ['first','second','2:1']],
 ['explicit position leaves implicit cursor unchanged','fn main(){println!("{}",format!("{1}{}{}",1,2));}', ['212']],
 ['owned formatted string return','fn build(value:i64)->String{format!("({0})/{0}",value)}fn main(){println!("{}",build(3));}', ['(3)/3']],
 ['tail print expression is Unit','fn report(){println!("tail")}fn main(){report();}', ['tail']],
 ['captured Unicode identifier is normalized','fn main(){let café=4;println!("字{cafe\\u{301}}");}', ['字4']],
 ['captured keyword resolves a raw source identifier','fn main(){let r#type=5;println!("{type}");}', ['5']],
 ['trailing placeholder whitespace','fn main(){println!("{0  }",6);}', ['6']],
 ['escaped braces retain the literal text','fn main(){println!("{{{0}}}",9);}', ['{9}']],
 ['empty print writes one empty line','fn report(){println!();}fn main(){report();}', ['']],
 ['selected branch output stays lazy','fn report(flag:bool){if flag{println!("selected");}else{println!("other");}}fn main(){report(true);report(false);}', ['selected','other']],
 ['print precedes terminal return','fn report()->i64{println!("before");return 7;}fn main(){println!("{}",report());}', ['before','7']],
 ['nested module context retains its own names','mod nested{fn value()->i64{8}pub fn report(){println!("module {}",value());}}fn main(){nested::report();}', ['module 8']],
 ['format temporary cannot capture source names','fn main(){let ml_tuple_value0=11;println!("{0}:{ml_tuple_value0}",12);}', ['12:11']],
 ['ordinary Unit guard exits before later output','fn report(flag:bool){if flag{return;}println!("after");}fn main(){report(true);report(false);}', ['after']],
 ['output before and after a selected guard','fn report(flag:bool)->i64{println!("before");if flag{return 7;}println!("after");9}fn main(){println!("{}",report(true));println!("{}",report(false));}', ['before','7','before','after','9']],
 ['implicit Unit fallback after a terminal return branch','fn report(flag:bool){if flag{return;}}fn main(){report(true);report(false);println!("done");}', ['done']],
];
const cases=inputs.map(([name,source,lines],index)=>({name,source,lines,module:'function_output_'+index,marker:'META_FUNCTION_OUTPUT_'+index}));
const source=cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\npub fn invoke(){main();}\n}').join('\n')+'\nfn main(){\n'+cases.map(row=>'println!("'+row.marker+'");'+row.module+'::invoke();').join('\n')+'\n}';
writeProjectData(new URL('../../parity/fixtures/rust-function-output.lino',import.meta.url),{
 schemaVersion:1,model:'Rust function print and shared Display formatting source traces 1',sourceLanguage:'Rust',cases,source,lines:cases.flatMap(row=>[row.marker,...row.lines]),
 refusals:[
  {name:'unused explicit operand is a compile-time error',source:'fn main(){println!("{1}",1,2);}',kind:'syntax',reason:'format string has fewer {} than arguments'},
  {name:'format specs retain their explicit obligation',source:'fn main(){println!("{0:04}",1);}',kind:'unsupported',reason:'format spec {0:04}: only plain {} Display formatting is portable'},
  {name:'bare underscore cannot be a captured argument',source:'fn main(){println!("{_}");}',kind:'unsupported',reason:'format argument {_}: outside the portable core'},
 ],
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; source identifier profile Unicode 17.0.0'},
 sourceBasis:'https://doc.rust-lang.org/std/fmt/ ; https://doc.rust-lang.org/std/macro.println.html ; https://doc.rust-lang.org/reference/expressions/return-expr.html',
 boundary:'Future original Rust and translated JavaScript/native full traces, complete source parse parity, definition/program graphs, codecs and four emissions. No observations. Inputs remain byte-for-byte inside composed modules, with a separate invocation facade and trace markers. Complete format specs, named argument assignments, custom Display/borrow effects, macro namespaces and every preceding requirement remain.',
});
