// Translates meta-language's own modules between JavaScript, TypeScript and
// Rust through meta-language links. The source is parsed losslessly with its
// native grammar; a translation into the same language writes the links back
// byte for byte, and a translation into the other language translates each
// top-level item the portable core can express and carries every other item
// verbatim in a marked comment. Both kinds of item keep their source as
// provenance, so translating an unedited translation back restores the
// source byte for byte.
import { createHash } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';
import {planOwnedRecursiveDeclarationPublication} from './translation/recursive-declaration-ownership.js';

import { decoratorSet } from './decorators.js';
import { decorateEmitted } from './grammar-emitters/common.js';
import { LinkNetwork } from './network.js';
import { ParseConfiguration } from './primitives.js';
import { parseProgrammingLanguage } from './programming-language-parser.js';
import { acceptSelfTranslationUtf16Member, requireSelfTranslationUtf16Representation } from './self-translation-representation-rules.js';
import { checkProgram } from './translation/check.js';
import { TranslationError } from './translation/diagnostics.js';
import { emitJavaScript } from './translation/emit-javascript.js';
import { emitRust, emitRustConstants } from './translation/emit-rust.js';
import { parseJavaScript } from './translation/javascript.js';
import { requiresReferenceSemantics } from './translation/reference-analysis.js';
import { tokenize } from './translation/lexer.js';
import { eraseTypeScript, requiresTypeScriptErasure } from './typescript-erasure.js';
import { projectTypeScriptTyping } from './typescript-portable-typing.js';
import { parseRust } from './translation/rust.js';
import { acceptBindingScope, acceptLiteralBinding, acceptModuleBindingScope, acceptRootSyntaxItem, acceptSourcePrefixRestoration, acceptSourceEnvelopeRestoration, acceptDeclarationSignature, findBindingRunEnd } from './translation/frontend-rules.js';
import {requireSelfTranslationLayoutEnvelope} from './self-translation-layout-rules.js';
import {readDeferredDeclarationBinding} from './translation/deferred-declaration-scope.js';
import { readDeclarationDependencyComponents, acceptRecursiveDeclarationGroup,
  acceptRecursiveDeclarationPublication, acceptDeclarationModuleScope,
  acceptDeclarationSignatureBinding, acceptDeclarationDeferredModuleScope } from './translation/declaration-dependency-rules.js';

/** The languages self-translation reads and writes. */
export const SELF_TRANSLATION_LANGUAGES = Object.freeze(['JavaScript', 'TypeScript', 'Rust']);

const HEADER = '// meta-language:self-translation:v1 ';
const CARRIED = '// meta-language:carried ';
const TRANSLATED = '// meta-language:translated ';
const SOURCE_LINE = '// |';
const PRELUDE_BEGIN = '// meta-language:prelude begin';
const PRELUDE_END = '// meta-language:prelude end';
// Portable-core Rust keeps the source's parentheses and names.
const RUST_ALLOW = '#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]';
const ALIASES = new Map([
  ['javascript', 'JavaScript'], ['js', 'JavaScript'], ['mjs', 'JavaScript'],
  ['typescript', 'TypeScript'], ['ts', 'TypeScript'],
  ['rust', 'Rust'], ['rs', 'Rust'],
]);
const COMMENTS = new Set(['comment', 'line_comment', 'block_comment']);

/** Thrown for an unknown language or a source the links do not reproduce. */
export class SelfTranslationError extends Error {
  constructor(message) {
    super(message);
    this.name = 'SelfTranslationError';
  }
}

/** The self-translation language `language` names, or `null`. */
export function selfTranslationLanguage(language) {
  return ALIASES.get(String(language).toLowerCase()) ?? null;
}

function required(language) {
  const name = selfTranslationLanguage(language);
  if (!name) throw new SelfTranslationError(`self-translation reads and writes ${SELF_TRANSLATION_LANGUAGES.join(', ')}, not ${language}`);
  return name;
}

// The family a language's code is written in: TypeScript items are read and
// written as JavaScript.
const family = (language) => (language === 'Rust' ? 'Rust' : 'JavaScript');
const sha256 = (text) => createHash('sha256').update(Buffer.from(text, 'utf8')).digest('hex');

/**
 * Translates `source` from `sourceLanguage` to `targetLanguage`, each one of
 * {@link SELF_TRANSLATION_LANGUAGES} or an alias of one.
 *
 * The result is `{ sourceLanguage, targetLanguage, code, items }`. Each item
 * `{ term, start, end, status, reason }` is a top-level item of the source
 * (byte offsets) with its status: `kept` (same language), `translated`,
 * `restored` (provenance gave back its source), `comment`, `provenance` (a
 * self-translation header or prelude) or `carried`, with the reason it was
 * carried.
 *
 * `options.decorators` (a DecoratorSet or an array of decorators) decorates
 * each translated item at the `emitter` level, one `{ format, number, line }`
 * record per line of its definitions with the target language as `format`,
 * before its provenance is recorded, so a translation can be brought to match
 * hand-written code and still restores its source.
 *
 * From JavaScript or TypeScript into Rust, an item may call and read the
 * module's other top-level items: each item is translated after the items it
 * names, with the signatures of those that translated bound, and the Rust
 * names them as its own translation does. `import { a, b as c } from
 * './m.mjs'` becomes `use crate::m::{a, b as c};`: `options.moduleDirectory`
 * (path segments, the crate root by default) is the module's directory inside
 * the crate, and `options.imports` maps an import specifier to the signatures
 * {@link selfTranslationSignatures} gives for that module, which bind the
 * imported names the same way.
 *
 * Report tooling can opt into `retainDiagnostics: true` to retain the actual
 * TranslationError kind, reason, message, span and details on carried items.
 * The diagnostic also records the complete source group's UTF-8 byte range;
 * the original frontend span is retained without coordinate conversion.
 * This does not change emitted code or ordinary item fields.
 */
export function selfTranslate(source, sourceLanguage, targetLanguage, options = {}) {
  return translateModule(source, sourceLanguage, targetLanguage, options).translation;
}

/**
 * The signatures of the exported items of the JavaScript or TypeScript module
 * `source` that translate into Rust, in source order, for
 * {@link selfTranslate}'s `options.imports` of a module that imports them:
 * `{ k: 'fn', name, params: [{ name, type }], ret }` for a function and
 * `{ k: 'const', name, type, literal }` for a constant, where `literal` tells
 * whether a literal gives its value. `options` are {@link selfTranslate}'s.
 */
export function selfTranslationSignatures(source, language, options = {}) {
  if (family(required(language)) !== 'JavaScript') {
    throw new SelfTranslationError(`signatures are read from JavaScript or TypeScript modules, not ${language}`);
  }
  return translateModule(source, language, 'Rust', options, true).signatures;
}

function translateModule(source, sourceLanguage, targetLanguage, { decorators, moduleDirectory = [], imports = {}, utf16Strings = false, referenceSemantics = false, retainDiagnostics = false } = {}, signaturesOnly = false) {
  const from = required(sourceLanguage);
  const to = required(targetLanguage);
  const set = decoratorSet(decorators);
  const text = String(source);
  const bytes = Buffer.from(text, 'utf8');
  const parsed = parseProgrammingLanguage(text, from);
  // Signature discovery still checks and translates every declaration. It
  // does not need a second complete source graph or a provenance envelope.
  const reconstructed = signaturesOnly ? bytes.toString('utf8') : LinkNetwork._parseWithLinks(text, from, ParseConfiguration.default(), parsed).network.reconstructText();
  if (reconstructed !== text) {
    throw new SelfTranslationError(`the ${from} links of the source do not reproduce it`);
  }
  const items = topLevelItems(bytes, parsed);
  const usesStringMethods = (node) => (node.term === 'member_expression' && node.children.some((child) =>
    child.field === 'property' && acceptSelfTranslationUtf16Member(bytes.subarray(child.span.byteRange.start, child.span.byteRange.end).toString('utf8'))))
    || node.children.some(usesStringMethods);
  const representation = requireSelfTranslationUtf16Representation(utf16Strings, family(from), !utf16Strings && family(from) === 'JavaScript' && usesStringMethods(parsed.tree));
  const keptFamily = from === 'JavaScript' || (from === 'TypeScript' && !requiresTypeScriptErasure(parsed));
  if (family(from) === family(to) && (from === to || (keptFamily && !text.startsWith(HEADER)))) {
    return { translation: freeze(from, to, reconstructed, items.map(({ term, start, end }) => ({ term, start, end, status: 'kept', reason: null }))), signatures: [] };
  }
  const blocks = [];
  const preludes = [];
  const recorded = [];
  let gap = '';
  const groups = groupItems(items, bytes);
  let references = referenceSemantics || (family(from) === 'JavaScript' && requiresReferenceSemantics(text));
  if (!referenceSemantics && references && family(from) === 'JavaScript') {
    // The complete frontend can prove an owned append rewrite and remove its
    // reference trigger. Preserve lexical admission when that parse refuses.
    try { references = parseJavaScript(text, { moduleDirectory }).referenceSemantics === true; }
    catch (error) { if (!(error instanceof TranslationError)) throw error; }
  }
  const pendingTypes = Object.values(imports).flat();
  const seenTypes = new Set();
  while (!references && pendingTypes.length) {
    const type = pendingTypes.pop();
    if (!type || typeof type !== 'object' || seenTypes.has(type)) continue;
    seenTypes.add(type);
    if (type.reference === true) references = true;
    else pendingTypes.push(...Object.values(type));
  }
  if (!references && family(from) === 'JavaScript') {
    // An identity observation in one declaration selects the same representation
    // for its siblings. Inspect inferred values rather than treating scalar
    // equality as a request for reference identity.
    for (const group of groups) {
      if (group.kind !== 'item') continue;
      try {
        const projected = from === 'TypeScript' ? projectTypeScriptTyping(group.text).code : group.text;
        if (parseJavaScript(projected, { moduleDirectory }).referenceSemantics) { references = true; break; }
      } catch (error) { if (!(error instanceof TranslationError)) throw error; }
    }
  }
  const { outs, scans, moduleScopeKnown, ownerCounts } = family(to) === 'Rust'
    ? translateBound(groups, from, to, set, { moduleDirectory, imports, moduleSource: text, utf16Strings: representation, referenceSemantics: references || undefined, retainDiagnostics })
    : { outs: groups.map((group) => translateGroup(group, from, to, set, { utf16Strings: representation, referenceSemantics: references || undefined, retainDiagnostics })), scans: [], moduleScopeKnown: true, ownerCounts: new Map() };
  const results = translateBindingGroups(groups, bytes, from, to, set, outs, { moduleDirectory, moduleScopeKnown, moduleBindingNames: [...ownerCounts.keys()], utf16Strings: representation, referenceSemantics: references || undefined, retainDiagnostics });
  const exportedNames = new Set(scans.filter(scan => scan?.exported).map(scan => scan.declares));
  // An isolated item proof does not certify its actual mutable module binding.
  // Public/importable signatures need the same complete scope and unique owner
  // facts as an ordinary dependency edge. Preserve isolated item output below.
  const signatures = results.flatMap(({ out }) => out.status === 'translated' ? (out.signatures ?? []).filter(signature =>
    exportedNames.has(signature.name) && acceptDeclarationSignatureBinding(moduleScopeKnown, ownerCounts.get(signature.name) ?? 0)) : []);
  if (signaturesOnly) return { signatures };
  for (const { group, out } of results) {
    for (const prelude of out.preludes ?? []) if (!preludes.includes(prelude)) preludes.push(prelude);
    for (const { term, start, end } of group.items) recorded.push({ term, start, end, status: out.status, reason: out.reason ?? null,
      ...(out.diagnostic ? { diagnostic: out.diagnostic } : {}) });
    if (out.code !== null) {
      blocks.push(`${blocks.length ? breaks(gap) : ''}${out.code}`);
      gap = group.after;
    } else if (!gap) {
      // A dropped group keeps the layout before it.
      gap = group.after;
    }
  }
  const body = `${blocks.join('')}\n`;
  // An unedited translation back gives the source its header describes.
  const header = items.find((item) => item.comment && item.text.startsWith(HEADER));
  if (header && header.text.includes(` source=${to} `) && header.text.includes(` sha256=${sha256(body)} `)) {
    return { translation: freeze(from, to, body, recorded), signatures };
  }
  if (header) {
    const envelope = / envelope-sha256=([a-f0-9]{64}) original=(.*)$/u.exec(header.text);
    if (envelope) {
      try {
        const original = JSON.parse(envelope[2]);
        const length = Number(/(?:^| )bytes=(\d+)(?: |$)/u.exec(header.text)?.[1]);
        if (typeof original === 'string' && acceptSourceEnvelopeRestoration(
          header.text.includes(` source=${to} `),
          sha256(bytes.subarray(header.end).toString('utf8')) === envelope[1],
          Buffer.byteLength(original, 'utf8') === length &&
          header.text.includes(` sha256=${sha256(original)} `),
        )) return { translation: freeze(from, to, original, recorded), signatures };
      } catch (error) { if (!(error instanceof SyntaxError)) throw error; }
    }
    const originalLength = Number(/(?:^| )bytes=(\d+)(?: |$)/u.exec(header.text)?.[1]);
    const bodyBytes = Buffer.from(body, 'utf8');
    if (Number.isSafeInteger(originalLength) && originalLength >= 0 && originalLength <= bodyBytes.length) {
      const original = bodyBytes.subarray(0, originalLength).toString('utf8');
      const layout = bodyBytes.subarray(originalLength).toString('utf8');
      if (Buffer.byteLength(original, 'utf8') === originalLength && acceptSourcePrefixRestoration(
        header.text.includes(` source=${to} `),
        header.text.includes(` sha256=${sha256(original)} `),
        /^\s*$/u.test(layout),
      )) return { translation: freeze(from, to, original, recorded), signatures };
    }
  }
  if (family(to) === 'Rust' && recorded.some(({ status }) => status === 'translated')) preludes.unshift(RUST_ALLOW);
  const lines = [`${HEADER}source=${from} target=${to} sha256=${sha256(text)} bytes=${bytes.length}`, ''];
  if (preludes.length) lines.push(PRELUDE_BEGIN, ...preludes.flatMap((prelude, index) => (index ? ['', prelude] : [prelude])), PRELUDE_END, '');
  if (requireSelfTranslationLayoutEnvelope(text.includes('\r'),
    (results[0]?.group.items[0]?.start ?? bytes.length) > 0,
    !['', '\n'].includes(results.at(-1)?.group.after ?? text),
    results.slice(0, -1).some(({group}) => group.after !== breaks(group.after)))) {
    const tail = `\n${lines.slice(1).join('\n')}\n${body}`;
    const original = JSON.stringify(text).replaceAll('\u2028', '\\u2028').replaceAll('\u2029', '\\u2029');
    lines[0] += ` envelope-sha256=${sha256(tail)} original=${original}`;
  }
  return { translation: freeze(from, to, `${lines.join('\n')}\n${body}`, recorded), signatures };
}

/**
 * Translates ordinary recursive declaration components as atomic checked
 * scopes before publishing their actual signatures. Other items retain the
 * dependency binder: only signatures of already translated dependencies bind.
 * A failed component supplies no provisional peers to the ordinary fallback.
 */
function translateBound(groups, from, to, decorators, { moduleDirectory, imports, moduleSource, utf16Strings, referenceSemantics, retainDiagnostics }) {
  const scans = groups.map((group) => (group.kind === 'item' ? scanItem(group.text) : null));
  const owners = new Map();
  const ownerCounts = new Map();
  scans.forEach((scan, index) => {
    const names = scan?.imports ? scan.imports.names.map(({ local }) => local) : scan?.declares ? [scan.declares] : [];
    for (const name of names) {
      ownerCounts.set(name, (ownerCounts.get(name) ?? 0) + 1);
      if (!owners.has(name)) owners.set(name, index);
    }
  });
  // These are conservative token dependency candidates, never a semantic
  // substitute for checking every complete body and its lexical ownership.
  const starts = [], counts = [], targets = [];
  for (const scan of scans) {
    starts.push(targets.length);
    if (scan && !scan.imports) for (const name of scan.mentions) {
      if (owners.has(name)) targets.push(owners.get(name));
    }
    counts.push(targets.length - starts.at(-1));
  }
  const leaders = readDeclarationDependencyComponents(starts, counts, targets);
  const components = new Map();
  leaders.forEach((leader, index) => {
    if (!components.has(leader)) components.set(leader, []);
    components.get(leader).push(index);
  });
  const attempted = new Set();
  const moduleBindingNames = [...owners.keys()];
  const outs = new Array(groups.length);
  const visiting = new Set();
  let moduleScopeKnown = false;
  const visit = (index) => {
    if (outs[index] || visiting.has(index)) return;
    const leader = leaders[index], members = components.get(leader);
    if (!attempted.has(leader) && acceptRecursiveDeclarationGroup(members.length,
      members.every(member => scans[member]?.ordinary === true),
      moduleScopeKnown && members.every(member => ownerCounts.get(scans[member]?.declares) === 1))) {
      attempted.add(leader);
      const memberSet = new Set(members);
      for (const member of members) visiting.add(member);
      const externals = new Map();
      for (const member of members) for (const name of scans[member].mentions) {
        const owner = owners.get(name);
        if (owner === undefined || memberSet.has(owner)
          || !acceptDeclarationSignatureBinding(moduleScopeKnown, ownerCounts.get(name) ?? 0)) continue;
        visit(owner);
        if (outs[owner]?.status !== 'translated' || scans[owner]?.async) continue;
        const signature = outs[owner].signatures?.find(candidate => candidate.name === name);
        if (signature && acceptDeclarationSignature(signature.k === 'const', signature.literal ?? false)) externals.set(name, signature);
      }
      const completed = translateRecursiveDeclarationGroup(members, groups, scans, from, to, decorators,
        { externals: [...externals.values()], moduleDirectory, moduleBindingNames, moduleScopeKnown, utf16Strings, referenceSemantics, retainDiagnostics });
      for (const member of members) visiting.delete(member);
      if (completed !== null) {
        // No checked peer signature or member output is visible until every
        // original body, signature and native mapping in this batch passes.
        for (const [member, out] of completed) outs[member] = out;
        return;
      }
      // Keep the ordinary dependency binder's original independent proofs
      // and refusals; no candidate peer signatures enter this fallback.
    }
    const scan = scans[index];
    if (!scan) {
      outs[index] = translateGroup(groups[index], from, to, decorators, { moduleBindingNames, moduleScopeKnown, utf16Strings, referenceSemantics, retainDiagnostics });
      return;
    }
    visiting.add(index);
    const externals = [];
    if (scan.imports) {
      const { specifier, names } = scan.imports;
      const provided = Object.hasOwn(imports, specifier) ? imports[specifier] : [];
      for (const { imported, local } of names) {
        const signature = provided.find(({ name }) => name === imported);
        if (signature && acceptDeclarationSignature(signature.k === 'const', signature.literal ?? false)) externals.push({ ...signature, name: local });
      }
    } else {
      for (const name of scan.mentions) {
        const owner = owners.get(name);
        if (owner === undefined || owner === index
          || !acceptDeclarationSignatureBinding(moduleScopeKnown, ownerCounts.get(name) ?? 0)) continue;
        visit(owner);
        if (outs[owner]?.status !== 'translated' || scans[owner].async) continue;
        const signature = outs[owner].signatures.find((candidate) => candidate.name === name);
        if (signature && acceptDeclarationSignature(signature.k === 'const', signature.literal ?? false)) externals.push(signature);
      }
    }
    outs[index] = translateGroup(groups[index], from, to, decorators, { externals, moduleDirectory, moduleBindingNames, moduleScopeKnown, utf16Strings, referenceSemantics, retainDiagnostics });
    visiting.delete(index);
  };
  // Named imports are dependency leaves. Only their actual normal translated
  // signatures are available to the optional complete-module safety proof.
  for (let index = 0; index < scans.length; index += 1) if (scans[index]?.imports) visit(index);
  const imported = outs.flatMap((out, index) => scans[index]?.imports && out?.status === 'translated' ? out.signatures ?? [] : []);
  moduleScopeKnown = readDeclarationModuleScope(groups, scans, from, to, decorators,
    { moduleDirectory, moduleSource, externals: imported, referenceSemantics, ownerCounts, moduleBindingNames });
  groups.forEach((_, index) => visit(index));
  return { outs, scans, moduleScopeKnown, ownerCounts };
}

function readDeclarationModuleScope(groups, scans, from, to, decorators, context) {
  // These counts come from every actual binding in the cheap admitted forms:
  // one named function/class, named import locals, or one certified literal.
  // Unknown forms still require the real complete-module check below. A
  // duplicate cannot be made unique by omitting another declaration or effect.
  for (const count of context.ownerCounts.values()) {
    if (!acceptDeclarationSignatureBinding(true, count)) return false;
  }
  const { literalGroups } = collectLiteralBindings(groups, true, from, to, decorators,
    { moduleBindingNames: context.moduleBindingNames });
  const forms = groups.map((group, index) => group.kind === 'comment' ? 'comment'
    : group.kind !== 'item' || group.term === 'ERROR' ? 'unknown module form'
    : scans[index]?.functionDeclaration ? 'named function declaration'
    : scans[index]?.imports ? 'named import'
    : scans[index]?.emptyClassDeclaration ? 'empty class declaration'
    : scans[index]?.deferredBinding?.kind === 1 ? 'deferred named class declaration'
    : scans[index]?.deferredBinding?.kind === 2 ? 'deferred function value binding'
    : literalGroups.has(group) ? 'primitive literal binding' : 'unknown module form');
  if (acceptDeclarationModuleScope(forms, false) || acceptDeclarationDeferredModuleScope(forms)) return true;
  if (typeof context.moduleSource !== 'string') return false;
  try {
    const source = from === 'TypeScript' ? projectTypeScriptTyping(context.moduleSource).code : context.moduleSource;
    // Check the real complete module, including every initializer/effect and
    // body. No joined subset, placeholder, fake peer signature or execution.
    checkProgram(parseJavaScript(source, { externals: context.externals, moduleDirectory: context.moduleDirectory,
      referenceSemantics: context.referenceSemantics, readonlyModuleConstants: true }));
    return acceptDeclarationModuleScope(forms, true);
  } catch (error) {
    if (!(error instanceof TranslationError)) throw error;
    return false;
  }
}

function translateRecursiveDeclarationGroup(members, groups, scans, from, to, decorators, context) {
  const names = members.map(member => scans[member].declares);
  const source = members.map(member => groups[member].text).join('\n\n');
  const combined = { ...groups[members[0]], text: source,
    items: members.flatMap(member => groups[member].items), after: groups[members.at(-1)].after };
  const checked = {};
  const whole = translateGroup(combined, from, to, decorators, context, checked);
  if (whole.status !== 'translated') return null;
  const declarations = [...checked.program.declarations.values()];
  const signatures = whole.signatures ?? [];
  const flat = checked.program.main.effects.length === 0 && !checked.program.imports?.length
    && declarations.every(entry => entry.k === 'fn' && entry.modulePath.length === 0 && !entry.generated && names.includes(entry.name))
    && new Set(declarations.map(entry => entry.name)).size === names.length
    && signatures.every(signature => signature.k === 'fn' && names.includes(signature.name))
    && new Set(signatures.map(signature => signature.name)).size === names.length;
  if (!acceptRecursiveDeclarationPublication(names.length, declarations.length, signatures.length, flat, true, true)) {
    return translateOwnedRecursiveDeclarationGroup(members, groups, scans, from, to, decorators, context, checked);
  }
  const wholeTargets = checked.emitted.mappings.filter(mapping => mapping.kind === 'function' && names.includes(mapping.source));
  if (wholeTargets.length !== names.length || new Set(wholeTargets.map(mapping => mapping.target)).size !== names.length) return null;
  const completed = new Map();
  for (const member of members) {
    const name = scans[member].declares;
    const peers = signatures.filter(signature => signature.name !== name);
    const capture = {};
    const out = translateGroup(groups[member], from, to, decorators,
      { ...context, externals: [...context.externals, ...peers] }, capture);
    if (out.status !== 'translated') return null;
    const own = out.signatures?.filter(signature => signature.name === name) ?? [];
    const expected = signatures.filter(signature => signature.name === name);
    const actualTargets = capture.emitted.mappings.filter(mapping => mapping.kind === 'function');
    const target = wholeTargets.find(mapping => mapping.source === name);
    const matching = isDeepStrictEqual(own, expected)
      && capture.program.declarations.size === 1 && actualTargets.length === 1
      && actualTargets[0].source === name && actualTargets[0].target === target?.target;
    if (!acceptRecursiveDeclarationPublication(names.length, declarations.length, signatures.length, flat, true, matching)) return null;
    completed.set(member, out);
  }
  return completed.size === members.length ? completed : null;
}

// Auxiliary admission keeps the original flat publication rule unchanged.
// Every published complete block comes from one coherent WHOLE emission.
function translateOwnedRecursiveDeclarationGroup(members, groups, scans, from, to, decorators, context, wholeCapture) {
  if (from !== 'JavaScript' || to !== 'Rust') return null;
  const names = members.map(member => scans[member].declares);
  const sources = members.map(member => groups[member].text);
  const signatures = wholeCapture.signatures ?? [];
  const captures = [];
  for (let owner = 0; owner < members.length; owner += 1) {
    const peers = signatures.filter(signature => signature.name !== names[owner]);
    const capture = {};
    const original = translateGroup(groups[members[owner]], from, to, decorators,
      {...context, externals: [...context.externals, ...peers]}, capture);
    if (original.status !== 'translated') return null;
    captures.push(capture);
  }
  const plan = planOwnedRecursiveDeclarationPublication(names, sources, wholeCapture, captures, context.moduleScopeKnown);
  if (plan === null) return null;
  const staged = new Map();
  for (let owner = 0; owner < members.length; owner += 1) {
    const member = members[owner], group = groups[member];
    const definitions = plan.indices[owner].map(index => wholeCapture.emitted.definitions[index]);
    // No empty successful peer, sliced source, helper rename or fake header.
    const record = {};
    const out = recordTranslatedDefinitions(group.text, group.term, from, to, decorators,
      definitions, wholeCapture.emitted.preludes,
      signatures.filter(signature => signature.name === names[owner]), record);
    if (out.status !== 'translated' || record.undecorated !== true) return null;
    staged.set(member, out);
  }
  return staged.size === members.length ? staged : null;
}

/**
 * What a JavaScript item declares and names, read from its tokens: the name
 * of the function or constant it declares, whether it is exported or async,
 * the identifiers it mentions, and the specifier and names of a named import.
 * Null when the item does not tokenize.
 */
function scanItem(text) {
  let tokens;
  try {
    ({ tokens } = tokenize(text, 'JavaScript'));
  } catch (error) {
    if (error instanceof TranslationError) return null;
    throw error;
  }
  const at = (index) => tokens[Math.min(index, tokens.length - 1)];
  const word = (index, value) => at(index).kind === 'identifier' && at(index).value === value;
  const exported = word(0, 'export');
  let index = exported ? 1 : 0;
  let declares = null;
  let isAsync = false;
  let ordinary = false;
  let functionDeclaration = false;
  let emptyClassDeclaration = null;
  let imports = null;
  const deferredCandidate = word(index, 'class') || word(index, 'const') && at(index + 1).kind === 'identifier'
    && at(index + 2).value === '=' && (word(index + 3, 'function') || at(index + 3).value === '('
      || at(index + 3).kind === 'identifier' && at(index + 4).value === '=>');
  const deferredBinding = deferredCandidate ? readDeferredDeclarationBinding(text) : null;
  if (word(index, 'async') && word(index + 1, 'function')) {
    isAsync = true;
    index += 1;
  }
  if (word(index, 'function') && at(index + 1).kind === 'identifier') {
    declares = at(index + 1).value;
    ordinary = !isAsync;
    functionDeclaration = true;
  } else if (word(index, 'class') && at(index + 1).kind === 'identifier') {
    // Count the actual class owner even when a wider class form refuses.
    declares = at(index + 1).value;
    if (at(index + 2).kind === 'punct' && at(index + 2).value === '{'
      && at(index + 3).kind === 'punct' && at(index + 3).value === '}'
      && at(index + 4).kind === 'eof') {
      // Exact actual token ranges prove an inert empty named class definition.
      // The real class stays carried. No inheritance, method, field, static
      // block, computed key, decorator or other eager expression is omitted.
      // Comments are already retained by the concrete source and skipped only
      // by the ordinary tokenizer; they cannot add an executable class member.
      declares = at(index + 1).value;
      emptyClassDeclaration = { start: at(index).start, end: at(index + 3).end,
        nameStart: at(index + 1).start, nameEnd: at(index + 1).end };
    }
  } else if (word(index, 'const') && at(index + 1).kind === 'identifier' && at(index + 2).value === '=') {
    declares = at(index + 1).value;
    isAsync = word(index + 3, 'async');
  } else if (word(0, 'import') && at(1).value === '{') {
    const names = [];
    let next = 2;
    while (at(next).kind === 'identifier') {
      const imported = at(next).value;
      const aliased = word(next + 1, 'as') && at(next + 2).kind === 'identifier';
      names.push({ imported, local: aliased ? at(next + 2).value : imported });
      next += aliased ? 3 : 1;
      if (at(next).value !== ',') break;
      next += 1;
    }
    if (at(next).value === '}' && word(next + 1, 'from') && at(next + 2).kind === 'string') imports = { specifier: at(next + 2).value, names };
  }
  const mentions = [...new Set(tokens.filter((token) => token.kind === 'identifier').map((token) => token.value))];
  if (deferredBinding && deferredBinding.name !== declares) return null;
  return { declares, exported, async: isAsync, ordinary, functionDeclaration, emptyClassDeclaration, deferredBinding, mentions, imports };
}

/**
 * The signatures a translated item gives the other items of its module: its
 * functions at the top level, other than the ones the translator makes up,
 * or its constant, when no data type is among their types and no parameter
 * is a guarded natural number.
 */
function declaredSignatures(program, constants) {
  const portable = (type) => (type.kind === 'array' ? portable(type.element) : type.kind !== 'data');
  if (constants) {
    const [{ name, value }] = program.main.effects;
    return portable(value.type) ? [{ k: 'const', name, type: value.type, literal: value.k === 'lit' }] : [];
  }
  return [...program.declarations.values()]
    .filter((entry) => entry.k === 'fn' && entry.modulePath.length === 0 && !entry.name.startsWith('ml_')
      && entry.params.every((param) => !param.guard && portable(param.type)) && portable(entry.ret))
    .map((entry) => ({ k: 'fn', name: entry.name, params: entry.params.map(({ name, type }) => ({ name, type })), ret: entry.ret }));
}

function freeze(sourceLanguage, targetLanguage, code, items) {
  return Object.freeze({
    sourceLanguage,
    targetLanguage,
    code,
    items: Object.freeze(items.map((item) => Object.freeze(item))),
  });
}

// The layout between two emitted blocks: its line breaks, at least one.
function breaks(gap) {
  return '\n'.repeat(Math.max(1, (gap.match(/\n/gu) ?? []).length));
}

/** The top-level items of `text`, with their text and the layout after each. */
function topLevelItems(bytes, parsed) {
  const root = parsed?.tree;
  const nodes = root && acceptRootSyntaxItem(root.term, root.children.length > 0, root.span.byteRange.end > root.span.byteRange.start) ? [root] : root?.children ?? [];
  const children = nodes
    .filter((child) => child.term !== 'whitespace')
    .map((child) => {
      const { start } = child.span.byteRange;
      let { end } = child.span.byteRange;
      // A Rust line comment ends with its line break, which is layout here.
      if (COMMENTS.has(child.term) && bytes[end - 1] === 0x0a) end -= bytes[end - 2] === 0x0d ? 2 : 1;
      return { term: child.term, start, end };
    });
  return children.map((item, index) => ({
    ...item,
    comment: COMMENTS.has(item.term),
    text: bytes.subarray(item.start, item.end).toString('utf8'),
    after: bytes.subarray(item.end, children[index + 1]?.start ?? bytes.length).toString('utf8'),
  }));
}

const lineBreak = (layout) => /^\r?\n$/u.test(layout);
const sourceLines = (lines) => lines.map(({ text }) => text.slice(SOURCE_LINE.length).replace(/^ /u, ''));

/**
 * Groups the items: a self-translation header, a prelude block, a carried or
 * translated item with its provenance, and an item with the comments directly
 * before it are one group each.
 */
function groupItems(items, bytes) {
  const groups = [];
  const textBetween = (first, last) => bytes.subarray(first.start, last.end).toString('utf8');
  for (let index = 0; index < items.length; index += 1) {
    const item = items[index];
    const take = (end, fields) => {
      groups.push({ ...fields, items: items.slice(index, end + 1), after: items[end].after });
      index = end;
    };
    // The `// |` source lines that follow the item at `at` line by line.
    const linesAfter = (at) => {
      let end = at;
      while (end + 1 < items.length && items[end + 1].comment && items[end + 1].text.startsWith(SOURCE_LINE) && lineBreak(items[end].after)) end += 1;
      return end;
    };
    if (item.comment && item.text.startsWith(HEADER)) {
      take(index, { kind: 'provenance' });
    } else if (item.comment && item.text === PRELUDE_BEGIN) {
      let end = index;
      while (end + 1 < items.length && items[end].text !== PRELUDE_END) end += 1;
      take(end, { kind: 'provenance' });
    } else if (item.comment && item.text.startsWith(CARRIED)) {
      const end = linesAfter(index);
      const [language] = item.text.slice(CARRIED.length).split(' ');
      take(end, { kind: 'carried', language, marker: item.text, lines: sourceLines(items.slice(index + 1, end + 1)) });
    } else if (item.comment && item.text.startsWith(TRANSLATED)) {
      const fields = Object.fromEntries(item.text.slice(TRANSLATED.length).split(' ').slice(2).map((field) => field.split('=')));
      const linesEnd = linesAfter(index);
      if (fields.items === '0' && fields.sha256 === sha256('')) {
        const [language] = item.text.slice(TRANSLATED.length).split(' ');
        take(linesEnd, { kind: 'carried', language, lines: sourceLines(items.slice(index + 1, linesEnd + 1)) });
        continue;
      }
      const last = definitionsEnd(items, linesEnd, Number(fields.items));
      if (last >= 0) {
        const code = textBetween(items[linesEnd + 1], items[last]);
        const [language] = item.text.slice(TRANSLATED.length).split(' ');
        const lines = sourceLines(items.slice(index + 1, linesEnd + 1));
        if (sha256(code) === fields.sha256) {
          take(last, { kind: 'carried', language, lines });
          continue;
        }
        // An edited translation is translated again; its provenance is dropped.
        take(linesEnd, { kind: 'provenance' });
        continue;
      }
      take(index, { kind: 'comment', text: item.text, term: item.term });
    } else {
      // Comments directly before an item document it and travel with it.
      let end = index;
      while (items[end].comment && end + 1 < items.length && lineBreak(items[end].after) && !isMarker(items[end + 1])) end += 1;
      // A run of comments no item follows is one comment group.
      take(end, { kind: items[end].comment ? 'comment' : 'item', text: textBetween(item, items[end]), term: items[end].term });
    }
  }
  return groups;
}

/**
 * The last item of the `count` definitions after the item at `after`, or -1:
 * a definition's attributes (`#[derive(…)]`) are items of their own in Rust.
 */
function definitionsEnd(items, after, count) {
  if (!Number.isSafeInteger(count) || count <= 0) return -1;
  let left = count;
  for (let at = after + 1; at < items.length; at += 1) {
    if (items[at].term !== 'attribute_item' && !items[at].comment) left -= 1;
    if (left === 0) return at;
  }
  return -1;
}

const isMarker = (item) => item.comment && [HEADER, CARRIED, TRANSLATED, PRELUDE_BEGIN].some((marker) => item.text.startsWith(marker));

// A failed isolated binding can refer to another declaration in its run.
// Retry the complete run as one checked scope. A failed run keeps the original
// per-item decisions, so a carried sibling is never represented by a stub.
function translateBindingGroups(groups, bytes, from, to, decorators, initial, context) {
  const bind = family(from) === 'JavaScript' && family(to) === 'Rust';
  if (bind && context.moduleScopeKnown === false) return groups.map((group, index) => ({ group, out: initial[index] }));
  const { literals, literalGroups } = collectLiteralBindings(groups, bind, from, to, decorators, context);
  const terms = groups.map((group) => group.kind === 'item' && !literalGroups.has(group) ? group.term : '');
  const results = [];
  for (let index = 0; index < groups.length;) {
    const end = bind ? findBindingRunEnd(terms, index) : index + 1;
    const run = groups.slice(index, end);
    const isolated = run.map((group, offset) => ({ group, out: initial[index + offset] }));
    if (acceptBindingScope(isolated.map(({ out }) => out.status), isolated.map(({ out }) => out.reason ?? ''))) {
      const items = run.flatMap((group) => group.items);
      const combined = { ...run[0], items, text: bytes.subarray(items[0].start, items.at(-1).end).toString('utf8'), after: run.at(-1).after };
      const out = translateGroup(combined, from, to, decorators, { ...context, literalBindings: literals });
      if (out.status === 'translated') results.push({ group: combined, out });
      else results.push(...isolated);
    } else results.push(...isolated);
    index = end;
  }
  const moduleTerms = groups.map((group) => group.kind === 'item' ? group.term : group.kind === 'comment' ? '' : 'blocked');
  if (bind && acceptModuleBindingScope(moduleTerms, results.some(({ out }) => out.status === 'carried'))) {
    const items = groups.flatMap((group) => group.items);
    const combined = { kind: 'item', term: groups[0].term, items, text: bytes.subarray(items[0].start, items.at(-1).end).toString('utf8'), after: groups.at(-1).after };
    const out = translateGroup(combined, from, to, decorators, { ...context, literalBindings: literals });
    if (out.status === 'translated') return [{ group: combined, out }];
  }
  return results;
}

function collectLiteralBindings(groups, bind, from, to, decorators, context = {}) {
  const literals = new Map();
  const literalGroups = new Set();
  if (bind) for (const group of groups) {
    if (group.kind !== 'item') continue;
    try {
      const parsed = parseJavaScript(from === 'TypeScript' ? projectTypeScriptTyping(group.text).code : group.text, { ...context, readonlyModuleConstants: family(to) === 'Rust' });
      const effects = parsed.main.effects;
      const effect = effects[0];
      if (effect && acceptLiteralBinding(effect.value?.k ?? '', effect.constant ?? false, effects.length, parsed.items.length)) {
        if (translateGroup(group, from, to, decorators, context).status !== 'translated') continue;
        literals.set(effect.name, effect.value);
        literalGroups.add(group);
      }
    } catch (error) { if (!(error instanceof TranslationError)) throw error; }
  }
  return { literals, literalGroups };
}

function translateGroup(group, from, to, decorators, context = {}, capture = null) {
  if (group.kind === 'provenance') return { code: null, status: 'provenance' };
  if (group.kind === 'carried') {
    if (family(group.language) === family(to)) return { code: group.lines.join('\n'), status: 'restored' };
    const marker = group.items[0].text;
    return { code: [marker, ...group.lines.map(sourceLine)].join('\n'), status: 'carried', reason: 'carried from another language' };
  }
  const { text, term } = group;
  if (term === 'ERROR') return carry(text, term, from, 'syntax');
  if (group.kind === 'comment') {
    // A copied comment has no provenance, so it must also fit the source.
    if (group.items.every((item) => commentFits(item.text, from) && commentFits(item.text, to))) return { code: text, status: 'comment' };
    return carry(text, term, from, 'comment the target cannot hold');
  }
  let emitted;
  let signatures;
  let program;
  try {
    if (from === 'TypeScript' && to === 'JavaScript') {
      const erased = eraseTypeScript(text);
      const erasedCode = erased.code.trim();
      const count = topLevelItems(Buffer.from(erasedCode), parseProgrammingLanguage(erasedCode, 'JavaScript')).filter((item) => !item.comment).length;
      const code = count === 0 ? '' : erasedCode;
      return {
        code: [`${TRANSLATED}${from} ${term} items=${count} sha256=${sha256(code)}`,
          ...text.split(/\r?\n/u).map(sourceLine), ...(code ? [code] : [])].join('\n'),
        status: 'translated',
      };
    }
    // The Rust frontend reads a program, so an item alone gets an empty main.
    program = checkProgram(family(from) === 'Rust'
      ? parseRust(/\bfn\s+main\s*\(/u.test(text) ? text : `${text}\nfn main() {}\n`)
      : parseJavaScript(from === 'TypeScript' ? projectTypeScriptTyping(text).code : text, { ...context, readonlyModuleConstants: family(to) === 'Rust' }));
    if (context.utf16Strings) program.utf16Strings = true;
    // A top-level constant is a Rust constant; other top-level statements run once, as a program.
    const constants = family(to) === 'Rust' ? emitRustConstants(program) : null;
    if (!constants && program.main.effects.length > 0) return carry(text, term, from, 'top-level statement');
    emitted = constants ?? (family(to) === 'Rust' ? emitRust : emitJavaScript)(program, family(to) === 'Rust' ? {arrayConsumption: true, recordDefinitionOwnership: capture !== null} : {});
    signatures = program.imports ? context.externals : declaredSignatures(program, constants);
  } catch (error) {
    if (!(error instanceof TranslationError)) throw error;
    const carried = carry(text, term, from, from === 'TypeScript' ? error.reason : error.kind);
    if (context.retainDiagnostics === true) carried.diagnostic = {
      kind: error.kind, reason: error.reason, message: error.message, span: error.span, details: error.details,
      sourceRange: { start: group.items[0].start, end: group.items.at(-1).end },
    };
    return carried;
  }
  if (emitted.definitions.length === 0) return carry(text, term, from, 'no definition');
  if (capture !== null) {
    capture.program = program; capture.emitted = emitted; capture.signatures = signatures;
    capture.source = text; capture.sourceLanguage = from;
  }
  return recordTranslatedDefinitions(text, term, from, to, decorators, emitted.definitions, emitted.preludes, signatures, capture);
}

function recordTranslatedDefinitions(text, term, from, to, decorators, definitions, preludes, signatures, capture = null) {
  const exported = family(to) === 'JavaScript' && (family(from) === 'Rust' ? /^pub(\([^)]*\))?\s/mu : /^export\s/mu).test(text);
  const generic = definitions.map((definition) => (exported ? `export ${definition}` : definition)).join('\n\n');
  const code = decorateEmitted(to, { source: generic }, decorators).source;
  if (code.trim() === '') return carry(text, term, from, 'dropped by a decorator');
  if (capture !== null) capture.undecorated = code === generic;
  const count = definitions.length;
  const marker = `${TRANSLATED}${from} ${term} items=${count} sha256=${sha256(code)}`;
  return {
    code: [marker, ...text.split(/\r?\n/u).map(sourceLine), code].join('\n'),
    preludes,
    status: 'translated',
    signatures,
  };
}

function carry(text, term, from, reason) {
  return {
    code: [`${CARRIED}${from} ${term} (${reason})`, ...text.split(/\r?\n/u).map(sourceLine)].join('\n'),
    status: 'carried',
    reason,
  };
}

const sourceLine = (line) => (line === '' ? SOURCE_LINE : `${SOURCE_LINE} ${line}`);

// A comment keeps its text when the target reads it as an ordinary comment:
// a Rust doc comment needs an item after it, and Rust block comments nest.
function commentFits(text, to) {
  if (text.startsWith('/*') && /\/\*|\*\//u.test(text.slice(2, -2))) return false;
  return family(to) !== 'Rust' || !/^(\/\/[/!]|\/\*[*!])/u.test(text);
}
