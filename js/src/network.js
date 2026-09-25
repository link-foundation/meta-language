import { Parser } from 'links-notation';

import {
  parseEmbeddedProgrammingLanguage,
  parseProgrammingLanguage,
} from './programming-language-parser.js';
import {
  ByteRange,
  Link,
  LinkFlags,
  LinkId,
  LinkMetadata,
  LinkType,
  ParseConfiguration,
  Point,
  SourceSpan,
  TriviaAttachmentPolicy,
  idKey,
} from './primitives.js';
import { LinkQuery, QueryCaptures, QueryMatch } from './query.js';
import { LinkCliSubstitution, SubstitutionReport } from './substitution.js';
import { ReplacementReport, ReplacementRule, TextReplacement } from './transform.js';
import { EmbeddedRegion, detectEmbeddedRegions, detectEmbeddedRegionsInTree } from './regions.js';
import { annotateNaturalLanguage } from './natural-language.js';
import { grammarProvenance } from './language-catalog.js';
import { seedStatehoodWorkedExample } from './concept-ontology.js';

const encoder = new TextEncoder();

export class LinkNetwork {
  constructor() {
    this._links = new Map();
    this._nextId = 1;
    // Named points interned by exact term, as Rust's `terms` table.
    this._terms = new Map();
    // Cached concept syntax keyed by `concept\u0000language`.
    this._conceptSyntax = new Map();
  }

  static parse(text, language, configuration = ParseConfiguration.default()) {
    const parsed = parseProgrammingLanguage(text, language);
    if (parsed) {
      const network = new LinkNetwork();
      const { root: document } = network._insertProgrammingLanguage(parsed, language, configuration);
      network._attachEmbeddedRegions(document, text, language, configuration, parsed);
      annotateNaturalLanguage(network, document, text, language);
      if (parsed.canonical === 'LiNo') {
        network._insertLinoSemantics(text);
      }
      return network;
    }
    return LinkNetwork.parseLosslessText(text, language, configuration);
  }

  static parseLosslessText(text, language, configuration = ParseConfiguration.default()) {
    const network = new LinkNetwork();
    network._parseLosslessText(text, language, configuration);
    return network;
  }

  /** Stores arbitrary file bytes without requiring UTF-8 decoding. */
  static parseBytes(bytes, format) {
    const network = new LinkNetwork();
    network.insertTypedPoint(LinkType.Language, format);
    const source = Uint8Array.from(bytes);
    const chunkSize = 4096;
    for (let offset = 0; offset < source.length; offset += chunkSize) {
      const chunk = source.slice(offset, offset + chunkSize);
      const span = new SourceSpan(
        new ByteRange(offset, offset + chunk.length),
        new Point(0, offset),
        new Point(0, offset + chunk.length),
      );
      const encoded = [...chunk].map((byte) => byte.toString(16).padStart(2, '0')).join('');
      network.insertSourceToken(format, `bytes:${encoded}`, span);
    }
    return network;
  }

  static parseFluent(text, language, configuration = ParseConfiguration.default()) {
    return new FluentPipeline(LinkNetwork.parse(text, language, configuration));
  }

  static parseWithRegistry(
    registry,
    text,
    language,
    configuration = ParseConfiguration.default(),
  ) {
    return registry.parse(text, language, configuration);
  }

  static parse_with_registry(registry, text, language, configuration = ParseConfiguration.default()) {
    return LinkNetwork.parseWithRegistry(registry, text, language, configuration);
  }

  static fromLino(source) {
    const network = new LinkNetwork();
    if (network._insertCanonicalLino(source)) {
      return network;
    }
    for (const parsed of new Parser().parse(source)) {
      network._insertParsedLinoLink(parsed);
    }
    return network;
  }

  insertLink(references = [], metadata = LinkMetadata.new()) {
    return this.insertLinkWithOptionalId(undefined, references, metadata);
  }

  insertLinkWithOptionalId(preferredId, references = [], metadata = LinkMetadata.new()) {
    const id = preferredId === undefined ? this._allocateId() : LinkId.from(preferredId);
    this._nextId = Math.max(this._nextId, id.asU64() + 1);
    this._links.set(id.asU64(), new Link(id, references, metadata));
    return id;
  }

  insertDynamicLink(references = [], term = undefined) {
    const id = this.insertLink(
      references,
      LinkMetadata.new().withLinkType(LinkType.Dynamic).withTerm(term),
    );
    if (term !== undefined) {
      this._terms.set(term, id);
    }
    return id;
  }

  /**
   * Inserts a self-referencing named point, reusing the point already
   * interned for `term`. A supplied definition replaces the stored one.
   */
  insertTypedPoint(linkType, term, definition = undefined) {
    const existing = this._terms.get(term);
    if (existing !== undefined && this.link(existing)) {
      if (definition !== undefined) {
        const link = this.link(existing);
        link.setMetadata(link.metadata().withDefinition(definition));
      }
      return existing;
    }
    const id = this._allocateId();
    this._links.set(
      id.asU64(),
      new Link(
        id,
        [id],
        LinkMetadata.new()
          .withLinkType(linkType)
          .withTerm(term)
          .withNamed(true)
          .withDefinition(definition),
      ),
    );
    this._terms.set(term, id);
    return id;
  }

  insertPoint(term) {
    return this.insertTypedPoint(LinkType.Concept, term);
  }

  insertObject(term) {
    return this.insertTypedPoint(LinkType.Object, term);
  }

  insertField(term) {
    return this.insertTypedPoint(LinkType.Field, term);
  }

  insertRelation(references = [], term = undefined) {
    return this.insertLink(
      references,
      LinkMetadata.new().withLinkType(LinkType.Relation).withTerm(term),
    );
  }

  insertSourceToken(language, text, span = undefined, flags = LinkFlags.clean()) {
    return this.insertLink(
      [],
      LinkMetadata.new()
        .withLinkType(LinkType.SourceToken)
        .withLanguage(language)
        .withTerm(text)
        .withSpan(span)
        .withFlags(flags),
    );
  }

  insertSyntaxNode(language, term, children = [], metadata = {}) {
    return this.insertLink(
      children,
      LinkMetadata.new()
        .withLinkType(LinkType.Syntax)
        .withLanguage(language)
        .withTerm(term)
        .withNamed(metadata.named ?? true)
        .withSpan(metadata.span)
        .withFlags(metadata.flags ?? LinkFlags.clean()),
    );
  }

  /**
   * Interns a language-free concept by exact identifier. Case, diacritic or
   * sense-suffix changes are distinct identifiers and mint distinct concepts.
   */
  internConcept(exactId, definition = undefined) {
    return this.insertTypedPoint(LinkType.Concept, exactId, definition);
  }

  /**
   * Inserts a language-bound expression linked to a language-free concept and
   * returns the semantic mapping link `[concept, language]`.
   */
  insertConceptExpression(concept, language, expression) {
    const conceptLink =
      this.findTerm(concept) ??
      this.internConcept(concept, 'A language-free concept shared by exact interlingual id.');
    return this._insertConceptSyntaxMapping(conceptLink, concept, language, expression, true);
  }

  insertConceptMapping(concept, language, syntax) {
    return this.insertConceptExpression(concept, language, syntax);
  }

  /** Attaches an external vocabulary id to a concept without changing its id. */
  insertConceptAlias(conceptLink, vocabulary, externalId) {
    return this._insertConceptAliasLink(conceptLink, vocabulary, externalId)[0];
  }

  /** Reconstructs a concept using a target language syntax mapping. */
  reconstructConcept(concept, language) {
    return this._conceptSyntax.get(conceptSyntaxKey(concept, language));
  }

  /** Seeds the Hawaii statehood worked example shared with the Rust runtime. */
  seedStatehoodWorkedExample() {
    return seedStatehoodWorkedExample(this);
  }

  _insertConceptAliasLink(conceptLink, vocabulary, externalId) {
    const vocabularyLink = this.insertTypedPoint(
      LinkType.Type,
      `external-id:${vocabulary}`,
      'External concept identifier vocabulary.',
    );
    const existing = this._findSemanticPair(conceptLink, vocabularyLink, externalId, vocabulary);
    if (existing !== undefined) {
      return [existing, false];
    }
    return [
      this.insertLink(
        [conceptLink, vocabularyLink],
        LinkMetadata.new()
          .withLinkType(LinkType.Semantic)
          .withNamed(true)
          .withTerm(externalId)
          .withLanguage(vocabulary),
      ),
      true,
    ];
  }

  _insertConceptSyntaxMapping(conceptLink, concept, language, syntax, updateReconstruction) {
    const languageLink = this.insertTypedPoint(LinkType.Language, language);
    const key = conceptSyntaxKey(concept, language);
    if (updateReconstruction || !this._conceptSyntax.has(key)) {
      this._conceptSyntax.set(key, syntax);
    }
    const existing = this._findSemanticPair(conceptLink, languageLink, syntax, language);
    if (existing !== undefined) {
      return existing;
    }
    return this.insertLink(
      [conceptLink, languageLink],
      LinkMetadata.new()
        .withLinkType(LinkType.Semantic)
        .withNamed(true)
        .withTerm(syntax)
        .withLanguage(language),
    );
  }

  _findSemanticPair(first, second, term, language) {
    return this.links().find((link) => {
      const references = link.references();
      const metadata = link.metadata();
      return (
        metadata.linkType === LinkType.Semantic &&
        references.length === 2 &&
        references[0].equals(first) &&
        references[1].equals(second) &&
        metadata.term === term &&
        metadata.language === language
      );
    })?.id();
  }

  link(id) {
    return this._links.get(idKey(id));
  }

  links() {
    return [...this._links.values()].sort((left, right) => left.id().asU64() - right.id().asU64());
  }

  len() {
    return this._links.size;
  }

  deleteLink(id) {
    this._links.delete(idKey(id));
    for (const [term, termId] of this._terms) {
      if (idKey(termId) === idKey(id)) {
        this._terms.delete(term);
      }
    }
  }

  setSpan(id, span) {
    const link = this.link(id);
    if (link) {
      link.setMetadata(link.metadata().withSpan(span));
    }
  }

  setFlags(id, flags) {
    const link = this.link(id);
    if (link) {
      link.setMetadata(link.metadata().withFlags(flags));
    }
  }

  setTerm(id, term) {
    const link = this.link(id);
    if (link) {
      link.setMetadata(link.metadata().withTerm(term));
    }
  }

  findTerm(term) {
    const interned = this._terms.get(term);
    return interned !== undefined && this.link(interned) ? interned : undefined;
  }

  /** Finds the definition attached to a term link. */
  definitionFor(id) {
    return this.link(id)?.metadata().definition;
  }

  queryLinks(query) {
    const normalized = query instanceof LinkQuery ? query : new LinkQuery(query);
    return this.links().filter((link) => normalized.matchesMetadata(link.metadata()));
  }

  find(query) {
    const normalized = query instanceof LinkQuery ? query : new LinkQuery(query);
    const matches = [];
    for (const link of this.queryLinks(normalized)) {
      const captures = new QueryCaptures();
      if (normalized.sexpression) {
        captures.set(normalized.sexpression.capture, link.id());
        if (!this._predicatesMatch(normalized.sexpression.predicates, captures)) {
          continue;
        }
      } else {
        captures.set('match', link.id());
      }
      matches.push(new QueryMatch(link.id(), captures));
    }
    return matches;
  }

  replace(matches, rule) {
    const normalized = rule instanceof ReplacementRule
      ? rule
      : ReplacementRule.capturedText(rule.captureName, rule.replacementText);
    const replacements = [];

    for (const match of matches) {
      const captured = match.captures.get(normalized.captureName);
      if (!captured) {
        continue;
      }
      const oldText = this.capturedText(captured);
      if (this._replaceCapturedText(captured, normalized.replacementText)) {
        replacements.push(new TextReplacement(captured, oldText, normalized.replacementText));
      }
    }

    return new ReplacementReport(replacements);
  }

  applySubstitution(rule) {
    const updated = [];
    for (const link of this.links()) {
      if (sameReferences(link.references(), rule.patternReferences)) {
        link.setReferences(rule.replacementReferences);
        updated.push(link.id());
      }
    }
    return new SubstitutionReport({ updated });
  }

  applyLinkCliSubstitutionText(source) {
    return LinkCliSubstitution.parse(source).apply(this);
  }

  toLino() {
    return this.links()
      .map((link) => {
        const references = link.references();
        if (references.length === 0) {
          return `(${link.id().asU64()})`;
        }
        return `(${link.id().asU64()}: ${references.map((id) => id.asU64()).join(' ')})`;
      })
      .join('\n');
  }

  snapshot(version, provenance) {
    return new NetworkSnapshot(version, provenance, this.clone());
  }

  verifyFullMatch(region = undefined) {
    const issues = [];
    for (const link of this.links()) {
      const metadata = link.metadata();
      if (!metadata.flags.hasRecoveryIssue()) {
        continue;
      }
      if (region && metadata.span && !region.contains(metadata.span.byteRange)) {
        continue;
      }
      issues.push(new VerificationIssue(link.id(), metadata.flags));
    }
    return new VerificationReport(issues);
  }

  reconstructText() {
    const reconstructed = [];
    let coveredUntil = 0;
    for (const link of this._sourceTokenLinks().sort(sourceOrder)) {
      const metadata = link.metadata();
      if (metadata.flags.isMissing) continue;
      const range = metadata.span?.byteRange;
      if (range && range.start < coveredUntil) continue;
      reconstructed.push(metadata.term ?? '');
      if (range) coveredUntil = range.end;
    }
    return reconstructed.join('');
  }

  /** Returns mixed-language regions discovered and parsed into this network. */
  embeddedRegions() {
    return this.links()
      .filter((link) => link.metadata().linkType === LinkType.Region)
      .map((link) => new EmbeddedRegion(link.metadata().language, link.metadata().span));
  }

  embedded_regions() {
    return this.embeddedRegions();
  }

  /** Reconstructs bytes stored by `parseBytes` in source order. */
  reconstructBytes() {
    const result = [];
    for (const link of this._sourceTokenLinks().sort(sourceOrder)) {
      const term = link.metadata().term ?? '';
      if (!/^bytes:(?:[0-9a-f]{2})+$/i.test(term)) {
        continue;
      }
      for (let offset = 6; offset < term.length; offset += 2) {
        result.push(Number.parseInt(term.slice(offset, offset + 2), 16));
      }
    }
    return Uint8Array.from(result);
  }

  renderSource(language) {
    return this._sourceTokenLinks()
      .filter((link) => link.metadata().language === language)
      .sort(sourceOrder)
      .map((link) => link.metadata().term ?? '')
      .join('');
  }

  reconstructTextAsWithRules(targetLanguage, _configuration, rules) {
    return rules.render(targetLanguage, this);
  }

  intoFluent() {
    return new FluentPipeline(this);
  }

  clone() {
    const clone = new LinkNetwork();
    clone._nextId = this._nextId;
    clone._terms = new Map(this._terms);
    clone._conceptSyntax = new Map(this._conceptSyntax);
    for (const [key, link] of this._links) {
      clone._links.set(key, link.clone());
    }
    return clone;
  }

  capturedText(id) {
    const link = this.link(id);
    if (!link) {
      return '';
    }
    if (link.metadata().linkType === LinkType.SourceToken) {
      return link.metadata().term ?? '';
    }
    return link.references().map((reference) => this.capturedText(reference)).join('');
  }

  _allocateId() {
    return new LinkId(this._nextId++);
  }

  _insertParsedLinoLink(parsed) {
    const id = parsed.id === null ? undefined : Number(parsed.id);
    const references = parsed.values.map((value) => LinkId.from(value.id));
    this.insertLinkWithOptionalId(id, references, LinkMetadata.new().withLinkType(LinkType.Relation));
  }

  _insertLinoSemantics(source) {
    let semantic;
    try {
      semantic = LinkNetwork.fromLino(source);
    } catch {
      return;
    }

    const remapped = new Map(
      semantic.links().map((link) => [link.id().asU64(), this._allocateId()]),
    );
    for (const link of semantic.links()) {
      this.insertLinkWithOptionalId(
        remapped.get(link.id().asU64()),
        link.references().map((reference) => remapped.get(reference.asU64())),
        link.metadata().clone(),
      );
    }
  }

  _insertCanonicalLino(source) {
    const lines = source
      .split(/\r?\n/)
      .map((line) => line.trim())
      .filter(Boolean);
    if (lines.length === 0) {
      return true;
    }

    const parsed = [];
    for (const line of lines) {
      const match = line.match(/^\((\d+)(?::\s*([0-9\s]+))?\)$/);
      if (!match) {
        return false;
      }
      const references = match[2]
        ? match[2]
            .trim()
            .split(/\s+/)
            .filter(Boolean)
            .map((value) => LinkId.from(value))
        : [];
      parsed.push({ id: Number(match[1]), references });
    }

    for (const link of parsed) {
      this.insertLinkWithOptionalId(
        link.id,
        link.references,
        LinkMetadata.new().withLinkType(
          link.references.length > 0 ? LinkType.Relation : LinkType.Concept,
        ),
      );
    }
    return true;
  }

  _parseLosslessText(text, language, configuration) {
    this.insertTypedPoint(LinkType.Language, language);
    const openParens = [];
    let byte = 0;
    let row = 0;
    let column = 0;

    for (const character of text) {
      const start = new Point(row, column);
      const bytes = encoder.encode(character).length;
      if (character === '\n') {
        row += 1;
        column = 0;
      } else {
        // Columns count UTF-8 bytes, as tree-sitter points do.
        column += bytes;
      }
      const span = new SourceSpan(new ByteRange(byte, byte + bytes), start, new Point(row, column));
      byte += bytes;

      const whitespace = /^\p{White_Space}$/u.test(character);
      let flags = LinkFlags.clean();
      if (whitespace) flags = flags.withExtra(true);
      let unmatchedClose = false;
      if (character === '(') {
        openParens.push(null);
      } else if (character === ')') {
        if (openParens.length === 0) unmatchedClose = true;
        else openParens.pop();
      }
      if (unmatchedClose) flags = flags.withError(true);

      const token = this.insertSourceToken(language, character, span, flags);
      if (character === '(') openParens[openParens.length - 1] = token;
    }

    const end = new Point(row, column);
    const missingSpan = new SourceSpan(new ByteRange(byte, byte), end, end);
    for (const token of openParens) {
      const link = this.link(token);
      link.setMetadata(link.metadata().withFlags(new LinkFlags({
        ...link.metadata().flags,
        hasError: true,
      })));
      this.insertSourceToken(language, ')', missingSpan, LinkFlags.clean().withMissing(true));
    }
  }


  _insertProgrammingLanguage(parsed, language, configuration, offset = undefined) {
    const languageLink = this.insertTypedPoint(LinkType.Language, language);
    this._recordGrammars(languageLink, language);
    const tokenIds = parsed.tokens.map((token) =>
      this.insertSourceToken(language, token.text, offsetSpan(token.span, offset), token.flags),
    );
    const context = {
      language,
      tokens: parsed.tokens,
      tokenIds,
      offset,
      triviaPolicy: configuration?.triviaAttachmentPolicy ?? TriviaAttachmentPolicy.Combined,
    };
    const insert = (node) => this._insertProgrammingTree(node, context);
    const leading = (parsed.leading ?? []).map(insert);
    const root = insert(parsed.tree);
    const trailing = (parsed.trailing ?? []).map(insert);
    return { root, outer: [...leading, root, ...trailing] };
  }

  /**
   * Records the grammars that parse `language` by default as Grammar links
   * below its Language link: term `<id>@<version>`, definition
   * `sha256:<parser digest>`. Recording the same grammar twice is a no-op.
   */
  _recordGrammars(languageLink, language) {
    for (const grammar of grammarProvenance(language)) {
      const term = `${grammar.id}@${grammar.version}`;
      const recorded = this.links().some((link) =>
        link.metadata().linkType === LinkType.Grammar &&
        link.references().length === 1 &&
        link.references()[0].asU64() === languageLink.asU64() &&
        link.metadata().term === term);
      if (!recorded) {
        this.insertLink(
          [languageLink],
          LinkMetadata.new()
            .withLinkType(LinkType.Grammar)
            .withNamed(true)
            .withTerm(term)
            .withDefinition(`sha256:${grammar.parserSha256}`)
            .withLanguage(language),
        );
      }
    }
  }

  /** Grammars recorded for the languages this network parsed, in insertion order. */
  parseGrammars() {
    return this.links().flatMap((link) => {
      const metadata = link.metadata();
      if (metadata.linkType !== LinkType.Grammar || link.references().length !== 1) return [];
      const language = this.link(link.references()[0]);
      if (language?.metadata().linkType !== LinkType.Language) return [];
      const match = /^([^@]+)@(.+)$/u.exec(metadata.term ?? '');
      const digest = /^sha256:(.+)$/u.exec(metadata.definition ?? '');
      if (!match || !digest) return [];
      return [{
        language: language.metadata().term,
        id: match[1],
        version: match[2],
        parserSha256: digest[1],
      }];
    });
  }

  _insertProgrammingTree(node, context) {
    const { language, tokens, tokenIds, offset } = context;
    if (node.gap) {
      return tokenIds[node.tokenIndex];
    }
    if (node.tokenIndex !== undefined) {
      const token = tokens[node.tokenIndex];
      const syntax = this.insertSyntaxNode(language, node.term, [tokenIds[node.tokenIndex]], {
        named: token.named,
        span: offsetSpan(token.span, offset),
        flags: token.flags,
      });
      this._attachExtraTrivia(syntax, node, context);
      return syntax;
    }

    const children = node.children.map((child) => this._insertProgrammingTree(child, context));
    const syntax = this.insertSyntaxNode(language, node.term, children, {
      named: node.named,
      span: offsetSpan(node.span, offset),
      flags: node.flags,
    });
    for (const [index, child] of node.children.entries()) {
      if (child.gap) {
        this._attachExtraTrivia(syntax, child, context);
      }
      if (child.field) {
        this.insertLink(
          [syntax, children[index]],
          LinkMetadata.new()
            .withLinkType(LinkType.Field)
            .withLanguage(language)
            .withTerm(child.field)
            .withNamed(true),
        );
      }
    }
    return syntax;
  }

  /**
   * Attaches Trivia links to the source token of `node` when the grammar marks
   * it extra, with `owner` the Syntax link directly above the token, as
   * `attach_trivia` in rust/src/link_network.rs: a containment link
   * `[owner, token]`, a token link `[token]`, or both.
   */
  _attachExtraTrivia(owner, node, { tokens, tokenIds, offset, triviaPolicy }) {
    const token = tokens[node.tokenIndex];
    if (!token.flags?.isExtra) return;
    const trivia = (references, term) =>
      this.insertLink(
        references,
        LinkMetadata.new()
          .withLinkType(LinkType.Trivia)
          .withTerm(term)
          .withSpan(offsetSpan(token.span, offset))
          .withFlags(LinkFlags.clean().withExtra()),
      );
    const tokenId = tokenIds[node.tokenIndex];
    if (triviaPolicy !== TriviaAttachmentPolicy.TokenLink) {
      trivia([owner, tokenId], 'containment trivia');
    }
    if (triviaPolicy !== TriviaAttachmentPolicy.ContainmentLink) {
      trivia([tokenId], 'token trivia');
    }
  }

  _attachEmbeddedRegions(document, text, language, configuration, parsed) {
    const policy = configuration.regionDetectionPolicy ?? 'Both';
    // HTML and Markdown regions come from the host CST already parsed.
    const host = parsed.canonical;
    const regions = host === 'HTML' || host === 'Markdown'
      ? detectEmbeddedRegionsInTree(parsed.tree, text, host, policy)
      : detectEmbeddedRegions(text, language, policy);
    for (const region of regions) {
      const regionLanguage = region.language();
      const languageLink = this.insertTypedPoint(LinkType.Language, regionLanguage);
      const regionLink = this.insertLink(
        [document, languageLink],
        LinkMetadata.new()
          .withLinkType(LinkType.Region)
          .withNamed(true)
          .withTerm(`${regionLanguage} region`)
          .withLanguage(regionLanguage)
          .withSpan(region.span()),
      );
      const { start, end } = region.span().byteRange;
      // A region spanning the whole document in the document's own language
      // already has its grammar CST below the document; reparsing it would
      // duplicate every node.
      if (
        regionLanguage.toLowerCase() === String(language).toLowerCase() &&
        start === 0 &&
        end === encoder.encode(text).length
      ) {
        continue;
      }
      this._recordGrammars(languageLink, regionLanguage);
      const parsed = parseEmbeddedProgrammingLanguage(
        sliceBytes(text, start, end),
        regionLanguage,
      );
      if (parsed) {
        const { outer } = this._insertProgrammingLanguage(
          parsed,
          regionLanguage,
          configuration,
          region.span(),
        );
        this.link(regionLink).setReferences([document, languageLink, ...outer]);
      }
    }
  }

  _sourceTokenLinks() {
    return this.links().filter((link) => link.metadata().linkType === LinkType.SourceToken);
  }

  _predicatesMatch(predicates, captures) {
    for (const predicate of predicates) {
      const captured = captures.get(predicate.capture);
      const text = this.capturedText(captured);
      if (predicate.operator === 'eq?' && text !== predicate.value) {
        return false;
      }
      if (predicate.operator === 'not-eq?' && text === predicate.value) {
        return false;
      }
    }
    return true;
  }

  _replaceCapturedText(id, replacementText) {
    const tokenLinks = this._capturedTokenLinks(id);
    if (tokenLinks.length === 0) {
      const link = this.link(id);
      if (!link) {
        return false;
      }
      link.setMetadata(link.metadata().withTerm(replacementText));
      return true;
    }

    tokenLinks[0].setMetadata(tokenLinks[0].metadata().withTerm(replacementText));
    for (const token of tokenLinks.slice(1)) {
      token.setMetadata(token.metadata().withTerm(''));
    }
    return true;
  }

  _capturedTokenLinks(id) {
    const link = this.link(id);
    if (!link) {
      return [];
    }
    if (link.metadata().linkType === LinkType.SourceToken) {
      return [link];
    }
    return link.references().flatMap((reference) => this._capturedTokenLinks(reference));
  }
}

export class NetworkSnapshot {
  constructor(version, provenance, network) {
    this._version = version;
    this._provenance = provenance;
    this._network = network;
  }

  version() {
    return this._version;
  }

  provenance() {
    return this._provenance;
  }

  network() {
    return this._network.clone();
  }
}

export class VerificationIssue {
  constructor(linkId, flags) {
    this.linkId = linkId;
    this.flags = flags;
  }
}

export class VerificationReport {
  constructor(issues = []) {
    this.issues = issues;
  }

  isClean() {
    return this.issues.length === 0;
  }
}

export class FluentPipeline {
  constructor(network) {
    this._network = network;
    this.matches = [];
    this._lastReport = ReplacementReport.empty();
  }

  find(query) {
    this.matches = this._network.find(query);
    return this;
  }

  replace(rule) {
    this._lastReport = this._network.replace(this.matches, rule);
    return this;
  }

  substitute(rule) {
    const report = this._network.applySubstitution(rule);
    this._lastReport = new ReplacementReport([], report);
    return this;
  }

  linkCliSubstitutionText(source) {
    const report = this._network.applyLinkCliSubstitutionText(source);
    this._lastReport = new ReplacementReport([], report);
    return this;
  }

  reconstruct() {
    return this._network.reconstructText();
  }

  serialize() {
    return this._network.toLino();
  }

  snapshot(version, provenance) {
    return this._network.snapshot(version, provenance);
  }

  translate(targetLanguage, configuration, rules) {
    return this._network.reconstructTextAsWithRules(targetLanguage, configuration, rules);
  }

  verify(region = undefined) {
    return this._network.verifyFullMatch(region);
  }

  lastReport() {
    return this._lastReport;
  }

  network() {
    return this._network;
  }

  intoNetwork() {
    return this._network;
  }
}

function conceptSyntaxKey(concept, language) {
  return `${concept}\u0000${language}`;
}

function sameReferences(left, right) {
  if (left.length !== right.length) {
    return false;
  }
  return left.every((reference, index) => idKey(reference) === idKey(right[index]));
}

function sourceOrder(left, right) {
  const leftSpan = left.metadata().span;
  const rightSpan = right.metadata().span;
  if (leftSpan && rightSpan && leftSpan.byteRange.start !== rightSpan.byteRange.start) {
    return leftSpan.byteRange.start - rightSpan.byteRange.start;
  }
  return left.id().asU64() - right.id().asU64();
}

function offsetSpan(span, offset) {
  if (!span || !offset) return span;
  const baseByte = offset.byteRange.start;
  const basePoint = offset.start;
  const point = ({ row, column }) => new Point(
    basePoint.row + row,
    row === 0 ? basePoint.column + column : column,
  );
  return new SourceSpan(
    new ByteRange(baseByte + span.byteRange.start, baseByte + span.byteRange.end),
    point(span.start),
    point(span.end),
  );
}

function sliceBytes(text, start, end) {
  return new TextDecoder().decode(encoder.encode(text).slice(start, end));
}
