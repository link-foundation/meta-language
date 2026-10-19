import assert from 'node:assert/strict';
import test from 'node:test';
import {describeGrammarRejection} from '../scripts/grammar-rejection-description.mjs';

test('grammar failure descriptions retain resource limits without invented positions',()=>{
  for(const [reason,limit] of [['stepLimit',314000],['stepLimit',243000],['memoryBudget',2000000],['nestingDepth',1000]]){
    const rejection=Object.freeze({reason,limit});
    assert.equal(describeGrammarRejection(rejection),`${reason} (limit ${limit})`);
    assert.deepEqual(rejection,{reason,limit});
  }
  assert.equal(describeGrammarRejection({reason:'stepLimit',limit:0}),'stepLimit (limit 0)');
  for(const limit of [undefined,null,-1,NaN,Infinity,'314000'])
    assert.equal(describeGrammarRejection({reason:'stepLimit',limit}),'stepLimit');
});

test('grammar failure descriptions preserve syntax positions and expected tokens',()=>{
  assert.equal(describeGrammarRejection({reason:'syntax',line:3,column:1,expected:['"\\n"','"\\r"']}),'syntax at 3:1, expected "\\n" or "\\r"');
  assert.equal(describeGrammarRejection({reason:'recovered',line:1,column:12}),'recovered at 1:12');
  assert.equal(describeGrammarRejection({reason:'syntax',line:2,column:4,limit:19,expected:['token']}),'syntax at 2:4 (limit 19), expected token');
  for(const [line,column] of [[undefined,1],[1,undefined],[0,1],[1,0],[-1,2],[1.5,2],['1',2]])
    assert.equal(describeGrammarRejection({reason:'syntax',line,column,expected:['token']}),'syntax, expected token');
});

test('grammar failure descriptions retain bounded external messages',()=>{
  assert.equal(describeGrammarRejection(new Error('source unavailable\nprivate detail')),'source unavailable');
  assert.equal(describeGrammarRejection(null),'null');
  assert.equal(describeGrammarRejection({message:'x'.repeat(500)}),'x'.repeat(300));
  const result=describeGrammarRejection({reason:'syntax',line:1,column:2,expected:['a'.repeat(500)]});
  assert.equal(result.length,300);assert.ok(result.startsWith('syntax at 1:2, expected '));
});
