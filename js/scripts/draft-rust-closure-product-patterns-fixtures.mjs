// Independent original pattern inputs. No represented source parsing/execution.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],refusals=[],premises=[];
function add(name,body,helpers='',abort=null,checks=[]){cases.push({name,module:'closure_product_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;',...checks]});}
for(const owner of ['u8','i8','u16','i16','u32','i32','u64','i64','u128','i128','usize','isize']){
 add(owner+' written tuple pattern retains positional owners','let read=|(first,last):('+owner+','+owner+')|->'+owner+'{first+last};read((3'+owner+',4'+owner+'))==7'+owner);
 add(owner+' contextual tuple pattern retains actual function arity','let read:fn(('+owner+','+owner+'))->'+owner+'=|(first,last)|first+last;read((3'+owner+',4'+owner+'))==7'+owner);
 add(owner+' tuple rest retains selected source positions','let read:fn(('+owner+','+owner+','+owner+','+owner+'))->'+owner+'=|(first,..,last)|first+last;read((3'+owner+',90'+owner+',91'+owner+',4'+owner+'))==7'+owner);
}
add('nested tuple patterns preserve complete source structure','let read:fn((u8,(u16,u32)))->u64=|(first,(middle,last))|u64::from(first)+u64::from(middle)+u64::from(last);read((1u8,(2u16,4u32)))==7u64');
add('several source parameters retain independent tuple projections','let read:fn((u8,u8),(u8,u8))->u8=|(first,second),(third,last)|first+second+third+last;read((1u8,2u8),(3u8,4u8))==10u8');
add('ignored tuple fields introduce no source binding','let read:fn((u8,u8,u8))->u8=|(_,value,_)|value;read((1u8,7u8,9u8))==7u8');
add('one-field tuple pattern retains its comma and tuple domain','let read:fn((u8,))->u8=|(value,)|value;read((7u8,))==7u8');
add('parenthesized name remains a scalar binding','let read:fn(u8)->u8=|(value)|value;read(7u8)==7u8');
add('unit pattern consumes an actual unit parameter','let read:fn(())->u8=|()|7u8;read(())==7u8');
add('nested unit pattern retains actual field ownership','let read:fn(((),u8))->u8=|((),value)|value;read(((),7u8))==7u8');
add('tuple pattern shadows outer names without capturing them','let first=99u8;let read:fn((u8,u8))->u8=|(first,last)|first+last;read((3u8,4u8))==7u8&&first==99u8');
add('nested closure captures actual projected scalar','let read:fn((u8,u8))->u8=|(first,last)|{let inner=||first+last;inner()};read((3u8,4u8))==7u8');
add('immutable outer capture remains independent of projections','let offset=3u8;let read=|(first,last):(u8,u8)|first+last+offset;read((1u8,3u8))==7u8');
add('Boolean and floating tuple fields retain distinct source owners','let read:fn((bool,f64))->f64=|(flag,value)|if flag{value}else{0.0f64};read((true,1.5f64))==1.5f64');
add('named callback context supplies actual tuple fields','apply(|(first,last)|first+last,(3u8,4u8))==7u8','pub fn apply(read:fn((u8,u8))->u8,value:(u8,u8))->u8{read(value)}',null,['const _: fn(fn((u8,u8))->u8,(u8,u8))->u8=CASE::apply;']);
const record='#[derive(Clone,Copy)]pub struct Pair{pub first:u8,pub last:u16}';
add('named record pattern retains actual field order','let read:fn(Pair)->u16=|Pair{last,first}|u16::from(first)+last;read(Pair{last:4u16,first:3u8})==7u16',record);
add('record pattern puns and explicit names retain source identity','let read:fn(Pair)->u16=|Pair{last:right,first:left}|u16::from(left)+right;read(Pair{first:3u8,last:4u16})==7u16',record);
add('record rest leaves the ignored field unnamed','let read:fn(Pair)->u8=|Pair{first,..}|first;read(Pair{first:7u8,last:99u16})==7u8',record);
add('tuple struct closure pattern reuses nominal binding rules','let read:fn(Pair)->u8=|Pair(first,last)|first+last;read(Pair(3u8,4u8))==7u8','#[derive(Clone,Copy)]pub struct Pair(pub u8,pub u8);');
add('nested record inside tuple closure pattern','let read:fn((Pair,u8))->u16=|(Pair{first,last},extra)|u16::from(first)+last+u16::from(extra);read((Pair{first:1u8,last:2u16},4u8))==7u16',record);
add('tuple argument expressions retain first failure source order','let read:fn((u8,u8))->u8=|(first,last)|first+last;read((first(),last()))==0u8','pub fn first()->u8{panic!("tuple first")}pub fn last()->u8{panic!("tuple last")}','tuple first');
for(const [name,header] of [['duplicate names within one tuple','|(value,value):(u8,u8)|->u8{value}'],['duplicate names across ordinary and tuple parameters','|value:u8,(value,last):(u8,u8)|->u8{value+last}']])refusals.push({name,source:'pub fn result()->u8{let read='+header+';0u8}fn main(){}',kind:'unsupported',reason:'Rust Copy closure: closure parameters require distinct actual bindings'});
for(const [role,expected] of [['(','product'],['_','wildcard'],['binding','binding'],['mut','unsupported']])premises.push({operation:'readRustCopyClosureParameterForm',args:[role],expected});
for(const names of [[],['first'],['first','last']])premises.push({operation:'readRustCopyClosurePatternBindingError',args:[names],expected:''});
for(const names of [['value','value'],['first','last','first']])premises.push({operation:'readRustCopyClosurePatternBindingError',args:[names],expected:'closure parameters require distinct actual bindings'});
writeProjectData(new URL('../../parity/fixtures/rust-closure-product-patterns.lino',import.meta.url),{schemaVersion:1,model:'Rust immutable closure product patterns 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-copy-closure-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/reference/expressions/closure-expr.html','https://doc.rust-lang.org/reference/patterns.html#tuple-patterns','https://doc.rust-lang.org/reference/patterns.html#struct-patterns'],boundary:'Unexecuted original source and codec/native controls. Existing product field plans, nominal ownership and irrefutability contracts remain required. Mutable/non-Copy, borrowed structural patterns, arbitrary inference and full language/ownership coverage remain active requirements.'});
