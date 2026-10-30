// Original sources and independently authored expectations, never evaluated.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const primitive=name=>({kind:'primitive',path:[name],arguments:[]}),nominal=name=>({kind:'declaration',path:name.split('.'),arguments:[]});
const position=(type,owner,wrappers=[],lifetimes=[])=>({type,owner,wrappers,lifetimes});
const fixed=(bits,signed)=>({kind:'fixed',bits,signed}),usize=position(fixed(64,false),primitive('usize'));
const cases=[];
function add(name,source,fields,dataTypeName='Value',constructors=null){cases.push({name,module:'data_field_case_'+cases.length,source:source+'pub fn result()->i64{7}fn main(){}',dataTypeName,signature:{model:'Rust data type field signatures 1',language:'Rust',dataTypeName,ctors:constructors??[{name:dataTypeName.split('.').at(-1),fields}]},value:'7',abort:null});}
for(const name of ['i8','u8','i16','u16','i32','u32','i64','u64','i128','u128','usize','isize']){
 const bits=name==='usize'||name==='isize'?64:Number(name.slice(1));
 add('actual '+name+' field owner remains distinct','struct Value{value:'+name+'}',[position(fixed(bits,name[0]==='i'),primitive(name))]);
}
add('tuple struct positions remain ordered','struct Value(usize,bool);',[usize,position({kind:'bool'},primitive('bool'))]);
add('unit structs retain an explicit empty constructor inventory','struct Value;',[]);
add('all unit tuple and named enum variants retain their actual positions','enum Value{Nothing,One(usize),Named{value:u64}}',[], 'Value',[
 {name:'Nothing',fields:[]},{name:'One',fields:[usize]},{name:'Named',fields:[position(fixed(64,false),primitive('u64'))]},
]);
add('transparent aliases retain the actual underlying field owner','type Position=usize;struct Value{value:Position}',[usize]);
const vectorOwner={kind:'library',path:['std','vec','Vec'],arguments:[primitive('usize')]};
add('standard vectors recursively retain their actual element owner','struct Value{values:Vec<usize>}',[position({kind:'array',element:fixed(64,false)},vectorOwner)]);
add('static slices retain their actual reference and element owners',"struct Value{values:&'static [usize]}",[position({kind:'array',element:fixed(64,false)},{kind:'slice',path:[],arguments:[primitive('usize')]},['reference'],['static'])]);
const child=position({kind:'data',name:'Child'},nominal('Child'));
add('direct nominal fields reference their complete actual declaration','struct Child;struct Value{child:Child}',[child]);
add('boxed nominal fields retain their distinct constructor layer','struct Child;struct Value{child:Box<Child>}',[position(child.type,{kind:'library',path:['std','boxed','Box'],arguments:[child.owner]},['box'])]);
add('recursive datatype fields use a nominal link instead of unfolding forever','struct Value{next:Box<Value>}',[position({kind:'data',name:'Value'},{kind:'library',path:['std','boxed','Box'],arguments:[nominal('Value')]},['box'])]);
add('forward mutually recursive datatypes retain the actual referenced owner','struct Left{right:Box<Right>}struct Right{left:Box<Left>}',[position({kind:'data',name:'Right'},{kind:'library',path:['std','boxed','Box'],arguments:[nominal('Right')]},['box'])],'Left');
add('qualified modules preserve complete datatype identities','mod nested{pub struct Value{value:usize}}',[usize],'nested.Value');
const identities=[['usize','u64'],['isize','i64']].map(types=>({name:types.join(' versus '),sources:types.map(type=>'struct Value{value:'+type+'}pub fn identity(value:Value)->Value{value}fn main(){}')}));
identities.push({name:'boxed versus direct nominal field',sources:['Child','Box<Child>'].map(type=>'struct Child;struct Value{child:'+type+'}pub fn identity(value:Value)->Value{value}fn main(){}')});
const malformed=[
 {path:['model'],value:'unknown'}, {path:['language'],value:'JavaScript'},
 {path:['dataTypeName'],value:'Different'}, {path:['ctors'],value:[]},
 {path:['ctors',0,'name'],value:'Different'}, {path:['ctors',0,'fields'],value:[]},
 {path:['ctors',0,'fields',0,'owner','path'],value:['i64']},
 {path:['ctors',0,'fields',0,'wrappers'],value:['box']},
 {path:['ctors',0,'fields',0,'lifetimes'],value:['static']},
 {path:['ctors',0,'fields',0,'type','signed'],value:true},
 {path:['ctors',0,'fields',0,'type','unrepresented'],value:true},
 {path:['ctors',0,'fields',0,'type','reference'],value:false},
 {path:['unrepresented'],value:true},
];
const model='Rust data type field signatures 1',frames=[];
for(const language of ['Rust','JavaScript'])for(const declared of ['Value','Other'])for(const constructors of [0,1,2,-1,0.5,100001])frames.push({model,language,declared,actual:'Value',constructors,expectedCount:1,expected:language==='Rust'&&declared==='Value'&&constructors===1});
frames.push({model:'unknown',language:'Rust',declared:'Value',actual:'Value',constructors:1,expectedCount:1,expected:false});
const fields=[];for(const name of ['Value','Other'])for(const count of [0,1,2,-1,0.5,100001])fields.push({name,actual:'Value',fields:count,expectedCount:1,expected:name==='Value'&&count===1});
const budgets=[];for(const visits of [-1,0,1,100000,100001,0.5])for(const depth of [-1,0,256,257,0.5])budgets.push({visits,depth,expected:Number.isInteger(visits)&&visits>=0&&visits<=100000&&Number.isInteger(depth)&&depth>=0&&depth<=256});
const containers=[];for(const kind of ['record','array','map','function'])for(const dense of [false,true])for(const unrepresented of [false,true])containers.push({kind,dense,unrepresented,expected:!unrepresented&&(kind==='record'||kind==='array'&&dense),programExpected:!unrepresented&&(kind==='record'||kind==='array'&&dense||kind==='map')});
const compact={sources:['&&','& &'].map(references=>'pub fn length(values:'+references+'[usize])->usize{values.len()}fn main(){}')};
const project={inputs:[{path:'lib.rs',source:'mod values;pub fn result()->usize{7usize}'},{path:'values.rs',source:'pub struct Value{value:usize}'}],entry:'lib.rs',options:{library:true,retainSourceTypeOwners:true}};
const revision='3f7b9c84',paths=['js/src/translation/rust.js','js/src/translation/rust-records.js','js/src/translation/check.js','js/src/typed-program-links.js','js/src/typed-program-links-validation.js','js/src/typed-program-checked-projection.js','js/src/data/typed-program-links-schema.lino','rust/src/translation/rust/items.rs','rust/src/translation/rust/records.rs','rust/src/translation/check/items.rs','rust/src/translation/surface.rs','rust/src/translation/ir/declarations.rs','rust/src/typed_program_links/checked/declarations.rs','rust/src/translation/mod.rs','rust/tests/unit/mod.rs','parity/fixtures/rust-source-owner-retention.lino'];
paths.push('rust/src/typed_program_links/checked.rs');
const history=paths.map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:8*1024*1024});return{path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-data-type-field-signatures.lino',import.meta.url),{schemaVersion:1,model,retainSourceTypeOwners:true,controlSource:'struct Value{value:usize}pub fn result()->i64{7}fn main(){}',cases,refusals:[],identities,malformed,frames,fields,budgets,containers,compact,project,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},references:['https://doc.rust-lang.org/reference/items/structs.html','https://doc.rust-lang.org/reference/items/enumerations.html','https://doc.rust-lang.org/reference/types/pointer.html'],assertions:['everyActualConstructorAndFieldRetainsItsCompleteOrderedSourceTypeOwner','nominalCyclesReferenceOwnedDeclarationsWithoutInfiniteUnfolding','equalPortableLayoutsDoNotCertifyEqualSourceTypeMeanings','malformedNullInheritedAccessorSparseAndUnrepresentedMetadataRefuses','plainParsingRemainsIndependentAndCompactReferenceTokensAlreadySplit','bothCodecsFourEmissionsAndSourceProjectConcreteGraphsPreserveOwners','everySharedDecisionAndCompleteOriginalNativeProductRemainsPaired'],boundary:'Unexecuted source draft. Opt-in source-owner retention includes datatype fields through the existing recursive source type interpretation. Nominal type references link actual declarations; complete constructor identities and field counts are checked. This does not implement general Rust aggregate ABI reconstruction, layout/representation attributes, generics, traits, named datatype lifetime parameters, arbitrary field coercions, full ownership, callable-field target encodings or a semantic equivalence certificate. Original source owners and complete predecessor context remain explicit.'});
