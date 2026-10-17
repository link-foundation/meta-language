import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';
// Pure decisions over mechanical surface facts. The complete module is input
// to the checked translator. No source syntax, lexical binding or type decision
// is delegated to the native fact adapter.

/**
 * @param {string} kind
 * @param {string} value
 * @param {string} following
 * @returns {boolean}
 */
export function acceptSurfaceReferenceClosureToken(kind, value, following) {
  return kind === 'punct' && value === '=>'
    || kind === 'identifier' && value === 'function' && following === '(';
}

/**
 * @param {string} method
 * @param {string[]} methods
 * @returns {boolean}
 */
export function acceptSurfaceReferenceMutatingMethod(method, methods) {
  let index = 0;
  while (index < methods.length) {
    if (readRequiredTextFact(methods, index) === method) return true;
    index += 1;
  }
  return false;
}

/**
 * @param {string[]} window
 * @param {boolean} closure
 * @param {boolean} mutatingMethod
 * @returns {boolean}
 */
export function requireSurfaceReferenceToken(window, closure, mutatingMethod) {
  const kind = readRequiredTextFact(window, 0);
  const value = readRequiredTextFact(window, 1);
  const previousKind = readRequiredTextFact(window, 2);
  const previous = readRequiredTextFact(window, 3);
  const beforePrevious = readRequiredTextFact(window, 4);
  const followingKind = readRequiredTextFact(window, 5);
  const following = readRequiredTextFact(window, 6);
  const afterFollowing = readRequiredTextFact(window, 7);
  const member = previous === '.' || previous === '?.';
  if (kind === 'identifier' && value === 'class' && !member) return true;
  if (kind === 'identifier' && member && following === '(' && mutatingMethod) return true;
  const assignment = value === '=' || value === '+=' || value === '-=' || value === '*='
    || value === '/=' || value === '%=' || value === '**=' || value === '<<='
    || value === '>>=' || value === '>>>=' || value === '&=' || value === '|='
    || value === '^=' || value === '&&=' || value === '||=' || value === '??=';
  const update = value === '++' || value === '--';
  if (kind !== 'punct' || !assignment && !update) return false;
  if (previous === ']' || previousKind === 'identifier' && (beforePrevious === '.' || beforePrevious === '?.')) return true;
  if (update && followingKind === 'identifier' && (afterFollowing === '.' || afterFollowing === '[')) return true;
  return closure && (value !== '=' || beforePrevious !== 'const' && beforePrevious !== 'let' && beforePrevious !== 'var');
}

/**
 * @param {string[]} columns
 * @param {number[]} rows
 * @param {number} size
 * @param {number} node
 * @param {string} field
 * @returns {number}
 */
function readSurfaceReferenceField(columns, rows, size, node, field) {
  if (node < 0 || node >= size) return -1;
  let offset = readRequiredNumberFact(rows, node);
  const end = offset + readRequiredNumberFact(rows, size + (node));
  while (offset < end) {
    if (readRequiredTextFact(columns, size * 2 + (offset)) === field) return readRequiredNumberFact(rows, size * 2 + (offset));
    offset += 1;
  }
  return -1;
}

/**
 * @param {string[]} columns
 * @param {number} size
 * @param {number} node
 * @returns {string}
 */
function readSurfaceReferenceText(columns, size, node) {
  return node < 0 || node >= size ? '' : readRequiredTextFact(columns, size + (node));
}

/**
 * @param {string[]} columns
 * @param {number} size
 * @param {number} node
 * @returns {boolean}
 */
function acceptSurfaceReferenceTruthyValue(columns, size, node) {
  if (node < 0 || node >= size) return false;
  const form = readRequiredTextFact(columns, node);
  if (form === 'object' || form === 'array') return true;
  if (form === 'boolean') return readRequiredTextFact(columns, size + (node)) === 'true';
  if (form === 'number') return readRequiredTextFact(columns, size + (node)) !== '0' && readRequiredTextFact(columns, size + (node)) !== '-0' && readRequiredTextFact(columns, size + (node)) !== 'NaN';
  return form === 'string' && readRequiredTextFact(columns, size + (node)) !== '';
}

/**
 * @param {string[]} columns
 * @param {number} node
 * @returns {number}
 */
function readSurfaceReferenceType(columns, node) {
  return node >= 0 && (readRequiredTextFact(columns, node) === 'undefined' || readRequiredTextFact(columns, node) === 'null') ? -1 : node;
}

/**
 * @param {string[]} columns
 * @param {number[]} rows
 * @param {number} size
 * @param {number} node
 * @returns {string}
 */
function readSurfaceReferenceKind(columns, rows, size, node) {
  return readSurfaceReferenceText(columns, size, readSurfaceReferenceField(columns, rows, size, node, 'k'));
}

/**
 * @param {string[]} columns
 * @param {number[]} rows
 * @param {number} size
 * @param {number} type
 * @returns {boolean}
 */
function acceptSurfaceReferenceStructure(columns, rows, size, type) {
  // -1 is no type, -2 is a synthesized nonstructural type, and -3 is a
  // synthesized structural type without fields or an element annotation.
  if (type < 0) return type === -3;
  const kind = readSurfaceReferenceText(columns, size, readSurfaceReferenceField(columns, rows, size, type, 'kind'));
  if (kind === 'array' || kind === 'record' || kind === 'set' || kind === 'map' || kind === 'data') return true;
  return kind === 'option' && acceptSurfaceReferenceStructure(columns, rows, size, readSurfaceReferenceField(columns, rows, size, type, 'element'));
}

/**
 * @param {string[]} columns
 * @param {number[]} rows
 * @param {number} size
 * @param {number} path
 * @returns {string}
 */
function readSurfaceReferencePath(columns, rows, size, path) {
  if (path < 0) return '';
  let result = '';
  let offset = readRequiredNumberFact(rows, path);
  const end = offset + readRequiredNumberFact(rows, size + (path));
  while (offset < end) {
    if (offset > readRequiredNumberFact(rows, path)) result += '.';
    result += readSurfaceReferenceText(columns, size, readRequiredNumberFact(rows, size * 2 + (offset)));
    offset += 1;
  }
  return result;
}

/**
 * @param {string[]} columns
 * @param {number[]} rows
 * @param {number} size
 * @param {number} items
 * @param {string} prefix
 * @param {string} path
 * @returns {number}
 */
function findSurfaceReferenceItemSignature(columns, rows, size, items, prefix, path) {
  if (items < 0) return -1;
  let found = -1;
  let offset = readRequiredNumberFact(rows, items);
  const end = offset + readRequiredNumberFact(rows, size + (items));
  while (offset < end) {
    const item = readRequiredNumberFact(rows, size * 2 + (offset));
    const kind = readSurfaceReferenceKind(columns, rows, size, item);
    const name = prefix + '.' + readSurfaceReferenceText(columns, size, readSurfaceReferenceField(columns, rows, size, item, 'name'));
    if (kind === 'fn' && name === path) found = item;
    if (kind === 'module') {
      const nested = findSurfaceReferenceItemSignature(columns, rows, size, readSurfaceReferenceField(columns, rows, size, item, 'items'), name, path);
      if (nested >= 0) found = nested;
    }
    offset += 1;
  }
  return found;
}

/**
 * @param {string[]} columns
 * @param {number[]} rows
 * @param {number} size
 * @param {number} program
 * @param {string} path
 * @returns {number}
 */
function findSurfaceReferenceSignature(columns, rows, size, program, path) {
  let found = findSurfaceReferenceItemSignature(columns, rows, size, readSurfaceReferenceField(columns, rows, size, program, 'items'), 'crate', path);
  const externals = readSurfaceReferenceField(columns, rows, size, program, 'externals');
  if (externals < 0) return found;
  let offset = readRequiredNumberFact(rows, externals);
  const end = offset + readRequiredNumberFact(rows, size + (externals));
  while (offset < end) {
    const external = readRequiredNumberFact(rows, size * 2 + (offset));
    const name = 'crate.' + readSurfaceReferenceText(columns, size, readSurfaceReferenceField(columns, rows, size, external, 'name'));
    if (name === path) found = external;
    offset += 1;
  }
  return found;
}

/**
 * @param {string[]} names
 * @param {number[]} types
 * @param {string} name
 * @returns {number}
 */
function readSurfaceReferenceScope(names, types, name) {
  let index = names.length;
  while (index > 0) {
    index -= 1;
    if (readRequiredTextFact(names, index) === name) return readRequiredNumberFact(types, index);
  }
  return -1;
}

/**
 * The pair is [type node identity, observed identity-comparison flag]. It
 * contains only numbers, allowing the entire recursive algorithm to translate
 * through the existing homogeneous-array core without host callbacks.
 * @param {string[]} columns
 * @param {number[]} rows
 * @param {number} size
 * @param {number} program
 * @param {number} node
 * @param {string[]} names
 * @param {number[]} types
 * @returns {number[]}
 */
function inspectSurfaceReferenceExpression(columns, rows, size, program, node, names, types) {
  if (node < 0 || readRequiredTextFact(columns, node) !== 'object' && readRequiredTextFact(columns, node) !== 'array') return [-1, 0];
  const kind = readSurfaceReferenceKind(columns, rows, size, node);
  if (kind === 'name') {
    const path = readSurfaceReferenceField(columns, rows, size, node, 'path');
    if (path >= 0 && readRequiredNumberFact(rows, size + (path)) === 1) return [readSurfaceReferenceType(columns, readSurfaceReferenceScope(names, types, readSurfaceReferenceText(columns, size, readRequiredNumberFact(rows, size * 2 + (readRequiredNumberFact(rows, path)))))), 0];
    const signature = findSurfaceReferenceSignature(columns, rows, size, program, readSurfaceReferencePath(columns, rows, size, path));
    return [readSurfaceReferenceType(columns, readSurfaceReferenceField(columns, rows, size, signature, 'type')), 0];
  }
  if (kind === 'let') {
    const value = inspectSurfaceReferenceExpression(columns, rows, size, program, readSurfaceReferenceField(columns, rows, size, node, 'value'), names, types);
    const name = readSurfaceReferenceText(columns, size, readSurfaceReferenceField(columns, rows, size, node, 'name'));
    const body = inspectSurfaceReferenceExpression(columns, rows, size, program, readSurfaceReferenceField(columns, rows, size, node, 'body'), names.concat([name]), types.concat([readRequiredNumberFact(value, 0)]));
    return [readRequiredNumberFact(body, 0), readRequiredNumberFact(value, 1) === 1 || readRequiredNumberFact(body, 1) === 1 ? 1 : 0];
  }
  if (kind === 'binary') {
    const leftNode = readSurfaceReferenceField(columns, rows, size, node, 'left');
    const rightNode = readSurfaceReferenceField(columns, rows, size, node, 'right');
    const left = inspectSurfaceReferenceExpression(columns, rows, size, program, leftNode, names, types);
    const right = inspectSurfaceReferenceExpression(columns, rows, size, program, rightNode, names, types);
    const operation = readSurfaceReferenceText(columns, size, readSurfaceReferenceField(columns, rows, size, node, 'op'));
    const observed = readRequiredNumberFact(left, 1) === 1 || readRequiredNumberFact(right, 1) === 1 || (operation === 'eq' || operation === 'ne')
      && readSurfaceReferenceKind(columns, rows, size, leftNode) !== 'none'
      && readSurfaceReferenceKind(columns, rows, size, rightNode) !== 'none'
      && (acceptSurfaceReferenceStructure(columns, rows, size, readRequiredNumberFact(left, 0)) || acceptSurfaceReferenceStructure(columns, rows, size, readRequiredNumberFact(right, 0)));
    const own = readSurfaceReferenceField(columns, rows, size, node, 'type');
    const resultType = readSurfaceReferenceType(columns, own) !== -1 ? own : operation === 'plus' || operation === 'add' ? readRequiredNumberFact(left, 0) : -1;
    return [resultType, observed ? 1 : 0];
  }
  if (kind === 'lambda') {
    let innerNames = names;
    let innerTypes = types;
    const params = readSurfaceReferenceField(columns, rows, size, node, 'params');
    if (params >= 0) {
      let offset = readRequiredNumberFact(rows, params);
      const end = offset + readRequiredNumberFact(rows, size + (params));
      while (offset < end) {
        const param = readRequiredNumberFact(rows, size * 2 + (offset));
        innerNames = innerNames.concat([readSurfaceReferenceText(columns, size, readSurfaceReferenceField(columns, rows, size, param, 'name'))]);
        innerTypes = innerTypes.concat([readSurfaceReferenceField(columns, rows, size, param, 'type')]);
        offset += 1;
      }
    }
    const body = inspectSurfaceReferenceExpression(columns, rows, size, program, readSurfaceReferenceField(columns, rows, size, node, 'body'), innerNames, innerTypes);
    return [-2, readRequiredNumberFact(body, 1)];
  }
  let observed = false;
  let offset = readRequiredNumberFact(rows, node);
  const end = offset + readRequiredNumberFact(rows, size + (node));
  while (offset < end) {
    const field = readRequiredTextFact(columns, size * 2 + (offset));
    if (field !== 'type' && field !== 'span' && field !== 'data') {
      const child = inspectSurfaceReferenceExpression(columns, rows, size, program, readRequiredNumberFact(rows, size * 2 + (offset)), names, types);
      if (readRequiredNumberFact(child, 1) === 1) observed = true;
    }
    offset += 1;
  }
  const own = readSurfaceReferenceField(columns, rows, size, node, 'type');
  if (acceptSurfaceReferenceTruthyValue(columns, size, own)) return [own, observed ? 1 : 0];
  if (kind === 'app') {
    const head = readSurfaceReferenceField(columns, rows, size, node, 'fn');
    if (readSurfaceReferenceKind(columns, rows, size, head) === 'name') {
      const path = readSurfaceReferenceField(columns, rows, size, head, 'path');
      const signature = findSurfaceReferenceSignature(columns, rows, size, program, readSurfaceReferencePath(columns, rows, size, path));
      return [readSurfaceReferenceType(columns, readSurfaceReferenceField(columns, rows, size, signature, 'ret')), observed ? 1 : 0];
    }
  }
  if (kind === 'field' || kind === 'index') {
    const object = inspectSurfaceReferenceExpression(columns, rows, size, program, readSurfaceReferenceField(columns, rows, size, node, 'object'), names, types);
    if (readRequiredNumberFact(object, 1) === 1) observed = true;
    if (kind === 'index') return [readSurfaceReferenceType(columns, readSurfaceReferenceField(columns, rows, size, readRequiredNumberFact(object, 0), 'element')), observed ? 1 : 0];
    const typeKind = readSurfaceReferenceText(columns, size, readSurfaceReferenceField(columns, rows, size, readRequiredNumberFact(object, 0), 'kind'));
    if (typeKind === 'record') {
      const wanted = readSurfaceReferenceText(columns, size, readSurfaceReferenceField(columns, rows, size, node, 'field'));
      const fields = readSurfaceReferenceField(columns, rows, size, readRequiredNumberFact(object, 0), 'fields');
      if (fields >= 0) {
        let position = readRequiredNumberFact(rows, fields);
        const finish = position + readRequiredNumberFact(rows, size + (fields));
        while (position < finish) {
          const field = readRequiredNumberFact(rows, size * 2 + (position));
          if (readSurfaceReferenceText(columns, size, readSurfaceReferenceField(columns, rows, size, field, 'name')) === wanted) return [readSurfaceReferenceType(columns, readSurfaceReferenceField(columns, rows, size, field, 'type')), observed ? 1 : 0];
          position += 1;
        }
      }
    }
    return [-1, observed ? 1 : 0];
  }
  if (kind === 'if') {
    const yes = inspectSurfaceReferenceExpression(columns, rows, size, program, readSurfaceReferenceField(columns, rows, size, node, 'then'), names, types);
    if (readRequiredNumberFact(yes, 1) === 1) observed = true;
    if (readRequiredNumberFact(yes, 0) !== -1) return [readRequiredNumberFact(yes, 0), observed ? 1 : 0];
    const no = inspectSurfaceReferenceExpression(columns, rows, size, program, readSurfaceReferenceField(columns, rows, size, node, 'else'), names, types);
    return [readRequiredNumberFact(no, 0), observed || readRequiredNumberFact(no, 1) === 1 ? 1 : 0];
  }
  if (kind === 'num' || kind === 'str' || kind === 'bool') return [-2, observed ? 1 : 0];
  if (kind === 'array' || kind === 'record' || kind === 'set' || kind === 'map') return [-3, observed ? 1 : 0];
  return [-1, observed ? 1 : 0];
}

/**
 * @param {string[]} columns
 * @param {number[]} rows
 * @param {number} size
 * @param {number} program
 * @param {number} items
 * @returns {boolean}
 */
function inspectSurfaceReferenceItems(columns, rows, size, program, items) {
  if (items < 0) return false;
  let observed = false;
  let offset = readRequiredNumberFact(rows, items);
  const end = offset + readRequiredNumberFact(rows, size + (items));
  while (offset < end) {
    const item = readRequiredNumberFact(rows, size * 2 + (offset));
    const kind = readSurfaceReferenceKind(columns, rows, size, item);
    if (kind === 'module' && inspectSurfaceReferenceItems(columns, rows, size, program, readSurfaceReferenceField(columns, rows, size, item, 'items'))) observed = true;
    if (kind === 'fn') {
      let names = [];
      let types = [];
      const params = readSurfaceReferenceField(columns, rows, size, item, 'params');
      if (params >= 0) {
        let position = readRequiredNumberFact(rows, params);
        const finish = position + readRequiredNumberFact(rows, size + (params));
        while (position < finish) {
          const param = readRequiredNumberFact(rows, size * 2 + (position));
          names = names.concat([readSurfaceReferenceText(columns, size, readSurfaceReferenceField(columns, rows, size, param, 'name'))]);
          types = types.concat([readSurfaceReferenceField(columns, rows, size, param, 'type')]);
          position += 1;
        }
      }
      const result = inspectSurfaceReferenceExpression(columns, rows, size, program, readSurfaceReferenceField(columns, rows, size, item, 'body'), names, types);
      if (readRequiredNumberFact(result, 1) === 1) observed = true;
    }
    offset += 1;
  }
  return observed;
}

/**
 * @param {string[]} columns
 * @param {number[]} rows
 * @param {number} size
 * @param {number} program
 * @returns {boolean}
 */
export function hasSurfaceStructuralIdentity(columns, rows, size, program) {
  let observed = inspectSurfaceReferenceItems(columns, rows, size, program, readSurfaceReferenceField(columns, rows, size, program, 'items'));
  const main = readSurfaceReferenceField(columns, rows, size, program, 'main');
  const effects = readSurfaceReferenceField(columns, rows, size, main, 'effects');
  if (effects < 0) return observed;
  let names = [];
  let types = [];
  let offset = readRequiredNumberFact(rows, effects);
  const end = offset + readRequiredNumberFact(rows, size + (effects));
  while (offset < end) {
    const effect = readRequiredNumberFact(rows, size * 2 + (offset));
    const binding = readSurfaceReferenceKind(columns, rows, size, effect) === 'let';
    const node = binding ? readSurfaceReferenceField(columns, rows, size, effect, 'value') : effect;
    const result = inspectSurfaceReferenceExpression(columns, rows, size, program, node, names, types);
    if (readRequiredNumberFact(result, 1) === 1) observed = true;
    if (binding) {
      names = names.concat([readSurfaceReferenceText(columns, size, readSurfaceReferenceField(columns, rows, size, effect, 'name'))]);
      types = types.concat([readRequiredNumberFact(result, 0)]);
    }
    offset += 1;
  }
  return observed;
}

/** @param {boolean} selected @param {string} kind @param {boolean} existing @returns {boolean} */
export function selectSurfaceReferenceAllocation(selected, kind, existing) {
  return selected && !existing && (kind === 'array' || kind === 'record' || kind === 'map' || kind === 'set');
}

/** @param {boolean} reference @returns {boolean} */
export function selectSurfaceReferenceRead(reference) {
  return reference;
}

/** @param {string} kind @returns {string} */
export function readSurfaceReferenceResultField(kind) {
  return kind === 'referenceRead' ? 'reference' : '';
}

/** @param {string} format @param {string} operation @param {string} value @param {string} temporary @returns {string} */
export function renderSurfaceReferenceOperation(format, operation, value, temporary) {
  if (format === 'JavaScript' && (operation === 'referenceAllocate' || operation === 'referenceRead')) return value;
  if (format === 'Rust' && operation === 'referenceAllocate') return 'std::rc::Rc::new(std::cell::RefCell::new(' + value + '))';
  if (format === 'Rust' && operation === 'referenceRead') return '{ let ' + temporary + ' = ' + value + '.borrow().clone(); ' + temporary + ' }';
  return '';
}

/** @returns {string} */
export function readSurfaceReferenceRefusal() {
  return 'shared reference state needs an explicit target state model';
}

/** @param {string} kind @param {boolean} statefulReceiver @param {boolean} referenceAllocation @returns {string} */
export function readSurfaceReferenceEmissionOperation(kind, statefulReceiver, referenceAllocation) {
  if (statefulReceiver && (kind === 'index' || kind === 'optionalIndex' || kind === 'length'
    || kind === 'recordField' || kind === 'tupleIndex' || kind === 'method'
    || kind === 'collectionSize' || kind === 'collectionRead' || kind === 'collectionItems')) return 'reference operation';
  if (referenceAllocation && (kind === 'array' || kind === 'record' || kind === 'map' || kind === 'set' || kind === 'append')) return 'reference allocation';
  return '';
}

/** @param {string} operation @returns {string} */
export function readSurfaceReferenceEmissionRefusal(operation) {
  if (operation === 'reference operation') return 'this operation needs the shared reference and slot runtime before target emission';
  if (operation === 'reference allocation') return 'this allocation needs the shared reference and slot runtime before target emission';
  return '';
}

/** A proved source interval can remove an indexed plain assignment. All eight
 * original token facts are still validated; other reference triggers survive.
 * @param {string[]} window @param {boolean} closure
 * @param {boolean} mutatingMethod @param {boolean} eliminated
 * @returns {boolean} */
export function requireSurfaceReferenceTokenAfterElimination(window, closure, mutatingMethod, eliminated) {
  const required = requireSurfaceReferenceToken(window, closure, mutatingMethod);
  return required && !(eliminated && readRequiredTextFact(window, 0) === 'punct'
    && readRequiredTextFact(window, 1) === '=' && readRequiredTextFact(window, 3) === ']');
}
