// Original multi-file inputs and independent expectations, never executed here.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,main,files={},value='7',options={}){cases.push({name,entry:'src/main.rs',inputs:[{path:'src/main.rs',source:main},...Object.entries(files).map(([path,source])=>({path,source}))],value,options});}
const show=expression=>'fn main(){println!("{}",'+expression+');}',read='pub fn read()->i64{7}';
add('ordinary module file','mod values;'+show('values::read()'),{'src/values.rs':read});
add('mod.rs module file','mod values;'+show('values::read()'),{'src/values/mod.rs':read});
add('ordinary module owns its subdirectory','mod values;'+show('values::read()'),{'src/values.rs':'mod nested;pub fn read()->i64{nested::read()}','src/values/nested.rs':read});
add('mod.rs shares its module directory','mod values;'+show('values::read()'),{'src/values/mod.rs':'mod nested;pub fn read()->i64{nested::read()}','src/values/nested/mod.rs':read});
add('inline module owns a subdirectory','mod outer{pub mod values;}'+show('outer::values::read()'),{'src/outer/values.rs':read});
add('external inline and external nesting','mod outer;'+show('outer::read()'),{'src/outer.rs':'mod inline{pub mod values;}pub fn read()->i64{inline::values::read()}','src/outer/inline/values.rs':read});
add('raw module names retain physical identity','mod r#move;'+show('r#move::read()'),{'src/move.rs':read});
add('Unicode module names retain their qualified owner','mod 𐐀;'+show('𐐀::read()'),{'src/𐐀.rs':read});
add('UTF16 and UTF8 source ranges are distinct','// 😀\r\nmod values; /* 𐐀 */'+show('values::read()'),{'src/values.rs':'/* 😀 */'+read});
add('trailing line comment never consumes a synthetic closing brace','mod values;'+show('values::read()'),{'src/values.rs':read+'// no trailing newline'});
add('module inner attributes remain attached to that module','mod values;'+show('values::read()'),{'src/values.rs':'#![allow(dead_code)]\n'+read});
add('actual constant value keeps its module owner','mod values;'+show('values::VALUE'),{'src/values.rs':'pub const VALUE:i64=7;'});
add('pointer constants remain callable values','mod values;'+show('values::READ(6)'),{'src/values.rs':'pub const READ:fn(i64)->i64=|value|value+1;'});
add('qualified sibling imports retain their provider','mod values;mod consumer;'+show('consumer::read()'),{'src/values.rs':'pub const VALUE:i64=7;','src/consumer.rs':'use crate::values::VALUE;pub fn read()->i64{VALUE}'});
add('a child may explicitly refer to its parent source constant','const OFFSET:i64=1;mod values;'+show('values::read()'),{'src/values.rs':'pub fn read()->i64{6+super::OFFSET}'});
add('all supplied unused concrete sources remain archived','mod values;'+show('values::read()'),{'src/values.rs':read,'src/unreferenced.rs':'pub fn extra()->i64{99}'});
add('a library records only its actual frontend entry normalization','mod values;pub fn result()->i64{values::read()}',{'src/values.rs':read},'7',{library:true});
add('multiple siblings retain original ordering','mod first;mod second;'+show('first::read()+second::read()'),{'src/first.rs':'pub fn read()->i64{3}','src/second.rs':'pub fn read()->i64{4}'});
add('a body raw identifier cannot invent a module','fn main(){let r#mod=7i64;println!("{}",r#mod);}');
add('nested record types retain their source namespace','mod values;'+show('values::read()'),{'src/values.rs':'struct Pair{left:i64,right:i64}pub fn read()->i64{let value=Pair{left:3,right:4};value.left+value.right}'});
add('a literal module path selects its actual supplied provider','#[path="alternate.rs"]mod values;'+show('values::read()'),{'src/alternate.rs':read,'src/values.rs':'pub fn read()->i64{99}'});
add('an explicit path is relative to the physical source directory','mod outer;'+show('outer::read()'),{'src/outer.rs':'#[path="alternate.rs"]mod values;pub fn read()->i64{values::read()}','src/alternate.rs':read,'src/outer/alternate.rs':'pub fn read()->i64{99}'});
add('a path-loaded ordinary filename has root-like child ownership','#[path="alternate.rs"]mod values;'+show('values::read()'),{'src/alternate.rs':'mod nested;pub fn read()->i64{nested::read()}','src/nested.rs':read,'src/alternate/nested.rs':'pub fn read()->i64{99}'});
add('a path-loaded source may itself have a path-loaded child','#[path="parts/alternate.rs"]mod values;'+show('values::read()'),{'src/parts/alternate.rs':'#[path="other.rs"]mod nested;pub fn read()->i64{nested::read()}','src/parts/other.rs':read});
add('an inline root module retains its ordinary source directory','mod outer{#[path="alternate.rs"]pub mod values;}'+show('outer::values::read()'),{'src/outer/alternate.rs':read});
add('an inline non-mod-rs module consumes its outstanding source stem','mod outer;'+show('outer::read()'),{'src/outer.rs':'mod inline{#[path="alternate.rs"]pub mod values;}pub fn read()->i64{inline::values::read()}','src/outer/inline/alternate.rs':read});
add('an inline path attribute supplies the actual directory','#[path="parts"]mod outer{pub mod values;}'+show('outer::values::read()'),{'src/parts/values.rs':read});
add('an inline path override discards an outstanding ordinary source stem','mod outer;'+show('outer::read()'),{'src/outer.rs':'#[path="parts"]mod inline{pub mod values;}pub fn read()->i64{inline::values::read()}','src/parts/values.rs':read,'src/outer/parts/values.rs':'pub fn read()->i64{99}'});
add('nested inline overrides retain their actual containing directory','mod outer{#[path="parts"]pub mod inline{pub mod values;}}'+show('outer::inline::values::read()'),{'src/outer/parts/values.rs':read,'src/parts/values.rs':'pub fn read()->i64{99}'});
add('a path-loaded inline parent combines with a child path','#[path="parts"]mod outer{#[path="alternate.rs"]pub mod values;}'+show('outer::values::read()'),{'src/parts/alternate.rs':read});
add('parent components may stay inside the complete project inventory','mod outer;'+show('outer::read()'),{'src/outer.rs':'#[path="../shared.rs"]mod values;pub fn read()->i64{values::read()}','shared.rs':read});
add('literal path normalization preserves source spelling separately','#[path="./parts/../alternate.rs"]mod values;'+show('values::read()'),{'src/alternate.rs':read});
add('raw string path literals retain their actual lexical value','#[path=r###"alternate.rs"###]mod values;'+show('values::read()'),{'src/alternate.rs':read});
add('literal paths retain Unicode scalar names and coordinates','// 😀\n#[path="語/𐐀.rs"]mod values;'+show('values::read()'),{'src/語/𐐀.rs':read});
add('a Rust module path need not have an rs file extension','#[path="parts/source.txt"]mod values;'+show('values::read()'),{'src/parts/source.txt':read});
add('ordinary attributes and restricted visibility retain their attachment','#[doc="actual path [ ]"]#[path="alternate.rs"]pub(crate) mod values;'+show('values::read()'),{'src/alternate.rs':read});
add('two source modules may load one file under separate owners','#[path="shared.rs"]mod first;#[path="shared.rs"]mod second;'+show('first::read()+second::read()-7'),{'src/shared.rs':read});
add('generated partition layout remains source-composable','#[path="rules/part_1.rs"]mod part_1;use self::part_1::read;'+show('read()'),{'src/rules/part_1.rs':read});
add('a complete source module retains its transparent alias definitions','mod values;'+show('values::read()'),{'src/values.rs':'pub type Number=i64;pub fn read()->Number{7}'});
add('an imported source alias retains its provider CST','mod values;use self::values::Number;fn read(value:Number)->Number{value+1}'+show('read(6)'),{'src/values.rs':'pub type Number=i64;'});
for(const sourceVersion of ['1.90.0','1.96.0','1.97.0','1.99.0',''])for(const retainSourceTypeOwners of [false,true]){
 add('explicit source snapshot '+(sourceVersion||'default')+' with owners '+retainSourceTypeOwners,'mod values;'+show('values::read()'),{'src/values.rs':read},'7',{sourceVersion,retainSourceTypeOwners});
 add('library source snapshot '+(sourceVersion||'default')+' with owners '+retainSourceTypeOwners,'mod values;pub fn result()->i64{values::read()}',{'src/values.rs':read},'7',{sourceVersion,retainSourceTypeOwners,library:true});
}
const versionRefusals=[
 ...['1.89.0','1.100.0','1.99','1.099.0','1.99.0-beta.1'].map(sourceVersion=>({name:'uncovered project source snapshot '+sourceVersion,options:{sourceVersion},message:'Rust source version: select a canonical stable Rust source version from 1.90.0 through 1.99.x'})),
 ...[null,99,true,[],{}].map(sourceVersion=>({name:'nontext project source snapshot '+JSON.stringify(sourceVersion),options:{sourceVersion},message:'Rust sourceVersion must be a string'})),
];
const refusals=[];
function refuse(name,inputs,message,options={}){refusals.push({name,entry:'src/main.rs',inputs,options,message});}
const files=(main,other={})=>[{path:'src/main.rs',source:main},...Object.entries(other).map(([path,source])=>({path,source}))];
refuse('missing original module',files('mod absent;fn main(){}'),'missing Rust project module file');
refuse('two physical module candidates',files('mod values;fn main(){}',{'src/values.rs':'','src/values/mod.rs':''}),'ambiguous Rust project module files');
refuse('duplicate original paths',[...files('fn main(){}'),...files('fn main(){}')],'duplicate Rust project source path');
refuse('paths cannot traverse the source inventory',[{path:'../main.rs',source:'fn main(){}'}],'invalid Rust project source path');
for(const attribute of ['#[cfg(any())]','#[cfg_attr(any(),path="elsewhere.rs")]','#[path="elsewhere.rs"]','#[macro_use]']){
 refuse('uninterpreted module attribute '+attribute,files(attribute+'mod values;fn main(){}',{'src/values.rs':''}),attribute==='#[path="elsewhere.rs"]'?'missing explicit Rust project module file: src/elsewhere.rs':'module attributes need an explicit configuration interpretation');
 if(attribute==='#[path="elsewhere.rs"]')refusals.at(-1).previousExpectation={message:'module attributes need an explicit configuration interpretation',revision:'4f946613',evidence:'https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute',correction:'The original input names an absent explicit provider; a present conventional filename cannot replace it. The original source and refusal coverage remain.'};
}
refuse('uninterpreted inner configuration',files('#![cfg(any())]fn main(){}'),'module attributes need an explicit configuration interpretation');
refuse('a physical mod.rs source cannot recursively load itself',files('mod r#mod;fn main(){}',{'src/mod.rs':'mod r#mod;'}),'cyclic Rust project module file');
refuse('inline module nesting uses the explicit depth bound',files('mod first{mod second{mod third{}}}fn main(){}'),'module expansion limit exceeded',{depthLimit:2});
refuse('all sibling expansions count',files('mod first;mod second;fn main(){}',{'src/first.rs':'','src/second.rs':''}),'module expansion limit exceeded',{expansionLimit:1});
refuse('composed source allocation is bounded',files('fn main(){}'),'composed source unit limit exceeded',{outputUnitLimit:1});
refuse('input bytes are bounded',files('fn main(){}'),'input byte limit exceeded',{byteLimit:1});
refuse('tokens are bounded',files('fn main(){}'),'token limit exceeded',{tokenLimit:1});
refuse('original file inventory is bounded',files('fn main(){}',{'src/other.rs':''}),'bounded nonempty source inventory',{fileLimit:1});
refuse('lexical groups retain actual closing kinds',files('fn main(){]'),'unbalanced Rust project source group');
refuse('a path cannot escape the supplied project inventory',files('#[path="../../outside.rs"]mod values;fn main(){}'),'literal path escapes supplied inventory');
refuse('absolute paths have no portable inventory interpretation',files('#[path="/host/source.rs"]mod values;fn main(){}'),'nonportable Rust project literal path');
refuse('duplicate path attributes remain explicit',files('#[path="first.rs"]#[path="last.rs"]mod values;fn main(){}'),'duplicate Rust project module path attributes');
refuse('a computed module path is not a literal',files('#[path=concat!("source", ".rs")]mod values;fn main(){}'),'path attribute needs an outer literal module path');
refuse('a path attribute must belong to its actual module',files('#[path="source.rs"]fn main(){}'),'path attribute needs its actual module owner');
refuse('an inner path needs its separate attachment interpretation',files('#![path="source.rs"]fn main(){}'),'path attribute needs an outer literal module path');
refuse('normalized explicit paths retain physical cycle checks',files('#[path="./main.rs"]mod values;fn main(){}'),'cyclic Rust project module file');
const planning=[{name:'macro token trees never invent loaded source modules',inputs:files('macro_rules! words { () => { mod phantom; } } fn main(){}'),entry:'src/main.rs',modules:0},{name:'macro path attributes never invent file interpretations',inputs:files('macro_rules! words { () => { #[path="missing.rs"]mod phantom; } } fn main(){}'),entry:'src/main.rs',modules:0}];
const revision='cd3e4be5',history=['js/src/translation/check.js','js/src/translation/rust.js','js/src/index.js','js/src/cli.js','rust/src/translation/check.rs','rust/src/translation/check/items.rs','rust/src/translation/mod.rs','rust/src/translation/rust/items.rs','rust/src/lib.rs','rust/src/main.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
for(const path of ['js/src/rust-project-source-rules.js','rust/src/rust_source_project.rs','parity/fixtures/rust-source-project.lino']){const revision='59ab34c2',bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:8*1024*1024});history.push({path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')});}
writeProjectData(new URL('../../parity/fixtures/rust-source-project.lino',import.meta.url),{schemaVersion:1,model:'complete Rust source module composition 1',cases,refusals,versionRefusals,planning,history,versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},references:['https://doc.rust-lang.org/reference/items/modules.html','https://github.com/rust-lang/rust/blob/1.90.0/compiler/rustc_expand/src/module.rs'],assertions:['actualLiteralPathsAndOutstandingDirectoryOwnership','interpretedAttributesRetainOriginalConcreteIntervals','competingPhysicalFilesNeverReplaceExplicitProviders','actualModuleOwnersAndPhysicalLayouts','allOriginalFilesAndCompleteConcreteGraphsRetained','sourceUnicodeCoordinatesAndSyntheticBoundariesExplicit','explicitSourceVersionSurvivesLibraryNormalizationAndBothCodecs','missingAmbiguousRecursiveConfiguredAndResourceCasesExplicit','ordinaryOwnershipTypesAndCallableValuesUseExistingFrontend','bothCodecsRestoreOriginalFilesWithoutSourceBuffers','completeSharedDefinitionsAndNativeProducts','allOriginalSourceFilesCompileIndependentlyInCI'],boundary:'Future controls only. No composition, source parse, product execution, oracle, test, compilation or observation ran. This supplies ordinary source composition and concrete archives; complete Rust privacy, configuration, inner/duplicate path-attribute and include/macro semantics, native grammar independence, Cargo/dependency ABI and every preceding requirement remain mandatory.'});
