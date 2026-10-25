// Future whole-module publications and consumer controls, never executed here.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const publications=[];
function add(name,source,expression,exports,checks=[],value='7',ret='i64'){
 publications.push({name,source:source+'\npub fn observe()->'+ret+'{'+expression+'}\n',exports:[...exports,'observe'],checks,value,ret});
}
const constant=(name,kind,value)=>({name,kind,value,call:null,sourceKind:'const'}),callable=(name,args,value,sourceKind='const')=>({name,kind:'function',value,call:args,sourceKind});
add('a public integer constant exports its actual value','pub const VALUE:i64=7;','VALUE',['VALUE'],[constant('VALUE','bigint','7')]);
add('a private integer constant remains a private helper','const VALUE:i64=7;','VALUE',[]);
add('a computed constant remains a public value','pub const VALUE:i64=3+4;','VALUE',['VALUE'],[constant('VALUE','bigint','7')]);
add('a public Boolean constant retains its actual kind','pub const FLAG:bool=true;','if FLAG{7}else{8}',['FLAG'],[constant('FLAG','boolean','true')]);
add('a public binary64 constant retains its actual kind','pub const VALUE:f64=7.0;','VALUE',['VALUE'],[constant('VALUE','number','7')],'7','f64');
add('a public unsigned constant keeps all source bits','pub const VALUE:u64=18446744073709551615;','VALUE',['VALUE'],[constant('VALUE','bigint','18446744073709551615')],'18446744073709551615','u64');
add('a public scalar string retains its real value','pub const TEXT:&str="😀";','TEXT.len()',['TEXT'],[constant('TEXT','string','😀')],'4','usize');
add('a public pointer constant exports its actual callable value','fn known(value:i64)->i64{value+1}pub const READ:fn(i64)->i64=known;','READ(6)',['READ'],[callable('READ',['6n'],'7')]);
add('a public pointer closure retains its real source signature','pub const READ:fn(i64)->i64=|value|value+1;','READ(6)',['READ'],[callable('READ',['6n'],'7')]);
add('a zero input public pointer constant remains callable','pub const READ:fn()->i64=||7;','READ()',['READ'],[callable('READ',[],'7')]);
add('a private pointer constant stays behind its public function','const READ:fn(i64)->i64=|value|value+1;pub fn compute()->i64{READ(6)}','compute()',['compute'],[callable('compute',[],'7','fn')]);
add('public functions and constants keep independent signature kinds','pub const OFFSET:i64=1;pub fn compute(value:i64)->i64{value+OFFSET}','compute(6)',['OFFSET','compute'],[constant('OFFSET','bigint','1'),callable('compute',['6n'],'7','fn')]);
add('a public constant alias retains its actual value','const FIRST:i64=7;pub const VALUE:i64=FIRST;','VALUE',['VALUE'],[constant('VALUE','bigint','7')]);
add('restricted constant visibility never becomes a public export','pub(crate) const VALUE:i64=7;','VALUE',[]);
add('public constant attributes retain their full lexical header','#[doc="brackets [ ] remain text"]pub const VALUE:i64=7;','VALUE',['VALUE'],[constant('VALUE','bigint','7')]);
add('a raw JavaScript keyword remains a legal export alias','pub const r#default:i64=7;','r#default',['default'],[constant('default','bigint','7')]);
add('a decomposed Rust identifier retains its canonical source binding','pub const e\u0301:i64=7;','e\u0301',['é'],[constant('é','bigint','7')]);
add('actual source function names survive the reserved helper prefix','pub fn ml_chosen()->i64{7}','ml_chosen()',['ml_chosen'],[callable('ml_chosen',[],'7','fn')]);
add('export allocation retains actual emitted identifier collisions','pub fn ml_source_constant_export_0()->i64{1}pub const VALUE:i64=7;','VALUE',['ml_source_constant_export_0','VALUE'],[constant('VALUE','bigint','7')]);
add('public pointer constants keep real global constant dependencies','const OFFSET:i64=1;pub const READ:fn(i64)->i64=|value|value+OFFSET;','READ(6)',['READ'],[callable('READ',['6n'],'7')]);
const fixed={kind:'fixed',bits:64,signed:true},fnType={kind:'function',params:[fixed],ret:fixed},zeroType={kind:'function',params:[],ret:fixed};
const consumers=[];
function consume(name,body,externals,setup,value='7',events=[]){consumers.push({name,source:'export function result(){'+body+'}',externals,setup,value,events});}
const read={k:'const',name:'READ',type:fnType,literal:false},zero={k:'const',name:'READ',type:zeroType,literal:false};
consume('an imported callable constant uses its actual supplied parameter','return READ(6n);',[read],'const READ=value=>value+1n;');
consume('an imported callable constant keeps its zero argument application','return READ();',[zero],'const READ=()=>7n;');
consume('a bare imported callable constant remains a value read','const read=READ;return read(6n);',[read],'const READ=value=>value+1n;');
consume('a passed imported callable constant retains its actual type','return apply(READ);',[read,{k:'fn',name:'apply',params:[{name:'read',type:fnType}],ret:fixed}],'const READ=value=>value+1n;const apply=read=>read(6n);');
consume('an imported factory remains a call before the returned value call','return MAKE()(6n);',[{k:'fn',name:'MAKE',params:[],ret:fnType}],'const MAKE=()=>value=>value+1n;');
consume('a local binding retains precedence over an external callable constant','const READ=value=>value+2n;return READ(5n);',[read],'const READ=value=>value+99n;');
consume('excess callable-constant arguments run before its actual call','return READ(7n,mark());',[read,{k:'fn',name:'mark',params:[],ret:fixed}],'const events=[];const READ=value=>{events.push("call");return value;};const mark=()=>{events.push("extra");return 9n;};','7',['extra','call']);
consume('excess zero input constant arguments retain their effects','return READ(mark());',[zero,{k:'fn',name:'mark',params:[],ret:fixed}],'const events=[];const READ=()=>{events.push("call");return 7n;};const mark=()=>{events.push("extra");return 9n;};','7',['extra','call']);
const cases=publications.map((row,index)=>({name:row.name,module:'module_value_case_'+index,source:row.source+'pub fn result()->'+row.ret+'{observe()}',value:row.value,abort:null}));
const revision='21af9724',history=['js/src/self-translation.js','js/src/translation/check.js','js/src/translation/javascript-infer.js','rust/src/self_translation/semantic_input.rs','rust/src/self_translation.rs','rust/src/translation/check.rs','rust/src/translation/check/expressions.rs','rust/src/translation/javascript/infer.rs','rust/src/translation/javascript/infer/solver.rs','rust/src/translation/javascript/infer/callables.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-module-values.lino',import.meta.url),{schemaVersion:1,model:'ordinary Rust source modules with actual public constant values 1',cases,refusals:[],publications,consumers,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; source target_pointer_width=64'},references:['https://doc.rust-lang.org/reference/const_eval.html','https://doc.rust-lang.org/reference/names/scopes.html'],assertions:['completeOriginalModuleOwnersAndPrivateBodiesChecked','actualPublicConstantValuesAndCheckedSignatureKinds','privateAndRestrictedVisibilityPreserved','normalizedOriginalNamesAndCollisionFreeTargetBindings','callableConstantReadsRemainDistinctFromSourceApplications','sameComputedArgumentChecksInferenceAndEvaluationOrder','checkedProjectionNeverReinterpretsAStoredValueRead','completePredecessorsBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. Covers existing ordinary flat function/constant modules published as JavaScript and the common callable constant application path. Source interfaces describe checked value meanings; full native provider ABI/representation bridges, nonordinary module composition, datatypes, imports into Rust source, generics, initialization/cycle semantics and all prior full-scope requirements remain obligations.'});
