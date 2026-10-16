import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {DecoratorSet} from '../src/decorators.js';
import {decorateEmitted} from '../src/grammar-emitters/common.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {readLexicalWinnerIndex,readUnitAlternativeExpressionForm} from '../src/translation/frontend-rules.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {tokenize} from '../src/translation/lexer.js';
import {frontendRuleNativeStyleId,readFrontendRuleStyleCorrections} from '../scripts/frontend-rule-style-corrections.mjs';
import {formatProductRuleSource} from '../scripts/product-rule-style-corrections.mjs';
import {simplifyRustOptionIdentityMatches} from '../scripts/reviewed-rust-branch-styles.mjs';

test('reviewed frontend styles repair all fifteen CI diagnostics in their three complete source owners',()=>{
  const producer=readFileSync(new URL('../scripts/product-rule-style-corrections.mjs',import.meta.url));
  assert.equal(createHash('sha256').update(producer).digest('hex'),'9728ead881e57f16401321be9eb00e25416aab74cdb4d7164485cd0a3abbc216');
  const input=readFileSync(new URL('../src/translation/frontend-rules.js',import.meta.url),'utf8');
  const emitted=emitRust(checkProgram(parseJavaScript(input)));
  const decorators=DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/frontend-rules-decorators.lino',import.meta.url),'utf8'));
  const base=new DecoratorSet(decorators.decorators.filter(definition=>definition.id!==frontendRuleNativeStyleId));
  let raw=formatProductRuleSource(decorateEmitted('Rust',{source:emitted.text},base).source);
  for(let round=0;round<10;round++){
    const next=formatProductRuleSource(base.decorate('emitter',{format:'Rust',scope:'formatted-module',source:raw}).source);
    if(next===raw)break;
    raw=next;
    assert.ok(round<9,'existing frontend styles reach their unchanged fixed point');
  }
  const actions=readFrontendRuleStyleCorrections(raw);
  assert.equal(actions.length,3);
  const names=['read_unit_alternative_expression_form','read_lexical_winner_index','ml_read_lexical_winner_index_loop29'];
  const literals=source=>tokenize(source,'Rust').tokens.filter(token=>['string','number'].includes(token.kind)).map(token=>source.slice(token.start,token.end));
  for(const[action,name]of actions.map((action,index)=>[action,names[index]])){
    assert.ok(action.from.startsWith('pub fn '+name+'('));
    assert.ok(action.from.endsWith('}'));
    assert.deepEqual(literals(action.to),literals(action.from),'every original string and numeric operand retains its value and order');
    assert.doesNotMatch(action.to,/#\[(?:allow|expect)\(/u);
  }
  assert.match(actions[0].to,/kind: &str/u);
  assert.doesNotMatch(actions[0].to,/else\s*\{\s*if\b/u,'both nested else-if diagnostics are removed');
  for(const action of actions.slice(1)){
    assert.match(action.to,/ends: &\[f64\]/u);
    assert.match(action.to,/eligible: &\[bool\]/u);
    assert.doesNotMatch(action.to,/(?:ends|eligible)\.clone\(\)|let best_2\s*=/u,'both redundant copies and both tail bindings are removed');
  }
  assert.equal((actions[2].to.match(/\.unwrap_or_default\(\)/gu)??[]).length,1);
  assert.equal((actions[2].to.match(/\.unwrap_or\(f64::NAN\)/gu)??[]).length,2);
  assert.doesNotMatch(actions[2].to,/Some\(ml_present\)\s*=>\s*ml_present/u,'all three identity matches cover both Option lint diagnostics');
  const styled=new DecoratorSet([{id:frontendRuleNativeStyleId,level:'emitter',order:3000,when:[['format','Rust'],['scope','formatted-module']],actions}]);
  const replay=styled.decorate('emitter',{format:'Rust',scope:'formatted-module',source:raw}).source;
  assert.deepEqual(readFrontendRuleStyleCorrections(replay),[],'the source-derived styles are idempotent');
  const recorded=decorators.decorators.find(definition=>definition.id===frontendRuleNativeStyleId);
  assert.ok(recorded,'the actual generator records the reviewed styles as shared Links data');
  assert.deepEqual(recorded.actions,actions);
});

test('Option identity styles preserve pure defaults and leave effects, nonidentity arms and target text intact',()=>{
  const before='pub fn choose(flag:Option<bool>,value:Option<f64>)->f64{let flag=match flag {Some(present)=>present,None=>false};if flag {match read(value) {Some(found)=>found,None=>f64::NAN,}}else{0f64}}';
  const after=simplifyRustOptionIdentityMatches(before);
  assert.match(after,/\(flag\)\.unwrap_or_default\(\)/u);
  assert.match(after,/\(read\(value\)\)\.unwrap_or\(f64::NAN\)/u);
  assert.equal(simplifyRustOptionIdentityMatches(after),after);
  for(const source of[
    'pub fn choose(value:Option<f64>)->f64{match value{Some(found)=>found+1f64,None=>f64::NAN}}',
    'pub fn choose(value:Option<f64>)->f64{match value{Some(found)=>found,None=>side_effect()}}',
    'pub fn choose(value:Option<f64>)->f64{match value{Some(found)=>other,None=>f64::NAN}}',
    'pub fn choose()->String{String::from("match value {Some(found)=>found,None=>false}")}',
  ])assert.equal(simplifyRustOptionIdentityMatches(source),source);
  assert.throws(()=>simplifyRustOptionIdentityMatches('pub fn choose(){match value{Some(found)=>found,None=>false}'),/closing delimiter/u);
});

test('frontend lexical election retains declaration order, bounded columns, sparse flags and unordered Number behavior',()=>{
  for(const[kind,expected]of[['ref',1],['choice',2],['alias',3],['capture',3],['precedence',3],['namedPrecedence',3],['dynamicPrecedence',3],['optional',3],['lexicalScope',3],['seq',0],['repeat',0],['unknown',0]]){
    assert.equal(readUnitAlternativeExpressionForm(kind),expected,kind);
  }
  const cases=[
    [[],[],-1],[[8],[], -1],[[],[true],-1],[[4,9],[true,true],1],
    [[9,9],[true,true],0],[[9,10],[true,false],0],[[9,10],[false,true],1],
    [[9,10],[false,false],-1],[[1,20,30],[true,true],1],[[1],[true,true],0],
    [[NaN,10],[true,true],0],[[10,NaN],[true,true],0],[[Infinity,Infinity],[true,true],0],
    [[-Infinity,0],[true,true],1],[[-0,0],[true,true],0],[[5,9],[,true],1],[[,9],[true,true],0],
  ];
  for(const[ends,eligible,expected]of cases){
    const originalEnds=ends.slice(),originalEligible=eligible.slice();
    assert.equal(readLexicalWinnerIndex(ends,eligible),expected);
    assert.deepEqual(ends,originalEnds);assert.deepEqual(eligible,originalEligible);
  }
});
