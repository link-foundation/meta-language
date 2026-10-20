import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {DecoratorSet} from '../src/decorators.js';
import {decorateEmitted} from '../src/grammar-emitters/common.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {readJavaScriptTemporalBindingRead,readLexicalWinnerIndex,readUnitAlternativeExpressionForm} from '../src/translation/frontend-rules.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {tokenize} from '../src/translation/lexer.js';
import {frontendRuleNativeStyleId,readFrontendRuleStyleCorrections,
  frontendTypeQueryNativeStyleId,readFrontendTypeQueryStyleCorrections,
  frontendCurrentBooleanNativeStyleId,readFrontendCurrentBooleanStyleCorrections,
  frontendTemporalBindingNativeStyleId,readFrontendTemporalBindingStyleCorrections,
  frontendLexicalDeclarationNativeStyleId,readFrontendLexicalDeclarationStyleCorrections} from '../scripts/frontend-rule-style-corrections.mjs';
import {formatProductRuleSource} from '../scripts/product-rule-style-corrections.mjs';
import {simplifyRustOptionIdentityMatches} from '../scripts/reviewed-rust-branch-styles.mjs';
import {readRustLibraryItems} from '../scripts/rust-library-packaging.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';

test('scalar declaration styles borrow actual token strings and preserve all other source owners',()=>{
  const input=readFileSync(new URL('../src/translation/frontend-rules.js',import.meta.url),'utf8');
  const emitted=emitRust(checkProgram(parseJavaScript(input)));
  const decorators=DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/frontend-rules-decorators.lino',import.meta.url),'utf8'));
  const base=new DecoratorSet(decorators.decorators.filter(definition=>definition.id!==frontendLexicalDeclarationNativeStyleId));
  let raw=formatProductRuleSource(decorateEmitted('Rust',{source:emitted.text},base).source);
  for(let round=0;round<10;round++){
    const next=formatProductRuleSource(base.decorate('emitter',{format:'Rust',scope:'formatted-module',source:raw}).source);
    if(next===raw)break;
    raw=next;assert.ok(round<9);
  }
  const actions=readFrontendLexicalDeclarationStyleCorrections(raw);
  const names=['read_java_script_lexical_declaration_step','require_java_script_branch_continuation',
    'read_java_script_field_substitution_plan'];
  assert.equal(actions.length,3);
  assert.deepEqual(actions.map(action=>readRustLibraryItems(action.from).items[0].name),names,
    'the original two owners and the constructor field alias decision each retain one complete action');
  const recorded=decorators.decorators.find(definition=>definition.id===frontendLexicalDeclarationNativeStyleId);
  assert.deepEqual(recorded.actions,actions);
  const replay=new DecoratorSet([recorded]).decorate('emitter',{format:'Rust',scope:'formatted-module',source:raw}).source;
  const before=readRustLibraryItems(raw).items,after=new Map(readRustLibraryItems(replay).items.map(item=>[item.name,item]));
  for(const item of before){
    const updated=after.get(item.name);assert.equal(updated.kind,item.kind);
    if(!names.includes(item.name)) {
      assert.equal(updated.source,item.source);continue;
    }
    const literals=(source,kind)=>tokenize(source,'Rust').tokens.filter(token=>token.kind===kind).map(token=>token.value);
    assert.deepEqual(literals(updated.source,'string'),literals(item.source,'string'));
    // Two guards returning the same reset code may share that return. Every
    // distinct scalar code and the complete ordered token comparisons remain.
    assert.deepEqual([...new Set(literals(updated.source,'number'))].sort(),[...new Set(literals(item.source,'number'))].sort());
    assert.doesNotMatch(updated.source,/#\[(?:allow|expect)\(/u);
    if(item.name==='read_java_script_lexical_declaration_step') {
      assert.match(updated.source,/kind: &str/u);assert.match(updated.source,/value: &str/u);
      assert.doesNotMatch(updated.source,/\b(?:kind|value)\.(?:clone|to_owned|to_vec)\(/u);
    }
    if(item.name==='read_java_script_field_substitution_plan') {
      for(const name of ['binding_name','subject','value_kind','value_name']) {
        assert.ok(updated.source.includes(name+': &str'),name+' retains its actual borrowed scalar interface');
        assert.ok(!updated.source.includes(name+'.clone()'),name+' needs no owned string copy');
      }
      assert.match(updated.source,/path_length: f64/u);
      assert.match(updated.source,/\) -> f64/u);
    }
  }
  assert.deepEqual(readFrontendLexicalDeclarationStyleCorrections(replay),[]);
  assert.throws(()=>readFrontendLexicalDeclarationStyleCorrections('type String=Other;\n'+raw),/primitive type ownership/u);
});

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

test('current frontend type-query styles append two source-derived owners while retaining the original three style actions',()=>{
  const input=readFileSync(new URL('../src/translation/frontend-rules.js',import.meta.url),'utf8');
  const emitted=emitRust(checkProgram(parseJavaScript(input)));
  const decorators=DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/frontend-rules-decorators.lino',import.meta.url),'utf8'));
  const historical=decorators.decorators.find(definition=>definition.id===frontendRuleNativeStyleId);
  assert.ok(historical,'the original reviewed style row remains registered');
  assert.equal(historical.actions.length,3,'all three original complete source owners remain in their original row');
  const base=new DecoratorSet(decorators.decorators.filter(definition=>definition.id!==frontendTypeQueryNativeStyleId));
  let raw=formatProductRuleSource(decorateEmitted('Rust',{source:emitted.text},base).source);
  for(let round=0;round<10;round++){
    const next=formatProductRuleSource(base.decorate('emitter',{format:'Rust',scope:'formatted-module',source:raw}).source);
    if(next===raw)break;
    raw=next;
    assert.ok(round<9,'every historical frontend row retains its existing fixed point');
  }
  const actions=readFrontendTypeQueryStyleCorrections(raw);
  assert.equal(actions.length,2);
  const names=['read_type_query_result','accept_java_script_optional_tuple_context'];
  const literals=source=>tokenize(source,'Rust').tokens.filter(token=>['string','number'].includes(token.kind)).map(token=>source.slice(token.start,token.end));
  for(const[action,name]of actions.map((action,index)=>[action,names[index]])){
    assert.ok(action.from.startsWith('pub fn '+name+'('));
    assert.ok(action.to.startsWith('pub fn '+name+'('));
    assert.deepEqual(literals(action.to),literals(action.from),'ordered comparison and result literals remain unchanged');
    assert.doesNotMatch(action.to,/#\[(?:allow|expect)\(/u);
  }
  assert.match(actions[0].to,/kind: &str/u);
  assert.doesNotMatch(actions[0].to,/else\s*\{\s*if\b/u);
  for(const name of['language','source_kind','expected_kind','present_kind'])assert.match(actions[1].to,new RegExp(name+': &str','u'));
  const styled=new DecoratorSet([{id:frontendTypeQueryNativeStyleId,level:'emitter',order:3100,
    when:[['format','Rust'],['scope','formatted-module']],actions}]);
  const replay=styled.decorate('emitter',{format:'Rust',scope:'formatted-module',source:raw}).source;
  assert.deepEqual(readFrontendTypeQueryStyleCorrections(replay),[]);
  const recorded=decorators.decorators.find(definition=>definition.id===frontendTypeQueryNativeStyleId);
  assert.ok(recorded,'the dedicated updater records the additive styles separately from every historical row');
  assert.deepEqual(recorded.actions,actions);
});

test('temporal-binding styles borrow the complete actual column family and preserve every historical owner',()=>{
  const input=readFileSync(new URL('../src/translation/frontend-rules.js',import.meta.url),'utf8');
  const emitted=emitRust(checkProgram(parseJavaScript(input)));
  const decorators=DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/frontend-rules-decorators.lino',import.meta.url),'utf8'));
  const historical=decorators.decorators.filter(definition=>definition.id!==frontendTemporalBindingNativeStyleId);
  assert.equal(historical.find(definition=>definition.id===frontendRuleNativeStyleId).actions.length,3);
  assert.equal(historical.find(definition=>definition.id===frontendTypeQueryNativeStyleId).actions.length,2);
  const base=new DecoratorSet(historical);
  let raw=formatProductRuleSource(decorateEmitted('Rust',{source:emitted.text},base).source);
  for(let round=0;round<10;round++){
    const next=formatProductRuleSource(base.decorate('emitter',{format:'Rust',scope:'formatted-module',source:raw}).source);
    if(next===raw)break;
    raw=next;
    assert.ok(round<9,'historical frontend styles retain their complete fixed point');
  }
  const actions=readFrontendTemporalBindingStyleCorrections(raw);
  assert.equal(actions.length,4,'one entry and three actual continuations own every new style');
  const originals=readRustLibraryItems(raw).items;
  const family=originals.filter(item=>item.kind==='fn'&&/^(?:ml_)?read_java_script_temporal_binding_read(?:_loop[0-9]+)?$/u.test(item.name));
  assert.equal(family.length,4);
  const columns={parents:'f64',bounds:'f64',names:'String',ends:'f64',read_scopes:'f64',read_names:'String',read_starts:'f64'};
  const literals=source=>{
    const body=source.slice(source.indexOf('{'));
    return tokenize(body,'Rust').tokens.filter(token=>['string','number'].includes(token.kind))
      .map(token=>[token.kind,token.kind==='number'?String(token.value).replaceAll('_',''):token.value]);
  };
  for(const action of actions){
    const before=readRustLibraryItems(action.from).items.find(item=>item.kind==='fn');
    const after=readSourceTextRustBlock(action.to,'fn',before.name);
    const header=after.slice(0,after.indexOf('{'));
    const signature=/^pub fn \w+\(([^)]*)\)/u.exec(after);
    assert.doesNotMatch(signature[1],/Vec</u,'all immutable column interfaces borrow their actual source values');
    for(const[column,type]of Object.entries(columns))if(new RegExp('\\b'+column+':','u').test(header)){
      assert.match(header,new RegExp('\\b'+column+': &\\['+type+'\\]','u'));
      assert.doesNotMatch(after,new RegExp('\\b'+column+'\\.(?:clone|to_vec)\\(','u'));
    }
    assert.doesNotMatch(after,/\b(?:scope|at|scope_2|ml_s[0-9]+|ml_result[0-9]+)\.clone\(\)/u,
      'primitive Copy values and proved final continuation payloads retain no redundant copies');
    assert.doesNotMatch(after,/Some\(ml_present\)\s*=>\s*ml_present/u);
    assert.deepEqual(literals(after),literals(before.source),'every original ordered literal operand retains its value');
    assert.doesNotMatch(action.to,/#\[allow\(/u);
    const arity=signature[1].split(',').filter(part=>part.trim()).length;
    assert.equal((action.to.match(/clippy::too_many_arguments/gu)??[]).length,arity>7?1:0,
      'only the actual generated continuation arity selects the existing conditional expectation');
    if(arity>7)assert.match(action.to,/checked source arguments and captured continuation state retain their original order/u);
  }
  const styled=new DecoratorSet([{id:frontendTemporalBindingNativeStyleId,level:'emitter',order:3200,
    when:[['format','Rust'],['scope','formatted-module']],actions}]);
  const replay=styled.decorate('emitter',{format:'Rust',scope:'formatted-module',source:raw}).source;
  assert.deepEqual(readFrontendTemporalBindingStyleCorrections(replay),[],'the complete new style is idempotent before formatting');
  const targets=new Map(readRustLibraryItems(replay).items.map(item=>[item.name,item]));
  for(const item of originals)if(!family.includes(item))assert.equal(targets.get(item.name).source,item.source,
    'every older function, runtime and result enum retains its exact source bytes');
  const recorded=decorators.decorators.find(definition=>definition.id===frontendTemporalBindingNativeStyleId);
  assert.ok(recorded,'the dedicated updater records the complete new style separately');
  assert.deepEqual(recorded.actions,actions);
  const copyOwner=family.find(item=>/\bscope\.clone\(\)/u.test(item.source));
  assert.ok(copyOwner,'the source-derived control retains a real optional-number copy site');
  for(const extra of[
    'fn nested(scope:String)->String{scope.clone()} ',
    'opaque!(); ',
  ]){
    const changed=copyOwner.source.replace('{','{'+extra);
    const conservative=readFrontendTemporalBindingStyleCorrections(raw.replace(copyOwner.source,changed));
    const kept=conservative.find(action=>action.from===changed)?.to??changed;
    assert.match(kept,/\bscope\.clone\(\)/u,'nested declarations and unknown macros cannot borrow a primitive parameter proof');
    assert.ok(kept.includes(extra.trim()),'an unproved body retains its exact opaque source');
  }
  assert.throws(()=>readFrontendTemporalBindingStyleCorrections('type Option=Other;\n'+raw),/primitive type ownership/u);
  assert.throws(()=>readFrontendTemporalBindingStyleCorrections('use custom::Vec;\n'+raw),/primitive type ownership/u);
});

test('temporal-binding column decisions retain local closure and module-capture refusals without changing inputs',()=>{
  const cases=[
    [[-1],[0,1,0],['value'],[10],[0],['value'],[9],[0,0]],
    [[-1],[0,1,0],['value'],[10],[0],['value'],[10],[]],
    [[-1,0],[0,1,0,1,0,1],['value'],[10],[1],['value'],[9],[0,1]],
    [[-1,0],[0,1,0,1,0,1],['value'],[10],[1],['value'],[11],[]],
    [[-1,0],[0,1,0,1,0,2],['value'],[10],[1],['value'],[9],[0,2]],
    [[-1,0],[0,1,0,1,0,2],['value'],[10],[1],['value'],[11],[0,2]],
    [[-1,0],[0,1,0,1,1,2],['value','value'],[10,-1],[1],['value'],[9],[]],
    [[-1],[0,1,0],['value'],[-1],[0],['value'],[0],[]],
    [[-1],[0,1,0],['value'],[10],[],[],[],[]],
    [[-1],[0,1,0],['value'],[10],[0],['other'],[0],[]],
    [[-1],[0,1,0],['value'],[NaN],[0],['value'],[-Infinity],[]],
    [[-1],[0,1,0],['value'],[Infinity],[0],['value'],[0],[0,0]],
  ];
  for(const row of cases){
    const args=row.slice(0,-1),before=args.map(column=>column.slice());
    assert.deepEqual(readJavaScriptTemporalBindingRead(...args),row.at(-1));
    assert.deepEqual(args,before,'the source decision observes its immutable columns without mutation');
  }
});

test('current Boolean styles transport only the historical uniquely bound temporary and preserve every original record byte',()=>{
  const data=readFileSync(new URL('../../parity/self-translation/frontend-rules-decorators.lino',import.meta.url),'utf8');
  assert.equal(createHash('sha256').update(data.slice(0,778522)).digest('hex'),
    '3078e31556afef9341fa3bdae9ab1b45be924e84039165b773229ee7006fef25',
    'the complete historical decorator file remains an exact byte prefix');
  const decorators=DecoratorSet.fromLino(data);
  const historical=decorators.decorators.find(definition=>definition.id==='reviewed-boolean-helper-styles');
  const recorded=decorators.decorators.find(definition=>definition.id===frontendCurrentBooleanNativeStyleId);
  assert.ok(historical);assert.ok(recorded);
  assert.equal(recorded.actions.length,1);
  const [action]=recorded.actions;
  assert.deepEqual(readFrontendCurrentBooleanStyleCorrections(action.from,historical),recorded.actions,
    'the current complete action is derived from the unchanged historical proof');
  assert.deepEqual(readFrontendCurrentBooleanStyleCorrections(action.to,historical),[],'the mapped target is idempotent');
  const temporary=tokenize(action.from,'Rust').tokens.find((token,index,array)=>/^ml_logical[0-9]+$/u.test(token.value)
    &&array[index-1]?.value==='let'&&array[index+1]?.value==='=')?.value;
  assert.ok(temporary);
  assert.doesNotMatch(action.to,new RegExp(temporary+'\\.clone\\(','u'));
  assert.equal((action.to.match(/\.unwrap_or_default\(\)/gu)??[]).length,2,'both original nullable Boolean false defaults remain');
  const renamed=source=>{
    const tokens=tokenize(source,'Rust').tokens.filter(token=>token.kind==='identifier'&&token.value===temporary);
    for(const token of tokens.reverse())source=source.slice(0,token.start)+'ml_logical999'+source.slice(token.end);
    return source;
  };
  assert.equal(readFrontendCurrentBooleanStyleCorrections(renamed(action.from),historical)[0].to,renamed(action.to),
    'only the same bijective local spelling is transported across future temporary numbers');
  for(const changed of[
    action.from.replace('None => false','None => true'),
    action.from.replace('let '+temporary+' =','let '+temporary+' = 1f64; let '+temporary+' ='),
    action.from.replace('let '+temporary+' =','let '+temporary+' = /* new source comment */'),
    action.from.replace('let '+temporary+' =','opaque!(); let '+temporary+' ='),
    'pub fn foreign_scope(){'+action.from+'}',
    action.from+'\n'+action.from,
  ])assert.throws(()=>readFrontendCurrentBooleanStyleCorrections(changed,historical),/complete function|top-level source function/u,
    'changed meaning, shadowing, comments, macros, nested functions and duplicate owners refuse historical proof reuse');
});


test('authentic frontend empty-string compiler styles preserve every other owner and the array provider',async()=>{
  const {readCompilerFeedbackCorrections}=await import('../scripts/compiler-style-feedback.mjs');
  const {readProjectData}=await import('../scripts/project-data.mjs');
  const address=new URL('../../parity/self-translation/frontend-rules-empty-string-compiler-feedback.lino',import.meta.url);
  const bytes=readFileSync(address);
  assert.equal(bytes.length,286635);
  assert.equal(createHash('sha256').update(bytes).digest('hex'),
    'd57157156e96da3230ec32e772221698111effbfb3b06dc8d46a7ca7fb4d8642');
  const feedback=readProjectData(address),original=structuredClone(feedback);
  assert.equal(feedback.runId,'39010575046');
  assert.equal(feedback.head,'373827ad822a03c6566e71a5ff429582ef7703a4');
  assert.equal(feedback.compiler,'clippy-driver');
  const diagnostics=feedback.diagnostics.map(row=>row.reason==='compiler-message'?row.message:row);
  assert.equal(diagnostics.filter(row=>row.code?.code==='clippy::manual_string_new').length,1);
  const relativePath='rust/src/translation/frontend_rules.rs';
  const correction=readCompilerFeedbackCorrections({feedback,allowedPaths:[relativePath]});
  assert.equal(correction.spans,1);assert.equal(correction.deferred,0);
  assert.equal(correction.actions.length,1);
  assert.equal(correction.inputs.size,1);assert.equal(correction.outputs.size,1);
  const [[path,source]]=correction.inputs;
  assert.equal(feedback.sources[path].relativePath,relativePath);
  assert.equal(createHash('sha256').update(source).digest('hex'),
    '37033e27b97bc145b0d51b89016606e141d5962c0f423ee131202432a301dad3');
  const changed=correction.outputs.get(path),before=source.toString('utf8'),after=changed.toString('utf8');
  const actual=readFileSync(new URL('../../'+relativePath,import.meta.url));
  assert.equal(actual.length,changed.length,'the complete current source length matches the retained transaction before comparing all bytes');
  assert.equal(createHash('sha256').update(actual).digest('hex'),createHash('sha256').update(changed).digest('hex'),
    'the complete current source digest matches the retained transaction before comparing all bytes');
  assert.deepEqual(changed,readFileSync(new URL('../../'+relativePath,import.meta.url)),
    'the complete actual generated module equals the authentic offline transaction output');
  const oldOwner=readSourceTextRustBlock(before,'fn','read_type_query_result');
  const newOwner=readSourceTextRustBlock(after,'fn','read_type_query_result');
  assert.equal(oldOwner.split('String::from("")').length,2);
  assert.equal(newOwner,oldOwner.replace('String::from("")','String::new()'));
  assert.equal(after,before.replace(oldOwner,newOwner),'every unrelated owner and surrounding byte remains exact');
  const provider=readSourceTextRustBlock(before,'mod','ml_array');
  assert.equal(readSourceTextRustBlock(after,'mod','ml_array'),provider);
  assert.equal(Buffer.byteLength(provider),3648);
  assert.equal(createHash('sha256').update(provider).digest('hex'),
    'c2a535a8fbc2ab09deca3f7c92cad278b0a08a526f5f87a54e2aebb822c8b76a');
  const document=readFileSync(new URL('../../parity/self-translation/frontend-rules-decorators.lino',import.meta.url),'utf8');
  const recorded=DecoratorSet.fromLino(document).decorators.find(row=>row.id==='reviewed-ci-39010575046-frontend-empty-string-style');
  assert.ok(recorded,'the authentic correction has a separate append-only style record');
  assert.deepEqual(recorded.actions,correction.actions);
  const styles=new DecoratorSet([recorded]);
  assert.equal(styles.decorate('emitter',{format:'Rust',scope:'formatted-module',source:before}).source,after);
  assert.equal(styles.decorate('emitter',{format:'Rust',scope:'formatted-module',source:after}).source,after,
    'the exact recorded correction is idempotent');
  assert.deepEqual(feedback,original,'every raw diagnostic, source snapshot and applicability field remains immutable');
  for(const field of['text','sha256']){
    const stale=structuredClone(feedback);
    stale.sources[path][field]+=field==='text'?'\n':'changed';
    assert.throws(()=>readCompilerFeedbackCorrections({feedback:stale,allowedPaths:[relativePath]}),/stale/u,
      'an edited source snapshot cannot reuse the authentic machine suggestion');
  }
});
