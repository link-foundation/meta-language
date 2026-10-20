// Finite source-owned loans; no native algorithm or test result is supplied here.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {tokenize} from '../../js/src/translation/lexer.js';
import {readRustLibraryItems} from '../../js/scripts/rust-library-packaging.mjs';
import {DecoratorSet} from '../../js/src/decorators.js';

const sourceSha256='2eee8f0d0a08286e0d1b110b8533ed809b4263e7ec9c5912f33e1e23cfa8608d';

/** Borrow only read-only primitive column locals in this exact closed library. */
export function readOriginalRawColumnLoanStyles(source) {
  assert.equal(createHash('sha256').update(source).digest('hex'),sourceSha256,'the complete reviewed borrowed raw source remains exact');
  const actions=[],loans=[];let current=source;
  for(const item of readRustLibraryItems(source).items){
    if(item.kind!=='fn')continue;
    const {tokens}=tokenize(item.source,'Rust'),body=tokens.findIndex(token=>token.value==='{'),header=item.source.slice(0,tokens[body].start);
    const pattern=/let ([a-z][a-z_0-9]*) = \(crate::ml_array::optional_at\((columns|policy), crate::ml_array::number_index\(([0-9]+)f64\)\)\)\.clone\(\)\.unwrap_or_else\(\|\| Vec::<f64>::new\(\)\);/gu;
    const edits=[],selected=[];
    for(const match of item.source.matchAll(pattern)){
      const [original,name,receiver,index]=match;
      assert.ok(Number.isSafeInteger(Number(index))&&Number(index)>=0&&Number(index)<=65535,'the reviewed fixed index fits even a sixteen-bit pointer');
      if(!new RegExp('(?:\\(|, )'+receiver+': &\\[Vec<f64>\\](?:,|\\))','u').test(header))continue;
      const start=tokens.findIndex(token=>token.start===match.index);
      if(start<0||tokens[start].value!=='let'||tokens[start+1]?.value!==name)continue;
      const finish=match.index+original.length,uses=tokens.flatMap((token,at)=>token.kind==='identifier'&&token.value===name
        &&!(token.start>=match.index&&token.end<=finish)?[at]:[]);
      const reads=[];let safe=true;
      for(const at of uses){
        if(tokens[at-1]?.value==='&')reads.push(tokens[at-1]);
        else if(tokens[at-1]?.value==='('&&tokens[at-2]?.value==='&'&&tokens[at+1]?.value===')')reads.push(tokens[at-2]);
        else if(tokens[at+1]?.value==='.'&&tokens[at+2]?.value==='len'&&tokens[at+3]?.value==='('&&tokens[at+4]?.value===')')continue;
        else{safe=false;break;}
      }
      if(!safe||uses.length===0)continue;
      const replacement='let '+name+': &[f64] = '+receiver+'.get('+index+').map_or(&[][..], Vec::as_slice);';
      edits.push({start:match.index,end:finish,from:original,to:replacement});
      for(const read of reads)edits.push({start:read.start,end:read.end,from:read.raw,to:''});
      selected.push({function:item.name,name,receiver,index:Number(index),readOnlyUses:uses.length});
    }
    if(edits.length===0)continue;
    edits.sort((left,right)=>left.start-right.start);let previous=0;
    for(const edit of edits){assert.ok(edit.start>=previous);assert.equal(item.source.slice(edit.start,edit.end),edit.from);previous=edit.end;}
    let transformed=item.source;
    for(const edit of [...edits].reverse())transformed=transformed.slice(0,edit.start)+edit.to+transformed.slice(edit.end);
    assert.equal(current.split(item.source).length,2,'one complete current raw function owns every local loan');
    current=current.replace(item.source,()=>transformed);
    actions.push({op:'replace',field:'source',from:item.source,to:transformed});loans.push(...selected);
  }
  assert.ok(loans.length>=26,'every reviewed primitive column in the three main readers is borrowed');
  let inverse=current;for(const action of [...actions].reverse()){assert.equal(inverse.split(action.to).length,2);inverse=inverse.replace(action.to,()=>action.from);}
  assert.equal(inverse,source,'every complete predecessor byte remains recoverable');
  const decorators=new DecoratorSet([{id:'original-raw-read-only-column-loans',level:'emitter',order:0,when:[['format','Rust']],actions}]);
  assert.equal(DecoratorSet.fromLino(decorators.toLino()).decorate('emitter',{format:'Rust',source}).source,current);
  return{source:current,actions,record:{sourceSha256,loans,decorators:decorators.toLino(),offsetUnit:'UTF-16 code units',
    rule:'The exact closed primitive library never observes vector addresses or custom allocation. Immutable borrowed Vec<f64> parameters cannot mutate. Selected locals have only immutable references or length reads, with no writes captures moves identity queries or escape. A fixed valid Number index selects the same primitive slice, or the same empty default, without copying either vector. Every other use context is retained.',
    boundary:'Finite reviewed source lowering with full predecessor inverse. All original invocation inputs independent result bits and the ten-second native execution bound remain mandatory. No native timing or execution is inferred.'}};
}
