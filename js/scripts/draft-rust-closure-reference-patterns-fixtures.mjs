// Independent original source controls, authored without execution or parsing.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],refusals=[],premises=[];
function add(name,body,helpers='',abort=null,checks=[]){cases.push({name,module:'closure_reference_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;',...checks]});}
for(const owner of ['u8','i8','u16','i16','u32','i32','u64','i64','u128','i128','usize','isize']){
 add(owner+' actual function pointer context retains reference signature','let read:fn(&'+owner+')->'+owner+'=|&value|value;let value=7'+owner+';read(&value)==7'+owner);
 add(owner+' written reference parameter copies the actual scalar','let read=|&value:&'+owner+'|->'+owner+'{value};let value=7'+owner+';read(&value)==7'+owner);
 add(owner+' iterator map destructures a borrowed item','let values=vec![3'+owner+',7'+owner+'];values.iter().map(|&value|value).collect::<Vec<_>>()[1]==7'+owner);
 add(owner+' iterator filter destructures both immutable layers','let values=vec![3'+owner+',7'+owner+'];values.iter().filter(|&&value|value==7'+owner+').count()==1usize');
 add(owner+' copied iterator filter receives exactly one reference','let values=vec![3'+owner+',7'+owner+'];values.iter().copied().filter(|&value|value==7'+owner+').count()==1usize');
}
add('two-layer pointer context distinguishes callable and binding origins','let read:fn(& &u8)->u8=|&&value|value;let value=7u8;let borrowed=&value;read(&borrowed)==7u8');
add('one destructuring layer retains the remaining source reference','let read:fn(& &u8)->u8=|&value|*value;let value=7u8;let borrowed=&value;read(&borrowed)==7u8');
add('spaced reference tokens retain the same two-layer meaning','let read:fn(& &u8)->u8=|& &value|value;let value=7u8;let borrowed=&value;read(&borrowed)==7u8');
add('multiple closure arguments retain independent source reference depths','let read:fn(&u8,& &u8)->u8=|&first,&&last|first+last;let first=3u8;let last=4u8;let borrowed=&last;read(&first,&borrowed)==7u8');
add('reference wildcard consumes the actual argument without capturing a name','let read:fn(&u8)->bool=|&_|true;let value=7u8;read(&value)');
add('copied binding shadows an outer scalar without capturing it','let value=99u8;let read:fn(&u8)->u8=|&value|value;let selected=7u8;read(&selected)==7u8&&value==99u8');
add('borrowed Boolean mapping preserves the actual Boolean owner','let values=vec![false,true];values.iter().map(|&value|!value).any(|value|value)');
add('borrowed floating mapping preserves binary64','let values=vec![1.5f64];values.iter().map(|&value|value+0.5f64).collect::<Vec<_>>()[0]==2.0f64');
add('unit reference pattern keeps its actual unit result','let read:fn(&())->()=|&value|value;read(&());true');
add('immutable captured scalar remains independent of the destructured argument','let offset=3u8;let values=vec![4u8];values.iter().map(|&value|value+offset).collect::<Vec<_>>()[0]==7u8');
add('nested closure captures the copied local binding','let read:fn(&u8)->u8=|&value|{let inner=||value;inner()};let value=7u8;read(&value)==7u8');
add('named callback parameter preserves its reference contract','let value=7u8;apply(|&value|value,&value)==7u8','pub fn apply(read:fn(&u8)->u8,value:&u8)->u8{read(value)}',null,['const _: fn(fn(&u8)->u8,&u8)->u8=CASE::apply;']);
add('filter does not invoke a destructuring callback on an empty source','let values:Vec<u8>=vec![];values.iter().filter(|&&value|panic!("empty callback")).count()==0usize');
add('short circuiting still suppresses later destructured callback failure','let values=vec![3u8,7u8];values.iter().map(|&value|if value==3u8{value}else{panic!("later callback")}).any(|value|value==3u8)');
add('nonempty count retains destructured callback effects','let values=vec![3u8];values.iter().map(|&value|->u8{panic!("mapped callback")}).count()==0usize','','mapped callback');
const error='a closure reference pattern needs its actual immutable Copy scalar reference layers';
for(const [name,body] of [['reference pattern cannot consume an owned integer','let read:fn(u8)->u8=|&value|value;read(7u8)'],['two reference layers cannot consume a single reference','let read:fn(&u8)->u8=|&&value|value;let value=7u8;read(&value)']])refusals.push({name,source:'pub fn result()->u8{'+body+'}fn main(){}',kind:'unsupported',reason:'Rust closure parameter: '+error});
for(const [token,expected] of [['&',1],['&&',2],['mut',0],['binding',0],['',0]])premises.push({operation:'readRustCopyClosureReferencePatternDepth',args:[token],expected});
for(const args of [[0,0,false,false],[1,1,true,true],[1,2,true,true],[2,2,true,true],[256,256,true,true]])premises.push({operation:'readRustCopyClosureReferencePatternError',args,expected:''});
for(const args of [[1,0,true,true],[2,1,true,true],[1,1,false,true],[1,1,true,false],[257,257,true,true],[1.5,2,true,true],[-1,2,true,true]])premises.push({operation:'readRustCopyClosureReferencePatternError',args,expected:error});
writeProjectData(new URL('../../parity/fixtures/rust-closure-reference-patterns.lino',import.meta.url),{schemaVersion:1,model:'Rust immutable scalar closure reference patterns 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-copy-closure-program','meta-language/source-program/rust-iterator-pipeline-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/reference/patterns.html#reference-patterns','https://doc.rust-lang.org/reference/expressions/closure-expr.html'],boundary:'Unexecuted actual-source controls. Callable reference layers remain distinct from local binding layers. Mutable, non-Copy and structural patterns, escaping reference results and full language/ownership coverage remain requirements.'});
