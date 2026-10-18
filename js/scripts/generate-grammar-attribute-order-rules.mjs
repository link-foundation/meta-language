// Translate the complete closed JavaScript kernel; hosts keep native map storage.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {writeRecordRuleLibrary} from './record-rule-library.mjs';
import {generateGrammarAttributeOrderFixtures} from './generate-grammar-attribute-order-fixtures.mjs';

import {tokenize} from '../src/translation/lexer.js';
import {readRustLibraryItems} from './rust-library-packaging.mjs';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';

const root=new URL('../../',import.meta.url);
/** Record only the two exact generated owners' primitive native spellings. */
export function readGrammarAttributeOrderNativeStyles(source) {
  const unchanged = {source, actions: []};
  let syntax, items;
  try {
    syntax = tokenize(source, 'Rust');
    items = readRustLibraryItems(source).items;
  } catch {
    return unchanged;
  }
  const {tokens} = syntax;
  if (items.some(item => !['inner attribute', 'fn', 'mod', 'enum'].includes(item.kind))) return unchanged;
  for (let at = 0; at < tokens.length; at += 1) {
    const token = tokens[at];
    if (token.kind === 'identifier' && ['use', 'extern', 'unsafe', 'type', 'trait', 'impl'].includes(token.value)) return unchanged;
    if (token.kind === 'identifier' && ['mod', 'struct', 'enum', 'union'].includes(token.value)
      && ['f64', 'usize', 'bool', 'std', 'core'].includes(tokens[at + 1]?.value)) return unchanged;
    // No macro definitions or unknown/qualified expansion can introduce a
    // shadowing primitive/provider. The actual runtime uses plain panic!.
    if (token.kind === 'macro' && (!['panic', 'vec', 'format', 'assert'].includes(token.value)
      || tokens[at - 1]?.value === '::')) return unchanged;
    if (token.kind !== 'punct' || token.value !== '#') continue;
    const open = tokens[at + 1]?.value === '!' ? at + 2 : at + 1;
    const attribute = tokens[open + 1]?.value;
    if (tokens[open]?.value !== '[' || !['allow', 'expect', 'must_use', 'derive'].includes(attribute)) return unchanged;
    if (attribute === 'derive') {
      for (let end = open + 2; end < tokens.length && tokens[end].value !== ']'; end += 1)
        if (tokens[end].kind === 'identifier' && !['Clone', 'Debug', 'PartialEq', 'Eq'].includes(tokens[end].value)) return unchanged;
    }
  }
  const key = text => {
    const tokens=tokenize(text,'Rust').tokens.filter(token=>token.kind!=='eof');
    const stack=[],optional=new Set();
    for(let at=0;at<tokens.length;at++){
      const token=tokens[at];
      if(token.kind!=='punct')continue;
      if(['(','[','{'].includes(token.value))stack.push({value:token.value,commas:[]});
      else if([ ')',']','}'].includes(token.value)){
        const frame=stack.pop();
        // Rustfmt adds a trailing comma to multiline tuples and calls.
        // Keep single-element tuple commas: removing those changes the type.
        if(token.value===')'&&frame.value==='('&&frame.commas.length>=2
          &&frame.commas.at(-1)===at-1)optional.add(at-1);
      }else if(token.value===',')stack.at(-1)?.commas.push(at);
    }
    return JSON.stringify(tokens.filter((_,at)=>!optional.has(at)).map(token=>[token.kind,token.raw]));
  };
  const expected = [
    ['compare_grammar_attribute_integer_keys', `pub fn compare_grammar_attribute_integer_keys(left: f64, right: f64) -> f64 {
      if left < 0f64 {
        if right < 0f64 { 0f64 } else { 1f64 }
      } else if (right < 0f64) || (left < right) {
        -1f64
      } else if left > right {
        1f64
      } else {
        0f64
      }
    }`],
    ['read_grammar_attribute_integer_key', `pub fn read_grammar_attribute_integer_key(units: &[f64]) -> f64 {
      if ((units.len() as f64) < 1f64) || ((units.len() as f64) > 10f64) {
        -1f64
      } else if ((units.len() as f64) > 1f64)
        && match (self::ml_array::optional_at(units, self::ml_array::number_index(0f64)), 48f64) {
          (Some(ml_left), ml_right) => ml_left == ml_right,
          _ => false,
        }
      {
        -1f64
      } else {
        {
          let value = 0f64;
          {
            let index = 0f64;
            {
              let ml_s1 = self::ml_read_grammar_attribute_integer_key_loop1(units, value, index);
              match ml_s1 {
                self::MlReadGrammarAttributeIntegerKeyLoop1Result::MlReadGrammarAttributeIntegerKeyLoop1Done(value_2) => {
                  if value_2 <= 4_294_967_294f64 { value_2 } else { -1f64 }
                }
                self::MlReadGrammarAttributeIntegerKeyLoop1Result::MlReadGrammarAttributeIntegerKeyLoop1Return(ml_result1) => {
                  ml_result1
                }
              }
            }
          }
        }
      }
    }`],
  ];
  let adapted = source;
  const actions = [];
  for (const [name, body] of expected) {
    const owners = items.filter(item => item.kind === 'fn' && item.name === name);
    if (owners.length !== 1) continue;
    const from = readSourceTextRustBlock(owners[0].source, 'fn', name);
    // The emitter omits the final match-arm comma that Rustfmt supplies.
    if (![key(body),key(body.replace('_ => false,','_ => false'))].includes(key(from))
      || source.split(from).length !== 2) continue;
    const own = tokenize(from, 'Rust');
    let to;
    if (name === 'compare_grammar_attribute_integer_keys') {
      // Exact primitive signature and complete call-free body above rule out
      // custom operators, references, local shadows, imports and captures.
      const fn = own.tokens.find(token => token.kind === 'identifier' && token.value === 'fn');
      to = from.slice(0, fn.start) + 'const ' + from.slice(fn.start);
    } else {
      // This exact first guard reads the immutable &[f64] parameter's finite
      // usize length before any local binding. It is not an arbitrary f64.
      const start = own.tokens.findIndex(token => token.kind === 'identifier' && token.value === 'if');
      const end = own.tokens.findIndex((token, at) => at > start && token.kind === 'punct' && token.value === '{');
      const first = own.tokens[start + 1].start, last = own.tokens[end - 1].end;
      if (own.comments.some(comment => comment.start < last && comment.end > first)) continue;
      to = from.slice(0, first) + '!(1usize..=10usize).contains(&units.len())' + from.slice(last);
    }
    actions.push({op: 'replace', field: 'source', from, to});
    adapted = adapted.replace(from, to);
  }
  return {source: adapted, actions};
}

export function generateGrammarAttributeOrderRules({check=false,native=false}={}) {
  const path='js/src/translation/grammar-attribute-order-rules.js';
  const source=readFileSync(new URL(path,root),'utf8');
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const emitted=emitRust(program,{arrayConsumption:true});
  assert.equal(emitted.definitions.length,program.declarations.size,
    'every checked source declaration and generated continuation is retained');
  const policy={schemaVersion:1,model:'ordinary grammar attribute key order 1',
    source:{path,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex'),text:source},
    owners:[...program.declarations.values()].map(item=>({name:item.name,kind:item.k,fullName:item.fullName,modulePath:item.modulePath})),
    inputs:'Hosts supply exact UTF-16 code units. At most eleven leading units suffice: more than ten units always classify as a non-index key.',
    ordering:'Canonical decimal array-index names from 0 through 4294967294 come first in numeric order. Other names retain insertion order. Overwriting a key retains its position.',
    equality:'The existing grammar attribute equality ignores dictionary entry order. Lexical and selection metadata keep their distinct existing ordered equality.',
    nativeStorage:{crate:'indexmap',version:'2.14.2',role:'Store every attribute entry and its observable position. Shared translated decisions select the index prefix; native map operations supply storage only.'},
    boundary:'This complete-source rule translation does not establish native execution, full CST outcome transport or whole-language coverage.'};
  for(const runtime of native?['js','rust']:['js']){
    const file=new URL(`${runtime}/src/data/grammar-attribute-order-rules.lino`,root);
    if(check)checkProjectData(file,policy);else writeProjectData(file,policy);
  }
  if(native)writeRecordRuleLibrary(emitted,'grammar_attribute_order_rules',{check,sourceStyles:readGrammarAttributeOrderNativeStyles});
  generateGrammarAttributeOrderFixtures({check});
  return policy;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const check=process.argv.includes('--check');
  generateGrammarAttributeOrderRules({check,native:!process.argv.includes('--javascript-only')&&(check||process.argv.includes('--native'))});
  console.log('Complete grammar attribute order rules and continuation owners match their shared source.');
}
