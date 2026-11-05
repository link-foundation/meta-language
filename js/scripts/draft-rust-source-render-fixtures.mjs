// Independent literal syntax/results and immutable preceding source only.
import {readProjectData,writeProjectData} from './project-data.mjs';
import {execFileSync} from 'node:child_process';
import {existsSync,readFileSync,writeFileSync,lstatSync} from 'node:fs';
import {createHash} from 'node:crypto';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),calls=[
 {name:'an empty argument list remains a function call',head:'crate::make',values:[],expected:'crate::make()'},
 {name:'argument expressions retain their source order',head:'crate::combine',values:['first()','second()'],expected:'crate::combine(first(), second())'},
 {name:'source literal Unicode scalars are transported without replacement',head:'crate::text',values:['String::from("\u{1f4a1}")'],expected:'crate::text(String::from("\u{1f4a1}"))'},
 {name:'nested source calls retain their complete spellings',head:'crate::outer',values:['crate::inner(1u32, 2u32)'],expected:'crate::outer(crate::inner(1u32, 2u32))'}];
const constructors=[
 {name:'Rust constructors retain named fields',language:'Rust',head:'crate::Parcel::Parcel',names:['amount','enabled'],values:['7u32','true'],expected:'crate::Parcel::Parcel { amount: 7u32, enabled: true }',named:true},
 {name:'raw keyword field spelling stays explicit',language:'Rust',head:'crate::Token::Token',names:['type'],values:['9u32'],expected:'crate::Token::Token { r#type: 9u32 }',named:true},
 {name:'JavaScript tagged constructors retain their tuple representation',language:'JavaScript',head:'crate::Packet::Packet',names:['items'],values:['Vec::new()'],expected:'crate::Packet::Packet(Vec::new())',named:false},
 {name:'tuple constructors do not invent named fields',language:'Rust',head:'crate::Choice::Pair',names:['',''],values:['3u32','false'],expected:'crate::Choice::Pair(3u32, false)',named:false},
 {name:'unit constructors remain paths without call arguments',language:'Rust',head:'crate::Unit::Marker',names:[],values:[],expected:'crate::Unit::Marker',named:false}];
const programs=[
 {name:'a function returning named data remains an ordinary call',source:'struct Parcel { amount: u32, enabled: bool }\nfn build(amount:u32)->Parcel { Parcel { amount, enabled:true } }\nfn read(value:Parcel)->u32 { if value.enabled { value.amount } else { 0u32 } }\nfn compute()->u32 { read(build(7u32)) }',expected:'7'},
 {name:'named and tuple constructor expressions retain distinct checked layouts',source:'enum Choice { Named { amount:u32 }, Pair(u32,bool) }\nfn build()->Choice { Choice::Named { amount:3u32 } }\nfn read(value:Choice)->u32 { match value { Choice::Named { amount }=>amount, Choice::Pair(amount,_)=>amount } }\nfn compute()->u32 { read(build()) }',expected:'3'},
 {name:'raw keyword data fields retain their constructor and pattern owners',source:'struct Token { r#type:u32 }\nfn build(value:u32)->Token { Token { r#type:value } }\nfn read(value:Token)->u32 { let Token { r#type }=value; r#type }\nfn compute()->u32 { read(build(9u32)) }',expected:'9'},
 {name:'a unit value and a zero-argument factory remain different operations',source:'enum Unit { Marker }\nfn build()->Unit { Unit::Marker }\nfn read(value:Unit)->bool { match value { Unit::Marker=>true } }\nfn compute()->bool { read(build()) }',expected:'true'}];
const revision=execFileSync('git',['rev-parse','91476c2e'],{cwd:root,encoding:'utf8'}).trim();
const predecessors=['js/src/translation/rust-record-rules.js','js/src/translation/emit-rust.js','rust/src/translation/emit_rust.rs','rust/src/translation/emit_rust/declarations.rs','rust/src/translation/emit_rust/expressions.rs',
 'rust/src/translation/emit_rust/nominal_array_loans.rs','rust/src/translation/rust/records.rs','rust/src/translation/rust/records/bindings.rs',
 'rust/src/translation/rust/records/namespaces.rs','rust/src/translation/rust/records/patterns.rs','rust/src/translation/rust/records/updates.rs',
 'rust/src/translation/rust/records/variants.rs','rust/src/translation/rust/items/product_bindings.rs','rust/src/translation/rust/ordinary_completions.rs',
 'rust/src/translation/rust/source_origins/patterns.rs','rust/src/translation/check/matches.rs','rust/src/translation/lexer/literals.rs'].map(path=>{
 const bytes=execFileSync('git',['show',revision+':'+path],{cwd:root,maxBuffer:4*1024*1024}),retained='parity/fixtures/history/rust-source-render-before-'+path.replaceAll('/','-')+'.lino',file=new URL(retained,root);
 const record={path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
 if(existsSync(file)){if(!lstatSync(file).isFile()||lstatSync(file).isSymbolicLink())throw new TypeError('retained source must be a regular file');const prior=readProjectData(file);if(Object.keys(prior).length!==Object.keys(record).length||Object.keys(record).some(key=>prior[key]!==record[key]))throw new TypeError('complete preceding source differs');}
 else writeProjectData(file,record);return {path,revision,retained,bytes:record.bytes,sha256:record.sha256};
});
for(const [path,name]of [['parity/self-translation/rust-record-rules-native.lino','rust-source-render-before-record-native.lino'],['js/src/data/rust-record-program.lino','rust-source-render-before-record-program.lino']]){
 const bytes=execFileSync('git',['show',revision+':'+path],{cwd:root,maxBuffer:16*1024*1024}),retained='parity/fixtures/history/'+name,file=new URL(retained,root);
 if(existsSync(file)){if(!lstatSync(file).isFile()||lstatSync(file).isSymbolicLink()||!readFileSync(file).equals(bytes))throw new TypeError('complete preceding artifact differs');}else writeFileSync(file,bytes,{flag:'wx'});
 predecessors.push({path,revision,retained,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),raw:true});
}
writeProjectData(new URL('parity/fixtures/rust-source-render-interfaces.lino',root),{schemaVersion:1,model:'native Rust source Unicode scalar interface 1',versions:{node:['>=22.15 <23','>=24 <25'],rust:['>=1.90 <1.100; edition 2024']},calls,constructors,programs,predecessors,
 boundary:'Native source/schema Unicode scalar transport only. Runtime JavaScript text remains UTF16; general namespace/trait/FFI composition and complete project translation remain required.'});
