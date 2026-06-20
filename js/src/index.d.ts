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

export const LinkType: Record<string, LinkTypeValue>;
export const ApiOperation: Record<string, string>;
export const ApiStyle: Record<string, string | string[]>;
export const ApiStyleFixtureKind: Record<string, string>;
export const API_OPERATIONS: ApiOperationEntry[];

export class LinkId {
  constructor(value: number | string | LinkId);
  static from(value: number | string | LinkId): LinkId;
  static fromU64(value: number): LinkId;
  asU64(): number;
  equals(other: number | string | LinkId): boolean;
}

export class LinkMetadata {
  static new(): LinkMetadata;
  withLinkType(linkType: LinkTypeValue): LinkMetadata;
  withTerm(term: string): LinkMetadata;
  withLanguage(language: string): LinkMetadata;
  withNamed(named?: boolean): LinkMetadata;
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
  constructor(name: string, query: LinkQuery);
  withTemplate(language: string, text: string): TranslationRule;
}

export class TranslationRuleSet {
  constructor(name: string);
  withRule(rule: TranslationRule): TranslationRuleSet;
  render(targetLanguage: string, network: LinkNetwork): string;
  toLino(): string;
  static fromLino(source: string): TranslationRuleSet;
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
