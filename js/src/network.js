import { Parser } from 'links-notation';

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
  idKey,
} from './primitives.js';
import { LinkQuery, QueryCaptures, QueryMatch } from './query.js';
import { LinkCliSubstitution, SubstitutionReport } from './substitution.js';
import { ReplacementReport, ReplacementRule, TextReplacement } from './transform.js';

const encoder = new TextEncoder();

export class LinkNetwork {
  constructor() {
    this._links = new Map();
    this._nextId = 1;
  }

  static parse(text, language, configuration = ParseConfiguration.default()) {
    if (language.toLowerCase() === 'lino') {
      return LinkNetwork.fromLino(text);
    }
    return LinkNetwork.parseLosslessText(text, language, configuration);
  }

  static parseLosslessText(text, language, configuration = ParseConfiguration.default()) {
    const network = new LinkNetwork();
    network._parseLosslessText(text, language, configuration);
    return network;
  }

  static parseFluent(text, language, configuration = ParseConfiguration.default()) {
    return new FluentPipeline(LinkNetwork.parse(text, language, configuration));
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
    return this.insertLink(
      references,
      LinkMetadata.new().withLinkType(LinkType.Dynamic).withTerm(term),
    );
  }

  insertTypedPoint(linkType, term) {
    const id = this._allocateId();
    this._links.set(
      id.asU64(),
      new Link(id, [id], LinkMetadata.new().withLinkType(linkType).withTerm(term).withNamed(true)),
    );
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

  insertSyntaxNode(language, term, children = []) {
    return this.insertLink(
      children,
      LinkMetadata.new()
        .withLinkType(LinkType.Syntax)
        .withLanguage(language)
        .withTerm(term)
        .withNamed(true),
    );
  }

  insertConceptExpression(concept, language, text) {
    const token = this.insertSourceToken(language, text);
    return this.insertLink(
      [token],
      LinkMetadata.new()
        .withLinkType(LinkType.Semantic)
        .withLanguage(language)
        .withTerm(`concept:${concept}`)
        .withNamed(true),
    );
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
    return this.links().find((link) => link.metadata().term === term)?.id();
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
    return this._sourceTokenLinks()
      .sort(sourceOrder)
      .filter((link) => !link.metadata().flags.isMissing)
      .map((link) => link.metadata().term ?? '')
      .join('');
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
    const tokenIdsByIndex = [];
    const openParens = [];
    let byte = 0;
    let row = 0;
    let column = 0;

    for (let index = 0; index < text.length; index += 1) {
      const character = text[index];
      const start = new Point(row, column);
      const bytes = encoder.encode(character).length;
      if (character === '\n') {
        row += 1;
        column = 0;
      } else {
        column += 1;
      }
      const span = new SourceSpan(new ByteRange(byte, byte + bytes), start, new Point(row, column));
      byte += bytes;

      let flags = LinkFlags.clean();
      if (/\s/.test(character)) {
        flags = flags.withExtra(
          configuration.triviaAttachmentPolicy !== undefined,
        );
      }
      if (character === '(') {
        openParens.push(index);
      } else if (character === ')') {
        if (openParens.length === 0) {
          flags = flags.withError(true);
        } else {
          openParens.pop();
        }
      }

      const token = this.insertSourceToken(language, character, span, flags);
      tokenIdsByIndex[index] = token;
    }

    for (const index of openParens) {
      const link = this.link(tokenIdsByIndex[index]);
      link.setMetadata(link.metadata().withFlags(link.metadata().flags.withMissing(true)));
    }

    this._indexJavaScriptIdentifiers(text, language, tokenIdsByIndex);
  }

  _indexJavaScriptIdentifiers(text, language, tokenIdsByIndex) {
    if (!['javascript', 'typescript', 'js', 'ts'].includes(language.toLowerCase())) {
      return;
    }
    const keyword = new Set([
      'break',
      'case',
      'catch',
      'class',
      'const',
      'else',
      'export',
      'for',
      'function',
      'if',
      'import',
      'let',
      'return',
      'var',
      'while',
    ]);
    const pattern = /[A-Za-z_$][A-Za-z0-9_$]*/g;
    let match = pattern.exec(text);
    while (match) {
      if (!keyword.has(match[0])) {
        const children = [];
        for (let index = match.index; index < match.index + match[0].length; index += 1) {
          children.push(tokenIdsByIndex[index]);
        }
        this.insertSyntaxNode(language, 'identifier', children);
      }
      match = pattern.exec(text);
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
