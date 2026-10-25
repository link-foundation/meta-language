// Actual named/closure/callable contexts retain their separate admission rules.
// Shared plans own syntax, BoundNames and entry binding decisions.
import { unsupported } from './diagnostics.js';
import { JAVASCRIPT_ARRAY_BINDING_POLICY } from './array-binding-policy.js';
import { findJavaScriptRecordParameterPattern, planJavaScriptSourceRecordParametersWithRest,readJavaScriptRecordFormalDefaultBindingPositions,readJavaScriptRecordFormalDefaultOperation,
  acceptJavaScriptRecordParameterProfile, readJavaScriptRecordParameterBindingStatement, findJavaScriptRecordParameterStrictDirective } from './javascript-record-parameter-rules.js';
import { findJavaScriptLineTerminator } from './javascript-line-terminators.js';

import { requireJavaScriptIdentifierRole } from './javascript-identifier-roles.js';
import {readJavaScriptRecordClosureContext,acceptJavaScriptRecordClosureParameterProfile} from './record-closure-parameter-rules.js';
import {readJavaScriptRecordConstantContext,acceptJavaScriptRecordCallableParameterProfile} from './record-callable-parameter-rules.js';
import {readJavaScriptFormalInitializer} from './formal-initializers.js';
import {constructJavaScriptObjectRestSource} from './object-rest.js';

export function readNamedJavaScriptRecordParameters(parser, doc, asynchronous) {
  return readJavaScriptRecordParameterSyntax(parser,doc,asynchronous,'ordinary named function declaration');
}
export function readJavaScriptRecordClosureParameters(parser,method,expression){
  return readJavaScriptRecordParameterSyntax(parser,null,false,readJavaScriptRecordClosureContext(method,expression));
}
export function readJavaScriptRecordConstantParameters(parser,doc,asynchronous,expression){
  return readJavaScriptRecordParameterSyntax(parser,doc,asynchronous,readJavaScriptRecordConstantContext(expression),true);
}
export function readJavaScriptRecordNamespaceParameters(parser,doc,asynchronous){
  return readJavaScriptRecordParameterSyntax(parser,doc,asynchronous,'ordinary namespace object method',true);
}
function readJavaScriptRecordParameterSyntax(parser,doc,asynchronous,context,callable=false){
  const c = parser.cursor;
  const start = c.index, opening = c.tokens[start - 1];
  // Only the actual formal interval supplies decision facts. Matching ignores
  // opaque regex/string/template tokens, as the ordinary parser already does.
  if (opening?.kind !== 'punct' || opening.value !== '(') return null;
  const close = parser.matching(start - 1), closing = c.tokens[close];
  if (closing?.kind !== 'punct' || closing.value !== ')') return null;
  const interval = c.tokens.slice(start, close + 1);
  const kinds = interval.map(token => token.kind), values = interval.map(token => token.value);
  const selected = findJavaScriptRecordParameterPattern(kinds, values, 0);
  if (selected < 0) return null;
  const profile=callable?acceptJavaScriptRecordCallableParameterProfile(context,asynchronous,doc?.params.size??0):context==='ordinary named function declaration'
    ?acceptJavaScriptRecordParameterProfile(context,asynchronous,doc?.params.size??0,false,false)
    :acceptJavaScriptRecordClosureParameterProfile(context,asynchronous,doc?.params.size??0);
  if(!profile&&callable)throw unsupported('record callable source profile','ordinary non-simple callables require synchronous bodies without parameter documentation tags',interval[selected]);
  if (!profile) throw unsupported('record parameter source profile',
    'the initial named declaration profile requires synchronous functions without parameter documentation tags', interval[selected]);
  const plan = planJavaScriptSourceRecordParametersWithRest(kinds,values,0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings);
  // Reason eight identifies the actual rejected BoundName, never a property.
  if (plan[0] === -8 && interval[plan[1]]) requireJavaScriptIdentifierRole(interval[plan[1]], true);
  if (plan[0] < 0) throw unsupported('record parameter binding',
    'a nonempty flat required object pattern needs distinct lexical names; defaults, arrays, nested/rest/computed patterns are separate obligations (reason ' + (-plan[0]) + ')',
    interval[plan[1]] ?? c.peek());
  if (plan[0] !== interval.length || parser.recordParameterTokenOwner !== c.tokens
    || !(parser.recordParameterNames instanceof Set) || !Number.isSafeInteger(parser.recordParameterOrdinal)
    || parser.recordParameterOrdinal < 0) throw unsupported('record parameter source transport',
    'the plan needs the complete actual formal interval and its parser-owned token/name inventory', opening);
  const used = parser.recordParameterNames;
  const parameters = [], bindings = [], sourceNames = [], sourcePlan = [start + plan[0], plan[1]];
  const actual = position => {
    if (!Number.isSafeInteger(position) || position < 0 || position >= interval.length) {
      throw unsupported('record parameter source transport', 'a selected position must belong to the actual formal interval', opening);
    }
    return interval[position];
  };
  const positions=readJavaScriptRecordFormalDefaultBindingPositions(plan);
  const readDefault=(first,last,binding)=>readJavaScriptFormalInitializer(parser,{tokens:interval,start,first,last,binding,positions,context,profile:'record formal default'});
  let cursor = 2;
  while (cursor < plan.length) {
    const form = plan[cursor++], slot = plan[cursor++];
    if (slot !== parameters.length) throw unsupported('record parameter source transport', 'logical arguments retain their actual ordered slots', opening);
    if (form === 0) {
      const position = plan[cursor++], token = actual(position);
      requireJavaScriptIdentifierRole(token, true);
      parameters.push({ name: token.value, type: null, span: { start: token.start, end: token.end } });
      sourceNames.push(token.value);sourcePlan.push(0, slot, start + position);continue;
    }
    if (form !== 1) throw unsupported('record parameter source transport', 'a parameter needs a represented ordinary or flat record form', opening);
    const openPosition = plan[cursor++], closePosition = plan[cursor++];
    const open = actual(openPosition), end = actual(closePosition);
    const count = plan[cursor++];
    sourcePlan.push(1, slot, start + openPosition, start + closePosition, count);
    let name;
    do {
      if (!Number.isSafeInteger(parser.recordParameterOrdinal + 1)) throw unsupported('record parameter source transport',
        'the parser-owned logical argument name ordinal exceeds its safe integer range', open);
      name = 'record_parameter_value' + parser.recordParameterOrdinal++;
    } while (used.has(name));
    used.add(name);
    // This is an existing SParam binder for one lowered logical argument.
    // Its span denotes the complete original pattern, never an identifier
    // token carrying this generated name. No extra IR/provenance field exists.
    const parameterSpan = { start: open.start, end: end.end };
    parameters.push({ name, type: null, span: parameterSpan });
    const excluded=[];
    for (let index = 0; index < count; index += 1) {
      const propertyPosition = plan[cursor++], localPosition = plan[cursor++],first=plan[cursor++],last=plan[cursor++];
      const property = actual(propertyPosition), local = actual(localPosition);
      requireJavaScriptIdentifierRole(local, true);
      const fallback=first>=0?readDefault(first,last,localPosition):null;
      bindings.push({ parameter: name, property: property.value, name: local.value,
        fallback,excluded:first===-2?excluded.slice():null,propertySpan:{start:property.start,end:property.end},parameterSpan, span: { start: property.start, end: fallback?.span?.end??local.end },
        bindingSpan: { start: local.start, end: local.end } });
      sourceNames.push(local.value);sourcePlan.push(start + propertyPosition, start + localPosition);
      if(first!==-2)excluded.push(property);
    }
  }
  if (parameters.length !== plan[1]) throw unsupported('record parameter source transport', 'every actual argument slot must be represented', opening);
  c.index = start + plan[0];
  return { parameters, bindings, sourceNames, sourcePlan };
}

export function readNamedJavaScriptRecordParameterBody(parser, syntax, readBody) {
  return readJavaScriptPatternParameterBody(parser,syntax,readBody,written=>syntax.bindings.map(binding=>{
    const object={k:'name',path:[binding.parameter],span:binding.parameterSpan};
    const value=binding.excluded!==null?constructJavaScriptObjectRestSource(object,binding.excluded,binding.span):binding.fallback?{k:'math',op:readJavaScriptRecordFormalDefaultOperation(),name:'',args:[{value:object,spread:false},{value:{k:'str',value:binding.property,span:binding.propertySpan},spread:false},{value:binding.fallback,spread:false}],span:binding.span}:{k:'field',object,field:binding.property,span:binding.span};
    return {s:readJavaScriptRecordParameterBindingStatement(written.has(binding.name)),name:binding.name,value,span:binding.bindingSpan};
  }));
}

export function readJavaScriptPatternParameterBody(parser,syntax,readBody,constructPrefix){
  const names = new Set(syntax.sourceNames);
  // A non-simple formal list cannot acquire a same-scope let/const binding.
  // Reuse actual declaration discovery; nested lexical shadows stay separate.
  const c = parser.cursor;
  if (c.peek().kind === 'punct' && c.peek().value === '{') {
    const opening = c.index, close = parser.matching(opening), closing = c.tokens[close];
    if (closing?.kind !== 'punct' || closing.value !== '}') throw unsupported('record parameter directive transport',
      'the original function body needs its complete actual closing brace', c.peek());
    const tokens = c.tokens.slice(opening + 1, close + 1);
    const kinds = tokens.map(token => token.kind), values = tokens.map(token => token.value);
    const raws = tokens.map(token => parser.source.slice(token.start, token.end));
    const lines = tokens.map((token, index) => findJavaScriptLineTerminator(parser.source,
      index === 0 ? c.tokens[opening].end : tokens[index - 1].end, token.start) < token.start);
    const strict = findJavaScriptRecordParameterStrictDirective(kinds, values, raws, lines, 0);
    if (strict >= 0) throw unsupported('record parameter strict directive',
      'a non-simple formal parameter list cannot have a Use Strict Directive in its function body', tokens[strict]);
    if (strict === -2) throw unsupported('record parameter directive prologue',
      'the selected body prologue needs a represented complete statement boundary', c.peek());
    const declarations = parser.blockDeclarations(c.index + 1);
    for (const name of syntax.sourceNames) if (declarations.has(name)) {
      throw unsupported('record parameter body', 'a top-level lexical declaration cannot redeclare parameter ' + name, c.peek());
    }
  }
  for (const name of names) {
    parser.scope.locals.add(name);
    parser.scope.mutable.add(name);
    parser.lexicalParameter(name);
    parser.rememberTagBinding(name, { type: null });
  }
  const before = parser.lexicalReads.length;
  const statements = readBody();
  // Actual writes initialize an ordinary mutable local before the body. A
  // same-spelled nested shadow can conservatively allocate an unused local
  // cell; the ordinary lexical lowerer still owns the actual assignment.
  const written=new Set(parser.lexicalReads.slice(before).filter(read=>read.assignment&&names.has(read.name)).map(read=>read.name));
  const prefix=constructPrefix(written);
  return [...prefix, ...statements];
}
