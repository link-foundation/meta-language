// Mechanical actual checked-program/emitter fact transport. Normative owner,
// closure, correspondence, emission and publication decisions are shared JS
// rules, normally translated to Rust. Complete bodies are never rewritten.
import {isDeepStrictEqual} from 'node:util';
import {readRustExternalTargets} from './emit-rust.js';
import {readRecursiveDeclarationSourceOwners, acceptRecursiveDeclarationAuxiliaryClosure,
  acceptRecursiveDeclarationAuxiliaryCorrespondence, acceptRecursiveDeclarationOwnedEmission,
  acceptRecursiveDeclarationOwnedPublication} from './declaration-dependency-rules.js';

function sameSpan(left, right) {
  return left !== null && left !== undefined && right !== null && right !== undefined
    && left.start === right.start && left.end === right.end;
}

function appendText(values, text) {
  values.push(text.length);
  for (let at = 0; at < text.length; at += 1) values.push(text.charCodeAt(at));
}

// Exact JSON-shaped type facts. Only a checked named-data reference becomes
// its actual declaration identity; strings in every other field stay literal.
function appendType(values, value, declarations, local, depth = 0) {
  if (depth > 128) return false;
  if (value === null || value === undefined) { values.push(0); return true; }
  if (typeof value === 'boolean') { values.push(value ? 2 : 1); return true; }
  if (typeof value === 'number') { if (!Number.isFinite(value)) return false; values.push(3, value); return true; }
  if (typeof value === 'string') { values.push(4); appendText(values, value); return true; }
  if (Array.isArray(value)) {
    values.push(5, value.length);
    for (const item of value) if (!appendType(values, item, declarations, local, depth + 1)) return false;
    return true;
  }
  if (!value || Object.getPrototypeOf(value) !== Object.prototype) return false;
  const fields = Object.keys(value).sort();
  values.push(6, fields.length);
  for (const field of fields) {
    appendText(values, field);
    if (value.kind === 'data' && field === 'name') {
      const actual = declarations.get(value.name);
      if (!actual || !local.has(actual)) return false;
      values.push(7, local.get(actual));
    } else if (!appendType(values, value[field], declarations, local, depth + 1)) return false;
  }
  return true;
}

// These are the actual checked reference fields used by typed-program-links.
// Its ordinary checker already validates each node. Span objects are excluded;
// matching string literals never become edges.
function readDeclarationReferences(entry) {
  const references = [];
  let visits = 0;
  function visit(node, depth = 0) {
    if (node === null || typeof node !== 'object') return true;
    if (depth > 256 || ++visits > 100_000) return false;
    if (node.k === 'call' || node.k === 'functionRef') references.push(node.fn);
    if (node.k === 'ctor') references.push(node.data);
    if (node.kind === 'data') references.push(node.name);
    if (Array.isArray(node)) return node.every(child => visit(child, depth + 1));
    for (const [field, child] of Object.entries(node)) {
      if (field === 'span') continue;
      if (!visit(child, depth + 1)) return false;
    }
    return true;
  }
  const fields = entry.k === 'fn' ? [entry.params.map(parameter => parameter.type), entry.ret, entry.body]
    : [entry.ctors.map(constructor => constructor.fields.map(field => field.type))];
  return visit(fields) ? [...new Set(references)] : null;
}

function readActualCapture(capture, names, originalNames) {
  const program = capture?.program, emitted = capture?.emitted;
  const metadata = emitted?.definitionOwnership;
  if (capture?.sourceLanguage !== 'JavaScript' || typeof capture?.source !== 'string'
    || program?.sourceLanguage !== 'JavaScript'
    || !(program?.declarations instanceof Map) || metadata?.schemaVersion !== 1 || !Array.isArray(metadata.blocks)
    || !Array.isArray(metadata.externalTargets) || !Array.isArray(emitted.definitions)
    || program.main?.effects.length !== 0 || program.imports?.length
    || program.outputThreaded || program.abortsThreaded || program.sourceRuntimeShell
    || capture.undecorated !== true) return null;
  const declarations = [...program.declarations.values()];
  const byName = new Map(declarations.map((entry, index) => [entry.fullName, index]));
  if (byName.size !== declarations.length || new Set(names).size !== names.length) return null;
  const roots = names.map(name => declarations.find(entry => entry.name === name && entry.fullName === name
    && entry.k === 'fn' && entry.modulePath?.length === 0 && !entry.generated));
  if (roots.some(root => !root?.span) || roots.some(root => root.params.some(parameter => parameter.guard || parameter.optional || parameter.default))) return null;
  const starts = declarations.map(entry => entry.span?.start ?? -1);
  const ends = declarations.map(entry => entry.span?.end ?? -1);
  const roles = declarations.map(entry => entry.modulePath?.length !== 0 ? -1
    : roots.includes(entry) ? 0 : entry.generated === true && entry.k === 'fn' ? 1
    : entry.generated === true && entry.k === 'data' && !entry.output ? 2 : -1);
  const owners = readRecursiveDeclarationSourceOwners(roots.map(root => root.span.start), roots.map(root => root.span.end), starts, ends, roles);
  if (owners.length !== declarations.length) return null;
  const referenceNames = declarations.map(entry => readDeclarationReferences(entry));
  const externals = new Set((program.externals ?? []).map(entry => entry.name));
  if (referenceNames.some(row => row === null || row.some(name => typeof name !== 'string'
    || !byName.has(name) && !externals.has(name)))) return null;
  const edges = referenceNames.map(row => row.filter(name => byName.has(name)).map(name => byName.get(name)));
  const edgeStarts = [], edgeCounts = [], edgeTargets = [];
  for (const row of edges) { edgeStarts.push(edgeTargets.length); edgeCounts.push(row.length); edgeTargets.push(...row); }
  if (!acceptRecursiveDeclarationAuxiliaryClosure(owners, roles, edgeStarts, edgeCounts, edgeTargets)) return null;
  const targetIds = new Map();
  const targetId = text => {
    if (typeof text !== 'string' || !text.length) return -1;
    if (!targetIds.has(text)) targetIds.set(text, targetIds.size);
    return targetIds.get(text);
  };
  const expectedExternals = (program.externals ?? []).filter(entry => !program.declarations.has(entry.name));
  if (metadata.externalTargets.length !== expectedExternals.length
    || metadata.externalTargets.some((row, index) => row.source !== expectedExternals[index].name || row.kind !== expectedExternals[index].k)) return null;
  if (!isDeepStrictEqual(metadata.externalTargets, readRustExternalTargets(program))) return null;
  const forbidden = metadata.externalTargets.map(row => targetId(row.target));
  if (forbidden.includes(-1)) return null;
  const blocks = [], mappings = [], constructors = declarations.map(entry => entry.k === 'data' ? entry.ctors.length : 0);
  if (metadata.blocks.length !== emitted.definitions.length) return null;
  for (const row of metadata.blocks) {
    const declaration = byName.get(row.source), entry = declarations[declaration];
    if (!entry || row.kind !== entry.k || !sameSpan(row.sourceSpan, entry.span)
      || !Number.isSafeInteger(row.mappingStart) || !Number.isSafeInteger(row.mappingEnd)
      || row.mappingStart < 0 || row.mappingEnd < row.mappingStart
      || row.mappingEnd > emitted.mappings.length || typeof emitted.definitions[row.definitionIndex] !== 'string'
      || !emitted.definitions[row.definitionIndex].trim() || row.definition !== emitted.definitions[row.definitionIndex]
      || !Array.isArray(row.targets) || row.targets.length !== row.mappingEnd - row.mappingStart) return null;
    blocks.push(declaration, row.mappingStart, row.mappingEnd - row.mappingStart, row.definitionIndex);
    for (let at = row.mappingStart; at < row.mappingEnd; at += 1) {
      const mapping = emitted.mappings[at], position = at - row.mappingStart;
      const expected = position === 0 ? entry.fullName : entry.k === 'data' ? entry.fullName + '.' + entry.ctors[position - 1]?.name : null;
      const kind = position === 0 ? entry.k === 'fn' ? 'function' : 'data' : 'constructor';
      if (mapping.source !== expected || mapping.kind !== kind || !sameSpan(mapping.sourceSpan, entry.span)
        || mapping.target !== row.targets[position]) return null;
      mappings.push(declaration, position, targetId(mapping.target));
    }
  }
  if (mappings.length !== emitted.mappings.length * 3
    || !acceptRecursiveDeclarationOwnedEmission(roles, constructors, blocks, mappings, forbidden)) return null;
  function inventory(owner) {
    const actual = declarations.filter((_, index) => owners[index] === owner);
    const local = new Map(actual.map((entry, index) => [entry, index]));
    const origin = roots[owner].span.start, result = [1, actual.length];
    for (const entry of actual) {
      const index = byName.get(entry.fullName);
      result.push(roles[index], entry.span.start - origin, entry.span.end - origin);
      if (entry.k === 'fn') {
        // An original interface cannot export a generated private data owner
        // at any nested type position. It has no stable external declaration
        // identity across independent member/source contexts.
        const typeOwners = roles[index] === 0 ? new Map() : local;
        result.push(entry.params.length);
        for (const parameter of entry.params) if (!appendType(result, parameter.type, program.declarations, typeOwners)) return null;
        if (!appendType(result, entry.ret, program.declarations, typeOwners)) return null;
      } else {
        result.push(entry.ctors.length);
        for (const constructor of entry.ctors) {
          result.push(constructor.fields.length);
          for (const field of constructor.fields) if (!appendType(result, field.type, program.declarations, local)) return null;
        }
      }
      result.push(referenceNames[index].length);
      for (const name of referenceNames[index]) {
        const value = declarations[byName.get(name)];
        if (value && local.has(value)) result.push(1, local.get(value));
        else if (originalNames.includes(name)) result.push(2, originalNames.indexOf(name));
        else { result.push(3); appendText(result, name); }
      }
    }
    return result;
  }
  return {program, emitted, roots, declarations, owners, roles, blocks: metadata.blocks, inventory};
}

/** Actual captures originate only in the normal complete-source checker and
 * emitter. Return complete WHOLE-emission indices, never rewritten definitions.
 * All graph/mapping cells have been observed before the final shared gate.
 */
export function planOwnedRecursiveDeclarationPublication(names, sources, wholeCapture, memberCaptures, moduleScopeKnown) {
  if (names.length !== sources.length || names.length !== memberCaptures.length
    || sources.some(source => typeof source !== 'string') || wholeCapture?.source !== sources.join('\n\n')) return null;
  const whole = readActualCapture(wholeCapture, names, names);
  if (!whole || wholeCapture.signatures?.length !== names.length
    || wholeCapture.signatures.some(signature => signature.k !== 'fn' || !names.includes(signature.name))
    || new Set(wholeCapture.signatures.map(signature => signature.name)).size !== names.length) return null;
  const indices = [], mappings = [];
  for (let owner = 0; owner < names.length; owner += 1) {
    const capture = memberCaptures[owner];
    if (capture?.source !== sources[owner]) return null;
    const member = readActualCapture(capture, [names[owner]], names);
    if (!member) return null;
    const expected = whole.inventory(owner), actual = member.inventory(0);
    if (!expected || !actual || !acceptRecursiveDeclarationAuxiliaryCorrespondence(expected, actual)) return null;
    const expectedSignature = wholeCapture.signatures.filter(signature => signature.name === names[owner]);
    if (!isDeepStrictEqual(capture.signatures, expectedSignature)) return null;
    const wholeRoot = whole.emitted.mappings.filter(mapping => mapping.kind === 'function' && mapping.source === names[owner]);
    const memberRoot = member.emitted.mappings.filter(mapping => mapping.kind === 'function' && mapping.source === names[owner]);
    if (wholeRoot.length !== 1 || memberRoot.length !== 1 || wholeRoot[0].target !== memberRoot[0].target) return null;
    const blocks = whole.blocks.filter(row => whole.owners[whole.declarations.findIndex(entry => entry.fullName === row.source)] === owner);
    if (!blocks.length) return null;
    indices.push(blocks.map(row => row.definitionIndex));
    mappings.push(blocks.flatMap(row => whole.emitted.mappings.slice(row.mappingStart, row.mappingEnd)));
  }
  const proofs = [1,1,1,1,1,1,moduleScopeKnown === true ? 1 : 0];
  return acceptRecursiveDeclarationOwnedPublication(names.length, whole.declarations.length,
    wholeCapture.signatures.length, memberCaptures.length, proofs) ? {indices, mappings} : null;
}
