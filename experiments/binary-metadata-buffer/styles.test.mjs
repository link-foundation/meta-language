import assert from 'node:assert/strict';
import test from 'node:test';
import {readBinaryMetadataCandidateStyles} from '../../js/scripts/binary-metadata-candidate-styles.mjs';
import {DecoratorSet} from '../../js/src/decorators.js';

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
