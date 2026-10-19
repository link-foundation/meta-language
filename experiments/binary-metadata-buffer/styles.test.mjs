import assert from 'node:assert/strict';
import test from 'node:test';
import {readBinaryMetadataCandidateStyles} from '../../js/scripts/binary-metadata-candidate-styles.mjs';
import {DecoratorSet} from '../../js/src/decorators.js';
import {parseRust} from '../../js/src/translation/rust.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {runInNewContext} from 'node:vm';

const adapter={id:'fixture-adapter',level:'emitter',order:0,when:[['format','Rust']],actions:[{op:'set',field:'fixture',value:'true'}]};
const prior='pub fn ml_transcode_link_network_binary_loop1(output: f64) -> f64 { let output_2 = output; output_2 }';
const styled='pub fn ml_transcode_link_network_binary_loop1(output: f64) -> f64 { output }';
const candidate='pub fn ml_transcode_link_network_binary_loop2(metadata: f64) -> f64 { let metadata_2 = metadata; metadata_2 }';
const derive=(current,previousStyled=styled)=>readBinaryMetadataCandidateStyles({items:[current],base:[current],previousItems:[prior],previousStyled:[previousStyled],adapter,format:source=>source});
const apply=(result,current)=>new DecoratorSet(result.decorators).decorate('emitter',{format:'Rust',scope:'formatted-item',source:current}).source;

test('whole-declaration alpha reuse preserves the actual parameter, result and declaration identity',()=>{
 const result=derive(candidate);
 assert.equal(result.receipt.declarations[0].method,'bijective bound-name spelling');
 assert.equal(apply(result,candidate),'pub fn ml_transcode_link_network_binary_loop2(metadata: f64) -> f64 { metadata }');
});

test('a changed value or nonbijective binding cannot inherit a frozen declaration correction',()=>{
 for(const current of[candidate.replace('= metadata;','= 1f64;'),candidate.replaceAll('metadata_2','metadata')]){
  const result=derive(current);
  assert.match(result.receipt.declarations[0].method,/fresh structural style/u);
  const corrected=apply(result,current);
  assert.doesNotMatch(corrected,/\boutput\b/u);
  if(current.includes('1f64'))assert.ok(corrected.includes('1f64'));
 }
});

test('a fresh binding cannot capture a name introduced by a prior reviewed spelling',()=>{
 const introduced='pub fn ml_transcode_link_network_binary_loop1(output: f64) -> f64 { let completed_output = output; completed_output }';
 const current=candidate.replaceAll('metadata_2','completed_output');
 const result=derive(current,introduced);
 assert.match(result.receipt.declarations[0].method,/fresh structural style/u);
 assert.ok(apply(result,current).includes('metadata'));
});

test('constructing a header cannot silently borrow it into a differently owned parameter',()=>{
 const current='pub fn transcode_link_network_binary() -> f64 { let header = vec![1f64]; super::consume(header) }';
 const consume='pub fn consume(header: Vec<f64>) -> f64 { 1f64 }';
 assert.throws(()=>readBinaryMetadataCandidateStyles({items:[current,consume],base:[current,consume],previousItems:[],previousStyled:[],adapter,format:source=>source}),/actual checked slice parameter/u);
});

test('a leading not operand inside a compound condition cannot be mistaken for whole-condition negation',()=>{
 const current='pub fn changed(a: bool, b: bool) -> f64 { if (!(a) || b) { 1f64 } else { 2f64 } }';
 const result=readBinaryMetadataCandidateStyles({items:[current],base:[current],previousItems:[],previousStyled:[],adapter,format:source=>source});
 assert.equal(apply(result,current),'#[must_use]\n'+current.replace('if (!(a) || b)','if !(a) || b'));
 const original=emitJavaScript(checkProgram(parseRust(current+' fn main() {}'))).definitions.join('\n');
 const corrected=emitJavaScript(checkProgram(parseRust(apply(result,current)+' fn main() {}'))).definitions.join('\n');
 for(const a of[false,true])for(const b of[false,true]){
  assert.equal(runInNewContext(corrected+`; changed(${a},${b});`),!a||b?1:2);
  assert.equal(runInNewContext(corrected+`; changed(${a},${b});`),runInNewContext(original+`; changed(${a},${b});`));
 }
 const ungrouped=current.replace('if (!(a) || b)','if !(a) || b');
 assert.throws(()=>readBinaryMetadataCandidateStyles({items:[ungrouped],base:[ungrouped],previousItems:[],previousStyled:[],adapter,format:source=>source}),/own the complete condition/u);
});

test('negative literal receivers cannot lose the grouping that precedes a method call',()=>{
 for(const expression of['(-1.2f64).floor()','((-((1.2f64)))).floor()']){
  const current=`pub fn changed() -> f64 { ${expression} }`;
  assert.throws(()=>readBinaryMetadataCandidateStyles({items:[current],base:[current],previousItems:[],previousStyled:[],adapter,format:source=>source}),/negative literal receiver/u);
 }
});

test('a local spelling shared by a fixed qualified callee cannot rename that callee through alpha reuse',()=>{
 const previous='pub fn ml_transcode_link_network_binary_loop1(output: f64) -> f64 { super::output(output) }';
 const current='pub fn ml_transcode_link_network_binary_loop2(metadata: f64) -> f64 { super::output(metadata) }';
 const result=readBinaryMetadataCandidateStyles({items:[current],base:[current],previousItems:[previous],previousStyled:[previous],adapter,format:source=>source});
 assert.match(result.receipt.declarations[0].method,/fresh structural style/u);
 assert.match(apply(result,current),/super::output\(metadata\)/u);
 assert.doesNotMatch(apply(result,current),/super::metadata/u);
});

test('a foreign callee sharing a local function name cannot inherit the local slice contract',()=>{
 const current='pub fn transcode_link_network_binary() -> f64 { let header = vec![1f64]; other::consume(header) }';
 const local='pub fn consume(header: &[f64]) -> f64 { 1f64 }';
 const foreign='pub mod other { pub fn consume(header: Vec<f64>) -> f64 { header.len() as f64 } }';
 assert.throws(()=>readBinaryMetadataCandidateStyles({items:[current,local,foreign],base:[current,local,foreign],previousItems:[],previousStyled:[],adapter,format:source=>source}),/actual generated sibling call path/u);
});
