// Future Rust products and exact predecessor source contexts, never executed.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value){cases.push({name,module:'copy_closure_case_'+cases.length,source:'pub fn result()->i64{'+body+'}',value,abort:null});}
add('a typed immutable closure is called through its local binding','let read=|value:i64|->i64{value+1};read(6)','7');
add('a zero-argument closure call remains an application','let read=||->i64{7};read()','7');
add('a zero-argument move closure retains its actual call','let read=move ||->i64{7};read()','7');
add('a closure captures an immutable scalar','let offset=3i64;let read=|value:i64|->i64{offset+value};read(4)','7');
add('a move closure copies an actual scalar capture','let offset=3i64;let read=move |value:i64|->i64{offset+value};read(4)','7');
add('a closure capture does not move its Copy source away','let offset=3i64;let read=move |value:i64|->i64{offset+value};read(1)+offset','7');
add('a closure retains several independent captures','let first=3i64;let last=4i64;let read=||->i64{first+last};read()','7');
add('a closure parameter shadows an outer scalar','let value=99i64;let read=|value:i64|->i64{value};read(7)','7');
add('a closure local shadows the outer name without capture','let value=99i64;let read=||->i64{let value=7i64;value};read()','7');
add('a closure can capture a recursively immutable Copy tuple','let pair=(3i64,4i64);let read=||->i64{pair.0+pair.1};read()','7');
add('a Boolean closure result keeps its original domain','let read=|flag:bool|->bool{!flag};if read(false){7}else{8}','7');
add('a fixed result retains its actual source width','let read=|value:i32|->i32{value+1};read(6) as i64','7');
add('a floating closure keeps binary64 operations','let read=|value:f64|->f64{value+1.0};read(6.0) as i64','7');
add('an empty Unit closure can be a discarded statement','let step=||->(){};step();7','7');
add('a parenthesized closure can be invoked immediately','(|value:i64|->i64{value+1})(6)','7');
add('a parenthesized empty closure can be invoked immediately','(||->i64{7})()','7');
add('a block closure retains terminal return construction','let read=|value:i64|->i64{return value+1;};read(6)','7');
add('a closure retains guarded ordinary return completion','let read=|value:i64|->i64{if value>0{return value;}7};read(3)','3');
add('a closure retains the ordinary conditional fallback','let read=|value:i64|->i64{if value>0{value}else{7}};read(0)','7');
add('a nested closure captures its actual outer parameter','let read=|value:i64|->i64{let inner=|last:i64|->i64{value+last};inner(4)};read(3)','7');
add('a closure may be called more than once','let read=|value:i64|->i64{value+1};read(2)+read(3)','7');
add('a Copy closure alias preserves its source identity','let read=|value:i64|->i64{value+1};let alias=read;alias(6)','7');
add('the same closure identity may pass through an ordinary join','let offset=3i64;let read=|value:i64|->i64{value+offset};let alias=if true{read}else{read};alias(4)','7');
add('a closure retains a trailing parameter comma','let read=|first:i64,last:i64,|->i64{first+last};read(3,4)','7');
const refusals=[
 {name:'an implicit closure result retains its source obligation',source:'fn f()->i64{let read=||{7};read()}fn main(){}',kind:'unsupported',reason:'Rust Copy closure: the closure result requires its actual explicit type'},
 {name:'a mutable closure parameter keeps its ownership boundary',source:'fn f()->i64{let read=|mut value:i64|->i64{value};read(7)}fn main(){}',kind:'unsupported',reason:'Rust closure parameter: a closure needs a named direct immutable typed parameter'},
 {name:'a reference parameter is not a direct Copy source argument',source:'fn f()->i64{let read=|value:&i64|->i64{*value};read(&7)}fn main(){}',kind:'unsupported',reason:'Rust Copy closure: closure parameters require direct immutable Copy source types'},
 {name:'an owned result needs its actual consumption proof',source:'fn f()->i64{let read=||->String{String::from("text")};7}fn main(){}',kind:'unsupported',reason:'Rust Copy closure: the closure result requires a direct immutable Copy source type'},
 {name:'an owned String capture needs its move interpretation',source:'fn f()->i64{let text=String::from("text");let read=move ||->i64{let moved=text;7};read()}fn main(){}',kind:'unsupported',reason:'Rust Copy closure: closure captures require proven direct immutable Copy source values'},
 {name:'a borrowed scalar capture retains its lifetime obligation',source:'fn f()->i64{let value=7i64;let reference=&value;let read=||->i64{*reference};read()}fn main(){}',kind:'unsupported',reason:'Rust Copy closure: closure captures require proven direct immutable Copy source values'},
 {name:'duplicate closure bindings remain invalid',source:'fn f()->i64{let read=|value:i64,value:i64|->i64{value};read(3,4)}fn main(){}',kind:'unsupported',reason:'Rust Copy closure: closure parameters require distinct actual bindings'},
 {name:'distinct anonymous capturing closure types cannot be silently joined',source:'fn f()->i64{let offset=3i64;let read=if true{|value:i64|->i64{value+offset}}else{|value:i64|->i64{value+offset}};7}fn main(){}',kind:'unsupported',reason:'Rust closure source join: distinct anonymous closure types require an actual source coercion'}
];
const revision='e0a2f8bd',history=['js/src/translation/rust.js','js/src/translation/rust-source-origins.js','js/src/translation/callable-source.js','rust/src/translation/rust/expressions.rs','rust/src/translation/rust/expressions/paths.rs','rust/src/translation/rust/expressions/control.rs','rust/src/translation/rust/source_origins.rs','js/tests/translation-rust-records.test.js'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-copy-closures.lino',import.meta.url),{schemaVersion:1,model:'ordinary explicitly typed immutable Rust Copy closures 1',cases,refusals,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},references:['https://doc.rust-lang.org/reference/expressions/closure-expr.html'],assertions:['originalTypedParametersAndExplicitCopyResult','exactFreeCaptureSelectionWithLexicalShadows','onlyDirectImmutableCopyCaptures','zeroArgumentCallsAreApplications','localAliasesAndParenthesizedInvocations','originalAnonymousSourceIdentityAtJoins','sameCoreLambdaApplyAndCompletionConstruction','completePredecessorsBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. The ordinary profile supports explicitly typed Copy arguments/results and proven immutable Copy captures. Source anonymous identities remain distinct despite equal callable signatures. Contextual type inference, arbitrary pattern parameters, owned/mutable/reference capture and result lifetimes, trait coercions, Rc/runtime wrappers, async and complete ownership/drop behavior remain obligations.'});
