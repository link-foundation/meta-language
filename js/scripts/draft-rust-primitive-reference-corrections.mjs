// Documentary assertion corrections and source/data authoring only.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {existsSync,mkdirSync,readFileSync,writeFileSync} from 'node:fs';
export function draftRustPrimitiveReferenceCorrection({fixture,cases,refusals,source,body,expected,prefix}){
 const index=refusals.findIndex(row=>row.source===source);
 if(index<0)throw new TypeError('the complete original reference input must remain present before its assertion correction');
 const previous=refusals[index],revision='911576d5',path='parity/fixtures/'+fixture+'.lino',bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:8*1024*1024}),historyPath='parity/fixtures/history/'+fixture+'-'+revision+'.lino',file=new URL('../../'+historyPath,import.meta.url);
 mkdirSync(new URL('.',file),{recursive:true});if(existsSync(file)){if(!readFileSync(file).equals(bytes))throw new TypeError('the original reference fixture archive changed');}else writeFileSync(file,bytes);
 const module=prefix+cases.length;
 cases.push({name:'the complete preceding borrowed receiver input now retains its actual primitive Copy method meaning',module,source:source+'pub fn result()->i64{'+body+'}',expected,value:expected,abort:null,resultType:'i64',checks:["const _: for<'value> fn(&'value i64)->i64=CASE::select;"]});
 refusals.splice(index,1);
 return [{source,previous,expected:{value:expected,sourceType:"for<'value> fn(&'value i64)->i64",body},basis:['https://doc.rust-lang.org/reference/expressions/method-call-expr.html','https://doc.rust-lang.org/std/primitive.i64.html'],versions:'Rust >=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',history:{path,revision,historyPath,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')},reason:'Immutable reference receiver lookup reaches the inherent by-value method of the actual Copy integer primitive. The complete original source input is retained as positive value/type coverage. Direct receiver-rule negative controls remain unchanged; Ord reference results and explicit function arguments retain their own obligations.'}];
}
