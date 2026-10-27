// Closed ordinary classes lower to the existing shared object/callable model.
// These are inspectable source transformations, not execution of user code.
// A profile refusal leaves the complete original class available to the caller.

/** @param {string[]} values @param {number} open @returns {number} */
export function findJavaScriptClassGroupEnd(values, open) {
  const first = values[open] ?? '';
  const last = first === '(' ? ')' : first === '[' ? ']' : first === '{' ? '}' : '';
  if (last === '') return -1;
  let expected = [last];
  for (let at = open + 1; at < values.length && at - open <= 8192; at += 1) {
    const value = values[at] ?? '';
    const closing = value === '(' ? ')' : value === '[' ? ']' : value === '{' ? '}' : '';
    if (closing !== '') {
      if (expected.length >= 128) return -1;
      expected = expected.concat([closing]);
    } else if (value === ')' || value === ']' || value === '}') {
      if (value !== (expected[expected.length - 1] ?? '')) return -1;
      expected = expected.slice(0, -1);
      if (expected.length === 0) return at;
    }
  }
  return -1;
}

/** Ordinary BoundNames and literal defaults retain the original initializer
 * source; defaults execute at invocation, never while installing a method.
 * @param {string[]} values @param {string[]} kinds @param {number} open
 * @param {number} close @returns {number[]} */
export function readJavaScriptClosedClassParameterPositions(values, kinds, open, close) {
  let result = [];
  for (let at = open + 1; at < close;) {
    if ((kinds[at] ?? '') !== 'identifier') return [-1, at];
    result = result.concat([at]);
    at += 1;
    if ((values[at] ?? '') === '=') {
      at += 1;
      const value = values[at] ?? '';
      if (value === '+' || value === '-') {
        at += 1;
        if ((kinds[at] ?? '') !== 'number' && !['NaN', 'Infinity'].includes(values[at] ?? '')) return [-1, at];
        at += 1;
      } else if ((kinds[at] ?? '') === 'number' || (kinds[at] ?? '') === 'string'
        || ['true', 'false', 'null', 'undefined', 'NaN', 'Infinity'].includes(value)) at += 1;
      else if ((value === '[' && (values[at + 1] ?? '') === ']') || (value === '{' && (values[at + 1] ?? '') === '}')) at += 2;
      else return [-1, at];
    }
    if (at < close) {
      if ((values[at] ?? '') !== ',') return [-1, at];
      at += 1;
    }
  }
  return result;
}

/** @param {string} source @param {string[]} values @param {number[]} starts
 * @param {number[]} ends @param {number} open @param {number} close
 * @returns {string} */
export function renderJavaScriptClosedClassParameterArguments(source, values, starts, ends, open, close) {
  let result = [];
  for (let at = open + 1; at < close;) {
    result = result.concat([source.slice(starts[at] ?? 0, ends[at] ?? 0)]);
    at += 1;
    if ((values[at] ?? '') === '=') {
      at += 1;
      const value = values[at] ?? '';
      if (value === '[' || value === '{') at += 2;
      else if (value === '+' || value === '-') at += 2;
      else at += 1;
    }
    if (at < close) at += 1;
  }
  return result.join(', ');
}

/** A non-instance constructor prologue may validate inputs and bind locals.
 * Only a complete top-level assignment boundary can begin initial fields.
 * A nested/earlier receiver use preserves the old whole-body route; it never
 * hoists a conditional field creation or splits an incomplete statement.
 * @param {string[]} values @param {number} first @param {number} last
 * @returns {number} */
export function findJavaScriptClosedClassInitializationStart(values, first, last) {
  let depth = 0;
  for (let at = first; at < last; at += 1) {
    const value = values[at] ?? '';
    const previous = values[at - 1] ?? '';
    if (value === 'this' && previous !== '.' && previous !== '?.') {
      const boundary = at === first || previous === ';' || previous === '}';
      return depth === 0 && boundary && (values[at + 1] ?? '') === '.'
        && (values[at + 3] ?? '') === '=' ? at : first;
    }
    if (value === '(' || value === '[' || value === '{') depth += 1;
    if (value === ')' || value === ']' || value === '}') depth -= 1;
  }
  return last;
}

/** A directory row is [class token, end, constructor name/parameters/body,
 * field count, initialization start, then field-name/value-start/value-end triples, then method
 * name/parameter-open/parameter-close/body-open/body-close quintuples].
 * A static member stores -(name token + 1); its actual token coordinate is
 * retained without changing any constructor/field/member interval.
 * Offsets refer to actual tokens, never reconstructed identifier spellings.
 * @param {string[]} values @param {string[]} kinds @param {number} first
 * @returns {number[]} */
export function readJavaScriptClosedClassDirectory(values, kinds, first) {
  if ((values[first] ?? '') !== 'class' || (kinds[first + 1] ?? '') !== 'identifier'
    || (values[first + 2] ?? '') !== '{') return [-1, first];
  const end = findJavaScriptClassGroupEnd(values, first + 2);
  if (end < 0) return [-1, first + 2];
  let constructor = [-1, -1, -1, -1, -1];
  let methods = [];
  let fields = [];
  let names = [];
  for (let at = first + 3; at < end;) {
    if ((values[at] ?? '') === ';') { at += 1; continue; }
    const staticMember = (values[at] ?? '') === 'static' && (kinds[at + 1] ?? '') === 'identifier'
      && (values[at + 2] ?? '') === '(';
    const position = staticMember ? at + 1 : at;
    const name = values[position] ?? '';
    const owner = (staticMember ? 'static:' : 'instance:') + name;
    if ((kinds[position] ?? '') !== 'identifier' || name === 'static' || name === 'get'
      || name === 'set' || name === 'async' || name === '__proto__'
      || name === 'prototype' || name === '$' || staticMember && name === 'constructor'
      || names.includes(owner) || (values[position + 1] ?? '') !== '(') return [-2, at];
    const parameters = findJavaScriptClassGroupEnd(values, position + 1);
    if (parameters < 0 || (values[parameters + 1] ?? '') !== '{') return [-2, at];
    const positions = readJavaScriptClosedClassParameterPositions(values, kinds, position + 1, parameters);
    if ((positions[0] ?? 0) < 0) return [-3, positions[1] ?? at];
    const body = findJavaScriptClassGroupEnd(values, parameters + 1);
    if (body < 0 || body > end) return [-2, at];
    names = names.concat([owner]);
    if (name === 'constructor' && !staticMember) constructor = [position, position + 1, parameters, parameters + 1, body];
    else methods = methods.concat([staticMember ? -position - 1 : position, position + 1, parameters, parameters + 1, body]);
    at = body + 1;
  }
  const bodyStart = constructor[3] ?? -1;
  const bodyEnd = constructor[4] ?? -1;
  const initialization = bodyStart < 0 ? -1 : findJavaScriptClosedClassInitializationStart(values, bodyStart + 1, bodyEnd);
  let cursor = initialization;
  while (bodyStart >= 0 && cursor < bodyEnd && (values[cursor] ?? '') === 'this'
    && (values[cursor + 1] ?? '') === '.' && (kinds[cursor + 2] ?? '') === 'identifier'
    && (values[cursor + 3] ?? '') === '=') {
    const name = values[cursor + 2] ?? '';
    if (name === '$' || names.includes('instance:' + name)) return [-4, cursor + 2];
    for (let previous = 0; previous < fields.length; previous += 3)
      if ((values[fields[previous] ?? 0] ?? '') === name) return [-4, cursor + 2];
    const start = cursor + 4;
    let finish = start;
    for (; finish < bodyEnd; finish += 1) {
      const value = values[finish] ?? '';
      if (value === ';') break;
      if (value === ',') return [-5, finish];
      if (value === 'this' && (values[finish - 1] ?? '') !== '.' && (values[finish - 1] ?? '') !== '?.') return [-5, finish];
      if (value === '(' || value === '[' || value === '{') {
        const group = findJavaScriptClassGroupEnd(values, finish);
        if (group < 0 || group >= bodyEnd) return [-5, finish];
        for (let inner = finish + 1; inner < group; inner += 1)
          if ((values[inner] ?? '') === 'this' && (values[inner - 1] ?? '') !== '.' && (values[inner - 1] ?? '') !== '?.') return [-5, inner];
        finish = group;
      }
    }
    if (finish === start || finish >= bodyEnd) return [-5, start];
    fields = fields.concat([cursor + 2, start, finish]);
    cursor = finish + 1;
  }
  if (bodyStart >= 0) for (let at = bodyStart + 1; at < bodyEnd; at += 1)
    if ((values[at] ?? '') === 'return') return [-6, at];
  return [first, end, ...constructor, fields.length / 3, initialization, ...fields, ...methods];
}

/** @param {number} code @returns {string} */
export function readJavaScriptClosedClassRefusal(code) {
  if (code === -1) return 'a closed class needs an ordinary named base-class declaration and balanced source groups';
  if (code === -2) return 'class fields, accessors, private names, inheritance, static initialization and duplicate member owners need their full descriptor model';
  if (code === -3) return 'this class profile requires ordinary fixed identifier parameters with literal defaults; patterns and dependent or instance-reading defaults need their complete parameter environment';
  if (code === -4) return 'initial own fields must be distinct and cannot replace a prototype method';
  if (code === -5) return 'initial own fields need a complete semicolon-terminated assignment prefix without instance escape or reads before allocation';
  if (code === -6) return 'constructor return overrides need their complete object replacement semantics';
  if (code === -7) return 'class values, reflective operations, detached methods, method replacement and observable prototypes need the full object model';
  if (code === -8) return 'closed class authoring exceeds its explicit source, token, nesting or class-count budget';
  if (code === -9) return 'class construction, receiver and member modifier keywords require their actual unescaped ECMAScript source spelling';
  return '';
}

/** @param {string} source @param {string[]} values @param {number[]} starts
 * @param {number[]} ends @param {number} first @param {number} last
 * @param {string} receiver @param {string[]} classNames @param {string[]} staticCalls @returns {string} */
export function renderJavaScriptClosedClassRange(source, values, starts, ends, first, last, receiver, classNames, staticCalls) {
  if (first >= last) return '';
  let result = '';
  let previous = starts[first] ?? 0;
  for (let at = first; at < last; at += 1) {
    const value = values[at] ?? '';
    const target = readJavaScriptClosedClassStaticTarget(values, classNames, staticCalls, at);
    if (target !== '') {
      result += source.slice(previous, starts[at] ?? previous) + target;
      previous = ends[at + 2] ?? previous;
      at += 2;
      continue;
    }
    const thisReference = receiver !== '' && value === 'this' && (values[at - 1] ?? '') !== '.' && (values[at - 1] ?? '') !== '?.';
    const construction = value === 'new' && classNames.includes(values[at + 1] ?? '') && (values[at + 2] ?? '') === '(';
    if (thisReference || construction) {
      result += source.slice(previous, starts[at] ?? previous) + (thisReference ? receiver : '');
      previous = ends[at] ?? previous;
    }
  }
  return result + source.slice(previous, ends[last - 1] ?? previous);
}

/** @param {string[]} values @param {number[]} directory @returns {string[]} */
export function readJavaScriptClosedClassMethods(values, directory) {
  let result = [];
  const first = 9 + 3 * (directory[7] ?? 0);
  for (let at = first; at < directory.length; at += 5) if ((directory[at] ?? -1) >= 0) result = result.concat([values[directory[at] ?? 0] ?? '']);
  return result;
}

/** @param {string[]} values @param {number[][]} directory @param {string} prefix
 * @returns {string[]} */
export function readJavaScriptClosedClassStaticCalls(values, directory, prefix) {
  let result = [];
  let ordinal = 0;
  for (const row of directory) {
    const name = values[(row[0] ?? 0) + 1] ?? '';
    for (let at = 9 + 3 * (row[7] ?? 0); at < row.length; at += 5) {
      const position = row[at] ?? 0;
      if (position < 0) result = result.concat([name + '.' + (values[-position - 1] ?? ''),
        prefix + 'class_static_' + String(ordinal) + '_' + String(at)]);
    }
    ordinal += 1;
  }
  return result;
}

/** @param {string[]} values @param {string[]} names @param {string[]} calls
 * @param {number} at @returns {string} */
export function readJavaScriptClosedClassStaticTarget(values, names, calls, at) {
  const previous = values[at - 1] ?? '';
  if (!names.includes(values[at] ?? '') || previous === 'new' || previous === '.' || previous === '?.'
    || (values[at + 1] ?? '') !== '.' || (values[at + 3] ?? '') !== '(') return '';
  const key = (values[at] ?? '') + '.' + (values[at + 2] ?? '');
  for (let index = 0; index < calls.length; index += 2) if ((calls[index] ?? '') === key) return calls[index + 1] ?? '';
  return '';
}

/** The profile is deliberately closed: its synthetic method slots cannot be
 * observed as own enumerable fields or as bound detached function objects.
 * Calls and shared state retain ordinary source receiver/argument order.
 * @param {string[]} values @param {string[]} kinds @param {number[][]} directory @param {string[]} staticCalls
 * @returns {number} */
export function readJavaScriptClosedClassObservationError(values, kinds, directory, staticCalls) {
  let names = [];
  let methods = [];
  for (const row of directory) {
    names = names.concat([values[(row[0] ?? 0) + 1] ?? '']);
    methods = methods.concat(readJavaScriptClosedClassMethods(values, row));
    for (let at = 9 + 3 * (row[7] ?? 0); at < row.length; at += 5) if ((row[at] ?? 0) < 0) {
      for (let inner = (row[at + 3] ?? 0) + 1; inner < (row[at + 4] ?? 0); inner += 1)
        if ((values[inner] ?? '') === 'this' && (values[inner - 1] ?? '') !== '.' && (values[inner - 1] ?? '') !== '?.') return inner;
    }
  }
  for (let at = 0; at < values.length; at += 1) {
    const value = values[at] ?? '';
    const kind = kinds[at] ?? '';
    const previous = values[at - 1] ?? '';
    const next = values[at + 1] ?? '';
    if (kind === 'identifier' && ['Object', 'Reflect', 'JSON', 'Symbol', 'Proxy', 'Function', 'eval',
      'arguments', 'super', 'export', 'deepEqual', 'notDeepEqual', 'deepStrictEqual', 'notDeepStrictEqual'].includes(value)) return at;
    if (value === '...' || value === '__proto__' || value === 'prototype') return at;
    if (value === 'function') {
      for (const row of directory) if (at > (row[0] ?? 0) && at < (row[1] ?? 0)) return at;
    }
    if (kind === 'identifier' && names.includes(value) && previous !== 'class'
      && (previous !== 'new' || next !== '(') && readJavaScriptClosedClassStaticTarget(values, names, staticCalls, at) === '') return at;
    if (value === 'this' && previous !== '.' && previous !== '?.' && next !== '.' && next !== '[') return at;
    if ((previous === '.' || previous === '?.') && (methods.includes(value) && next !== '(' || value === 'constructor')) return at;
    if (previous === '[' && kind === 'string' && methods.includes(value.slice(1))) return at;
  }
  return -1;
}

/** @param {string} source @param {string[]} values @param {number[]} starts
 * @param {number[]} ends @param {number[]} row @param {string} prefix
 * @param {number} ordinal @param {string[]} classNames @param {string[]} staticCalls @returns {string} */
export function renderJavaScriptClosedClass(source, values, starts, ends, row, prefix, ordinal, classNames, staticCalls) {
  const name = values[(row[0] ?? 0) + 1] ?? '';
  const receiver = prefix + 'class_receiver_' + String(ordinal);
  const brand = prefix + 'class_identity_' + String(ordinal);
  const fieldCount = row[7] ?? 0;
  const methods = 9 + fieldCount * 3;
  const parameterFirst = (row[3] ?? -1) + 1;
  const parameterLast = row[4] ?? -1;
  const parameters = parameterLast < 0 ? '' : renderJavaScriptClosedClassRange(source, values, starts, ends, parameterFirst, parameterLast, '', classNames, staticCalls);
  let stubs = '';
  let fields = brand + ': (void 0)';
  for (let at = 9; at < methods; at += 3) {
    const field = values[row[at] ?? 0] ?? '';
    const value = renderJavaScriptClosedClassRange(source, values, starts, ends, row[at + 1] ?? 0, row[at + 2] ?? 0, '', classNames, staticCalls);
    fields += ', ' + field + ': (' + value + ')';
  }
  let assignments = '';
  for (let at = methods; at < row.length; at += 5) {
    const member = row[at] ?? 0;
    const position = member < 0 ? -member - 1 : member;
    const method = values[position] ?? '';
    const args = renderJavaScriptClosedClassRange(source, values, starts, ends, (row[at + 1] ?? 0) + 1, row[at + 2] ?? 0, '', classNames, staticCalls);
    const arguments_ = renderJavaScriptClosedClassParameterArguments(source, values, starts, ends, row[at + 1] ?? 0, row[at + 2] ?? 0);
    const body = renderJavaScriptClosedClassRange(source, values, starts, ends, (row[at + 3] ?? 0) + 1, row[at + 4] ?? 0, member < 0 ? '' : receiver, classNames, staticCalls);
    const stub = prefix + 'class_uninitialized_' + String(ordinal) + '_' + String(at);
    const declaration = member < 0 ? position - 1 : position;
    const documentation = source.slice(ends[declaration - 1] ?? 0, starts[declaration] ?? 0);
    if (member < 0) {
      stubs += '\n' + documentation + '\nfunction ' + prefix + 'class_static_' + String(ordinal) + '_' + String(at) + '(' + args + ') { ' + body + ' }\n';
      continue;
    }
    const environment = prefix + 'class_method_environment_' + String(ordinal) + '_' + String(at);
    stubs += '\n' + documentation + '\nfunction ' + stub + '(' + args + ') { return ' + stub + '(' + arguments_ + '); }\n';
    // The method's lexical environment is the original module, not the
    // constructor's parameter/local scope. Only its explicit private receiver
    // enters this environment factory; defaults still execute on invocation.
    stubs += '\nfunction ' + environment + '(' + receiver + ') { return (' + args + ') => { ' + body + ' }; }\n';
    fields += ', ' + method + ': ' + stub;
    assignments += '\n' + receiver + '.' + method + ' = ' + environment + '(' + receiver + ');\n';
  }
  const constructorBody = row[5] ?? -1;
  const constructorEnd = row[6] ?? -1;
  const initialization = row[8] ?? -1;
  const prologue = constructorBody < 0 ? '' : renderJavaScriptClosedClassRange(source, values, starts, ends,
    constructorBody + 1, initialization, '', classNames, staticCalls);
  const remainder = constructorBody < 0 ? '' : renderJavaScriptClosedClassRange(source, values, starts, ends,
    fieldCount === 0 ? initialization : (row[methods - 1] ?? 0) + 1, constructorEnd, receiver, classNames, staticCalls);
  const constructorName = row[2] ?? -1;
  const documentation = constructorName < 0 ? '' : source.slice(ends[constructorName - 1] ?? 0, starts[constructorName] ?? 0);
  return stubs + '\n' + documentation + '\nfunction ' + name + '(' + parameters + ') {\n' + prologue + '\nconst ' + receiver + ' = { ' + fields + ' };\n'
    + assignments + remainder + '\nreturn ' + receiver + ';\n}\n';
}

/** Complete source plus conservative original intervals for each emitted chunk.
 * A changed class retains its entire original interval, never a fabricated
 * point location. Unchanged chunks retain exact original text coordinates.
 * @param {string} source @param {string[]} values @param {string[]} kinds
 * @param {number[]} starts @param {number[]} ends @param {string} prefix
 * @returns {{text:string, intervals:number[], brands:string[], error:string, at:number, changed:boolean}} */
export function lowerJavaScriptClosedClassSource(source, values, kinds, starts, ends, prefix) {
  if (!acceptJavaScriptClosedClassSourceSize(source.length, values.length)
    || kinds.length !== values.length || starts.length !== values.length || ends.length !== values.length)
    return { text: source, intervals: [], brands: [], error: readJavaScriptClosedClassRefusal(-8), at: 0, changed: false };
  // Literal contents are not delimiters, keywords, or source identifier owners.
  let syntax = [];
  for (let at = 0; at < values.length; at += 1) syntax = syntax.concat([
    (kinds[at] ?? '') === 'identifier' || (kinds[at] ?? '') === 'punct'
      ? values[at] ?? '' : '#' + (values[at] ?? '')]);
  values = syntax;
  let directory = [];
  let names = [];
  let depth = 0;
  let declarationsFinished = false;
  let error = 0;
  let errorAt = 0;
  if (source.length > 131072 || values.length > 8192 || kinds.length !== values.length
    || starts.length !== values.length || ends.length !== values.length) error = -8;
  for (let at = 0; at < values.length && error === 0; at += 1) {
    const value = values[at] ?? '';
    if (value === 'class' && (kinds[at] ?? '') === 'identifier'
      && (values[at - 1] ?? '') !== '.' && (values[at - 1] ?? '') !== '?.'
      && ((kinds[at + 1] ?? '') === 'identifier' || (values[at + 1] ?? '') === '{')) {
      if (directory.length >= 64) { error = -8; errorAt = at; break; }
      if (depth !== 0 || declarationsFinished) { error = -7; errorAt = at; break; }
      const row = readJavaScriptClosedClassDirectory(values, kinds, at);
      if ((row[0] ?? -1) < 0) { error = row[0] ?? -1; errorAt = row[1] ?? at; break; }
      const name = values[at + 1] ?? '';
      if (names.includes(name)) { error = -7; errorAt = at + 1; break; }
      names = names.concat([name]);
      directory = directory.concat([row]);
      at = row[1] ?? at;
      continue;
    }
    if (depth === 0 && value === 'import') {
      let end = at;
      for (; end < values.length && (values[end] ?? '') !== ';'; end += 1) {
        if ((kinds[end] ?? '') === 'string') break;
      }
      if (!['#node:assert', '#node:assert/strict', '#assert', '#assert/strict'].includes(values[end] ?? '')) { error = -7; errorAt = at; break; }
      at = end;
      continue;
    }
    if (depth === 0 && value !== ';' && value !== '#use strict') declarationsFinished = true;
    if (value === '(' || value === '[' || value === '{') depth += 1;
    if (value === ')' || value === ']' || value === '}') depth -= 1;
  }
  if (error === 0 && directory.length > 0) for (let at = 0; at < values.length; at += 1) {
    const value = values[at] ?? '';
    const previous = values[at - 1] ?? '';
    if (previous !== '.' && previous !== '?.' && (value === 'class' || value === 'this'
      || value === 'static' && (kinds[at + 1] ?? '') === 'identifier'
      || value === 'new' && names.includes(values[at + 1] ?? ''))
      && source.slice(starts[at] ?? 0, ends[at] ?? 0) !== value) { error = -9; errorAt = at; break; }
  }
  const staticCalls = readJavaScriptClosedClassStaticCalls(values, directory, prefix);
  if (error === 0 && directory.length > 0) {
    const observation = readJavaScriptClosedClassObservationError(values, kinds, directory, staticCalls);
    if (observation >= 0) { error = -7; errorAt = observation; }
  }
  if (error !== 0) return { text: source, intervals: [], brands: [], error: readJavaScriptClosedClassRefusal(error), at: errorAt, changed: false };
  if (directory.length === 0) return { text: source, intervals: [], brands: [], error: '', at: 0, changed: false };
  let text = '';
  let intervals = [];
  let previous = 0;
  let ordinal = 0;
  let brands = [];
  for (const row of directory) {
    const first = row[0] ?? 0;
    const last = row[1] ?? 0;
    const before = source.slice(previous, starts[first] ?? previous);
    intervals = intervals.concat([text.length, text.length + before.length, previous, starts[first] ?? previous]);
    text += before;
    const replacement = renderJavaScriptClosedClass(source, values, starts, ends, row, prefix, ordinal, names, staticCalls);
    intervals = intervals.concat([text.length, text.length + replacement.length, starts[first] ?? 0, ends[last] ?? 0]);
    text += replacement;
    previous = ends[last] ?? previous;
    brands = brands.concat([prefix + 'class_identity_' + String(ordinal)]);
    ordinal += 1;
  }
  let first = 0;
  while (first < values.length && (starts[first] ?? 0) < previous) first += 1;
  const rest = first >= values.length ? source.slice(previous) : source.slice(previous, starts[first] ?? previous)
    + renderJavaScriptClosedClassRange(source, values, starts, ends, first, values.length, '', names, staticCalls)
    + source.slice(ends[values.length - 1] ?? source.length);
  intervals = intervals.concat([text.length, text.length + rest.length, previous, source.length]);
  text += rest;
  if (text.length > 1048576) return { text: source, intervals: [], brands: [], error: readJavaScriptClosedClassRefusal(-8), at: 0, changed: false };
  return { text, intervals, brands, error: '', at: 0, changed: true };
}

/** @param {number} units @param {number} tokens @returns {boolean} */
export function acceptJavaScriptClosedClassSourceSize(units, tokens) {
  return Number.isSafeInteger(units) && Number.isSafeInteger(tokens)
    && units >= 0 && units <= 131072 && tokens >= 0 && tokens <= 8192;
}

/** @param {boolean} shared @param {number} externals @param {number} constants
 * @param {number} bindings @param {boolean} known @param {boolean} module
 * @returns {string} */
export function readJavaScriptClosedClassContextError(shared, externals, constants, bindings, known, module) {
  return shared && externals === 0 && constants === 0 && bindings === 0 && known && !module ? ''
    : 'the closed profile needs shared reference semantics and the complete local module rather than unproved external consumers';
}

/** A source spelling is insufficient: inspect the actual checked receiver and
 * operation. Unknown indirect lookups cannot expose synthetic slots, even when
 * their source key is computed or a destructuring pattern renames a method.
 * @param {string} kind @param {string} operation @param {boolean} instance
 * @param {boolean} privateField @param {boolean} callable @param {boolean} direct
 * @returns {string} */
export function readJavaScriptClosedClassUseError(kind, operation, instance, privateField, callable, direct) {
  if (!instance) return '';
  if (kind === 'recordField' && !privateField && (!callable || direct)) return '';
  if (kind === 'method' && operation.startsWith('writeRecordField')) return '';
  return 'closed class instances require direct public field operations and receiver-retaining method calls; private slots, detached callables and indirect reflective lookups need the full object model';
}

/** @param {number[]} intervals @param {number} first @param {number} last
 * @returns {number[]} */
export function projectJavaScriptClassSourceInterval(intervals, first, last) {
  let low = -1;
  let high = -1;
  for (let at = 0; at < intervals.length; at += 4) {
    const start = intervals[at] ?? 0;
    const end = intervals[at + 1] ?? 0;
    if (end < first || start > last || start === end) continue;
    const originalStart = intervals[at + 2] ?? 0;
    const originalEnd = intervals[at + 3] ?? 0;
    // Equal length alone cannot establish that a transformed chunk is exact.
    // Whole intervals are conservative even for the unmodified surroundings.
    low = low < 0 ? originalStart : Math.min(low, originalStart);
    high = Math.max(high, originalEnd);
  }
  return low < 0 ? [] : [low, high];
}
