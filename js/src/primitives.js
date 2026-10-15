export const LinkType = Object.freeze({
  Concept: 'Concept',
  Document: 'Document',
  Dynamic: 'Dynamic',
  Field: 'Field',
  Grammar: 'Grammar',
  Language: 'Language',
  Object: 'Object',
  Reference: 'Reference',
  Region: 'Region',
  Relation: 'Relation',
  Semantic: 'Semantic',
  SourceToken: 'SourceToken',
  // Rust-compatible public alias for lossless source-token links.
  Token: 'SourceToken',
  Syntax: 'Syntax',
  Trivia: 'Trivia',
  Type: 'Type',
});

export const NetworkProjection = Object.freeze({
  ConcreteSyntax: 'ConcreteSyntax',
  AbstractSyntax: 'AbstractSyntax',
  Semantic: 'Semantic',
});

export const TriviaAttachmentPolicy = Object.freeze({
  ContainmentLink: 'ContainmentLink',
  TokenLink: 'TokenLink',
  Combined: 'Combined',
});

function readLinkIdValue(value) {
  const identifier = value instanceof LinkId;
  const descriptor = identifier ? Object.getOwnPropertyDescriptor(value, 'value') : null;
  const original = identifier
    ? descriptor && Object.hasOwn(descriptor, 'value') ? descriptor.value : undefined
    : value;
  const text = typeof original === 'string' ? original.trim() : null;
  const numeric = typeof original === 'number' || typeof original === 'bigint'
    || text !== null && /^[0-9]+$/u.test(text)
    ? Number(original) : Number.NaN;
  if (identifier && typeof original !== 'number' || !Number.isSafeInteger(numeric) || numeric < 1) {
    throw new TypeError('link id must be an exact positive safe integer');
  }
  return numeric;
}

export class LinkId {
  constructor(value) {
    this.value = readLinkIdValue(value);
  }

  static from(value) {
    if (value instanceof LinkId) {
      readLinkIdValue(value);
      return value;
    }
    return new LinkId(value);
  }

  static fromU64(value) {
    return LinkId.from(value);
  }

  asU64() {
    return readLinkIdValue(this);
  }

  equals(other) {
    return readLinkIdValue(this) === LinkId.from(other).value;
  }

  toJSON() {
    return readLinkIdValue(this);
  }

  toString() {
    return String(readLinkIdValue(this));
  }

  valueOf() {
    return readLinkIdValue(this);
  }
}

export function idKey(value) {
  return LinkId.from(value).value;
}

export class ByteRange {
  constructor(start = 0, end = start) {
    this.start = start;
    this.end = end;
  }

  contains(other) {
    return this.start <= other.start && this.end >= other.end;
  }

  clone() {
    return new ByteRange(this.start, this.end);
  }
}

export class Point {
  constructor(row = 0, column = 0) {
    this.row = row;
    this.column = column;
  }

  clone() {
    return new Point(this.row, this.column);
  }
}

export class SourceSpan {
  constructor(byteRange = new ByteRange(), start = new Point(), end = new Point()) {
    this.byteRange = byteRange;
    this.start = start;
    this.end = end;
  }

  clone() {
    return new SourceSpan(this.byteRange.clone(), this.start.clone(), this.end.clone());
  }
}

export class LinkFlags {
  constructor({
    isError = false,
    hasError = false,
    isMissing = false,
    isExtra = false,
  } = {}) {
    this.isError = isError;
    this.hasError = hasError;
    this.isMissing = isMissing;
    this.isExtra = isExtra;
  }

  static clean() {
    return new LinkFlags();
  }

  /** Flags for an error link, which also reports that it has an error. */
  static error() {
    return new LinkFlags({ isError: true, hasError: true });
  }

  /** Flags for a link that contains an error below it. */
  static containingError() {
    return new LinkFlags({ hasError: true });
  }

  /** Flags for a missing link, which also reports that it has an error. */
  static missing() {
    return new LinkFlags({ isMissing: true, hasError: true });
  }

  withError(value = true) {
    return new LinkFlags({ ...this, isError: value, hasError: value || this.hasError });
  }

  withMissing(value = true) {
    return new LinkFlags({ ...this, isMissing: value, hasError: value || this.hasError });
  }

  withExtra(value = true) {
    return new LinkFlags({ ...this, isExtra: value });
  }

  hasRecoveryIssue() {
    return this.isError || this.hasError || this.isMissing;
  }

  clone() {
    return new LinkFlags(this);
  }
}

export class LinkMetadata {
  constructor({
    linkType = undefined,
    term = undefined,
    language = undefined,
    named = false,
    definition = undefined,
    span = undefined,
    flags = LinkFlags.clean(),
  } = {}) {
    this.linkType = linkType;
    this.term = term;
    this.language = language;
    this.named = named;
    this.definition = definition;
    this.span = span;
    this.flags = flags instanceof LinkFlags ? flags : new LinkFlags(flags);
  }

  static new() {
    return new LinkMetadata();
  }

  clone(overrides = {}) {
    return new LinkMetadata({
      linkType: this.linkType,
      term: this.term,
      language: this.language,
      named: this.named,
      definition: this.definition,
      span: this.span?.clone(),
      flags: this.flags.clone(),
      ...overrides,
    });
  }

  withLinkType(linkType) {
    return this.clone({ linkType });
  }

  withTerm(term) {
    return this.clone({ term });
  }

  withLanguage(language) {
    return this.clone({ language });
  }

  withNamed(named = true) {
    return this.clone({ named });
  }

  withDefinition(definition) {
    return this.clone({ definition });
  }

  withSpan(span) {
    return this.clone({ span });
  }

  withFlags(flags) {
    return this.clone({ flags });
  }
}

export class Link {
  constructor(id, references = [], metadata = LinkMetadata.new()) {
    this._id = LinkId.from(id);
    this._references = references.map((reference) => LinkId.from(reference));
    this._metadata = metadata instanceof LinkMetadata ? metadata : new LinkMetadata(metadata);
  }

  id() {
    return this._id;
  }

  references() {
    return [...this._references];
  }

  metadata() {
    return this._metadata;
  }

  setReferences(references) {
    this._references = references.map((reference) => LinkId.from(reference));
  }

  setMetadata(metadata) {
    this._metadata = metadata instanceof LinkMetadata ? metadata : new LinkMetadata(metadata);
  }

  clone() {
    return new Link(this._id, this._references, this._metadata.clone());
  }
}

export class ParseConfiguration {
  constructor({
    triviaAttachmentPolicy = TriviaAttachmentPolicy.Combined,
    regionDetectionPolicy = 'Both',
    accessMode = 'mutable',
  } = {}) {
    this.triviaAttachmentPolicy = triviaAttachmentPolicy;
    this.regionDetectionPolicy = regionDetectionPolicy;
    this.accessMode = accessMode;
  }

  static default() {
    return new ParseConfiguration();
  }

  withTriviaAttachmentPolicy(triviaAttachmentPolicy) {
    return new ParseConfiguration({ ...this, triviaAttachmentPolicy });
  }

  withRegionDetectionPolicy(regionDetectionPolicy) {
    return new ParseConfiguration({ ...this, regionDetectionPolicy });
  }

  with_region_detection_policy(regionDetectionPolicy) {
    return this.withRegionDetectionPolicy(regionDetectionPolicy);
  }

  withAccessMode(accessMode) {
    return new ParseConfiguration({ ...this, accessMode });
  }
}
