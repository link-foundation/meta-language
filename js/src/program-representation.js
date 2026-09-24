import { languageSupport } from './language-support.js';
import { LinkNetwork } from './network.js';
import { LinkType } from './primitives.js';
import { createProgramSnapshot, readProgramSnapshot } from './program-snapshot.js';

/** Schema revision for resolved four-language program representations. */
export const PROGRAM_REPRESENTATION_SCHEMA_VERSION = 1;

export const SEMANTIC_CONSTRUCTS = Object.freeze([
  'modules-and-imports',
  'scopes-and-bindings',
  'recursive-definitions',
  'types-and-universes',
  'effects',
  'attributes',
  'macros-and-notation',
  'proof-terms-and-tactics',
  'surface-expansion-elaboration-traces',
  'project-context-and-dependencies',
]);

export class BindingRenameError extends Error {}
export class ProgramTransformationError extends Error {}

/**
 * Parses source into a CST-backed program model with scopes, bindings, source
 * mappings, language-specific constructs, and project provenance.
 */
export function analyzeProgram(source, language, project = {}) {
  return new ProgramRepresentation(source, language, project);
}

/** Constructs a program and rejects syntax that cannot be represented cleanly. */
export function constructProgram(source, language, project = {}) {
  const program = new ProgramRepresentation(source, language, project);
  if (!program.network.verifyFullMatch().isClean()) {
    throw new ProgramTransformationError('constructed program does not parse cleanly');
  }
  return program;
}

/** Constructs a clean program from ordered structured source fragments. */
export function constructProgramFromFragments(fragments, language, project = {}) {
  if (!fragments || typeof fragments[Symbol.iterator] !== 'function') {
    throw new ProgramTransformationError('program fragments must be iterable');
  }
  return constructProgram([...fragments].map(String).join(''), language, project);
}

export const analyze_program = analyzeProgram;

export class ProgramRepresentation {
  /** Reloads a validated snapshot without access to its original source buffer. */
  static fromSnapshot(snapshot) {
    const { source, language, project } = readProgramSnapshot(snapshot);
    return constructProgram(source, language, project);
  }

  constructor(source, language, project = {}) {
    const support = languageSupport(language);
    if (!support) {
      throw new TypeError(`program semantics are not registered for ${language}`);
    }
    this.schemaVersion = PROGRAM_REPRESENTATION_SCHEMA_VERSION;
    this.language = support.name;
    this.source = String(source);
    this.project = normalizeProject(project);
    this.network = LinkNetwork.parse(this.source, this.language);

    const syntax = syntaxFacts(this.network, this.source);
    const tokens = semanticTokens(this.source, this.language, syntax);
    const resolution = resolveBindings(tokens, this.source, this.language);
    this.scopes = Object.freeze(resolution.scopes.map(freezeRecord));
    this.bindings = Object.freeze(resolution.bindings.map(freezeBinding));
    this.unresolvedReferences = Object.freeze(resolution.unresolved.map(freezeRecord));
    this.sourceMappings = Object.freeze(syntax.map(freezeRecord));
    this.modules = Object.freeze(moduleFacts(tokens, this.language, this.project).map(freezeRecord));
    this.types = Object.freeze(typeFacts(tokens, syntax, this.language).map(freezeRecord));
    this.extensions = Object.freeze(extensionFacts(tokens, syntax, this.source, this.language).map(freezeRecord));
    this.proofs = Object.freeze(proofFacts(tokens, syntax, this.language).map(freezeRecord));
    this.diagnostics = Object.freeze([
      ...diagnosticFacts(this.network),
      ...projectDiagnostics(this.modules, this.project),
    ].map(freezeRecord));
    this.constructs = Object.freeze(constructFacts(this).map(freezeConstruct));
    Object.freeze(this.project);
  }

  emit() {
    return this.network.reconstructText();
  }

  /** Returns a source-buffer-independent snapshot of retained token fragments. */
  snapshot() {
    return createProgramSnapshot(this);
  }

  /** Serializes a source-buffer-independent snapshot. */
  serializeSnapshot() {
    return JSON.stringify(this.snapshot());
  }

  /** Returns exact ranges of CST nodes with the requested grammar term. */
  querySyntax(term) {
    return Object.freeze(this.sourceMappings
      .filter((mapping) => mapping.term === term)
      .map(({ start, end }) => Object.freeze({ start, end })));
  }

  query_syntax(term) {
    return this.querySyntax(term);
  }

  /** Replaces one exact source range and reparses the result. */
  replace(range, replacement) {
    const { start, end } = this.#validatedRange(range);
    return this.#reparse(`${this.source.slice(0, start)}${replacement}${this.source.slice(end)}`);
  }

  /** Inserts target-language source at an exact source boundary. */
  insert(offset, inserted) {
    this.#validatedRange({ start: offset, end: offset });
    return this.#reparse(`${this.source.slice(0, offset)}${inserted}${this.source.slice(offset)}`);
  }

  /** Deletes one exact source range. */
  delete(range) {
    return this.replace(range, '');
  }

  /** Clones one exact source range at an exact source boundary. */
  clone(range, destination) {
    const { start, end } = this.#validatedRange(range);
    this.#validatedRange({ start: destination, end: destination });
    return this.insert(destination, this.source.slice(start, end));
  }

  /** Moves one exact source range using offsets from the original source. */
  move(range, destination) {
    const { start, end } = this.#validatedRange(range);
    this.#validatedRange({ start: destination, end: destination });
    if (destination > start && destination < end) {
      throw new ProgramTransformationError('move destination is inside the moved range');
    }
    if (destination === start || destination === end) return this;
    const fragment = this.source.slice(start, end);
    const without = `${this.source.slice(0, start)}${this.source.slice(end)}`;
    const adjusted = destination > end ? destination - (end - start) : destination;
    return this.#reparse(`${without.slice(0, adjusted)}${fragment}${without.slice(adjusted)}`);
  }

  /** Renames exactly one resolved binding and all of its references. */
  renameBinding(bindingId, replacement) {
    const binding = this.bindings.find(({ id }) => id === bindingId);
    if (!binding) {
      throw new BindingRenameError(`unknown binding ${bindingId}`);
    }
    validateIdentifier(replacement, this.language);
    if (binding.name === replacement) {
      return this;
    }
    const conflicting = this.bindings.find((candidate) =>
      candidate.id !== binding.id &&
      candidate.name === replacement &&
      rangesOverlap(scopeFor(this.scopes, binding.scope), scopeFor(this.scopes, candidate.scope))
    );
    if (conflicting) {
      throw new BindingRenameError(
        `rename would capture ${replacement} at ${conflicting.declaration.start} (binding conflict)`,
      );
    }

    const ranges = [binding.declaration, ...binding.references]
      .map(({ start, end }) => ({ start, end }))
      .sort((left, right) => right.start - left.start);
    let edited = this.source;
    for (const range of ranges) {
      edited = `${edited.slice(0, range.start)}${replacement}${edited.slice(range.end)}`;
    }
    const reparsed = new ProgramRepresentation(edited, this.language, this.project);
    if (!reparsed.network.verifyFullMatch().isClean()) {
      throw new BindingRenameError('renamed program does not reparse cleanly');
    }
    return reparsed;
  }

  rename_binding(bindingId, replacement) {
    return this.renameBinding(bindingId, replacement);
  }

  #validatedRange(range) {
    const start = Number(range?.start);
    const end = Number(range?.end);
    if (!Number.isSafeInteger(start) || !Number.isSafeInteger(end) || start < 0 || end < start || end > this.source.length) {
      throw new ProgramTransformationError(`invalid source range ${start}..${end}`);
    }
    if (!codePointBoundary(this.source, start) || !codePointBoundary(this.source, end)) {
      throw new ProgramTransformationError(`source range ${start}..${end} splits a Unicode code point`);
    }
    return { start, end };
  }

  #reparse(source) {
    const reparsed = new ProgramRepresentation(source, this.language, this.project);
    if (!reparsed.network.verifyFullMatch().isClean()) {
      throw new ProgramTransformationError('structured edit does not reparse cleanly');
    }
    return reparsed;
  }

  normalized() {
    return {
      schemaVersion: this.schemaVersion,
      language: this.language,
      scopes: this.scopes,
      bindings: this.bindings,
      unresolvedReferences: this.unresolvedReferences,
      modules: this.modules,
      types: this.types,
      extensions: this.extensions,
      proofs: this.proofs,
      diagnostics: this.diagnostics,
      constructs: this.constructs,
      project: this.project,
    };
  }
}

function codePointBoundary(source, offset) {
  if (offset <= 0 || offset >= source.length) return true;
  const unit = source.charCodeAt(offset);
  return unit < 0xdc00 || unit > 0xdfff;
}

function syntaxFacts(network, source) {
  const byteToOffset = byteOffsetMap(source);
  return network.links()
    .filter((link) => link.metadata().linkType === LinkType.Syntax && link.metadata().span)
    .map((link) => {
      const { start, end } = link.metadata().span.byteRange;
      return {
        linkId: link.id().asU64(),
        term: link.metadata().term,
        start: byteToOffset.get(start),
        end: byteToOffset.get(end),
        byteStart: start,
        byteEnd: end,
      };
    })
    .filter(({ start, end }) => start !== undefined && end !== undefined);
}

function semanticTokens(source, language, syntax) {
  const mask = codeMask(source, language);
  for (const fact of syntax) {
    if (/regex/u.test(fact.term) && language === 'JavaScript') {
      mark(mask, fact.start, fact.end, false);
    }
  }
  const keywords = KEYWORDS[language];
  const tokens = [];
  for (let offset = 0; offset < source.length;) {
    const character = codePointAt(source, offset);
    if (!mask[offset] || /\s/u.test(character)) {
      offset += character.length;
      continue;
    }
    if (identifierStart(character, language)) {
      const start = offset;
      offset += character.length;
      while (offset < source.length) {
        const next = codePointAt(source, offset);
        if (!mask[offset] || !identifierContinue(next, language)) break;
        offset += next.length;
      }
      const text = source.slice(start, offset);
      tokens.push({
        kind: keywords.has(text) ? 'keyword' : 'identifier',
        text,
        start,
        end: offset,
      });
      continue;
    }
    const operator = OPERATORS.find((candidate) => source.startsWith(candidate, offset));
    const end = offset + (operator?.length ?? character.length);
    tokens.push({ kind: 'punctuation', text: operator ?? character, start: offset, end });
    offset = end;
  }
  return tokens;
}

function resolveBindings(tokens, source, language) {
  const scopes = [{ id: 'scope:0', parent: null, start: 0, end: source.length, depth: 0 }];
  const stack = [scopes[0]];
  const braceScopes = new Map();
  for (const [index, token] of tokens.entries()) {
    token.scope = stack.at(-1).id;
    if (token.text === '{') {
      const scope = {
        id: `scope:${scopes.length}`,
        parent: stack.at(-1).id,
        start: token.end,
        end: source.length,
        depth: stack.length,
      };
      scopes.push(scope);
      braceScopes.set(index, scope.id);
      stack.push(scope);
    } else if (token.text === '}' && stack.length > 1) {
      stack.pop().end = token.start;
      token.scope = stack.at(-1).id;
    }
  }

  const declarations = [];
  const declared = new Set();
  const declare = (index, kind, scope = tokens[index]?.scope) => {
    const token = tokens[index];
    if (!token || token.kind !== 'identifier' || declared.has(index)) return;
    declared.add(index);
    declarations.push({ tokenIndex: index, kind, scope });
  };
  if (language === 'JavaScript') {
    declareJavaScript(tokens, braceScopes, declare);
  } else if (language === 'Rust') {
    declareRust(tokens, braceScopes, declare);
  } else {
    declareProofLanguage(tokens, language, declare);
  }

  const scopeById = new Map(scopes.map((scope) => [scope.id, scope]));
  const bindings = declarations
    .sort((left, right) => tokens[left.tokenIndex].start - tokens[right.tokenIndex].start)
    .map((declaration) => {
      const token = tokens[declaration.tokenIndex];
      return {
        id: `${language}:${utf8Length(source.slice(0, token.start))}`,
        name: token.text,
        kind: declaration.kind,
        scope: declaration.scope,
        declaration: rangeRecord(token),
        references: [],
        tokenIndex: declaration.tokenIndex,
      };
    });
  const byToken = new Map(bindings.map((binding) => [binding.tokenIndex, binding]));
  const unresolved = [];
  for (const [index, token] of tokens.entries()) {
    if (token.kind !== 'identifier' || byToken.has(index)) continue;
    const candidates = bindings.filter((binding) => {
      if (binding.name !== token.text || binding.declaration.start > token.start) return false;
      return scopeContains(scopeById, binding.scope, token.scope);
    });
    candidates.sort((left, right) => {
      const depth = scopeById.get(right.scope).depth - scopeById.get(left.scope).depth;
      return depth || right.declaration.start - left.declaration.start;
    });
    if (candidates[0]) {
      candidates[0].references.push(rangeRecord(token));
    } else {
      unresolved.push({ kind: 'unresolved', name: token.text, ...rangeRecord(token) });
    }
  }
  return {
    scopes,
    bindings: bindings.map(({ tokenIndex: _tokenIndex, ...binding }) => binding),
    unresolved,
  };
}

function declareJavaScript(tokens, braceScopes, declare) {
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    if (['const', 'let', 'var', 'class'].includes(token.text)) {
      declare(nextIdentifier(tokens, index + 1), token.text);
    }
    if (token.text === 'function') {
      const name = nextIdentifier(tokens, index + 1);
      declare(name, 'function');
      const open = findToken(tokens, name + 1, '(');
      const close = matchingDelimiter(tokens, open, '(', ')');
      const body = findToken(tokens, close + 1, '{');
      const scope = braceScopes.get(body) ?? tokens[name]?.scope;
      declareDelimitedParameters(tokens, open, close, scope, declare);
    }
    if (token.text === 'catch') {
      const open = findToken(tokens, index + 1, '(');
      const close = matchingDelimiter(tokens, open, '(', ')');
      const body = findToken(tokens, close + 1, '{');
      declareDelimitedParameters(tokens, open, close, braceScopes.get(body), declare);
    }
    if (token.text === 'import') {
      const from = findToken(tokens, index + 1, 'from');
      const end = from < 0 ? findToken(tokens, index + 1, ';') : from;
      for (let cursor = index + 1; cursor >= 0 && cursor < end; cursor += 1) {
        if (tokens[cursor].kind !== 'identifier') continue;
        if (tokens[cursor - 1]?.text === 'as' || tokens[cursor + 1]?.text !== 'as') {
          declare(cursor, 'import');
        }
      }
    }
  }
}

function declareRust(tokens, braceScopes, declare) {
  const items = new Set(['struct', 'enum', 'trait', 'type', 'const', 'static', 'mod', 'union']);
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    if (token.text === 'let') declare(nextIdentifier(tokens, index + 1), 'let');
    if (items.has(token.text)) declare(nextIdentifier(tokens, index + 1), token.text);
    if (token.text === 'fn') {
      const name = nextIdentifier(tokens, index + 1);
      declare(name, 'function');
      const open = findToken(tokens, name + 1, '(');
      const close = matchingDelimiter(tokens, open, '(', ')');
      const body = findToken(tokens, close + 1, '{');
      const scope = braceScopes.get(body) ?? tokens[name]?.scope;
      for (let cursor = open + 1; cursor < close; cursor += 1) {
        if (tokens[cursor].kind === 'identifier' && tokens[cursor + 1]?.text === ':') {
          declare(cursor, 'parameter', scope);
        }
      }
    }
    if (token.text === 'macro_rules' && tokens[index + 1]?.text === '!') {
      declare(nextIdentifier(tokens, index + 2), 'macro');
    }
    if (token.text === 'as') declare(nextIdentifier(tokens, index + 1), 'import');
  }
}

function declareProofLanguage(tokens, language, declare) {
  const markers = language === 'Lean'
    ? new Set(['def', 'theorem', 'lemma', 'axiom', 'constant', 'inductive', 'structure', 'class', 'namespace', 'section', 'variable', 'macro'])
    : new Set(['Definition', 'Theorem', 'Lemma', 'Axiom', 'Parameter', 'Variable', 'Inductive', 'Record', 'Class', 'Module', 'Section', 'Fixpoint', 'CoFixpoint', 'Ltac']);
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    if (markers.has(token.text)) declare(nextIdentifier(tokens, index + 1), token.text);
    if (token.text === 'let') declare(nextIdentifier(tokens, index + 1), 'let');
    if (
      token.kind === 'identifier' &&
      tokens[index + 1]?.text === ':' &&
      insideBinder(tokens, index)
    ) {
      declare(index, 'parameter');
    }
  }
}

function declareDelimitedParameters(tokens, open, close, scope, declare) {
  if (open < 0 || close < 0) return;
  let segmentStart = open + 1;
  for (let index = open + 1; index <= close; index += 1) {
    if (index === close || tokens[index].text === ',') {
      const candidate = nextIdentifier(tokens, segmentStart);
      if (candidate >= 0 && candidate < index) declare(candidate, 'parameter', scope);
      segmentStart = index + 1;
    }
  }
}

function moduleFacts(tokens, language, project) {
  const markers = MODULE_MARKERS[language];
  const facts = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (!markers.has(tokens[index].text)) continue;
    const names = [];
    for (let cursor = index + 1; cursor < tokens.length; cursor += 1) {
      if ([';', '\n'].includes(tokens[cursor].text) || MODULE_MARKERS[language].has(tokens[cursor].text)) break;
      if (tokens[cursor].kind === 'identifier') names.push(tokens[cursor].text);
      if (names.length === 3) break;
    }
    facts.push({ kind: tokens[index].text, name: names.join('.'), ...rangeRecord(tokens[index]) });
  }
  for (const dependency of project.dependencies) {
    facts.push({ kind: 'resolved-project-dependency', name: dependency, start: 0, end: 0 });
  }
  return facts;
}

function typeFacts(tokens, syntax, language) {
  const facts = syntax
    .filter(({ term }) => /type|universe/u.test(term.toLowerCase()))
    .map(({ term, start, end }) => ({ kind: 'syntax-type', name: term, start, end, phase: 'surface' }));
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].text === ':' && tokens[index + 1]) {
      facts.push({ kind: 'annotation', name: tokens[index + 1].text, ...rangeRecord(tokens[index + 1]), phase: 'resolved' });
    }
    if (['universe', 'Universe', 'Type'].includes(tokens[index].text)) {
      facts.push({ kind: 'universe', name: tokens[index + 1]?.text ?? tokens[index].text, ...rangeRecord(tokens[index]), phase: 'surface' });
    }
  }
  if (language === 'JavaScript') {
    facts.push({ kind: 'dynamic-type', name: 'ECMAScript value', start: 0, end: 0, phase: 'runtime' });
  }
  return uniqueFacts(facts);
}

function extensionFacts(tokens, syntax, source, language) {
  const facts = [];
  const patterns = EXTENSION_MARKERS[language];
  for (const token of tokens) {
    if (patterns.has(token.text)) facts.push({ kind: token.text, name: token.text, ...rangeRecord(token) });
  }
  for (const fact of syntax) {
    if (/macro|attribute|decorator|quotation|template|notation/u.test(fact.term.toLowerCase())) {
      facts.push({ kind: fact.term, name: fact.term, start: fact.start, end: fact.end });
    }
  }
  if (language === 'JavaScript' && source.includes('"use strict"')) {
    facts.push({ kind: 'directive', name: 'use strict', start: source.indexOf('"use strict"'), end: source.indexOf('"use strict"') + 12 });
  }
  return uniqueFacts(facts);
}

function proofFacts(tokens, syntax, language) {
  if (language === 'JavaScript' || language === 'Rust') return [];
  const markers = PROOF_MARKERS[language];
  return uniqueFacts([
    ...tokens.filter(({ text }) => markers.has(text)).map((token) => ({ kind: token.text, name: token.text, ...rangeRecord(token) })),
    ...syntax.filter(({ term }) => /theorem|proof|tactic/u.test(term.toLowerCase())).map(({ term, start, end }) => ({ kind: term, name: term, start, end })),
  ]);
}

function diagnosticFacts(network) {
  return network.verifyFullMatch().issues.map((issue) => {
    const metadata = network.link(issue.linkId)?.metadata();
    return {
      kind: metadata?.flags.isMissing ? 'missing' : 'parse-error',
      term: metadata?.term ?? '',
      start: metadata?.span?.byteRange.start ?? 0,
      end: metadata?.span?.byteRange.end ?? 0,
    };
  });
}

function projectDiagnostics(modules, project) {
  const sourceModules = modules.filter(({ kind }) => kind !== 'resolved-project-dependency');
  if (sourceModules.length === 0 || project.dependencies.length > 0) return [];
  return sourceModules.map(({ name, start, end }) => ({
    kind: 'missing-project-context',
    term: name,
    start,
    end,
  }));
}

function constructFacts(program) {
  const tokenEvidence = (terms) => terms.map((name) => ({ kind: 'project', name, start: 0, end: 0 }));
  const recursive = program.bindings
    .filter(({ kind, name, references }) => kind === 'function' || kind === 'Fixpoint' || kind === 'CoFixpoint' || kind === 'def')
    .filter(({ name, references }) => references.some(() => name.length > 0))
    .map(({ name, declaration }) => ({ kind: 'recursive', name, ...declaration }));
  const evidence = new Map([
    ['modules-and-imports', program.modules],
    ['scopes-and-bindings', program.bindings.map(({ kind, name, declaration }) => ({ kind, name, ...declaration }))],
    ['recursive-definitions', recursive],
    ['types-and-universes', program.types],
    ['effects', effectEvidence(program)],
    ['attributes', program.extensions.filter(({ kind }) => /attribute|directive|allow|local|simp|#/u.test(kind))],
    ['macros-and-notation', program.extensions.filter(({ kind }) => /macro|notation|template|tagged|prefix|postfix|infix|syntax/iu.test(kind))],
    ['proof-terms-and-tactics', program.proofs],
    ['surface-expansion-elaboration-traces', program.sourceMappings.slice(0, 32).map(({ term, start, end }) => ({ kind: 'syntax', name: term, start, end }))],
    ['project-context-and-dependencies', tokenEvidence([...program.project.files, ...program.project.dependencies])],
  ]);
  return SEMANTIC_CONSTRUCTS.map((kind) => {
    const facts = evidence.get(kind) ?? [];
    if (kind === 'proof-terms-and-tactics' && ['JavaScript', 'Rust'].includes(program.language)) {
      return { kind, status: 'not-applicable', evidence: [], rationale: `${program.language} defines no proof/tactic sublanguage` };
    }
    return {
      kind,
      status: facts.length > 0 ? 'represented' : 'not-present',
      evidence: facts,
      rationale: facts.length > 0 ? undefined : 'the analyzed source contains no instance of this construct',
    };
  });
}

function effectEvidence(program) {
  const markers = EFFECT_MARKERS[program.language];
  const evidence = [];
  for (const marker of markers) {
    let start = program.source.indexOf(marker);
    while (start >= 0) {
      evidence.push({ kind: 'effect', name: marker, start, end: start + marker.length });
      start = program.source.indexOf(marker, start + marker.length);
    }
  }
  return uniqueFacts(evidence);
}

function codeMask(source, language) {
  const mask = new Array(source.length).fill(true);
  const lineComment = language === 'Lean' ? '--' : language === 'Rocq' ? null : '//';
  const blockOpen = language === 'Lean' ? '/-' : language === 'Rocq' ? '(*' : '/*';
  const blockClose = language === 'Lean' ? '-/' : language === 'Rocq' ? '*)' : '*/';
  for (let offset = 0; offset < source.length;) {
    if (lineComment && source.startsWith(lineComment, offset)) {
      const end = source.indexOf('\n', offset);
      mark(mask, offset, end < 0 ? source.length : end, false);
      offset = end < 0 ? source.length : end;
    } else if (source.startsWith(blockOpen, offset)) {
      offset = maskNested(source, mask, offset, blockOpen, blockClose);
    } else if (source[offset] === '"' || ((language === 'JavaScript' || language === 'Rust') && source[offset] === "'")) {
      offset = maskQuoted(source, mask, offset, source[offset]);
    } else if (language === 'JavaScript' && source[offset] === '`') {
      offset = maskTemplate(source, mask, offset);
    } else {
      offset += codePointAt(source, offset).length;
    }
  }
  return mask;
}

function maskQuoted(source, mask, start, quote) {
  let offset = start;
  mask[offset++] = false;
  while (offset < source.length) {
    mask[offset] = false;
    if (source[offset] === '\\') {
      if (offset + 1 < source.length) mask[offset + 1] = false;
      offset += 2;
    } else if (source[offset++] === quote) {
      break;
    }
  }
  return offset;
}

function maskNested(source, mask, start, open, close) {
  let offset = start;
  let depth = 0;
  while (offset < source.length) {
    if (source.startsWith(open, offset)) {
      mark(mask, offset, offset + open.length, false);
      depth += 1;
      offset += open.length;
    } else if (source.startsWith(close, offset)) {
      mark(mask, offset, offset + close.length, false);
      depth -= 1;
      offset += close.length;
      if (depth === 0) break;
    } else {
      mask[offset] = false;
      offset += 1;
    }
  }
  return offset;
}

function maskTemplate(source, mask, start) {
  let offset = start;
  mask[offset++] = false;
  while (offset < source.length) {
    if (source[offset] === '\\') {
      mark(mask, offset, Math.min(offset + 2, source.length), false);
      offset += 2;
    } else if (source[offset] === '`') {
      mask[offset++] = false;
      break;
    } else if (source.startsWith('${', offset)) {
      mark(mask, offset, offset + 2, false);
      offset = maskTemplateExpression(source, mask, offset + 2);
    } else {
      mask[offset++] = false;
    }
  }
  return offset;
}

function maskTemplateExpression(source, mask, start) {
  let offset = start;
  let depth = 1;
  while (offset < source.length && depth > 0) {
    if (source.startsWith('//', offset)) {
      const end = source.indexOf('\n', offset);
      mark(mask, offset, end < 0 ? source.length : end, false);
      offset = end < 0 ? source.length : end;
    } else if (source.startsWith('/*', offset)) {
      offset = maskNested(source, mask, offset, '/*', '*/');
    } else if (source[offset] === '"' || source[offset] === "'") {
      offset = maskQuoted(source, mask, offset, source[offset]);
    } else if (source[offset] === '`') {
      offset = maskTemplate(source, mask, offset);
    } else if (source[offset] === '{') {
      depth += 1;
      offset += 1;
    } else if (source[offset] === '}') {
      depth -= 1;
      if (depth === 0) mask[offset] = false;
      offset += 1;
    } else {
      offset += codePointAt(source, offset).length;
    }
  }
  return offset;
}

function normalizeProject(project) {
  return {
    root: String(project.root ?? ''),
    files: Object.freeze([...(project.files ?? [])].map(String)),
    dependencies: Object.freeze([...(project.dependencies ?? [])].map(String)),
    extensions: Object.freeze([...(project.extensions ?? [])].map(String)),
  };
}

function validateIdentifier(identifier, language) {
  const value = String(identifier);
  const characters = [...value];
  if (
    characters.length === 0 ||
    !identifierStart(characters[0], language) ||
    !characters.slice(1).every((character) => identifierContinue(character, language)) ||
    KEYWORDS[language].has(value)
  ) {
    throw new BindingRenameError(`${JSON.stringify(value)} is not a valid ${language} identifier`);
  }
}

function identifierStart(character, language) {
  return /[_$\p{L}]/u.test(character) && !(language !== 'JavaScript' && character === '$');
}

function identifierContinue(character, language) {
  return /[_$'\p{L}\p{N}\p{M}\u200C\u200D]/u.test(character) &&
    !(language === 'Rust' && character === "'") &&
    !(language !== 'JavaScript' && character === '$');
}

function scopeContains(scopeById, declarationScope, referenceScope) {
  let current = scopeById.get(referenceScope);
  while (current) {
    if (current.id === declarationScope) return true;
    current = current.parent ? scopeById.get(current.parent) : undefined;
  }
  return false;
}

function scopeFor(scopes, id) {
  return scopes.find((scope) => scope.id === id) ?? { start: 0, end: 0 };
}

function rangesOverlap(left, right) {
  return left.start <= right.end && right.start <= left.end;
}

function nextIdentifier(tokens, start) {
  return tokens.findIndex((token, index) => index >= start && token.kind === 'identifier');
}

function findToken(tokens, start, text) {
  return tokens.findIndex((token, index) => index >= start && token.text === text);
}

function matchingDelimiter(tokens, start, open, close) {
  if (start < 0) return -1;
  let depth = 0;
  for (let index = start; index < tokens.length; index += 1) {
    if (tokens[index].text === open) depth += 1;
    if (tokens[index].text === close && --depth === 0) return index;
  }
  return -1;
}

function insideBinder(tokens, index) {
  let depth = 0;
  for (let cursor = index - 1; cursor >= 0; cursor -= 1) {
    if (tokens[cursor].text === ')' || tokens[cursor].text === '}') depth += 1;
    if (tokens[cursor].text === '(' || tokens[cursor].text === '{') {
      if (depth === 0) return true;
      depth -= 1;
    }
    if (depth === 0 && [';', ':=', '.'].includes(tokens[cursor].text)) return false;
  }
  return false;
}

function rangeRecord(token) {
  return { start: token.start, end: token.end };
}

function uniqueFacts(facts) {
  const seen = new Set();
  return facts.filter((fact) => {
    const key = `${fact.kind}:${fact.name}:${fact.start}:${fact.end}`;
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

function byteOffsetMap(source) {
  const result = new Map([[0, 0]]);
  let bytes = 0;
  for (let offset = 0; offset < source.length;) {
    const character = codePointAt(source, offset);
    offset += character.length;
    bytes += utf8Length(character);
    result.set(bytes, offset);
  }
  return result;
}

function utf8Length(value) {
  return new TextEncoder().encode(value).length;
}

function codePointAt(value, offset) {
  return String.fromCodePoint(value.codePointAt(offset));
}

function mark(mask, start, end, value) {
  for (let index = start; index < end; index += 1) mask[index] = value;
}

function freezeRecord(record) {
  return Object.freeze({ ...record });
}

function freezeBinding(binding) {
  return Object.freeze({
    ...binding,
    declaration: freezeRecord(binding.declaration),
    references: Object.freeze(binding.references.map(freezeRecord)),
  });
}

function freezeConstruct(construct) {
  return Object.freeze({
    ...construct,
    evidence: Object.freeze(construct.evidence.map(freezeRecord)),
  });
}

const OPERATORS = ['...', '::=', '=>', '->', ':=', '::', '==', '!=', '<=', '>=', '&&', '||', '?.', '??', '**', '++', '--', '${'];

const KEYWORDS = Object.freeze({
  JavaScript: new Set(['as', 'async', 'await', 'break', 'case', 'catch', 'class', 'const', 'continue', 'default', 'delete', 'do', 'else', 'export', 'extends', 'finally', 'for', 'from', 'function', 'if', 'import', 'in', 'instanceof', 'let', 'new', 'of', 'return', 'static', 'super', 'switch', 'throw', 'try', 'typeof', 'var', 'void', 'while', 'with', 'yield']),
  Rust: new Set(['as', 'async', 'await', 'break', 'const', 'continue', 'crate', 'dyn', 'else', 'enum', 'extern', 'false', 'fn', 'for', 'if', 'impl', 'in', 'let', 'loop', 'macro_rules', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return', 'self', 'Self', 'static', 'struct', 'super', 'trait', 'true', 'type', 'union', 'unsafe', 'use', 'where', 'while']),
  Lean: new Set(['axiom', 'by', 'class', 'constant', 'def', 'deriving', 'do', 'else', 'end', 'export', 'for', 'from', 'fun', 'if', 'import', 'in', 'inductive', 'instance', 'let', 'macro', 'match', 'mutual', 'namespace', 'notation', 'open', 'partial', 'postfix', 'prefix', 'private', 'protected', 'section', 'structure', 'syntax', 'theorem', 'universe', 'variable', 'where', 'with']),
  Rocq: new Set(['Axiom', 'Class', 'CoFixpoint', 'Definition', 'End', 'Fixpoint', 'From', 'Import', 'Inductive', 'Lemma', 'Ltac', 'Module', 'Notation', 'Parameter', 'Proof', 'Qed', 'Record', 'Require', 'Section', 'Theorem', 'Universe', 'Variable', 'as', 'at', 'end', 'fix', 'forall', 'fun', 'if', 'in', 'let', 'match', 'return', 'then', 'with']),
});

const MODULE_MARKERS = Object.freeze({
  JavaScript: new Set(['import', 'export', 'from']),
  Rust: new Set(['use', 'mod', 'crate']),
  Lean: new Set(['import', 'namespace', 'open', 'export']),
  Rocq: new Set(['From', 'Require', 'Import', 'Module', 'Section']),
});

const EXTENSION_MARKERS = Object.freeze({
  JavaScript: new Set(['String']),
  Rust: new Set(['macro_rules', '#']),
  Lean: new Set(['macro', 'notation', 'syntax', 'postfix', 'prefix', 'infix', 'infixl', 'infixr', '@']),
  Rocq: new Set(['Notation', 'Ltac', '#']),
});

const PROOF_MARKERS = Object.freeze({
  Lean: new Set(['theorem', 'lemma', 'by', 'rfl', 'simp', 'exact', 'apply']),
  Rocq: new Set(['Theorem', 'Lemma', 'Proof', 'Qed', 'Defined', 'reflexivity', 'intros', 'exact', 'apply']),
});

const EFFECT_MARKERS = Object.freeze({
  JavaScript: new Set(['async', 'await', 'throw', 'yield']),
  Rust: new Set(['async', 'await', 'unsafe', 'panic']),
  Lean: new Set(['IO', 'do', 'pure']),
  Rocq: new Set(['Proof', 'Ltac', 'Qed']),
});
