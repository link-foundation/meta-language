import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {createRustTextValueProjection,emitRustTextValueProjection,emitGenericRustTextValueProjection} from './model.mjs';
const main=' fn main(){}';
const leaf=(parameters,body)=>'pub fn choose('+parameters+')->String{'+body+'}'+main;
const identityInputs=[['',''],['Ω😀','Ω😀'],['\0\b\t\n\r\f\u007f😀','\0\b\t\n\r\f\u007f😀']].map(([text,value])=>({args:[text],value}));
const cases=[
 {id:'owned-move',source:leaf('text:String','text'),operation:'owned standard String move',observations:identityInputs},
 {id:'owned-clone',source:leaf('text:String','text.clone()'),operation:'standard String clone',observations:identityInputs},
 {id:'shared-string-clone',source:leaf('text:&String','text.clone()'),operation:'standard String clone',observations:identityInputs},
 {id:'borrowed-str-construction',source:leaf('text:&str','String::from(text)'),operation:'standard String construction from borrowed text',observations:identityInputs},
 {id:'borrowed-str-owned',source:leaf('text:&str','text.to_owned()'),operation:'standard borrowed str owned construction',observations:identityInputs},
 {id:'borrowed-str-display',source:leaf('text:&str','text.to_string()'),operation:'standard borrowed str display construction',observations:identityInputs},
 {id:'cooked-literal-construction',source:leaf('','String::from("Ω😀")'),operation:'standard String construction from borrowed text',observations:[{args:[],value:'Ω😀'}]},
 {id:'raw-literal-construction',source:leaf('','String::from(r###"raw " Ω😀"###)'),operation:'standard String construction from borrowed text',observations:[{args:[],value:'raw " Ω😀'}]},
 {id:'literal-owned',source:leaf('','"\\0\\x7f\\u{1_f600}".to_owned()'),operation:'standard borrowed str owned construction',observations:[{args:[],value:'\0\u007f😀'}]},
 {id:'raw-literal-display',source:leaf('','r"Ω😀".to_string()'),operation:'standard borrowed str display construction',observations:[{args:[],value:'Ω😀'}]},
 {id:'valid-clone-retarget',source:leaf('left:String,right:&String','left.clone()'),operation:'standard String clone',observations:[{args:['first','second'],value:'first'}]},
 {id:'invalid-slice-retarget',source:leaf('left:String,right:&str','left.clone()'),operation:'standard String clone',observations:[{args:['first','second'],value:'first'}]},
 {id:'equal-valued-distinct-owners',source:'pub fn choose(left:String)->String{left.clone()} pub fn alternate(right:String)->String{right.clone()}'+main,operation:'standard String clone',observations:[{args:['Ω😀'],value:'Ω😀'}]},
];
const invalid=[
 ['raw-borrowed-literal-result',leaf('','r"zero"')],['cooked-borrowed-literal-result',leaf('','"zero"')],
 ['slice-clone',leaf('text:&str','text.clone()')],['literal-clone',leaf('','"zero".clone()')],['raw-literal-clone',leaf('','r"zero".clone()')],
 ['shared-string-move',leaf('text:&String','*text')],['double-reference-clone',leaf('text:&&String','text.clone()')],['mutable-string-reference',leaf('text:&mut String','text.clone()')],
 ['Boolean-construction',leaf('','String::from(true)')],['integer-construction',leaf('','String::from(42i32)')],
 ['borrowed-result','pub fn choose(text:&str)->&str{text}'+main],['fabricated-static-result',"pub fn choose(text:&str)->&'static str{text}"+main],
 ['explicit-lifetime',"pub fn choose<'a>(text:&'a str)->String{text.to_owned()}"+main],
 ['shadow-standard-owner','struct String;'+leaf('','String::from("zero")')],['shadow-owner-input',leaf('String:&str','String::from(String)')],['external-trait','use custom::Clone;'+leaf('text:String','text.clone()')],
 ['local-binding',leaf('text:String','let value=text;value')],['borrowed-String-from',leaf('text:&String','String::from(text)')],
 ['empty-main-effect',leaf('text:String','text').replace('fn main(){}','fn main(){println!("effect");}')],
 ['extra-parameter',leaf('one:String,two:String,three:String','one')],['forbidden-bool-parameter',leaf('flag:bool','String::from("zero")')],
 ['borrowed-slice-move',leaf('text:&str','*text')],['constructor-method-chain',leaf('','String::from("zero").clone()')],
 ['active-comment','pub/** documentation */ fn choose()->String{String::from("zero")}'+main],
 ['non-Rust-whitespace',leaf('text:String','text').replace('pub fn','pub\u00a0fn')],
 ['invalid-cooked-escape',leaf('','String::from("\\b")')],
];
export function rustTextValueProjectionFixtures(){
 const protectedPaths=['../../parity/fixtures/rust-source-indirection.lino','../../parity/fixtures/rust-source-signatures.lino','../rust-owned-local-state-fixture.lino','../owned-tuple-call-fixture.lino','../generated-owned-rust/data/policy.lino','../generated-owned-rust/data/rules.rs','../generated-owned-rust/data/fixtures.lino','../generated-owned-rust/data/value-edit-policy.lino','../generated-owned-rust/data/value-edit-fixtures.lino'];
 const protectedArtifacts=protectedPaths.map(path=>({path,sha256:createHash('sha256').update(readFileSync(new URL(path,import.meta.url))).digest('hex')}));
 const admitted=cases.map(row=>{const value=createRustTextValueProjection(row.source),emitted=emitGenericRustTextValueProjection(value.program,value.proof);let refusal;try{createRustTextValueProjection(emitted.text);throw new Error('generic wrapper unexpectedly admitted');}catch(error){if(error.message==='generic wrapper unexpectedly admitted')throw error;refusal={name:error.name,reason:error.reason??error.message,span:error.span??null};}return{...row,sourceBytes:Buffer.byteLength(row.source),sourceSha256:createHash('sha256').update(row.source).digest('hex'),fullGenericRust:emitted.text,genericBytes:Buffer.byteLength(emitted.text),genericSha256:createHash('sha256').update(emitted.text).digest('hex'),genericRefusal:refusal};});
 const refused=invalid.map(([id,source])=>{try{createRustTextValueProjection(source);}catch(error){return{id,source,name:error.name,reason:error.reason??error.message,span:error.span??null};}throw new Error('invalid source was admitted: '+id);});
 return{version:1,model:'closed standard Rust text value projection 1',cases:admitted,invalid:refused,protectedArtifacts};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){const file=new URL('./data/fixtures.lino',import.meta.url),value=rustTextValueProjectionFixtures();if(process.argv.includes('--check'))checkProjectData(file,value);else writeProjectData(file,value);}
