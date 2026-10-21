// A finite reviewed source lowering; the original algorithms stay byte-exact.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {tokenize} from '../src/translation/lexer.js';
import {DecoratorSet} from '../src/decorators.js';

const profiles={
  'experiments/grammar-outcome-network/native-transport/inverse/integer-decimal-rules.mjs':
    {sha256:'3f1db6f29893e8da9589ed284461714987fdf047be250532aeb9771873f64a5b',calls:4},
  'experiments/grammar-outcome-network/native-transport/inverse/integer-projection-rules.mjs':
    {sha256:'8c8c42d636bd48624daaa43fabbf170b978395476608d7f9a80f567d2ac76675',calls:3},
};

/** Lower the exact typed source's nonnegative integer single-unit accesses. */
export function readPortableIntegerCharacterSourceView(file,source) {
  assert.equal(typeof source,'string');const profile=profiles[file];assert.ok(profile,'covered closed integer source');
  assert.equal(createHash('sha256').update(source).digest('hex'),profile.sha256,'the complete reviewed original integer source remains unchanged');
  const tokens=tokenize(source,'JavaScript').tokens,edits=[];
  for(let at=0;at<tokens.length;at++){
    if(tokens[at].value!=='.'||tokens[at+1]?.value!=='charAt'||tokens[at+2]?.value!=='(')continue;
    const receiver=tokens[at-1];assert.ok(receiver?.kind==='identifier'||receiver?.kind==='string');
    assert.ok(['text','limit','digits'].includes(receiver.value)||receiver.kind==='string'&&receiver.value==='0123456789');
    let end=at+3;while(end<tokens.length&&tokens[end].value!==')')end++;
    const argument=tokens.slice(at+3,end),spelling=argument.map(token=>token.raw).join('');
    assert.ok(['0','offset','index','index-offset','next'].includes(spelling),'only the complete reviewed integer index contexts are lowered');
    assert.ok(argument.every(token=>['identifier','number'].includes(token.kind)||token.value==='-'),'the repeated index has no calls getters stores or conversions');
    const original=source.slice(tokens[at+1].start,tokens[end].end),value=source.slice(argument[0].start,argument.at(-1).end);
    edits.push({start:tokens[at+1].start,end:tokens[end].end,from:original,to:'slice('+value+', ('+value+') + 1)',receiver:receiver.value,index:spelling});
    at=end;
  }
  assert.equal(edits.length,profile.calls,'every original character access remains represented');
  let current=source;for(const edit of [...edits].reverse())current=current.slice(0,edit.start)+edit.to+current.slice(edit.end);
  let inverse=current;for(const edit of edits){assert.equal(inverse.slice(edit.start,edit.start+edit.to.length),edit.to);inverse=inverse.slice(0,edit.start)+edit.from+inverse.slice(edit.start+edit.to.length);}
  assert.equal(inverse,source,'every complete original algorithm byte survives the source-view inverse');
  const decorators=new DecoratorSet([{id:file.includes('projection')?'integer-projection-character-source-view':'integer-decimal-character-source-view',level:'emitter',order:0,
    when:[['format','JavaScript']],actions:[{op:'replace',field:'source',from:source,to:current}]}]);
  assert.equal(DecoratorSet.fromLino(decorators.toLino()).decorate('emitter',{format:'JavaScript',source}).source,current);
  return{source:current,record:{file,original:{bytes:Buffer.byteLength(source),sha256:profile.sha256,sourceText:source},
    lowered:{bytes:Buffer.byteLength(current),sha256:createHash('sha256').update(current).digest('hex'),sourceText:current},offsetUnit:'UTF-16 code units',edits,decorators:decorators.toLino(),
    context:'Primitive String and Number inputs in the complete fixed integer algorithms. Zero/offset/index/index-offset are nonnegative integers under the retained length guards and loop initialization/increments; next is a decimal quotient digit after complete canonical admission. Repeated argument expressions contain only primitive local reads and subtraction.',
    rule:'For a primitive UTF-16 string and nonnegative integer i, charAt(i) and slice(i,i+1) return the same single code unit or empty sequence. Negative/coerced indices and different source contexts are outside this finite lowering.',
    boundary:'A reviewed context-specific source lowering and exact inverse, not a general range inference engine, concept equivalence certificate or native execution observation.'}};
}
