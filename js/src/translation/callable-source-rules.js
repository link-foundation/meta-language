// Closed callable algorithms over mechanical object/array facts. Node numbers
// identify the actual input values; they are never source text or host callbacks.

import { readRequiredNumberFact, readRequiredTextFact, readRequiredBooleanFact } from './fact-column-rules.js';
import { acceptJavaScriptWhitespaceUnit } from './javascript-whitespace-rules.js';
import { readTypeSeparatorPositions } from './type-syntax-rules.js';

// Linked policy decisions begin.
/** @param {string} kind @returns {string} */
export function readCallableStatementRole(kind) {
  if (kind === 'const' || kind === 'let') return 'declare';
  if (kind === 'assign') return 'assign';
  if (kind === 'assignReference') return 'reference';
  if (kind === 'return' || kind === 'expr' || kind === 'print') return 'read';
  if (kind === 'throw') return 'exception';
  if (kind === 'block') return 'block';
  if (kind === 'if') return 'branch';
  if (kind === 'switch') return 'switch';
  if (kind === 'while' || kind === 'doWhile') return 'loop';
  if (kind === 'for') return 'for';
  return '';
}
// Linked policy decisions end.

// Linked callable type whitespace decisions begin.
/** @param {number} unit @returns {boolean} */
export function acceptCallableTypeWhitespace(unit) {
  return acceptJavaScriptWhitespaceUnit(unit);
}
// Linked callable type whitespace decisions end.

/** @param {number[]} units @param {number} start @param {number} end @returns {number} */
function callableTypeTrimStart(units, start, end) {
  let at = start;
  while (at < end && acceptCallableTypeWhitespace(readRequiredNumberFact(units, at))) at += 1;
  return at;
}

/** @param {number[]} units @param {number} start @param {number} end @returns {number} */
function callableTypeTrimEnd(units, start, end) {
  let at = end;
  while (at > start && acceptCallableTypeWhitespace(readRequiredNumberFact(units, at - 1))) at -= 1;
  return at;
}

/** @param {number[]} units @param {number} start @param {number} end @param {boolean} named @returns {number[]} */
function callableTypeParameterSegments(units, start, end, named) {
  const part = units.slice(start, end);
  const colons = named ? readTypeSeparatorPositions(part, 58) : [];
  if (!colons.length) return [1, start, end];
  let from = readRequiredNumberFact(colons, 0) + 1;
  let at = 1;
  let segments = [];
  while (at <= colons.length) {
    const to = at === colons.length ? part.length : readRequiredNumberFact(colons, at);
    const begin = callableTypeTrimStart(part, from, to);
    segments = segments.concat([start + begin, start + callableTypeTrimEnd(part, begin, to)]);
    from = to + 1;
    at += 1;
  }
  return [colons.length].concat(segments);
}

/** @param {number[]} units @param {number} start @param {number} end @param {boolean} named @returns {number[]} */
function callableTypeParameters(units, start, end, named) {
  const part = units.slice(start, end);
  const commas = readTypeSeparatorPositions(part, 44);
  let from = 0;
  let at = 0;
  let count = 0;
  let plan = [];
  while (at <= commas.length) {
    const to = at === commas.length ? part.length : readRequiredNumberFact(commas, at);
    const begin = callableTypeTrimStart(part, from, to);
    const finish = callableTypeTrimEnd(part, begin, to);
    if (finish > begin) {
      count += 1;
      plan = plan.concat(callableTypeParameterSegments(units, start + begin, start + finish, named));
    }
    from = to + 1;
    at += 1;
  }
  return [count].concat(plan);
}

/**
 * [] means another type form; [-1] is the original missing arrow result.
 * [0,start,end] projects parentheses. Function plans start
 * [1,resultStart,resultEnd,parameterCount], then each parameter has an
 * actual segment count and ordered UTF16 slices joined with a colon.
 * @param {number[]} units @returns {number[]}
 */
export function planCallableTypeSyntax(units) {
  if (!units.length) return [];
  if (readRequiredNumberFact(units, 0) === 40) {
    let depth = 0;
    let close = -1;
    let at = 0;
    while (at < units.length && close < 0) {
      const unit = readRequiredNumberFact(units, at);
      if (unit === 40) depth += 1;
      if (unit === 41) {
        depth -= 1;
        if (depth === 0) close = at;
      }
      at += 1;
    }
    const arrow = callableTypeTrimStart(units, close + 1, units.length);
    if (close >= 0 && arrow + 1 < units.length && readRequiredNumberFact(units, arrow) === 61 && readRequiredNumberFact(units, arrow + 1) === 62) {
      const start = callableTypeTrimStart(units, arrow + 2, units.length);
      const end = callableTypeTrimEnd(units, start, units.length);
      return start === end ? [-1] : [1, start, end].concat(callableTypeParameters(units, 1, close, true));
    }
  }
  const prefix = [102, 117, 110, 99, 116, 105, 111, 110];
  let at = 0;
  while (at < prefix.length && at < units.length && readRequiredNumberFact(units, at) === readRequiredNumberFact(prefix, at)) at += 1;
  if (at === prefix.length) {
    const open = callableTypeTrimStart(units, at, units.length);
    if (open < units.length && readRequiredNumberFact(units, open) === 40) {
      let close = units.length;
      while (close > open + 1) {
        close -= 1;
        const colon = callableTypeTrimStart(units, close + 1, units.length);
        if (readRequiredNumberFact(units, close) === 41 && colon < units.length && readRequiredNumberFact(units, colon) === 58 && colon + 1 < units.length) {
          const result = callableTypeTrimStart(units, colon + 1, units.length);
          return [1, result < units.length ? result : units.length - 1, units.length].concat(callableTypeParameters(units, open + 1, close, false));
        }
      }
    }
  }
  return readRequiredNumberFact(units, 0) === 40 && readRequiredNumberFact(units, units.length - 1) === 41 ? [0, 1, units.length - 1] : [];
}

/** @param {string} kind @param {number} parameters @param {number} argumentsCount @returns {string} */
export function readCallableLinkedApplicationError(kind, parameters, argumentsCount) {
  if (kind !== 'function') return 'a linked application does not target a function';
  return parameters === argumentsCount ? '' : 'a linked application has the wrong number of arguments';
}

/** @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} field @returns {number} */
function callableField(starts, counts, labels, targets, node, field) {
  if (node < 0 || node >= starts.length) return -1;
  let at = readRequiredNumberFact(starts, node);
  while (at < readRequiredNumberFact(starts, node) + readRequiredNumberFact(counts, node)) {
    if (readRequiredTextFact(labels, at) === field) return readRequiredNumberFact(targets, at);
    at += 1;
  }
  return -1;
}

/** @param {string[]} texts @param {number} node @returns {string} */
function callableText(texts, node) {
  return node < 0 || node >= texts.length ? '' : readRequiredTextFact(texts, node);
}

/** @param {string[]} forms @param {string[]} texts @param {number} node @returns {boolean} */
function callableTruthy(forms, texts, node) {
  if (node < 0 || node >= forms.length) return false;
  if (readRequiredTextFact(forms, node) === 'object' || readRequiredTextFact(forms, node) === 'array') return true;
  if (readRequiredTextFact(forms, node) === 'boolean') return readRequiredTextFact(texts, node) === 'true';
  if (readRequiredTextFact(forms, node) === 'number') return readRequiredTextFact(texts, node) !== '0' && readRequiredTextFact(texts, node) !== '-0' && readRequiredTextFact(texts, node) !== 'NaN';
  return readRequiredTextFact(forms, node) === 'string' && readRequiredTextFact(texts, node) !== '';
}

/** @param {string[]} names @param {string} name @returns {boolean} */
function callableBound(names, name) {
  let at = names.length;
  while (at > 0) {
    at -= 1;
    if (readRequiredTextFact(names, at) === name) return true;
  }
  return false;
}

/** @param {number[]} starts @param {number[]} counts @param {number[]} targets @param {number} node @param {number} index @returns {number} */
function callableElement(starts, counts, targets, node, index) {
  return node < 0 || index < 0 || index >= readRequiredNumberFact(counts, node) ? -1 : readRequiredNumberFact(targets, readRequiredNumberFact(starts, node) + index);
}

/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} field @returns {string} */
function callableTag(texts, starts, counts, labels, targets, node, field) {
  return callableText(texts, callableField(starts, counts, labels, targets, node, field));
}

/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} parameters @returns {string[]} */
function callableParameterNames(texts, starts, counts, labels, targets, parameters) {
  let names = [];
  if (parameters < 0) return names;
  let at = 0;
  while (at < readRequiredNumberFact(counts, parameters)) {
    names = names.concat([callableTag(texts, starts, counts, labels, targets, callableElement(starts, counts, targets, parameters, at), 'name')]);
    at += 1;
  }
  return names;
}

/** Names bound by complete source patterns are local to their actual row.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} node
 * @returns {string[]} */
export function readCallableSourcePatternNames(texts,starts,counts,labels,targets,node){
 if(node<0)return [];
 const kind=callableTag(texts,starts,counts,labels,targets,node,'k');
 let names=[];
 if(kind==='alias'||kind==='bind'||kind==='bindOrCtor')names.push(callableTag(texts,starts,counts,labels,targets,node,'name'));
 if(kind==='alias'||kind==='natAdd')return names.concat(readCallableSourcePatternNames(texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,node,kind==='alias'?'pattern':'inner')));
 if(kind==='ctor'||kind==='tuple'){
  const args=callableField(starts,counts,labels,targets,node,'args');let at=0;
  while(args>=0&&at<readRequiredNumberFact(counts,args)){names=names.concat(readCallableSourcePatternNames(texts,starts,counts,labels,targets,callableElement(starts,counts,targets,args,at)));at+=1;}
 }
 return names;
}

/**
 * Surface references retain the original first-use order. Lambda and let
 * binders suppress only their own body references, never initializer uses.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} node @param {string[]} bound @param {boolean} checked
 * @returns {number[]}
 */
export function readCallableFreeReferences(forms, texts, starts, counts, labels, targets, node, bound, checked) {
  if (node < 0 || readRequiredTextFact(forms, node) !== 'object' && readRequiredTextFact(forms, node) !== 'array') return [];
  const kind = callableTag(texts, starts, counts, labels, targets, node, 'k');
  if (kind === 'lambda') {
    const params = callableField(starts, counts, labels, targets, node, 'params');
    const names = callableParameterNames(texts, starts, counts, labels, targets, params);
    let references = readCallableFreeReferences(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, node, 'body'), bound.concat(names), checked);
    let at = 0;
    while (!checked && params >= 0 && at < readRequiredNumberFact(counts, params)) {
      const parameter = callableElement(starts, counts, targets, params, at);
      references = references.concat(readCallableFreeReferences(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, parameter, 'default'), bound, checked));
      at += 1;
    }
    return references;
  }
  if (kind === 'let') {
    const value = readCallableFreeReferences(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, node, 'value'), bound, checked);
    const name = callableTag(texts, starts, counts, labels, targets, node, 'name');
    return value.concat(readCallableFreeReferences(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, node, 'body'), bound.concat([name]), checked));
  }
  if(!checked&&kind==='match'){
    let references=readCallableFreeReferences(forms,texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,node,'scrutinees'),bound,checked);
    const rows=callableField(starts,counts,labels,targets,node,'rows');let at=0;
    while(rows>=0&&at<readRequiredNumberFact(counts,rows)){
      const row=callableElement(starts,counts,targets,rows,at),patterns=callableField(starts,counts,labels,targets,row,'patterns');let index=0,names=[];
      while(patterns>=0&&index<readRequiredNumberFact(counts,patterns)){names=names.concat(readCallableSourcePatternNames(texts,starts,counts,labels,targets,callableElement(starts,counts,targets,patterns,index)));index+=1;}
      references=references.concat(readCallableFreeReferences(forms,texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,row,'guard'),bound.concat(names),checked));
      references=references.concat(readCallableFreeReferences(forms,texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,row,'body'),bound.concat(names),checked));at+=1;
    }
    return references;
  }
  if (checked && kind === 'match') {
    let references = readCallableFreeReferences(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, node, 'scrutinee'), bound, checked);
    const cases = callableField(starts, counts, labels, targets, node, 'cases');
    let at = 0;
    while (cases >= 0 && at < readRequiredNumberFact(counts, cases)) {
      const branch = callableElement(starts, counts, targets, cases, at);
      const pattern = callableField(starts, counts, labels, targets, branch, 'pattern');
      const patternKind = callableTag(texts, starts, counts, labels, targets, pattern, 'k');
      let names = [];
      if (patternKind === 'bind' || patternKind === 'natSucc') names = [callableTag(texts, starts, counts, labels, targets, pattern, 'name')];
      else {
        const binds = callableField(starts, counts, labels, targets, pattern, 'binds');
        let index = 0;
        while (binds >= 0 && index < readRequiredNumberFact(counts, binds)) {
          const binding = callableElement(starts, counts, targets, binds, index);
          if (callableTruthy(forms, texts, binding)) names = names.concat([callableText(texts, binding)]);
          index += 1;
        }
      }
      references = references.concat(readCallableFreeReferences(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, branch, 'body'), bound.concat(names), checked));
      at += 1;
    }
    return references;
  }
  let references = [];
  if (checked && kind === 'var') {
    const name = callableTag(texts, starts, counts, labels, targets, node, 'name');
    if (!callableBound(bound, name)) references = [node];
  }
  if (!checked && kind === 'name') {
    const path = callableField(starts, counts, labels, targets, node, 'path');
    if (path >= 0 && readRequiredNumberFact(counts, path) === 1) {
      const name = callableElement(starts, counts, targets, path, 0);
      if (!callableBound(bound, callableText(texts, name))) references = [name];
    }
  }
  let at = readRequiredNumberFact(starts, node);
  while (at < readRequiredNumberFact(starts, node) + readRequiredNumberFact(counts, node)) {
    const label = readRequiredTextFact(labels, at);
    if (label !== 'span' && label !== 'type' && (checked || label !== 'data')) references = references.concat(readCallableFreeReferences(forms, texts, starts, counts, labels, targets, readRequiredNumberFact(targets, at), bound, checked));
    at += 1;
  }
  return references;
}

/** @param {number[]} references @returns {number[]} */
function callableReadEvents(references) {
  let events = [];
  let at = 0;
  while (at < references.length) {
    events = events.concat([0, readRequiredNumberFact(references, at)]);
    at += 1;
  }
  return events;
}

/** Assigned cell carriers retain lexical lambda/let boundaries just like reads.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} node @param {string[]} bound @returns {number[]} */
function callableAssignedExpressionReferences(forms, texts, starts, counts, labels, targets, node, bound) {
  if (node < 0 || readRequiredTextFact(forms, node) !== 'object' && readRequiredTextFact(forms, node) !== 'array') return [];
  const kind = callableTag(texts, starts, counts, labels, targets, node, 'k');
  if(kind==='match'){
    let references=callableAssignedExpressionReferences(forms,texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,node,'scrutinees'),bound);
    const rows=callableField(starts,counts,labels,targets,node,'rows');let at=0;
    while(rows>=0&&at<readRequiredNumberFact(counts,rows)){
      const row=callableElement(starts,counts,targets,rows,at),patterns=callableField(starts,counts,labels,targets,row,'patterns');let index=0,names=[];
      while(patterns>=0&&index<readRequiredNumberFact(counts,patterns)){names=names.concat(readCallableSourcePatternNames(texts,starts,counts,labels,targets,callableElement(starts,counts,targets,patterns,index)));index+=1;}
      references=references.concat(callableAssignedExpressionReferences(forms,texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,row,'guard'),bound.concat(names)));
      references=references.concat(callableAssignedExpressionReferences(forms,texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,row,'body'),bound.concat(names)));at+=1;
    }
    return references;
  }
  if (kind === 'let') {
    const value = callableAssignedExpressionReferences(forms,texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,node,'value'),bound);
    return value.concat(callableAssignedExpressionReferences(forms,texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,node,'body'),bound.concat([callableTag(texts,starts,counts,labels,targets,node,'name')])));
  }
  if (kind === 'lambda') {
    const params = callableField(starts,counts,labels,targets,node,'params');
    let assigned = callableAssignedExpressionReferences(forms,texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,node,'body'),bound.concat(callableParameterNames(texts,starts,counts,labels,targets,params)));
    let index = 0;
    while (params >= 0 && index < readRequiredNumberFact(counts,params)) {
      const parameter = callableElement(starts,counts,targets,params,index);
      assigned = assigned.concat(callableAssignedExpressionReferences(forms,texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,parameter,'default'),bound));
      index += 1;
    }
    return assigned;
  }
  let assigned = [];
  if (kind === 'cellWrite') {
    const cell = callableField(starts,counts,labels,targets,node,'cell');
    const path = callableField(starts,counts,labels,targets,cell,'path');
    if (callableTag(texts,starts,counts,labels,targets,cell,'k') === 'name' && path >= 0 && readRequiredNumberFact(counts,path) === 1) {
      const name = callableElement(starts,counts,targets,path,0);
      if (!callableBound(bound,callableText(texts,name))) assigned = [1,name];
    }
  }
  let at = readRequiredNumberFact(starts,node);
  while (at < readRequiredNumberFact(starts,node) + readRequiredNumberFact(counts,node)) {
    const label = readRequiredTextFact(labels,at);
    if (label !== 'span' && label !== 'type' && label !== 'data')
      assigned = assigned.concat(callableAssignedExpressionReferences(forms,texts,starts,counts,labels,targets,readRequiredNumberFact(targets,at),bound));
    at += 1;
  }
  return assigned;
}

/** @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} node @param {string[]} bound @returns {number[]} */
function callableExpressionEvents(forms,texts,starts,counts,labels,targets,node,bound) {
  return callableReadEvents(readCallableFreeReferences(forms,texts,starts,counts,labels,targets,node,bound,false))
    .concat(callableAssignedExpressionReferences(forms,texts,starts,counts,labels,targets,node,bound));
}

/**
 * Event pairs are [0, free name], [1, assigned name], [2, declaration].
 * Declaration events are visible to the following statement, and leave a
 * nested block only when the original block carries inline:true.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} statements @param {string[]} bound @returns {number[]}
 */
export function readCallableStatementUses(forms, texts, starts, counts, labels, targets, statements, bound) {
  let events = [];
  let scope = bound;
  let at = 0;
  while (statements >= 0 && at < readRequiredNumberFact(counts, statements)) {
    const statement = callableElement(starts, counts, targets, statements, at);
    const next = callableStatementUses(forms, texts, starts, counts, labels, targets, statement, scope);
    let index = 0;
    while (index < next.length) {
      if (readRequiredNumberFact(next, index) === 2) scope = scope.concat([callableText(texts, readRequiredNumberFact(next, index + 1))]);
      index += 2;
    }
    events = events.concat(next);
    at += 1;
  }
  return events;
}

/** @param {number[]} events @returns {number[]} */
function callableNestedEvents(events) {
  let result = [];
  let at = 0;
  while (at < events.length) {
    if (readRequiredNumberFact(events, at) !== 2) result = result.concat([readRequiredNumberFact(events, at), readRequiredNumberFact(events, at + 1)]);
    at += 2;
  }
  return result;
}

/** @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} statement @param {string[]} bound @returns {number[]} */
function callableStatementUses(forms, texts, starts, counts, labels, targets, statement, bound) {
  const role = readCallableStatementRole(callableTag(texts, starts, counts, labels, targets, statement, 's'));
  const name = callableField(starts, counts, labels, targets, statement, 'name');
  const value = callableField(starts, counts, labels, targets, statement, 'value');
  if (role === 'declare') return callableExpressionEvents(forms, texts, starts, counts, labels, targets, value, bound).concat([2, name]);
  if (role === 'assign') {
    const reads = callableExpressionEvents(forms, texts, starts, counts, labels, targets, value, bound);
    return callableBound(bound, callableText(texts, name)) ? reads : reads.concat([0, name, 1, name]);
  }
  if (role === 'read') return callableExpressionEvents(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, statement, 'expr'), bound);
  if (role === 'exception') return callableExpressionEvents(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, statement, 'exception'), bound);
  if (role === 'reference') {
    return callableExpressionEvents(forms,texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,statement,'object'),bound)
      .concat(callableExpressionEvents(forms,texts,starts,counts,labels,targets,callableField(starts,counts,labels,targets,statement,'index'),bound),
        callableExpressionEvents(forms,texts,starts,counts,labels,targets,value,bound));
  }
  if (role === 'block') {
    const events = readCallableStatementUses(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, statement, 'body'), bound);
    return callableTruthy(forms, texts, callableField(starts, counts, labels, targets, statement, 'inline')) ? events : callableNestedEvents(events);
  }
  if (role === 'branch' || role === 'loop') {
    const reads = callableExpressionEvents(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, statement, 'cond'), bound);
    if (role === 'loop') return reads.concat(callableNestedEvents(readCallableStatementUses(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, statement, 'body'), bound)));
    const yes = readCallableStatementUses(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, statement, 'then'), bound);
    const no = readCallableStatementUses(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, statement, 'else'), bound);
    return reads.concat(callableNestedEvents(yes), callableNestedEvents(no));
  }
  if (role === 'switch') {
    let events = callableExpressionEvents(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, statement, 'discriminant'), bound);
    const clauses = callableField(starts, counts, labels, targets, statement, 'clauses');
    let at = 0;
    while (clauses >= 0 && at < readRequiredNumberFact(counts, clauses)) {
      const clause = callableElement(starts, counts, targets, clauses, at);
      events = events.concat(callableNestedEvents(readCallableStatementUses(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, clause, 'body'), bound)));
      at += 1;
    }
    return events;
  }
  if (role === 'for') {
    const initial = readCallableStatementUses(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, statement, 'init'), bound);
    let inner = bound;
    let at = 0;
    while (at < initial.length) {
      if (readRequiredNumberFact(initial, at) === 2) inner = inner.concat([callableText(texts, readRequiredNumberFact(initial, at + 1))]);
      at += 2;
    }
    let events = callableNestedEvents(initial);
    events = events.concat(callableExpressionEvents(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, statement, 'cond'), inner));
    const update = readCallableStatementUses(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, statement, 'update'), inner);
    at = 0;
    while (at < update.length) {
      if (readRequiredNumberFact(update, at) === 2) inner = inner.concat([callableText(texts, readRequiredNumberFact(update, at + 1))]);
      at += 2;
    }
    return events.concat(callableNestedEvents(update), callableNestedEvents(readCallableStatementUses(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, statement, 'body'), inner)));
  }
  return [];
}

/**
 * Outer assignments do not descend into a closure. Returns closure/name pairs
 * for the first captured assignment of each closure, in original visit order.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} node @returns {number[]}
 */
export function readCallableCaptureConflicts(forms, texts, starts, counts, labels, targets, node) {
  const observations = callableOuterAssignments(forms, texts, starts, counts, labels, targets, node);
  let assigned = [];
  let closures = [];
  let at = 0;
  while (at < observations.length) {
    if (readRequiredNumberFact(observations, at) === 0) assigned = assigned.concat([callableText(texts, readRequiredNumberFact(observations, at + 1))]);
    else closures = closures.concat([readRequiredNumberFact(observations, at + 1)]);
    at += 2;
  }
  let result = [];
  at = 0;
  while (at < closures.length) {
    const captures = callableField(starts, counts, labels, targets, readRequiredNumberFact(closures, at), 'captures');
    let index = 0;
    let found = false;
    while (captures >= 0 && index < readRequiredNumberFact(counts, captures) && !found) {
      const capture = callableElement(starts, counts, targets, captures, index);
      if (callableBound(assigned, callableText(texts, capture))) {
        result = result.concat([readRequiredNumberFact(closures, at), capture]);
        found = true;
      }
      index += 1;
    }
    at += 1;
  }
  return result;
}

/** @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @returns {number[]} */
function callableOuterAssignments(forms, texts, starts, counts, labels, targets, node) {
  if (node < 0 || readRequiredTextFact(forms, node) !== 'object' && readRequiredTextFact(forms, node) !== 'array') return [];
  if (callableTag(texts, starts, counts, labels, targets, node, 'k') === 'lambda') return [1, node];
  let result = [];
  if (callableTag(texts, starts, counts, labels, targets, node, 's') === 'assign') result = [0, callableField(starts, counts, labels, targets, node, 'name')];
  let at = readRequiredNumberFact(starts, node);
  while (at < readRequiredNumberFact(starts, node) + readRequiredNumberFact(counts, node)) {
    if (readRequiredTextFact(labels, at) !== 'span' && readRequiredTextFact(labels, at) !== 'type') result = result.concat(callableOuterAssignments(forms, texts, starts, counts, labels, targets, readRequiredNumberFact(targets, at)));
    at += 1;
  }
  return result;
}

/** @param {string} surfaceKind @param {string} expectedKind @param {string} declarationKind @returns {boolean} */
export function acceptCallableFunctionReference(surfaceKind, expectedKind, declarationKind) {
  return declarationKind === 'fn' && (surfaceKind === 'functionValue' || surfaceKind === 'name' && expectedKind === 'function');
}

/** @param {string} headKind @param {number} pathLength @param {string} localKind @returns {boolean} */
export function requireCallableComputedApplication(headKind, pathLength, localKind) {
  return headKind !== 'name' && headKind !== 'dotCtor' || headKind === 'name' && pathLength === 1 && localKind === 'function';
}

/** @param {string} kind @param {number} parameters @param {number} argumentsCount @returns {string} */
export function readCallableApplicationError(kind, parameters, argumentsCount) {
  if (kind !== 'function') return 'the called value is not a function';
  return parameters !== argumentsCount ? 'function expects ' + String(parameters) + ' arguments' : '';
}

/**
 * Parameter/type/declared triples preserve each original identity. Missing
 * types stay -1, so the checker retains its original named diagnostic/span.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} parameters @param {number} expected @returns {number[]}
 */
export function planCallableParameterTypes(forms, texts, starts, counts, labels, targets, parameters, expected) {
  const expectedKind = callableTag(texts, starts, counts, labels, targets, expected, 'kind');
  const expectedParams = callableField(starts, counts, labels, targets, expected, 'params');
  let result = [];
  let at = 0;
  while (parameters >= 0 && at < readRequiredNumberFact(counts, parameters)) {
    const parameter = callableElement(starts, counts, targets, parameters, at);
    const explicit = callableField(starts, counts, labels, targets, parameter, 'type');
    const type = callableTruthy(forms, texts, explicit) ? explicit : expectedKind === 'function' ? callableElement(starts, counts, targets, expectedParams, at) : -1;
    result = result.concat([parameter, type, callableTruthy(forms, texts, explicit) ? 1 : 0]);
    at += 1;
  }
  return result;
}

/**
 * A function constructor plan is [result type, parameter types...]. The
 * declaration and lambda paths use actual type identities from the input.
 * @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} parameters
 * @param {number} result @returns {number[]}
 */
export function planCallableFunctionType(starts, counts, labels, targets, parameters, result) {
  let plan = [result];
  let at = 0;
  while (parameters >= 0 && at < readRequiredNumberFact(counts, parameters)) {
    const parameter = callableElement(starts, counts, targets, parameters, at);
    plan = plan.concat([callableField(starts, counts, labels, targets, parameter, 'type')]);
    at += 1;
  }
  return plan;
}

/**
 * Exact function unification pairs; -1 means a parameter-count conflict.
 * @param {number[]} starts @param {number[]} counts @param {string[]} labels
 * @param {number[]} targets @param {number} left @param {number} right
 * @returns {number[]}
 */
export function planCallableTypeConstraints(starts, counts, labels, targets, left, right) {
  const leftParams = callableField(starts, counts, labels, targets, left, 'params');
  const rightParams = callableField(starts, counts, labels, targets, right, 'params');
  if (leftParams < 0 || rightParams < 0 || readRequiredNumberFact(counts, leftParams) !== readRequiredNumberFact(counts, rightParams)) return [-1];
  let pairs = [];
  let at = 0;
  while (at < readRequiredNumberFact(counts, leftParams)) {
    pairs = pairs.concat([callableElement(starts, counts, targets, leftParams, at), callableElement(starts, counts, targets, rightParams, at)]);
    at += 1;
  }
  return pairs.concat([callableField(starts, counts, labels, targets, left, 'ret'), callableField(starts, counts, labels, targets, right, 'ret')]);
}

/**
 * Application inference chooses the existing result only for an actual
 * function with matching arity. Otherwise it requests one whole function
 * constraint, preserving the ordinary unifier's existing refusal behavior.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} fn
 * @param {number} argumentsNode @param {number} candidate @returns {number[]}
 */
export function planCallableApplicationConstraints(texts, starts, counts, labels, targets, fn, argumentsNode, candidate) {
  const params = callableField(starts, counts, labels, targets, fn, 'params');
  if (callableTag(texts, starts, counts, labels, targets, fn, 'kind') !== 'function' || params < 0 || argumentsNode < 0 || readRequiredNumberFact(counts, params) !== readRequiredNumberFact(counts, argumentsNode)) return [-1, fn, candidate];
  let plan = [callableField(starts, counts, labels, targets, fn, 'ret')];
  let at = 0;
  while (at < readRequiredNumberFact(counts, params)) {
    plan = plan.concat([callableElement(starts, counts, targets, argumentsNode, at), callableElement(starts, counts, targets, params, at)]);
    at += 1;
  }
  return plan;
}

/** @param {string} kind @param {string} typeKind @param {string} operation @returns {boolean} */
export function requireCallableStructuralInference(kind, typeKind, operation) {
  if (kind === 'lambda' || kind === 'functionValue' || typeKind === 'function') return true;
  if (kind === 'stringTest' && operation === 'includes') return true;
  if (kind === 'none' || kind === 'coalesce' || kind === 'optionalField' || kind === 'record' || kind === 'method' || kind === 'map' || kind === 'set' || kind === 'arrayFrom' || kind === 'arrayIsArray' || kind === 'copySlice' || kind === 'forOfIterable' || kind === 'stringPoints' || kind === 'cellAllocate' || kind === 'cellRead' || kind === 'cellWrite' || kind === 'referenceWrite' || kind === 'index' || kind === 'tuple' || kind === 'tupleIndex') return true;
  return typeKind === 'option' || typeKind === 'tuple' || typeKind === 'record' || typeKind === 'map' || typeKind === 'set';
}

/** @param {string} headKind @param {number} pathLength @param {boolean} localPresent @returns {boolean} */
export function requireCallableNamedInference(headKind, pathLength, localPresent) {
  return headKind === 'name' && (pathLength !== 1 || !localPresent);
}

/** @param {string} kind @param {number} parameters @param {number} argumentsCount @returns {boolean} */
export function acceptCallableApplicationInferenceResult(kind, parameters, argumentsCount) {
  return kind === 'function' && parameters === argumentsCount;
}

/**
 * Every parameter is owned by its actual lambda and ordinal. The same
 * descriptor shared by two binders remains the same identity for validation.
 * @param {number[]} starts @param {number[]} counts @param {string[]} labels
 * @param {number[]} targets @param {number} owner @returns {number[]}
 */
export function planCallableParameterOwnership(starts, counts, labels, targets, owner) {
  const parameters = callableField(starts, counts, labels, targets, owner, 'params');
  let plan = [];
  let at = 0;
  while (parameters >= 0 && at < readRequiredNumberFact(counts, parameters)) {
    plan = plan.concat([owner, at, callableElement(starts, counts, targets, parameters, at)]);
    at += 1;
  }
  return plan;
}

/** @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} type @returns {number[]} */
export function readCallableFunctionChildren(starts, counts, labels, targets, type) {
  const params = callableField(starts, counts, labels, targets, type, 'params');
  let children = [];
  let at = 0;
  while (params >= 0 && at < readRequiredNumberFact(counts, params)) {
    children = children.concat([callableElement(starts, counts, targets, params, at)]);
    at += 1;
  }
  return children.concat([callableField(starts, counts, labels, targets, type, 'ret')]);
}

/**
 * The actual lambda supplies explicit types first, then the contextual
 * function's slots, then fresh-variable requests (-1). The result starts
 * [return identity, explicit-return flag] followed by parameter triples.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} lambda @param {number} expected @returns {number[]}
 */
export function planCallableLambdaInference(forms, texts, starts, counts, labels, targets, lambda, expected) {
  const explicit = callableField(starts, counts, labels, targets, lambda, 'ret');
  const present = callableTruthy(forms, texts, explicit);
  const contextual = callableTag(texts, starts, counts, labels, targets, expected, 'kind') === 'function';
  const ret = present ? explicit : contextual ? callableField(starts, counts, labels, targets, expected, 'ret') : -1;
  const params = planCallableParameterTypes(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, lambda, 'params'), expected);
  return [ret, present ? 1 : 0].concat(params);
}

/** @param {boolean} expression @param {string} tokenKind @param {string} tokenValue @returns {string} */
export function readCallableClosurePrefixError(expression, tokenKind, tokenValue) {
  if (expression && tokenKind === 'identifier') return 'named function expression';
  return tokenValue === '*' ? 'generator function' : '';
}

/** @param {string[]} names @param {boolean[]} defaults @returns {string} */
export function readCallableClosureParameterError(names, defaults) {
  let at = 0;
  while (at < names.length) {
    let previous = 0;
    while (previous < at) {
      if (readRequiredTextFact(names, previous) === readRequiredTextFact(names, at)) return 'duplicate closure parameter';
      previous += 1;
    }
    at += 1;
  }
  at = 0;
  while (at < defaults.length) {
    if (readRequiredBooleanFact(defaults, at)) return 'closure parameter default';
    at += 1;
  }
  return '';
}

/** @param {string[]} texts @param {number[]} identities @param {number} identity @returns {number[]} */
function callableAppendName(texts, identities, identity) {
  let at = 0;
  while (at < identities.length) {
    if (callableText(texts, readRequiredNumberFact(identities, at)) === callableText(texts, identity)) return identities;
    at += 1;
  }
  return identities.concat([identity]);
}

/**
 * Capture selection uses the actual outer lexical names, preserving the
 * original assigned/free insertion order and excluding declared locals.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} statements @param {string[]} bound @param {string[]} outer
 * @returns {number[]}
 */
export function planCallableStatementCaptures(forms, texts, starts, counts, labels, targets, statements, bound, outer) {
  const events = readCallableStatementUses(forms, texts, starts, counts, labels, targets, statements, bound);
  let assigned = [];
  let free = [];
  let at = 0;
  while (at < events.length) {
    const name = readRequiredNumberFact(events, at + 1);
    if (callableBound(outer, callableText(texts, name))) {
      if (readRequiredNumberFact(events, at) === 0) free = callableAppendName(texts, free, name);
      if (readRequiredNumberFact(events, at) === 1) assigned = callableAppendName(texts, assigned, name);
    }
    at += 2;
  }
  return [assigned.length].concat(assigned, free);
}

/**
 * Scope slots are actual scalar name identities. Output starts with the
 * three lengths, followed by locals, remaining TDZ names and mutable names.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} outer
 * @param {number} tokens @returns {number[]}
 */
export function planCallableClosureScope(texts, starts, counts, labels, targets, outer, tokens) {
  const oldLocals = callableField(starts, counts, labels, targets, outer, 'locals');
  const oldTemporal = callableField(starts, counts, labels, targets, outer, 'tdz');
  const oldMutable = callableField(starts, counts, labels, targets, outer, 'mutable');
  let locals = [];
  let temporal = [];
  let mutable = [];
  let names = [];
  let at = 0;
  while (tokens >= 0 && at < readRequiredNumberFact(counts, tokens)) {
    const token = callableElement(starts, counts, targets, tokens, at);
    names = names.concat([callableText(texts, callableField(starts, counts, labels, targets, token, 'value'))]);
    at += 1;
  }
  at = 0;
  while (oldLocals >= 0 && at < readRequiredNumberFact(counts, oldLocals)) {
    locals = callableAppendName(texts, locals, callableElement(starts, counts, targets, oldLocals, at));
    at += 1;
  }
  at = 0;
  while (oldTemporal >= 0 && at < readRequiredNumberFact(counts, oldTemporal)) {
    const name = callableElement(starts, counts, targets, oldTemporal, at);
    if (!callableBound(names, callableText(texts, name))) temporal = temporal.concat([name]);
    at += 1;
  }
  at = 0;
  while (oldMutable >= 0 && at < readRequiredNumberFact(counts, oldMutable)) {
    mutable = callableAppendName(texts, mutable, callableElement(starts, counts, targets, oldMutable, at));
    at += 1;
  }
  at = 0;
  while (tokens >= 0 && at < readRequiredNumberFact(counts, tokens)) {
    const token = callableElement(starts, counts, targets, tokens, at);
    const name = callableField(starts, counts, labels, targets, token, 'value');
    locals = callableAppendName(texts, locals, name);
    mutable = callableAppendName(texts, mutable, name);
    at += 1;
  }
  return [locals.length, temporal.length, mutable.length].concat(locals, temporal, mutable);
}

/**
 * Surface parameter records retain exact token spelling and UTF-16 bounds.
 * Type:null is the original explicit inference annotation.
 * @param {number[]} starts @param {number[]} counts @param {string[]} labels
 * @param {number[]} targets @param {number} tokens @returns {number[]}
 */
export function planCallableSurfaceParameters(starts, counts, labels, targets, tokens) {
  let plan = [];
  let at = 0;
  while (tokens >= 0 && at < readRequiredNumberFact(counts, tokens)) {
    const token = callableElement(starts, counts, targets, tokens, at);
    plan = plan.concat([callableField(starts, counts, labels, targets, token, 'value'), callableField(starts, counts, labels, targets, token, 'start'), callableField(starts, counts, labels, targets, token, 'end')]);
    at += 1;
  }
  return plan;
}

/** @param {number} count @returns {boolean} */
export function requireCallableAssignedCaptureField(count) {
  return count > 0;
}

/** @param {string} form @param {boolean} knownFunction @returns {string} */
export function readCallableSurfaceConstructor(form, knownFunction) {
  if (form === 'closure') return 'lambda';
  if (form === 'computed-call') return 'app';
  return form === 'name' && knownFunction ? 'functionValue' : '';
}

/** @param {string} format @param {string} fn @returns {string} */
export function renderCallableFunctionReference(format, fn) {
  return format === 'Rust' ? 'std::rc::Rc::new(' + fn + ')' : fn;
}

/** @param {string} format @param {string} fn @param {string[]} args @returns {string} */
export function renderCallableApplication(format, fn, args) {
  return format === 'Rust' || format === 'JavaScript' ? '(' + fn + ')(' + args.join(', ') + ')' : '(' + fn + ' ' + args.join(' ') + ')';
}

/** @param {string} format @param {string[]} names @param {string[]} types @param {string} ret @param {string} body @param {string[]} copies @returns {string} */
export function renderCallableLambda(format, names, types, ret, body, copies) {
  if (format === 'JavaScript') return '((' + names.join(', ') + ') => ' + body + ')';
  const parameters = renderCallableParameters(format, names, types);
  if (format !== 'Rust') return '(fun ' + parameters.join(' ') + ' => ' + body + ')';
  const closure = 'std::rc::Rc::new(move |' + parameters.join(', ') + '| -> ' + ret + ' { ' + body + ' })';
  return copies.length ? '{ ' + copies.join(' ') + ' ' + closure + ' }' : closure;
}

/** @param {string} format @param {string[]} names @param {string[]} types @returns {string[]} */
function renderCallableParameters(format, names, types) {
  let parameters = [];
  let at = 0;
  while (at < names.length) {
    parameters = parameters.concat([format === 'Rust' ? readRequiredTextFact(names, at) + ': ' + readRequiredTextFact(types, at) : '(' + readRequiredTextFact(names, at) + ' : ' + readRequiredTextFact(types, at) + ')']);
    at += 1;
  }
  return parameters;
}

/** @param {string} format @param {string[]} parameters @param {string} ret @returns {string} */
export function renderCallableFunctionType(format, parameters, ret) {
  return '(' + parameters.concat([ret]).join(format === 'Lean' ? ' → ' : ' -> ') + ')';
}

/**
 * Mutable cells are the ordered intersection of actual captured and assigned
 * source names. This walk enters nested closures, unlike capture-conflict
 * validation, and ignores metadata and the capture lists after reading them.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} node @returns {number[]}
 */
export function planCallableMutableCells(forms, texts, starts, counts, labels, targets, node) {
  const events = callableCellObservations(forms, texts, starts, counts, labels, targets, node);
  let captures = [];
  let assigned = [];
  let at = 0;
  while (at < events.length) {
    if (readRequiredNumberFact(events, at) === 0) captures = callableAppendName(texts, captures, readRequiredNumberFact(events, at + 1));
    else assigned = assigned.concat([callableText(texts, readRequiredNumberFact(events, at + 1))]);
    at += 2;
  }
  let cells = [];
  at = 0;
  while (at < captures.length) {
    if (callableBound(assigned, callableText(texts, readRequiredNumberFact(captures, at)))) cells = cells.concat([readRequiredNumberFact(captures, at)]);
    at += 1;
  }
  return cells;
}

/** @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @returns {number[]} */
function callableCellObservations(forms, texts, starts, counts, labels, targets, node) {
  if (node < 0 || readRequiredTextFact(forms, node) !== 'object' && readRequiredTextFact(forms, node) !== 'array') return [];
  let events = [];
  if (callableTag(texts, starts, counts, labels, targets, node, 's') === 'assign') events = [1, callableField(starts, counts, labels, targets, node, 'name')];
  if (callableTag(texts,starts,counts,labels,targets,node,'k') === 'cellWrite') {
    const cell = callableField(starts,counts,labels,targets,node,'cell');
    const path = callableField(starts,counts,labels,targets,cell,'path');
    if (callableTag(texts,starts,counts,labels,targets,cell,'k') === 'name' && path >= 0 && readRequiredNumberFact(counts,path) === 1) {
      const name = callableElement(starts,counts,targets,path,0);
      events = events.concat([0,name,1,name]);
    }
  }
  if (callableTag(texts, starts, counts, labels, targets, node, 'k') === 'lambda') {
    const captured = callableField(starts, counts, labels, targets, node, 'captures');
    const assigned = callableField(starts, counts, labels, targets, node, 'assignedCaptures');
    let at = 0;
    while (captured >= 0 && at < readRequiredNumberFact(counts, captured)) {
      events = events.concat([0, callableElement(starts, counts, targets, captured, at)]);
      at += 1;
    }
    at = 0;
    while (assigned >= 0 && at < readRequiredNumberFact(counts, assigned)) {
      events = events.concat([1, callableElement(starts, counts, targets, assigned, at)]);
      at += 1;
    }
  }
  let at = readRequiredNumberFact(starts, node);
  while (at < readRequiredNumberFact(starts, node) + readRequiredNumberFact(counts, node)) {
    const label = readRequiredTextFact(labels, at);
    if (label !== 'span' && label !== 'type' && label !== 'data' && label !== 'captures' && label !== 'assignedCaptures') events = events.concat(callableCellObservations(forms, texts, starts, counts, labels, targets, readRequiredNumberFact(targets, at)));
    at += 1;
  }
  return events;
}

/** @param {string} operation @param {boolean} reference @returns {string} */
export function readCallableCellLowering(operation, reference) {
  if (!reference) return '';
  if (operation === 'read') return 'cellRead';
  if (operation === 'assign') return 'cellWrite';
  return operation === 'declare' ? 'cellAllocate' : '';
}

/** @param {string} source @param {boolean} reference @returns {string} */
export function renderCallableCellParameterName(source, reference) {
  return reference ? 'ml_cell_' + source : source;
}

/** @param {string} kind @param {string} operation @returns {string} */
export function readCallableCellError(kind, operation) {
  if (kind === 'cell') return '';
  return operation === 'cellRead' ? 'reading a value requires a reference cell' : 'writing a value requires a reference cell';
}

/**
 * Explicit allocation annotations precede contextual element types. A missing
 * type stays -1; the checker then uses the actual initializer's type.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} node @param {number} expected @returns {number[]}
 */
export function planCallableCellElement(forms, texts, starts, counts, labels, targets, node, expected) {
  const type = callableField(starts, counts, labels, targets, node, 'type');
  const element = callableField(starts, counts, labels, targets, type, 'element');
  return callableTruthy(forms, texts, element) ? [element, 1] : [callableField(starts, counts, labels, targets, expected, 'element'), 0];
}

/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} cell @param {number} resolved @returns {number[]} */
export function planCallableCellReadInference(texts, starts, counts, labels, targets, cell, resolved) {
  return callableTag(texts, starts, counts, labels, targets, resolved, 'kind') === 'cell' ? [callableField(starts, counts, labels, targets, resolved, 'element')] : [-1, cell];
}

/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} resolved @param {number} value @returns {number[]} */
export function planCallableCellOptionBinding(texts, starts, counts, labels, targets, resolved, value) {
  const element = callableField(starts, counts, labels, targets, resolved, 'element');
  if (callableTag(texts, starts, counts, labels, targets, resolved, 'kind') !== 'cell' || callableTag(texts, starts, counts, labels, targets, element, 'kind') !== 'var' || callableTag(texts, starts, counts, labels, targets, value, 'kind') !== 'option') return [];
  return [callableField(starts, counts, labels, targets, element, 'id'), value];
}

/** @param {string} format @param {string} operation @param {string} cell @param {string} value @param {string} body @param {string} cellName @param {string} valueName @returns {string} */
export function renderCallableCellOperation(format, operation, cell, value, body, cellName, valueName) {
  if (format === 'JavaScript') {
    if (operation === 'cellAllocate') return '({ value: ' + value + ' })';
    return operation === 'cellRead' ? '(' + cell + ').value' : '((' + cell + ').value = ' + value + ', ' + body + ')';
  }
  if (operation === 'cellAllocate') return 'std::rc::Rc::new(std::cell::RefCell::new(' + value + '))';
  if (operation === 'cellRead') return '{ let ml_cell_value = ' + cell + '.borrow().clone(); ml_cell_value }';
  return '{ let ' + cellName + ' = ' + cell + '; let ' + valueName + ' = ' + value + '; *' + cellName + '.borrow_mut() = ' + valueName + '; ' + body + ' }';
}

/** @param {boolean} temporal @param {boolean} local @returns {string} */
export function readCallableReferenceObligation(temporal, local) {
  if (temporal) return 'the binding is in its temporal dead zone, where reading it throws a ReferenceError';
  return local ? '' : 'functions are only portable when called';
}

/** @param {string} name @param {boolean} temporal @returns {string} */
export function readCallableReferenceConstruct(name, temporal) {
  return temporal ? 'use of ' + name + ' before its declaration' : 'function value ' + name;
}

/** @param {number} field @returns {string} */
export function readCallableRenamingField(field) {
  if (field === 0) return 'path';
  if (field === 1) return 'params';
  if (field === 2) return 'body';
  if (field === 3) return 'cell';
  if (field === 4) return 'value';
  if (field === 5) return 'default';
  if (field === 6) return 'k';
  return field === 7 ? 'span' : '';
}

/** @param {number} constant @returns {string} */
export function readCallableRenamingConstant(constant) {
  return constant === 0 ? 'cellRead' : '';
}

/**
 * Reconstruction opcodes: 0 copies an actual value, 1 constructs an array,
 * 2 copies an object and applies field plans, 3 reads a shared constant.
 * Scope triples are [source name identity, target name identity, cell flag].
 * Negative field codes select the shared names above; other field codes are
 * actual edge positions. No host visitor decides scopes or cell operations.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} node @param {number} scope @returns {number[]}
 */
export function planCallableSourceRenaming(forms, texts, starts, counts, labels, targets, node, scope) {
  let bindings = [];
  let at = 0;
  while (scope >= 0 && at < readRequiredNumberFact(counts, scope)) {
    const binding = callableElement(starts, counts, targets, scope, at);
    const source = callableField(starts, counts, labels, targets, binding, 'source');
    const target = callableField(starts, counts, labels, targets, binding, 'ir');
    const reference = callableTruthy(forms, texts, callableField(starts, counts, labels, targets, binding, 'reference'));
    bindings = bindings.concat([source, target, reference ? 1 : 0]);
    at += 1;
  }
  return callableRenamedNode(forms, texts, starts, counts, labels, targets, node, bindings, false);
}

/** @param {string[]} texts @param {number[]} bindings @param {number} name @returns {number[]} */
function callableRenamingBinding(texts, bindings, name) {
  let at = bindings.length;
  while (at > 0) {
    at -= 3;
    if (callableText(texts, readRequiredNumberFact(bindings, at)) === callableText(texts, name)) return [readRequiredNumberFact(bindings, at + 1), readRequiredNumberFact(bindings, at + 2)];
  }
  return [name, 0];
}

/** @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {number[]} bindings @param {boolean} carrier @returns {number[]} */
function callableRenamedNode(forms, texts, starts, counts, labels, targets, node, bindings, carrier) {
  if (node < 0 || readRequiredTextFact(forms, node) !== 'object' && readRequiredTextFact(forms, node) !== 'array') return [0, node];
  if (readRequiredTextFact(forms, node) === 'array') {
    let plan = [1, readRequiredNumberFact(counts, node)];
    let at = 0;
    while (at < readRequiredNumberFact(counts, node)) {
      plan = plan.concat(callableRenamedNode(forms, texts, starts, counts, labels, targets, callableElement(starts, counts, targets, node, at), bindings, false));
      at += 1;
    }
    return plan;
  }
  const kind = callableTag(texts, starts, counts, labels, targets, node, 'k');
  const path = callableField(starts, counts, labels, targets, node, 'path');
  if (kind === 'name' && path >= 0 && readRequiredNumberFact(counts, path) === 1) {
    const binding = callableRenamingBinding(texts, bindings, callableElement(starts, counts, targets, path, 0));
    const renamed = [2, node, 1, -1, 1, 1, 0, readRequiredNumberFact(binding, 0)];
    if (carrier || readRequiredNumberFact(binding, 1) === 0) return renamed;
    const span = callableField(starts, counts, labels, targets, node, 'span');
    return [2, -1, 3, -7, 3, 0, -4].concat(renamed, [-8, 0, span]);
  }
  if (kind === 'cellRead' || kind === 'cellWrite') {
    const cell = callableField(starts, counts, labels, targets, node, 'cell');
    const value = callableField(starts, counts, labels, targets, node, 'value');
    const body = callableField(starts, counts, labels, targets, node, 'body');
    const hasValue = callableTruthy(forms, texts, value);
    const hasBody = callableTruthy(forms, texts, body);
    let plan = [2, node, 1 + (hasValue ? 1 : 0) + (hasBody ? 1 : 0), -4].concat(callableRenamedNode(forms, texts, starts, counts, labels, targets, cell, bindings, true));
    if (hasValue) plan = plan.concat([-5], callableRenamedNode(forms, texts, starts, counts, labels, targets, value, bindings, false));
    if (hasBody) plan = plan.concat([-3], callableRenamedNode(forms, texts, starts, counts, labels, targets, body, bindings, false));
    return plan;
  }
  if (kind === 'lambda') {
    const params = callableField(starts, counts, labels, targets, node, 'params');
    let inner = bindings;
    let parameters = [1, readRequiredNumberFact(counts, params)];
    let at = 0;
    while (at < readRequiredNumberFact(counts, params)) {
      const parameter = callableElement(starts, counts, targets, params, at);
      const name = callableField(starts, counts, labels, targets, parameter, 'name');
      inner = inner.concat([name, name, 0]);
      const value = callableField(starts, counts, labels, targets, parameter, 'default');
      parameters = parameters.concat(callableTruthy(forms, texts, value)
        ? [2, parameter, 1, -6].concat(callableRenamedNode(forms, texts, starts, counts, labels, targets, value, bindings, false)) : [2, parameter, 0]);
      at += 1;
    }
    return [2, node, 2, -2].concat(parameters, [-3], callableRenamedNode(forms, texts, starts, counts, labels, targets, callableField(starts, counts, labels, targets, node, 'body'), inner, false));
  }
  let plan = [2, node, readRequiredNumberFact(counts, node)];
  let at = readRequiredNumberFact(starts, node);
  while (at < readRequiredNumberFact(starts, node) + readRequiredNumberFact(counts, node)) {
    const label = readRequiredTextFact(labels, at);
    const value = readRequiredNumberFact(targets, at);
    plan = plan.concat([at], label === 'span' || label === 'type' || label === 'data' ? [0, value] : callableRenamedNode(forms, texts, starts, counts, labels, targets, value, bindings, false));
    at += 1;
  }
  return plan;
}
