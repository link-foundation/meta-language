export type LinkTypeValue =
  | 'Concept'
  | 'Dynamic'
  | 'Field'
  | 'Language'
  | 'Object'
  | 'Relation'
  | 'Semantic'
  | 'SourceToken'
  | 'Syntax'
  | 'Trivia';

export const LinkType: Record<string, LinkTypeValue> & {
  Concept: 'Concept';
  Dynamic: 'Dynamic';
  Field: 'Field';
  Language: 'Language';
  Object: 'Object';
  Relation: 'Relation';
  Semantic: 'Semantic';
  SourceToken: 'SourceToken';
  Token: 'SourceToken';
  Syntax: 'Syntax';
  Trivia: 'Trivia';
};
export const ApiOperation: Record<string, string>;
export const ApiStyle: Record<string, string | string[]>;
export const ApiStyleFixtureKind: Record<string, string>;
export const API_OPERATIONS: ApiOperationEntry[];

export type TruthValueName = 'True' | 'False' | 'Unknown' | 'Both';

export class TruthValue {
  constructor(value: TruthValueName);
  static True: TruthValue;
  static False: TruthValue;
  static Unknown: TruthValue;
  static Both: TruthValue;
  static from(value: TruthValueName | TruthValue): TruthValue;
  and(other: TruthValueName | TruthValue): TruthValue;
  or(other: TruthValueName | TruthValue): TruthValue;
  negate(): TruthValue;
  equals(other: TruthValueName | TruthValue): boolean;
  toJSON(): TruthValueName;
  toString(): TruthValueName;
}

export class Probability {
  constructor(basisPoints: number);
  static ZERO: Probability;
  static ONE: Probability;
  static from(value: number | Probability): Probability;
  static fromBasisPoints(basisPoints: number): Probability | undefined;
  static from_basis_points(basisPoints: number): Probability | undefined;
  static fromRatio(numerator: number | bigint, denominator: number | bigint): Probability | undefined;
  static from_ratio(numerator: number | bigint, denominator: number | bigint): Probability | undefined;
  basisPoints(): number;
  basis_points(): number;
  complement(): Probability;
  equals(other: number | Probability): boolean;
  toJSON(): number;
  valueOf(): number;
}

export class ProbabilisticTruthValue {
  constructor(trueProbability: number | Probability);
  static fromRatio(
    numerator: number | bigint,
    denominator: number | bigint,
  ): ProbabilisticTruthValue | undefined;
  static from_ratio(
    numerator: number | bigint,
    denominator: number | bigint,
  ): ProbabilisticTruthValue | undefined;
  trueProbability(): Probability;
  true_probability(): Probability;
  falseProbability(): Probability;
  false_probability(): Probability;
  negate(): ProbabilisticTruthValue;
  and(other: number | Probability | ProbabilisticTruthValue): ProbabilisticTruthValue;
  or(other: number | Probability | ProbabilisticTruthValue): ProbabilisticTruthValue;
  equals(other: number | Probability | ProbabilisticTruthValue): boolean;
  toJSON(): { trueProbability: number };
}

export class LinkId {
  constructor(value: number | string | LinkId);
  static from(value: number | string | LinkId): LinkId;
  static fromU64(value: number): LinkId;
  asU64(): number;
  equals(other: number | string | LinkId): boolean;
}

export class LinkMetadata {
  static new(): LinkMetadata;
  definition?: string;
  withLinkType(linkType: LinkTypeValue): LinkMetadata;
  withTerm(term: string): LinkMetadata;
  withLanguage(language: string): LinkMetadata;
  withNamed(named?: boolean): LinkMetadata;
  withDefinition(definition?: string): LinkMetadata;
}

export class LinkNetwork {
  constructor();
  static parse(text: string, language: string, configuration?: ParseConfiguration): LinkNetwork;
  static parseLosslessText(
    text: string,
    language: string,
    configuration?: ParseConfiguration,
  ): LinkNetwork;
  static parseFluent(
    text: string,
    language: string,
    configuration?: ParseConfiguration,
  ): FluentPipeline;
  static fromLino(source: string): LinkNetwork;
  insertPoint(term: string): LinkId;
  insertLink(references?: Array<LinkId | number>, metadata?: LinkMetadata): LinkId;
  insertLinkWithOptionalId(
    id: number | undefined,
    references?: Array<LinkId | number>,
    metadata?: LinkMetadata,
  ): LinkId;
  insertSourceToken(language: string, text: string): LinkId;
  insertSyntaxNode(language: string, term: string, children?: Array<LinkId | number>): LinkId;
  insertConceptExpression(concept: string, language: string, text: string): LinkId;
  link(id: LinkId | number): Link | undefined;
  links(): Link[];
  len(): number;
  queryLinks(query: LinkQuery): Link[];
  find(query: LinkQuery): QueryMatch[];
  replace(matches: QueryMatch[], rule: ReplacementRule): ReplacementReport;
  applySubstitution(rule: SubstitutionRule): SubstitutionReport;
  applyLinkCliSubstitutionText(source: string): SubstitutionReport;
  toLino(): string;
  snapshot(version: number, provenance: string): NetworkSnapshot;
  verifyFullMatch(): VerificationReport;
  reconstructText(): string;
  renderSource(language: string): string;
  reconstructTextAsWithRules(
    targetLanguage: string,
    configuration: ParseConfiguration,
    rules: TranslationRuleSet,
  ): string;
  intoFluent(): FluentPipeline;
}

export class Link {
  id(): LinkId;
  references(): LinkId[];
  metadata(): LinkMetadata;
}

export class ParseConfiguration {
  static default(): ParseConfiguration;
}

export class LinkQuery {
  static byType(linkType: LinkTypeValue): LinkQuery;
  static byTerm(term: string): LinkQuery;
  static fromSexpression(source: string): LinkQuery;
  withTerm(term: string): LinkQuery;
  withLanguage(language: string): LinkQuery;
  withNamed(named?: boolean): LinkQuery;
}

export class QueryMatch {
  linkId: LinkId;
}

export class ReplacementRule {
  static capturedText(captureName: string, replacementText: string): ReplacementRule;
}

export class ReplacementReport {
  isEmpty(): boolean;
  substitution(): SubstitutionReport | undefined;
}

export class SubstitutionRule {
  constructor(patternReferences: Array<LinkId | number>, replacementReferences: Array<LinkId | number>);
}

export class SubstitutionReport {
  created(): LinkId[];
  updated(): LinkId[];
  deleted(): LinkId[];
  isEmpty(): boolean;
}

export class LinkCliSubstitution {
  static parse(source: string): LinkCliSubstitution;
  static linkId(value: number): LinkId;
  kind(): string;
}

export const LinkCliSubstitutionKind: Record<string, string>;

export class NetworkSnapshot {
  version(): number;
  provenance(): string;
  network(): LinkNetwork;
}

export class VerificationReport {
  isClean(): boolean;
}

export class FluentPipeline {
  matches: QueryMatch[];
  find(query: LinkQuery): FluentPipeline;
  replace(rule: ReplacementRule): FluentPipeline;
  substitute(rule: SubstitutionRule): FluentPipeline;
  linkCliSubstitutionText(source: string): FluentPipeline;
  reconstruct(): string;
  serialize(): string;
  snapshot(version: number, provenance: string): NetworkSnapshot;
  translate(
    targetLanguage: string,
    configuration: ParseConfiguration,
    rules: TranslationRuleSet,
  ): string;
  verify(): VerificationReport;
  lastReport(): ReplacementReport;
  network(): LinkNetwork;
  intoNetwork(): LinkNetwork;
}

export class TranslationRule {
  constructor(name: string, query: LinkQuery, referenceCaptures?: Record<string, number>);
  withReferenceCapture(name: string, referenceIndex: number): TranslationRule;
  with_reference_capture(name: string, referenceIndex: number): TranslationRule;
  withTemplate(language: string, text: string): TranslationRule;
}

export class TranslationRuleSet {
  constructor(name: string, rules?: TranslationRule[]);
  withRule(rule: TranslationRule): TranslationRuleSet;
  render(targetLanguage: string, network: LinkNetwork): string;
  toLino(): string;
  toJson(): string;
  static fromLino(source: string): TranslationRuleSet;
  static fromJson(source: string | unknown): TranslationRuleSet;
}

export class GrammarBuilder {
  constructor(start: string);
  terminal(name: string, expression: unknown): GrammarBuilder;
  nonterminal(name: string, expression: unknown): GrammarBuilder;
  build(): unknown;
  static literal(value: string): unknown;
  static ref(name: string): unknown;
  static seq(...items: unknown[]): unknown;
  static choice(...items: unknown[]): unknown;
  static repeat0(item: unknown): unknown;
  static repeat1(item: unknown): unknown;
  static optional(item: unknown): unknown;
  static charRange(start: string, end: string): unknown;
  static charClass(value: string): unknown;
  static any(): unknown;
}

export const ExprBuilder: typeof GrammarBuilder;
export function emitPeggy(grammar: unknown): string;
export function emitJavascriptParser(grammar: unknown): string;

export class ApiOperationEntry {
  operation: string;
  name(): string;
  styles(): ApiStyleCell[];
  style(style: string): ApiStyleCell | undefined;
}

export class ApiStyleCell {
  fixture: { kind: string; value: string };
  style(): string;
}

export function runApiStyleFixture(name: string): void;

// --- access (read-only network views) ---

export type AccessModeValue = 'mutable' | 'read-only';
export const AccessMode: { Mutable: 'mutable'; ReadOnly: 'read-only' };
export function accessModeIsMutable(mode: AccessModeValue): boolean;
export function accessModeIsReadOnly(mode: AccessModeValue): boolean;
export function accessModeLabel(mode: AccessModeValue): string;

export class ReadOnlyViolation extends Error {
  constructor(message?: string);
}

export class ReadOnlyNetwork {
  constructor(network: LinkNetwork);
  static new(network: LinkNetwork): ReadOnlyNetwork;
  static fromShared(network: LinkNetwork): ReadOnlyNetwork;
  static from_shared(network: LinkNetwork): ReadOnlyNetwork;
  static from(network: LinkNetwork): ReadOnlyNetwork;
  network(): LinkNetwork;
  shared(): LinkNetwork;
  intoShared(): LinkNetwork;
  into_shared(): LinkNetwork;
  sharedCount(): number;
  shared_count(): number;
  toMutable(): LinkNetwork;
  to_mutable(): LinkNetwork;
  intoMutable(): LinkNetwork;
  into_mutable(): LinkNetwork;
  equals(other: ReadOnlyNetwork): boolean;
}

export class EngineNetwork {
  constructor(mode: AccessModeValue, value: LinkNetwork | ReadOnlyNetwork);
  static withAccessMode(network: LinkNetwork, accessMode?: AccessModeValue): EngineNetwork;
  static with_access_mode(network: LinkNetwork, accessMode?: AccessModeValue): EngineNetwork;
  static mutable(network: LinkNetwork): EngineNetwork;
  static readOnly(view: LinkNetwork | ReadOnlyNetwork): EngineNetwork;
  static read_only(view: LinkNetwork | ReadOnlyNetwork): EngineNetwork;
  accessMode(): AccessModeValue;
  access_mode(): AccessModeValue;
  isMutable(): boolean;
  is_mutable(): boolean;
  isReadOnly(): boolean;
  is_read_only(): boolean;
  network(): LinkNetwork;
  asMutable(): LinkNetwork;
  as_mutable(): LinkNetwork;
  intoReadOnly(): ReadOnlyNetwork;
  into_read_only(): ReadOnlyNetwork;
  intoMutable(): LinkNetwork;
  into_mutable(): LinkNetwork;
}

export function freeze(network: LinkNetwork): ReadOnlyNetwork;
export function asReadOnly(network: LinkNetwork): ReadOnlyNetwork;
export function as_read_only(network: LinkNetwork): ReadOnlyNetwork;
export function parseEngine(
  text: string,
  language: string,
  configuration?: ParseConfiguration,
): EngineNetwork;
export function parse_engine(
  text: string,
  language: string,
  configuration?: ParseConfiguration,
): EngineNetwork;

// --- regions (embedded-language detection) ---

export type RegionDetectionPolicyValue = 'NameDriven' | 'ContentDriven' | 'Both';
export const RegionDetectionPolicy: {
  NameDriven: 'NameDriven';
  ContentDriven: 'ContentDriven';
  Both: 'Both';
};

export class EmbeddedRegion {
  constructor(language: string, span: unknown);
  language(): string;
  span(): unknown;
}

export function detectEmbeddedRegions(
  text: string,
  language: string,
  policy: RegionDetectionPolicyValue,
): EmbeddedRegion[];
export function sniffLanguage(content: string): string | null;

// --- language profiles ---

export class LanguageProfile {
  constructor(name: string, language: string);
  static new(name: string, language: string): LanguageProfile;
  static javascript(): LanguageProfile;
  static builtin(name: string): LanguageProfile | undefined;
  static fromRuleSet(name: string, language: string, ruleSet: unknown): LanguageProfile;
  static from_rule_set(name: string, language: string, ruleSet: unknown): LanguageProfile;
  name(): string;
  language(): string;
  linkTypes(): LinkTypeValue[];
  link_types(): LinkTypeValue[];
  concepts(): string[];
  translationRules(): string[];
  translation_rules(): string[];
  fallbacks(): Map<string, string>;
  withLinkType(linkType: LinkTypeValue): LanguageProfile;
  with_link_type(linkType: LinkTypeValue): LanguageProfile;
  withConcept(concept: string): LanguageProfile;
  with_concept(concept: string): LanguageProfile;
  withTranslationRule(rule: string): LanguageProfile;
  with_translation_rule(rule: string): LanguageProfile;
  withConceptFallback(concept: string, fallback: string): LanguageProfile;
  with_concept_fallback(concept: string, fallback: string): LanguageProfile;
  conceptFallback(concept: string): string | undefined;
  concept_fallback(concept: string): string | undefined;
  supportsLinkType(linkType: LinkTypeValue): boolean;
  supports_link_type(linkType: LinkTypeValue): boolean;
  supportsConcept(concept: string): boolean;
  supports_concept(concept: string): boolean;
  supportsTranslationRule(rule: string): boolean;
  supports_translation_rule(rule: string): boolean;
  declareIn(network: LinkNetwork): LanguageProfileLinks;
  declare_in(network: LinkNetwork): LanguageProfileLinks;
  validateNetwork(network: LinkNetwork): void;
  validate_network(network: LinkNetwork): void;
}

export class LanguageProfileLinks {
  constructor(profile: LinkId, capabilities?: LinkId[]);
  profile(): LinkId;
  capabilities(): LinkId[];
}

export class LanguageProfileViolation extends Error {
  constructor(feature: string, message: string);
  feature(): string;
}

// --- query algebra (link rules) ---

export class LinkRuleParseError extends Error {
  constructor(message: string);
}

export class LinkRuleCapture {
  constructor(name: string, linkIds?: Array<LinkId | number>, text?: string);
  name(): string;
  linkIds(): LinkId[];
  text(): string | undefined;
}

export class LinkRuleCaptures {
  constructor(values?: LinkRuleCapture[]);
  values: LinkRuleCapture[];
  withLink(name: string, linkId: LinkId | number): LinkRuleCaptures;
  withText(name: string, text: string, linkIds: Array<LinkId | number>): LinkRuleCaptures;
  merged(other: LinkRuleCaptures): LinkRuleCaptures;
  first(name: string): LinkId | undefined;
  text(name: string): string | undefined;
  iter(): LinkRuleCapture[];
  [Symbol.iterator](): Iterator<LinkRuleCapture>;
}

export class LinkRuleMatch {
  constructor(linkId: LinkId | number, captures?: LinkRuleCaptures);
  static fromQueryMatch(queryMatch: QueryMatch): LinkRuleMatch;
  withLinkCapture(name: string, linkId: LinkId | number): LinkRuleMatch;
  merge(other: LinkRuleMatch): LinkRuleMatch | undefined;
  mergeAs(linkId: LinkId | number, other: LinkRuleMatch): LinkRuleMatch;
  linkId(): LinkId;
  captures(): LinkRuleCaptures;
}

export class LinkRule {
  static query(query: LinkQuery): LinkRule;
  static kind(kind: string): LinkRule;
  static linkType(linkType: LinkTypeValue): LinkRule;
  static link_type(linkType: LinkTypeValue): LinkRule;
  static language(language: string): LinkRule;
  static namedFlag(named: boolean): LinkRule;
  static named_flag(named: boolean): LinkRule;
  static capture(name: string, rule: LinkRule): LinkRule;
  static typedMetavariable(name: string, kind: string): LinkRule;
  static typed_metavariable(name: string, kind: string): LinkRule;
  static inside(rule: LinkRule, ancestor: LinkRule): LinkRule;
  static has(rule: LinkRule, descendant: LinkRule): LinkRule;
  static precedes(rule: LinkRule, following: LinkRule): LinkRule;
  static follows(rule: LinkRule, preceding: LinkRule): LinkRule;
  static all(rules: LinkRule[]): LinkRule;
  static any(rules: LinkRule[]): LinkRule;
  static negate(rule: LinkRule): LinkRule;
  static named(name: string): LinkRule;
  static ellipsisGap(before: LinkRule, after: LinkRule): LinkRule;
  static ellipsis_gap(before: LinkRule, after: LinkRule): LinkRule;
  static text(pattern: string): LinkRule;
  static fromSexpression(source: string): LinkRule;
  static from_sexpression(source: string): LinkRule;
  matches(network: LinkNetwork, registry: LinkRuleRegistry): LinkRuleMatch[];
}

export class LinkRuleRegistry {
  constructor();
  static new(): LinkRuleRegistry;
  rules: Map<string, LinkRule>;
  withRule(name: string, rule: LinkRule): LinkRuleRegistry;
  with_rule(name: string, rule: LinkRule): LinkRuleRegistry;
  insert(name: string, rule: LinkRule): void;
  get(name: string): LinkRule | undefined;
}

export class TraversalReport {
  constructor(iterations?: number, visited?: number, changed?: number);
  iterations(): number;
  visited(): number;
  changed(): number;
}

export class TraversalStrategy {
  static TopDown: TraversalStrategy;
  static BottomUp: TraversalStrategy;
  static Innermost: TraversalStrategy;
  static Fixpoint: (options: number | { maxIterations: number }) => TraversalStrategy;
  matches(network: LinkNetwork, rule: LinkRule, registry: LinkRuleRegistry): LinkRuleMatch[];
  applyMut(
    network: LinkNetwork,
    rule: LinkRule,
    registry: LinkRuleRegistry,
    visitor: (network: LinkNetwork, match: LinkRuleMatch) => boolean,
  ): TraversalReport;
  apply_mut(
    network: LinkNetwork,
    rule: LinkRule,
    registry: LinkRuleRegistry,
    visitor: (network: LinkNetwork, match: LinkRuleMatch) => boolean,
  ): TraversalReport;
}

export type LinkRuleSnapshotExpectationValue = 'Valid' | 'Invalid';
export const LinkRuleSnapshotExpectation: { Valid: 'Valid'; Invalid: 'Invalid' };

export class LinkRuleSnapshotCase {
  constructor(
    name: string,
    source: string,
    language: string,
    expectation: LinkRuleSnapshotExpectationValue,
  );
  static new(
    name: string,
    source: string,
    language: string,
    expectation: LinkRuleSnapshotExpectationValue,
  ): LinkRuleSnapshotCase;
  name(): string;
  source(): string;
  language(): string;
  expectation(): LinkRuleSnapshotExpectationValue;
}

export class LinkRuleSnapshotSuite {
  constructor(rule: LinkRule);
  static new(rule: LinkRule): LinkRuleSnapshotSuite;
  withCase(snapshotCase: LinkRuleSnapshotCase): LinkRuleSnapshotSuite;
  with_case(snapshotCase: LinkRuleSnapshotCase): LinkRuleSnapshotSuite;
  run(
    registry: LinkRuleRegistry,
    configuration?: ParseConfiguration,
    networkFactory?: (source: string, language: string, configuration?: ParseConfiguration) => LinkNetwork,
  ): LinkRuleSnapshotReport;
}

export class LinkRuleSnapshotReport {
  constructor(cases?: LinkRuleSnapshotResult[]);
  isSuccess(): boolean;
  is_success(): boolean;
  cases(): LinkRuleSnapshotResult[];
}

export class LinkRuleSnapshotResult {
  constructor(
    name: string,
    expectation: LinkRuleSnapshotExpectationValue,
    matched: boolean,
    matchCount: number,
    passed: boolean,
  );
  name(): string;
  expectation(): LinkRuleSnapshotExpectationValue;
  matched(): boolean;
  matchCount(): number;
  match_count(): number;
  passed(): boolean;
}
