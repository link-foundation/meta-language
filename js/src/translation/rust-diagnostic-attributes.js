// Owned source provenance for the bounded diagnostic-only const attribute model.
import { readRuntimeProjectData } from '../project-data-loader.js';
import { tokenize } from './lexer.js';
import { typeError } from './diagnostics.js';
import { checkOwnedProgramContainers } from './owned-program-containers.js';
import { acceptRustConstFunctionContainer, acceptRustConstFunctionOffset, acceptRustConstFunctionInterval } from './rust-const-function-rules.js';
import { planRustDiagnosticAttributePrefix } from './rust-diagnostic-attribute-rules.js';

const policy = readRuntimeProjectData(new URL('./data/rust-diagnostic-attributes.lino', import.meta.url));
const fail = reason => { throw typeError('Rust diagnostic source attributes: ' + reason); };
const owns = (value, name) => Object.hasOwn(value, name);

export function readRustDiagnosticAttributeNormalization() {
  return { ...policy.normalization };
}

function diagnosticRecord(value, fields) {
  if (!value || typeof value !== 'object' || Array.isArray(value)
    || Object.keys(value).length !== fields.length || fields.some(field => !owns(value, field))) {
    fail('an owned descriptor has missing or unrepresented fields');
  }
}

function diagnosticSpan(value, nonempty = true) {
  diagnosticRecord(value, policy.fields.span);
  if (typeof value.start !== 'number' || typeof value.end !== 'number'
    || !acceptRustConstFunctionOffset(value.start, Object.is(value.start, -0))
    || !acceptRustConstFunctionOffset(value.end, Object.is(value.end, -0))
    || !acceptRustConstFunctionInterval(value.start, value.end, nonempty)) {
    fail('a source span needs integral absolute UTF16 offsets');
  }
}

function sameDiagnosticSpan(left, right) {
  diagnosticSpan(left);
  diagnosticSpan(right);
  return left.start === right.start && left.end === right.end;
}

function diagnosticMarkerOwners(root) {
  const owners = new Set(), visited = new WeakSet(), active = new WeakSet();
  let cyclic = false;
  function visit(value) {
    if (!value || typeof value !== 'object') return;
    if (active.has(value)) { cyclic = true; return; }
    if (visited.has(value)) return;
    visited.add(value); active.add(value);
    if (owns(value, 'sourceDiagnosticAttributes')) owners.add(value);
    if (value instanceof Map) {
      for (const [key, child] of Map.prototype.entries.call(value)) { visit(key); visit(child); }
    } else {
      for (const child of Object.values(value)) visit(child);
    }
    active.delete(value);
  }
  visit(root);
  if (owners.size && cyclic) fail('a source descriptor contains a structural cycle');
  return owners;
}

export function checkRustDiagnosticAttributeContainers(program) {
  const selected = checkOwnedProgramContainers(program, {
    selectedFields: ['sourceDiagnosticAttributes'],
    acceptContainer: acceptRustConstFunctionContainer, fail,
  });
  return selected ? checkRustDiagnosticAttributePlacement(program) : false;
}

// The caller must run the existing owned-container preflight first. This
// selected-only walk authenticates placement before Surface lowering can drop
// metadata; it performs no second container preflight.
export function checkRustDiagnosticAttributePlacement(program) {
  const owners = diagnosticMarkerOwners(program);
  if (!owners.size) return false;
  if (!Array.isArray(program.items)) fail('the source facet requires actual ordered items');
  const declarations = new Set(), pending = [...program.items], seen = new Set();
  while (pending.length) {
    const entry = pending.pop();
    if (!entry || typeof entry !== 'object' || seen.has(entry)) continue;
    seen.add(entry);
    if (entry.k === 'fn') declarations.add(entry);
    else if (entry.k === 'module') {
      if (!Array.isArray(entry.items)) fail('an actual module needs ordered items');
      pending.push(...entry.items);
    }
  }
  for (const owner of owners) {
    if (!declarations.has(owner)) fail('a source facet is not attached to an actual function item');
  }
  return true;
}

function validateDiagnosticFacet(facet, entry, modulePath) {
  diagnosticRecord(facet, policy.fields.facet);
  if (facet.model !== policy.facet.model || facet.language !== policy.facet.language
    || facet.edition !== policy.facet.edition) fail('the registered source model or Rust edition changed');
  if (entry.k !== 'fn' || !owns(entry, 'sourceConstFunction')
    || !entry.sourceConstFunction || typeof entry.sourceConstFunction !== 'object'
    || Array.isArray(entry.sourceConstFunction)) fail('an actual const function proof is required');
  diagnosticRecord(facet.attachment, policy.fields.attachment);
  if (typeof facet.attachment.sourceName !== 'string' || facet.attachment.sourceName !== entry.name
    || !Array.isArray(facet.attachment.modulePath) || !Array.isArray(modulePath)
    || facet.attachment.modulePath.length !== modulePath.length
    || facet.attachment.modulePath.some((name, at) => typeof name !== 'string' || name !== modulePath[at])) {
    fail('the attribute prefix belongs to a different function owner');
  }
  const proof = entry.sourceConstFunction;
  if (!sameDiagnosticSpan(facet.attachment.constKeywordSpan, proof.keywordSpan)
    || !sameDiagnosticSpan(facet.attachment.functionKeywordSpan, proof.functionKeywordSpan)) {
    fail('the attribute attachment differs from the actual const header');
  }
  diagnosticSpan(facet.span);
  if (typeof facet.raw !== 'string' || facet.raw.length !== facet.span.end - facet.span.start
    || facet.span.end !== proof.keywordSpan.start || !Array.isArray(facet.tokens)) {
    fail('the complete raw prefix must end at the actual const keyword');
  }
  // Authenticate the boundary too: a trailing identifier or line comment
  // cannot absorb the next actual const keyword after source-free edits.
  const constKeyword = policy.constKeyword;
  const actual = tokenize(facet.raw + constKeyword.value, policy.facet.language).tokens.filter(token => token.kind !== 'eof');
  const following = actual.pop();
  if (!following || following.kind !== constKeyword.kind || following.value !== constKeyword.value
    || following.raw !== constKeyword.value || following.start !== facet.raw.length
    || following.end !== facet.raw.length + constKeyword.value.length) {
    fail('the retained prefix absorbs or changes the actual following const token');
  }
  if (actual.length !== facet.tokens.length || !actual.length || actual[0].start !== 0) {
    fail('the facet must retain every actual token from its first outer attribute');
  }
  for (let at = 0; at < actual.length; at += 1) {
    const token = facet.tokens[at], original = actual[at];
    diagnosticRecord(token, policy.fields.token);
    diagnosticSpan(token.span);
    if (typeof token.kind !== 'string' || typeof token.value !== 'string' || typeof token.raw !== 'string'
      || token.kind !== original.kind || token.value !== original.value || token.raw !== original.raw
      || token.span.start !== facet.span.start + original.start
      || token.span.end !== facet.span.start + original.end
      || token.raw !== facet.raw.slice(original.start, original.end)) {
      fail('stored token kind value spelling or absolute UTF16 span differs from the actual prefix');
    }
  }
  const plan = planRustDiagnosticAttributePrefix(actual.map(token => token.kind), actual.map(token => token.value));
  if (plan.length !== 4 || plan[0] !== actual.length || plan[2] < 1) {
    fail('the complete prefix has no admitted diagnostic attribute interpretation');
  }
  return plan;
}

export function createRustDiagnosticAttributeFacet(source, tokens, firstAttributeIndex, constKeywordIndex, entry, modulePath) {
  if (typeof source !== 'string' || !Array.isArray(tokens)
    || !Number.isSafeInteger(firstAttributeIndex) || firstAttributeIndex < 0
    || !Number.isSafeInteger(constKeywordIndex) || constKeywordIndex <= firstAttributeIndex
    || constKeywordIndex >= tokens.length) fail('actual complete source and token positions are required');
  checkOwnedProgramContainers(entry, { selectedFields: ['sourceConstFunction'], acceptContainer: acceptRustConstFunctionContainer, fail });
  const proof = entry.sourceConstFunction;
  const first = tokens[firstAttributeIndex], keyword = tokens[constKeywordIndex];
  if (!proof || keyword.kind !== policy.constKeyword.kind || keyword.value !== policy.constKeyword.value
    || keyword.start !== proof.keywordSpan.start || keyword.end !== proof.keywordSpan.end) {
    fail('the prefix must be attached to the original actual const token');
  }
  const facet = {
    model: policy.facet.model, language: policy.facet.language, edition: policy.facet.edition,
    span: { start: first.start, end: keyword.start },
    raw: source.slice(first.start, keyword.start),
    tokens: tokens.slice(firstAttributeIndex, constKeywordIndex).map(token => ({
      kind: token.kind, value: token.value, raw: token.raw,
      span: { start: token.start, end: token.end },
    })),
    attachment: { sourceName: entry.name, modulePath: [...modulePath],
      constKeywordSpan: { ...proof.keywordSpan }, functionKeywordSpan: { ...proof.functionKeywordSpan } },
  };
  validateDiagnosticFacet(facet, entry, modulePath);
  return facet;
}

export function checkRustDiagnosticAttributes(program) {
  if (!checkRustDiagnosticAttributeContainers(program)) return false;
  if (program.sourceLanguage !== 'Rust' || !(program.declarations instanceof Map)) {
    fail('the source facet requires actual checked Rust declarations');
  }
  const owners = diagnosticMarkerOwners(program), verified = new Set();
  for (const [name, entry] of Map.prototype.entries.call(program.declarations)) {
    if (!owns(entry, 'sourceDiagnosticAttributes')) continue;
    if (!Array.isArray(entry.modulePath) || typeof entry.name !== 'string'
      || entry.fullName !== name || [...entry.modulePath, entry.name].join('.') !== name) {
      fail('the facet does not target its actual declaration binding');
    }
    validateDiagnosticFacet(entry.sourceDiagnosticAttributes, entry, entry.modulePath);
    verified.add(entry);
  }
  if (owners.size !== verified.size || [...owners].some(owner => !verified.has(owner))) {
    fail('the source facet escaped its actual checked function');
  }
  return true;
}

export function renderRustDiagnosticAttributePrefix(entry, source) {
  if (!owns(entry, 'sourceDiagnosticAttributes')) return source;
  const facet = entry.sourceDiagnosticAttributes;
  const plan = validateDiagnosticFacet(facet, entry, entry.modulePath);
  if (plan[3] < 0) return facet.raw + source;
  // Only the canonical emitter prefix changes; the original raw prefix, its
  // comments and actual visibility token are retained without text searching.
  if (!source.startsWith(policy.emission.canonicalPublicPrefix)) fail('the canonical const emitter header changed');
  return facet.raw + source.slice(policy.emission.visibilityPrefix.length);
}
