// Literal independent originals. Source authoring never parses or runs them.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value,ret='u8',declarations=''){cases.push({name,module:'owned_binding_case_'+cases.length,source:declarations+'pub fn result()->'+ret+'{'+body+'}',value,abort:null});}
add('a named vector transfers to a new owner','let a=vec![65u8];let b=a;b[0]','65');
add('successive transfers retain the payload','let a=vec![65u8];let b=a;let c=b;c[0]','65');
add('shadowing allocates a distinct owner identity','let a=vec![65u8];let a=a;a[0]','65');
add('cloning retains the original owner','let a=vec![65u8];let b=a.clone();a[0]+b[0]','130');
add('a copy survives consumption of the original vector','let a=vec![65u8];let b=a.clone();let c=a;b[0]+c[0]','130');
add('a copied scalar survives its vector move','let a=vec![65u8];let byte=a[0];let b=a;byte+b[0]','130');
add('an owned vector parameter may transfer to another binding','take(vec![65u8])','65','u8','fn take(a:Vec<u8>)->u8{let b=a;b[0]}');
add('a named argument may transfer into an owned parameter','let a=vec![65u8];take(a)','65','u8','fn take(a:Vec<u8>)->u8{a[0]}');
add('a borrowed parameter preserves the original owner','let a=vec![65u8];let byte=read(&a);let b=a;byte+b[0]','130','u8','fn read(a:&Vec<u8>)->u8{a[0]}');
add('a reference may be read before its owner transfers','let a=vec![65u8];let r=&a;let byte=r[0];let b=a;byte+b[0]','130');
add('an element reference may be read before its owner transfers','let a=vec![65u8];let r=&a[0];let byte=*r;let b=a;byte+b[0]','130');
add('both returning alternatives may consume the same incoming owner','let a=vec![65u8];if true{let b=a;b[0]}else{let c=a;c[0]}','65');
add('the same incoming owner is available independently in match arms','let a=vec![65u8];match true{true=>{let b=a;b[0]},false=>{let c=a;c[0]}}','65');
add('a nested scope may transfer its result into an outer owner','let a={let b=vec![65u8];b};a[0]','65');
add('a scalar copied out of a nested scope outlives its vector','let byte={let a=vec![65u8];a[0]};byte','65');
add('a vector wildcard match leaves its place available','let a=vec![65u8];let byte=match a{_=>1u8};a[0]+byte','66');
add('a vector binding pattern transfers its matched place','let a=vec![65u8];match a{b=>b[0]}','65');
add('a wildcard let does not consume an existing vector','let a=vec![65u8];let _=a;a[0]','65');
add('a named vector enters an owned immediate iterator','let a=vec![65u8,66u8];a.into_iter().count()','2','usize');
add('owned reduction preserves source values after a transfer','let a=vec![2u8,3u8];let b=a;b.into_iter().sum::<u8>()','5');
add('counting owned strings consumes only their vector','let a=vec![String::from("a"),String::from("b")];a.into_iter().count()','2','usize');
add('an earlier scalar argument is evaluated before the later owner transfer','let a=vec![65u8];take(a[0],a)','130','u8','fn take(byte:u8,a:Vec<u8>)->u8{byte+a[0]}');
const refusals=[];
function refuse(name,body,error,ret='u8',declarations=''){refusals.push({name,source:declarations+'pub fn result()->'+ret+'{'+body+'}fn main(){}',kind:'unsupported',reason:'Rust owned binding: '+error});}
const moved='a moved source binding cannot be read or consumed again';
refuse('an earlier owned argument is consumed before a later scalar read','let a=vec![65u8];take(a,a[0])',moved,'u8','fn take(a:Vec<u8>,byte:u8)->u8{a[0]+byte}');
refuse('an earlier owned argument is consumed before a nested later borrowed call','let a=vec![65u8];take(a,read(&a))',moved,'u8','fn read(a:&Vec<u8>)->u8{a[0]}fn take(a:Vec<u8>,byte:u8)->u8{a[0]+byte}');
refuse('a moved vector cannot be indexed again','let a=vec![65u8];let b=a;a[0]',moved);
refuse('a moved vector cannot be borrowed again','let a=vec![65u8];let b=a;let r=&a;r[0]',moved);
refuse('a moved vector cannot transfer twice','let a=vec![65u8];let b=a;let c=a;c[0]',moved);
refuse('a vector reference cannot read a transferred owner','let a=vec![65u8];let r=&a;let b=a;r[0]',moved);
refuse('an element reference cannot read a transferred owner','let a=vec![65u8];let r=&a[0];let b=a;*r',moved);
refuse('owned calls invalidate their named argument','let a=vec![65u8];let byte=take(a);a[0]+byte',moved,'u8','fn take(a:Vec<u8>)->u8{a[0]}');
refuse('owned iteration invalidates its receiver','let a=vec![65u8];let count=a.into_iter().count();a[0]',moved);
refuse('one returning branch moving an owner prevents subsequent reuse','let a=vec![65u8];let byte=if true{let b=a;b[0]}else{1u8};a[0]+byte','a source binding moved on a returning branch cannot be reused');
refuse('an owning match binding invalidates its original place','let a=vec![65u8];let byte=match a{b=>b[0]};a[0]+byte',moved);
refuse('a scoped vector cannot lend an escaping reference','let r={let a=vec![65u8];&a};r[0]','an owned source binding does not outlive this use');
refuse('an indexed loan cannot outlive its scoped vector','let r={let a=vec![65u8];&a[0]};*r','an owned source binding does not outlive this use');
refuse('a moved block value is not silently borrowed as its original place','let a=vec![65u8];let r=&{a};r[0]','owned temporary borrows need their complete lifetime proof');
refuse('a guard cannot conceal a move on its fallthrough path','let a=vec![65u8];match true{true if check(a)=>1u8,_=>2u8}','owned consumption in guards needs its complete fallthrough path proof','u8','fn check(a:Vec<u8>)->bool{a[0]==65}');
const premises=[];function premise(operation,args,expected){premises.push({operation,args,expected});}
for(const collecting of [false,true])for(const excluded of [false,true])premise('acceptRustOwnedBindingTracking',[collecting,excluded],!collecting&&!excluded);
for(const [kind,storage,layers,known]of [['array','owned vector',0,true],['array','immutable slice',0,true],['array','owned vector',1,true],['array','owned vector',0,false],['string','owned vector',0,true]])premise('acceptRustOwnedBindingValue',[kind,storage,layers,known],kind==='array'&&storage==='owned vector'&&layers===0&&known);
for(const [state,error,consumed,expired]of [['available','','moved','dropped'],['moved',moved,'moved','moved'],['possibly moved','a source binding moved on a returning branch cannot be reused','possibly moved','dropped'],['dropped','an owned source binding does not outlive this use','dropped','dropped'],['unreachable','a source binding needs its actual ownership state','unreachable','unreachable'],['','a source binding needs its actual ownership state','','']]){premise('readRustOwnedBindingReadError',[state],error);premise('consumeRustOwnedBindingState',[state],consumed);premise('expireRustOwnedBindingState',[state],expired);}
for(const [left,right,lr,rr,expected]of [['available','available',true,true,'available'],['available','moved',true,true,'possibly moved'],['moved','available',true,true,'possibly moved'],['moved','moved',true,true,'moved'],['moved','available',false,true,'available'],['available','moved',true,false,'available'],['moved','available',false,false,'unreachable'],['absent','available',true,true,'available'],['available','absent',true,true,'available']])premise('mergeRustOwnedBindingStates',[left,right,lr,rr],expected);
for(const [before,after,expected]of [[[],[],true],[['available'],['available'],true],[['available'],['moved'],false],[['available'],[],false],[['moved','available'],['moved','available'],true]])premise('acceptRustOwnedBindingGuard',[before,after],expected);
writeProjectData(new URL('../../parity/fixtures/rust-owned-bindings.lino',import.meta.url),{schemaVersion:1,model:'ordinary Rust owned vector binding consumption 1',retainSourceTypeOwners:true,cases,refusals,premises,source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=32 or 64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},boundary:'Unexecuted originals and declared decisions, never observations. Tracks direct standard Vec owners, shared loans and indexed places on ordinary acyclic function paths. Scoped owners expire; returning branch states join. String/record/Box/resource ownership, mutation, backedges, full borrow regions, conditional pattern fallthrough and temporary lifetime extension remain full requirements. Legacy local-state/shared-handle/scalar-loop/const profiles retain their own source contracts.'});
