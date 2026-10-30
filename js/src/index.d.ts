export * from './query-algebra.js';

export type DecoratorLevel =
  | 'importer'
  | 'grammar-rule'
  | 'merge-decision'
  | 'concept-mapping'
  | 'executor'
  | 'recovery'
  | 'cst-to-ast'
  | 'transformation'
  | 'emitter'
  | 'translation-rule';

export const DECORATOR_LEVELS: readonly DecoratorLevel[];

export type DecoratorRecord = Record<string, string>;

export type DecoratorAction =
  | { op: 'set'; field: string; value: string }
  | { op: 'replace'; field: string; from: string; to: string }
  | { op: 'drop' };

export interface Decorator {
  readonly id: string;
  readonly level: DecoratorLevel;
  readonly order: number;
  readonly when: ReadonlyArray<readonly [string, string]>;
  readonly actions: readonly DecoratorAction[];
}

export interface DecoratorSpec {
  id: string;
  level: DecoratorLevel;
  order?: number;
  when?: Array<[string, string]>;
  actions: DecoratorAction[];
}

export class DecoratorError extends Error {}

export function decorator(spec: DecoratorSpec): Decorator;

export class DecoratorSet {
  constructor(decorators?: Array<Decorator | DecoratorSpec>);
  readonly decorators: readonly Decorator[];
  readonly size: number;
  static empty(): DecoratorSet;
  static fromLino(text: string): DecoratorSet;
  ids(): string[];
  add(...decorators: Array<Decorator | DecoratorSpec>): DecoratorSet;
  remove(id: string): DecoratorSet;
  forLevel(level: DecoratorLevel): Decorator[];
  has(level: DecoratorLevel): boolean;
  decorate(level: DecoratorLevel, record: DecoratorRecord): DecoratorRecord | null;
  decorateAll(level: DecoratorLevel, records: DecoratorRecord[]): DecoratorRecord[];
  toLino(): string;
}

export type DecoratorInput = DecoratorSet | Array<Decorator | DecoratorSpec> | null | undefined;

/** The options every decorated hook takes. */
export interface DecoratorOptions {
  decorators?: DecoratorInput;
}

export function decoratorSet(decorators: DecoratorInput): DecoratorSet;
export function decorateGrammar(grammar: Grammar, decorators: DecoratorInput, level?: 'importer' | 'grammar-rule'): Grammar;
export function decorateSyntaxTree(tree: SyntaxTreeNode | null, decorators: DecoratorInput): SyntaxTreeNode | null;

export type SelfTranslationLanguage = 'JavaScript' | 'TypeScript' | 'Rust';

/** The languages self-translation reads and writes. */
export const SELF_TRANSLATION_LANGUAGES: readonly SelfTranslationLanguage[];

export class SelfTranslationError extends Error {}

/** The self-translation language `language` (a name or file extension) names, or `null`. */
export function selfTranslationLanguage(language: string): SelfTranslationLanguage | null;

export type SelfTranslationStatus = 'kept' | 'translated' | 'restored' | 'comment' | 'provenance' | 'carried';

/** One top-level source item and what self-translation did with it. */
export interface SelfTranslationItem {
  readonly term: string;
  readonly start: number;
  readonly end: number;
  readonly status: SelfTranslationStatus;
  readonly reason: string | null;
  readonly diagnostic?: SelfTranslationDiagnostic;
}

export interface SelfTranslationDiagnostic {
  readonly kind: 'syntax' | 'type' | 'unsupported';
  readonly reason: string;
  readonly message: string;
  readonly span: { start: number; end: number } | null;
  readonly details: Record<string, unknown>;
  readonly sourceRange: { start: number; end: number };
}

export interface SelfTranslation {
  readonly sourceLanguage: SelfTranslationLanguage;
  readonly targetLanguage: SelfTranslationLanguage;
  readonly code: string;
  readonly items: readonly SelfTranslationItem[];
}

/** A portable-core type, as the checker writes it (`{ kind: 'float' }`, `{ kind: 'array', element }`). */
export type SelfTranslationType = { readonly kind: string; readonly [field: string]: unknown };

/**
 * The signature of an exported item of a JavaScript or TypeScript module,
 * which binds the name a module that imports it uses.
 */
export type SelfTranslationSignature =
  | {
      readonly k: 'fn';
      readonly name: string;
      readonly params: readonly { readonly name: string; readonly type: SelfTranslationType }[];
      readonly ret: SelfTranslationType;
    }
  | { readonly k: 'const'; readonly name: string; readonly type: SelfTranslationType; readonly literal: boolean };

export interface SelfTranslationOptions {
  /** Reparse actual emitted code; saved originals never restore semantics. */
  readonly semanticTranslation?: boolean;
  /** Selects an independently checked standard Rust shared-array source interpretation. */
  readonly rustSharedArrayHandleSourceModel?: '' | 'standard Rust shared array identity handles 1' | 'standard Rust shared array identity handles 2';
  /** Uses one UTF-16 string representation across all emitted module items. */
  readonly utf16Strings?: boolean;
  /** Forces shared reference values across module items; observable identity and mutation also select them automatically. */
  readonly referenceSemantics?: boolean;
  /** Emitter decorators applied to each translated item. */
  readonly decorators?: unknown;
  /** The module's directory inside its crate, as path segments; the crate root by default. */
  readonly moduleDirectory?: readonly string[];
  /** The signatures `selfTranslationSignatures` gives for each imported module, by import specifier. */
  readonly imports?: Readonly<Record<string, readonly SelfTranslationSignature[]>>;
}
export interface TypeScriptErasure {
  start: number;
  end: number;
  term: string;
}
export interface ErasedTypeScript {
  code: string;
  erasures: TypeScriptErasure[];
}
export function eraseTypeScript(source: string): ErasedTypeScript;
/** Projects supported TypeScript compile-time annotations into the shared JavaScript frontend's JSDoc form. */
export function projectTypeScriptTyping(source: string): string;

/**
 * Translates one of meta-language's own modules between JavaScript,
 * TypeScript and Rust through links: same-language translation is byte for
 * byte, and translating an unedited translation back restores the source.
 * Into Rust, an item may call and read the module's other items, and a
 * relative named import becomes a `use crate::…` declaration.
 */
export function selfTranslate(source: string, from: string, to: string, options?: SelfTranslationOptions): SelfTranslation;
export function selfTranslateWithDiagnostics(source: string, from: string, to: string, options?: SelfTranslationOptions): {
  translation: SelfTranslation; diagnostics: Array<SelfTranslationDiagnostic | null>;
};

/**
 * The signatures of the exported items of a JavaScript or TypeScript module
 * that translate into Rust, for the `imports` of a module that imports them.
 */
export function selfTranslationSignatures(source: string, language: string, options?: SelfTranslationOptions): SelfTranslationSignature[];

export type NetworkProjectionValue = 'Lossless' | 'ConcreteSyntax' | 'AbstractSyntax' | 'Semantic';
export const NetworkProjection: Readonly<{
  Lossless: 'Lossless';
  ConcreteSyntax: 'ConcreteSyntax';
  AbstractSyntax: 'AbstractSyntax';
  Semantic: 'Semantic';
}>;

export type LinkTypeValue =
  | 'Concept'
  | 'Document'
  | 'Dynamic'
  | 'Field'
  | 'Grammar'
  | 'Language'
  | 'Object'
  | 'Reference'
  | 'Region'
  | 'Relation'
  | 'Semantic'
  | 'SourceToken'
  | 'Syntax'
  | 'Trivia'
  | 'Type';

export const LinkType: Record<string, LinkTypeValue> & {
  Concept: 'Concept';
  Document: 'Document';
  Dynamic: 'Dynamic';
  Field: 'Field';
  Grammar: 'Grammar';
  Language: 'Language';
  Object: 'Object';
  Reference: 'Reference';
  Region: 'Region';
  Relation: 'Relation';
  Semantic: 'Semantic';
  SourceToken: 'SourceToken';
  Token: 'SourceToken';
  Syntax: 'Syntax';
  Trivia: 'Trivia';
  Type: 'Type';
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

export const QUERY_PLAN_VERSION: 1;
export const QueryOperation: {
  Select: 'select';
  Insert: 'insert';
  Update: 'update';
  Delete: 'delete';
};
export type QueryOperationValue = typeof QueryOperation[keyof typeof QueryOperation];

export const QueryAuthorization: { readonly Required: 'Required' };
export const SqlAdapterErrorKind: {
  UnsupportedLanguage: 'UnsupportedLanguage';
  InvalidConcreteSyntax: 'InvalidConcreteSyntax';
  Syntax: 'Syntax';
  Semantic: 'Semantic';
  Registry: 'Registry';
};
export type SqlAdapterErrorKindValue =
  typeof SqlAdapterErrorKind[keyof typeof SqlAdapterErrorKind];

export class SqlAdapterError extends Error {
  constructor(kind: SqlAdapterErrorKindValue, message: string, offset?: number);
  kind: SqlAdapterErrorKindValue;
  offset?: number;
}

export const QueryComparisonOperator: {
  Equal: 'eq';
  NotEqual: 'neq';
  LessThan: 'lt';
  LessThanOrEqual: 'lte';
  GreaterThan: 'gt';
  GreaterThanOrEqual: 'gte';
  In: 'in';
  NotIn: 'not-in';
  Like: 'like';
  IsNull: 'is-null';
};
export type QueryComparisonOperatorValue =
  typeof QueryComparisonOperator[keyof typeof QueryComparisonOperator];
export const QuerySortDirection: { Ascending: 'asc'; Descending: 'desc' };
export type QuerySortDirectionValue =
  typeof QuerySortDirection[keyof typeof QuerySortDirection];
export const QueryAggregateFunction: {
  Count: 'count';
  Sum: 'sum';
  Average: 'avg';
  Minimum: 'min';
  Maximum: 'max';
  PopulationVariance: 'variance-population';
  PopulationStandardDeviation: 'stddev-population';
};
export type QueryAggregateFunctionValue =
  typeof QueryAggregateFunction[keyof typeof QueryAggregateFunction];

export type QueryValue = null | boolean | number | string | QueryValue[] | {
  [field: string]: QueryValue;
};
export type QueryFilter =
  | { compare: { field: string; operator: QueryComparisonOperatorValue; value: QueryValue } }
  | { and: QueryFilter[] }
  | { or: QueryFilter[] }
  | { not: QueryFilter };

export class QuerySourceEvidence {
  constructor(role: string, span: SourceSpan);
  role(): string;
  span(): SourceSpan;
}

export class QueryPlan {
  constructor(operation: QueryOperationValue, resource: string);
  operation: QueryOperationValue;
  resource: string;
  projection: string[];
  filter: QueryFilter | null;
  order: Array<{ field: string; direction: QuerySortDirectionValue }>;
  limit: number | null;
  offset: number | null;
  groupBy: string[];
  aggregates: Array<{
    function: QueryAggregateFunctionValue;
    field: string | null;
    alias: string | null;
  }>;
  mutation: Record<string, QueryValue>;
  sourceEvidence(): QuerySourceEvidence[];
  authorization(): 'Required';
  toCanonicalObject(): object;
  canonicalJson(): string;
}

export const GraphQlOperationType: { Query: 'query'; Mutation: 'mutation' };
export type GraphQlOperationTypeValue =
  typeof GraphQlOperationType[keyof typeof GraphQlOperationType];
export const GraphQlArgumentRole: {
  Filter: 'filter';
  Order: 'order';
  Limit: 'limit';
  Offset: 'offset';
  Group: 'group';
  MutationInput: 'mutation-input';
};
export type GraphQlArgumentRoleValue =
  typeof GraphQlArgumentRole[keyof typeof GraphQlArgumentRole];

export class GraphQlAdapterError extends Error {}

export class GraphQlRootMapping {
  constructor(
    sourceOperation: GraphQlOperationTypeValue,
    sourceField: string,
    operation: QueryOperationValue,
    resource: string,
  );
  withArgument(sourceName: string, role: GraphQlArgumentRoleValue): GraphQlRootMapping;
  withField(sourceName: string, canonicalField: string): GraphQlRootMapping;
  withAggregate(
    sourceName: string,
    aggregate: QueryAggregateFunctionValue,
  ): GraphQlRootMapping;
}

export class GraphQlSchemaRegistry {
  constructor();
  registerRoot(mapping: GraphQlRootMapping): GraphQlSchemaRegistry;
  static fromJson(value: unknown): GraphQlSchemaRegistry;
}

export class LoweredQueryPlan {
  plan(): QueryPlan;
  network(): LinkNetwork;
  rootLink(): LinkId;
}

export function lowerGraphQl(source: string, registry: GraphQlSchemaRegistry): LoweredQueryPlan;
export const lowerGraphQL: typeof lowerGraphQl;
export const lowerGraphql: typeof lowerGraphQl;

export interface SqlDialectProfile {
  readonly key: string;
  readonly vendor: string;
}

export const SQL_DIALECT_PROFILES: readonly SqlDialectProfile[];

export class SqlRelationMapping {
  constructor(sourceRelation: string, resource: string);
  sourceRelation: string;
  resource: string;
  fields: Map<string, string>;
  withField(sourceName: string, canonicalField: string): SqlRelationMapping;
}

export class SqlSchemaRegistry {
  constructor();
  registerRelation(mapping: SqlRelationMapping): SqlSchemaRegistry;
  static fromJson(value: unknown): SqlSchemaRegistry;
}

export function lowerSql(
  source: string,
  language: string,
  registry: SqlSchemaRegistry,
): LoweredQueryPlan;
export const lowerSQL: typeof lowerSql;
export const lower_sql: typeof lowerSql;

export interface GrammarProvenance {
  id: string;
  version: string;
  parserSha256: string;
}

export interface LanguageCatalogEntry {
  name: string;
  family: string;
  aliases: string[];
  extensions: string[];
  grammars: GrammarProvenance[];
}

export interface ParseGrammar extends GrammarProvenance {
  language: string;
}

export const LANGUAGE_CATALOG: { readonly languages: readonly LanguageCatalogEntry[] };
export function languageEntry(language: string): LanguageCatalogEntry | undefined;
export function canonicalLanguageName(language: string): string | undefined;
export function languageCandidatesForPath(path: string): string[];
export function languageForPath(path: string): string | undefined;
export function grammarProvenance(language: string): readonly GrammarProvenance[];

/** Whether a canonical name is a concept (a noun phrase) or an operation (a verb phrase). */
export type ConceptRole = 'concept' | 'operation';

/** The name a source (a grammar format, a programming language, a surface) gives a concept. */
export interface SourceAlias {
  readonly source: string;
  readonly name: string;
}

/** A concept whose phrase is a synonym of another's, with what distinguishes them. */
export interface ConceptDistinction {
  readonly id: string;
  readonly reason: string;
}

/** The record of one canonical concept or operation (parity/naming/canonical-concepts.json). */
export interface ConceptRecord {
  readonly id: string;
  readonly phrase: string;
  readonly role: ConceptRole;
  readonly definition: string;
  readonly constraints: readonly string[];
  readonly sourceAliases: readonly SourceAlias[];
  readonly formerNames: readonly string[];
  readonly represents?: string;
  readonly distinctFrom?: readonly ConceptDistinction[];
}

export interface ConceptRecordSeedReport {
  concepts: number;
  links: number;
}

export interface ConceptOntologyImportReport {
  concepts: number;
  assigned: number;
  renamed: number;
  aliasLinks: number;
  syntaxMappings: number;
}

export const CONCEPT_RECORDS: readonly ConceptRecord[];
export const FORMER_CONCEPT_ID_VOCABULARY: 'meta-language';
export const FORMER_CONCEPT_IDS: readonly (readonly [former: string, current: string])[];
export function currentConceptId(id: string): string;
export function conceptRecords(): readonly ConceptRecord[];
export function conceptRecord(name: string): ConceptRecord | undefined;
export function conceptRecordsForSourceName(source: string, name: string): ConceptRecord[];
export function insertConceptRecord(
  network: LinkNetwork,
  nameOrRecord: string | ConceptRecord,
): { concept: LinkId; links: number };

/** Two concepts that must never be merged, with the reason. */
export interface RequiredDistinction {
  readonly concepts: readonly [string, string];
  readonly reason: string;
}

export type CorrespondenceRelation = 'shared' | 'distinct' | 'ambiguous' | 'unknown';

/** The relation between two source spellings and what justifies it. */
export interface ConceptCorrespondence {
  relation: CorrespondenceRelation;
  first: string[];
  second: string[];
  shared: string | null;
  justification: string | null;
  correspondence: string | null;
}

export interface ConceptDistinctionProblem {
  kind: string;
  subject: string;
  message: string;
}

export interface ConceptCorrespondenceOptions {
  within?: string;
  records?: readonly ConceptRecord[];
  register?: unknown;
}

/** One use of precedence in a grammar and the concept it expresses. */
export interface PrecedenceUse {
  rule: string;
  label: string;
  concept: 'grammar.lexical-precedence' | 'grammar.syntactic-precedence';
}

export const REQUIRED_CONCEPT_DISTINCTIONS: readonly RequiredDistinction[];
export const REQUIRED_FOUNDATION_DISTINCTIONS: readonly (readonly [string, string])[];
export function sourceMeanings(alias: SourceAlias, options?: ConceptCorrespondenceOptions): string[];
export function conceptCorrespondence(first: SourceAlias, second: SourceAlias, options?: ConceptCorrespondenceOptions): ConceptCorrespondence;
export function checkConceptDistinctions(records?: readonly ConceptRecord[], register?: unknown): ConceptDistinctionProblem[];
/** The concept a grammar expression denotes; a choice is ordered or unordered choice. */
export function grammarExprConceptId(expression: { kind: string; ordered?: boolean }): string;
export function grammarPrecedenceConcepts(grammar: Grammar): PrecedenceUse[];

/** A rule of a native grammar and the concept it names. */
export interface NativeRuleConcept {
  rule: string;
  concept: string | null;
}

export type NativeConceptProblemKind =
  | 'rule-without-concept'
  | 'concept-without-record'
  | 'rule-without-alias'
  | 'alias-without-rule'
  | 'namespace-mismatch';

export interface NativeConceptProblem {
  kind: NativeConceptProblemKind;
  grammar: string;
  rule: string;
  concept: string | null;
}

export interface NativeConceptCheckOptions {
  records?: readonly ConceptRecord[];
  grammars?: readonly string[];
  ruleConcepts?: (grammar: string) => NativeRuleConcept[];
}

/** The per-language reuse report of the native grammars. */
export interface NativeConceptReuse {
  grammars: {
    grammar: string;
    language: string;
    rules: number;
    shared: { rule: string; concept: string; languages: string[] }[];
    specific: { rule: string; concept: string }[];
  }[];
  concepts: { concept: string; languages: string[]; shared: boolean }[];
}

export type ConstructTranslationRelation = 'translated' | 'untranslatable' | 'ambiguous' | 'unknown';

export interface ConstructTranslation {
  relation: ConstructTranslationRelation;
  concept: string | null;
  rules: string[];
}

/** A node of a native parse tree whose kind is a rule of the grammar. */
export interface ConstructTree {
  kind: string;
  concept: string;
  children: ConstructTree[];
}

export function nativeGrammarIds(): string[];
export function nativeGrammarLanguage(id: string): string;
export function nativeGrammarSource(id: string): string;
export function nativeGrammarRuleConcepts(id: string): NativeRuleConcept[];
export function checkNativeGrammarConcepts(options?: NativeConceptCheckOptions): NativeConceptProblem[];
export function nativeGrammarConceptReuse(options?: { records?: readonly ConceptRecord[]; grammars?: readonly string[] }): NativeConceptReuse;
export function translateNativeConstruct(
  from: string,
  rule: string,
  to: string,
  options?: { records?: readonly ConceptRecord[] } & DecoratorOptions,
): ConstructTranslation;
export function nativeConstructTree(id: string, source: string): ConstructTree;
export function translateNativeConstructTree(
  tree: ConstructTree,
  from: string,
  to: string,
  options?: { records?: readonly ConceptRecord[] },
): { tree: ConstructTree | null; problems: { kind: string; concept: string | null; relation: ConstructTranslationRelation }[] };

/** Node kind and field names of a default grammar; `fields` starts at field id 1. */
export interface GrammarNames {
  nodeKinds: string[];
  fields: string[];
}
export function grammarNames(grammarId: string): GrammarNames | undefined;

export class LinkId {
  constructor(value: number | string | LinkId);
  static from(value: number | string | LinkId): LinkId;
  static fromU64(value: number): LinkId;
  asU64(): number;
  equals(other: number | string | LinkId): boolean;
}
export type LinkIdValue = LinkId | number | string;

export class ByteRange {
  constructor(start?: number, end?: number);
  start: number;
  end: number;
  contains(other: ByteRange): boolean;
}

export class Point {
  constructor(row?: number, column?: number);
  row: number;
  column: number;
}

export class SourceSpan {
  constructor(byteRange?: ByteRange, start?: Point, end?: Point);
  byteRange: ByteRange;
  start: Point;
  end: Point;
}

export class LinkFlags {
  constructor(options?: {
    isError?: boolean;
    hasError?: boolean;
    isMissing?: boolean;
    isExtra?: boolean;
  });
  isError: boolean;
  hasError: boolean;
  isMissing: boolean;
  isExtra: boolean;
  static clean(): LinkFlags;
  withError(value?: boolean): LinkFlags;
  withMissing(value?: boolean): LinkFlags;
  withExtra(value?: boolean): LinkFlags;
  hasRecoveryIssue(): boolean;
}

export class LinkMetadata {
  term?: string;
  static new(): LinkMetadata;
  definition?: string;
  span?: SourceSpan;
  flags: LinkFlags;
  withLinkType(linkType: LinkTypeValue): LinkMetadata;
  withTerm(term: string): LinkMetadata;
  withLanguage(language: string): LinkMetadata;
  withNamed(named?: boolean): LinkMetadata;
  withDefinition(definition?: string): LinkMetadata;
  withSpan(span: SourceSpan | undefined): LinkMetadata;
  withFlags(flags: LinkFlags): LinkMetadata;
}

export type LanguageParserFunction = (
  text: string,
  language: string,
  configuration: ParseConfiguration,
) => LinkNetwork;

export interface LanguageParserObject {
  parseSource(
    text: string,
    language: string,
    configuration: ParseConfiguration,
  ): LinkNetwork;
}

export type LanguageParser = LanguageParserFunction | LanguageParserObject;

export class ParserRegistry {
  constructor(fallback?: LanguageParserFunction);
  register(language: string, parser: LanguageParser): ParserRegistry;
  withParser(language: string, parser: LanguageParser): ParserRegistry;
  with_parser(language: string, parser: LanguageParser): ParserRegistry;
  parserFor(language: string): LanguageParser | undefined;
  parser_for(language: string): LanguageParser | undefined;
  isRegistered(language: string): boolean;
  is_registered(language: string): boolean;
  size(): number;
  len(): number;
  isEmpty(): boolean;
  is_empty(): boolean;
  parse(text: string, language: string, configuration?: ParseConfiguration): LinkNetwork;
}

export const LANGUAGE_REPRESENTATION_SCHEMA_VERSION: 2;
export const RepresentationLevel: {
  readonly Preserved: 'preserved';
  readonly ConcreteSyntax: 'concrete-syntax';
  readonly Parsed: 'parsed';
  readonly Resolved: 'resolved';
  readonly Elaborated: 'elaborated';
  readonly Opaque: 'opaque';
  readonly NotApplicable: 'not-applicable';
  readonly Unavailable: 'unavailable';
};
export type RepresentationLevelValue =
  typeof RepresentationLevel[keyof typeof RepresentationLevel];

export interface LanguageSupport {
  readonly schemaVersion: 2;
  readonly name: 'JavaScript' | 'Rust' | 'Lean' | 'Rocq';
  readonly aliases: readonly string[];
  readonly version: string;
  readonly edition: string;
  readonly extensions: readonly string[];
  readonly sourceBytes: RepresentationLevelValue;
  readonly concreteSyntax: RepresentationLevelValue;
  readonly bindingResolution: RepresentationLevelValue;
  readonly typeElaboration: RepresentationLevelValue;
  readonly dynamicExtensions: RepresentationLevelValue;
  readonly proofSyntax: RepresentationLevelValue;
  readonly emitter: 'ordered source-token emitter';
}

export const TranslationSupport: {
  readonly PortableEncoding: 'portable-encoding';
  readonly SemanticSubset: 'semantic-subset';
  readonly SemanticTranslation: 'semantic-translation';
};
export type TranslationSupportValue =
  typeof TranslationSupport[keyof typeof TranslationSupport];

export interface TranslationContract {
  readonly schemaVersion: 2;
  readonly source: LanguageSupport['name'];
  readonly target: LanguageSupport['name'];
  readonly support: TranslationSupportValue;
  readonly observation: string;
  readonly requiredRuntime: string;
  readonly encoding: string;
  readonly assumptions: readonly string[];
  readonly obligation: string | null;
}

export function languageSupport(languageName: string): LanguageSupport | undefined;
export function fourLanguageSupport(): readonly LanguageSupport[];
export function translationContracts(): readonly TranslationContract[];
export function translationContract(
  sourceLanguage: string,
  targetLanguage: string,
): TranslationContract | undefined;

export const SEMANTIC_OBSERVATION: string;
export const SEMANTIC_ENCODING: string;

export interface TranslationSpan {
  readonly start: number;
  readonly end: number;
}

export interface TranslationMapping {
  readonly kind: string;
  readonly source: string;
  readonly target: string;
  readonly sourceSpan: TranslationSpan | null;
}

export interface TranslationEncoding {
  readonly id: string;
  readonly statement: string;
}

export interface TranslationAssumption {
  readonly id: string;
  readonly statement: string;
  readonly details: readonly string[];
}

export interface TranslationObligation {
  readonly source: string;
  readonly target: string;
  readonly kind: 'theorem' | 'assertion';
  readonly closedGoal: boolean;
  readonly discharge: 'target-kernel' | 'source-kernel' | 'runtime-assertion';
  readonly check: 'bounded' | null;
}

export interface TranslationProvenance {
  readonly translator: string;
  readonly sourceLanguage: LanguageSupport['name'];
  readonly sourceSha256: string;
  readonly sourceBytes: number;
  readonly header: string;
}

export interface SemanticTranslation {
  readonly entry: string | null;
  readonly observationProcedure: string;
  readonly encodings: readonly TranslationEncoding[];
  readonly assumptions: readonly TranslationAssumption[];
  readonly obligations: readonly TranslationObligation[];
  readonly mappings: readonly TranslationMapping[];
  readonly runtimeDependencies: readonly string[];
  readonly provenance: TranslationProvenance;
}

export interface TranslationDiagnostic {
  readonly kind: 'syntax' | 'type' | 'unsupported';
  readonly message: string;
  readonly span: TranslationSpan | null;
}

export interface ProgramTranslation {
  readonly sourceLanguage: LanguageSupport['name'];
  readonly targetLanguage: LanguageSupport['name'];
  readonly code: string;
  readonly contract: TranslationContract;
  /** Present when the program is in the portable core and was translated. */
  readonly semantics: SemanticTranslation | null;
  /** Why the program stayed outside the portable core, when it did. */
  readonly diagnostic: TranslationDiagnostic | null;
}

export interface ReadTranslationProvenance {
  readonly sourceLanguage: LanguageSupport['name'];
  readonly sourceSha256: string;
  readonly sourceBytes: number;
}

export interface DecodedProgramTranslation {
  readonly sourceLanguage: LanguageSupport['name'];
  readonly source: string;
}

export function translateProgram(
  source: string,
  sourceLanguage: string,
  targetLanguage: string,
  options?: { utf16Strings?: boolean; referenceSemantics?: boolean },
): ProgramTranslation;
export function decodeProgramTranslation(
  code: string,
  targetLanguage: string,
): DecodedProgramTranslation;
export function readTranslationProvenance(
  code: string,
  targetLanguage: string,
): ReadTranslationProvenance;

/** A source-free checked semantic program with direct declaration and binder links. */
export interface TypedProgramLinks {
  readonly network: LinkNetwork;
  readonly root: LinkId;
}
export interface CheckedTypedProgram {
  readonly language: string;
  readonly declarations: Map<string, object>;
  readonly main: object;
  readonly [field: string]: unknown;
}
export interface TypedProgramLinksEmission {
  readonly code: string;
  readonly language: 'JavaScript' | 'Rust' | 'Lean' | 'Rocq';
  readonly mappings: readonly TranslationMapping[];
  readonly assumptions: readonly TranslationAssumption[];
  readonly encodings: readonly TranslationEncoding[];
  readonly theorems: readonly TranslationObligation[];
}
/** Selects supported emission models from validated owned program facts. */
export interface TypedProgramLinksEmissionOptions {
  /** Preserves selected immutable-reference Rust headers and closed call interfaces. */
  readonly rustSourceHeaderEmissionModel?: 'primitive borrowed Rust function header 1' | 'closed primitive borrowed Rust call interface 1' | 'retained scalar Rust function interface 1' | 'retained mathematical Rust function interface 1' | 'retained textual Rust function interface 1' | 'retained string predicate Rust function interface 1' | 'retained constructed Rust string function interface 1' | 'retained displayed Rust string function interface 1';
  /** Allocates JavaScript aliases from actual checked local binding owners. */
  readonly javaScriptScopedBindingEmissionModel?: 'owned JavaScript scoped bindings 1';
}
/** Actual UTF-16 units of primitive Unicode scalar text; rejects unpaired surrogates. */
export function readSourceTextUnits(text: string): number[];
/** Exact UTF-8 byte length under the bounded Unicode scalar text model. */
export function readSourceTextByteLength(text: string): number;
/** Exact UTF-8 byte, or null for a nonintegral or out-of-bounds offset. */
export function readSourceTextByte(text: string, offset: number): number | null;
/** Unchanged substring at ordered UTF-8 scalar boundaries, or null. */
export function sliceSourceTextBytes(text: string, start: number, end: number): string | null;

/** Imports the supported checked JavaScript or Rust core as editable semantic links. */
export function createTypedProgramLinks(
  source: string,
  language: string,
  options?: { provenance?: unknown; utf16Strings?: boolean; context?: object;
    /** Imports one explicitly selected standard Rust shared-array source domain. */
    rustSharedArrayHandleSourceModel?: 'standard Rust shared array identity handles 1' | 'standard Rust shared array identity handles 2';
    rustArrayResultModel?: 'immutable fresh scalar vector results 1';
    rustSourceVersion?: string;
    rustCompletionModel?: 'closed guarded function returns 1';
    /** Retains const qualification on closed primitive predicates; no compile-time evaluation requests. */
    rustConstFunctionModel?: 'closed pure Rust const predicates 1' | 'closed Boolean Rust const decision trees 1';
    /** Retains owned source header facts; does not promise source ABI emission. */
    rustSourceSignatureModel?: 'Rust source function signature 1',
    /** Resolves qualified and imported standard binary64 owners under explicit source checks. */
    rustPrimitiveOwnerModel?: 'standard primitive Rust binary64 owner 1',
    /** Retains checked owned text state, source moves and canonical local copies. */
    rustOwnedLocalStateModel?: 'closed owned text local state 1' | 'canonical owned text local copies 1' | 'closed owned text tuple initialization calls 1',
    /** Retains closed zero-argument private or bare-public pure Unit helpers and owned calls. */
    rustRuntimeShellModel?: 'closed pure Rust unit runtime shell 1',
    rustPureUnitCallModel?: 'closed pure Rust unit calls 1' | 'closed public pure Rust unit calls 1',
    /** Resolves owned Rust header paths against the separate type namespace. */
    rustTypeNamespaceModel?: 'resolved Rust type namespace 1',
    /** Captures closed zero-argument primitive empty-option Rust functions. */
    rustEmptyOptionSourceModel?: 'closed empty primitive Rust option 1',
    rustScalarLoopModel?: 'closed primitive scalar loop 1' | 'closed declaration scalar loop 1',
    javaScriptScalarLoopModel?: 'closed primitive scalar loop 1' | 'closed declaration scalar loop 1' },
): TypedProgramLinks;
export function encodeTypedProgramLinks(
  program: CheckedTypedProgram,
  options?: { provenance?: unknown; valueSharing?: 'exact scalars, types and source spans 1'; recordShapes?: 'ordered typed program record fields 1' },
): TypedProgramLinks;
export function encodeTypedProgramLinksWithValueSharing(program:CheckedTypedProgram,provenance:unknown,profile:string):TypedProgramLinks;
export function encodeTypedProgramLinksWithRecordShapes(program:CheckedTypedProgram,provenance:unknown,profile:string):TypedProgramLinks;
export const SOURCE_IDENTIFIER_UNICODE_VERSION: '17.0.0';
export function normalizeUnicodeText(source: string): string;
export function acceptLinkAllocationWord(value: number): boolean;
export function acceptLinkAllocationCapacity(
  first: number, second: number, third: number, fourth: number,
  exhausted: boolean, count: number,
): boolean;
export function createNumberBitwiseProgramLinks(): TypedProgramLinks;
/** Complete ordinary Rust headers retained under source signature version two. */
export function createRustSourceOwnerProgramLinks(source: string, options?: { sourceVersion?: string }): TypedProgramLinks;
export interface RustSourceProjectSource { path:string; source:string }
export interface RustSourceProjectOptions {
  library?:boolean; retainSourceTypeOwners?:boolean; provenance?:unknown;
  fileLimit?:number; byteLimit?:number; tokenLimit?:number; depthLimit?:number;
  expansionLimit?:number; outputUnitLimit?:number; nodeLimit?:number;
  referenceLimit?:number;
}
export interface RustSourceProjectMapping {
  path:string; start:number; end:number; startUnit:number; endUnit:number;
  module:string; provider:string;
}
export interface RustSourceProjectComposition {
  schemaVersion:1; model:'complete Rust source module composition 1';
  entry:string; source:string;
  files:(RustSourceProjectSource & {sourceBytes:number;sourceSha256:string})[];
  modules:RustSourceProjectMapping[];
  mappings:(RustSourceProjectMapping & {kind:string;outputStart:number;outputEnd:number})[];
  unusedSources:string[];
}
export function composeRustSourceProject(inputs:RustSourceProjectSource[],entry:string,options?:RustSourceProjectOptions):RustSourceProjectComposition;
export function parseRustSourceProject(inputs:RustSourceProjectSource[],entry:string,options?:RustSourceProjectOptions):{composition:RustSourceProjectComposition;surface:unknown;entryNormalization:string};
export function createRustSourceProjectProgramLinks(inputs:RustSourceProjectSource[],entry:string,options?:RustSourceProjectOptions):TypedProgramLinks;
export function readRustSourceProjectConcreteGraphs(network:LinkNetwork,root:LinkIdValue,options?:RustSourceProjectOptions):{path:string;source:string;network:LinkNetwork}[];
export function createFixedIntegerCastProgramLinks(): TypedProgramLinks;
export function createReferenceMethodProgramLinks(): TypedProgramLinks;
export function createRecordFieldWriteProgramLinks(): TypedProgramLinks;
export function createBigIntegerBitwiseProgramLinks(): TypedProgramLinks;
export function createRustPrimitiveBitwiseProgramLinks(): TypedProgramLinks;
export function createArrayStateMethodProgramLinks(): TypedProgramLinks;
export function createRustShiftProgramLinks(): TypedProgramLinks;
export function createRustRecordProgramLinks(): TypedProgramLinks;
export function createJavaScriptOptionalCallProgramLinks(): TypedProgramLinks;
export function createJavaScriptOptionalChainProgramLinks(): TypedProgramLinks;
export function createJavaScriptNullishFieldProgramLinks(): TypedProgramLinks;
export function createJavaScriptRecordOrderProgramLinks(): TypedProgramLinks;
export function acceptJavaScriptRecordOrderNames(names: number[][]): boolean;
export function acceptJavaScriptRecordOrderPermutation(count: number, order: number[]): boolean;
export function acceptJavaScriptRecordPropertyOrder(names: number[][], order: number[]): boolean;
export function createJavaScriptRecordPropertyOrder(names: number[][], insertion: number[]): number[];
export function mergeJavaScriptRecordPropertyOrder(names: number[][], chunks: number[][]): number[];
export function rebaseJavaScriptRecordPropertyOrder(target: number[][], source: number[][], order: number[]): number[];
export function readJavaScriptRecordPropertyKeys(names: number[][], order: number[]): number[][];
export function createJavaScriptOwnPropertyProgramLinks(): TypedProgramLinks;
export function createJavaScriptLiteralPropertyProgramLinks(): TypedProgramLinks;
export function createJavaScriptPropertyCallProgramLinks(): TypedProgramLinks;
export function createLinkAllocationProgramLinks(): TypedProgramLinks;
export function createRustInherentProgramLinks(): TypedProgramLinks;
export function createUnicodeScalarProgramLinks(): TypedProgramLinks;
export function createJavaScriptRecordParameterProgramLinks(): TypedProgramLinks;
export function createModuleBindingProgramLinks(): TypedProgramLinks;
export function createCallableSourceProgramLinks(): TypedProgramLinks;
export function createScalarAssignmentProgramLinks(): TypedProgramLinks;
export function createSourceIdentifierProgramLinks(): TypedProgramLinks;
export function createReferenceArrayProjectionProgramLinks(): TypedProgramLinks;
export function createJavaScriptIdentifierProgramLinks(): TypedProgramLinks;
export function createRegularGrammarInferenceProgramLinks(): TypedProgramLinks;
export interface RegularGrammarInferenceOptions {
  strategy?: 'rpni' | 'edsm' | 'alergia'; alpha?: number;
  stateLimit?: number; pairLimit?: number; attemptLimit?: number;
  symbolLimit?: number; byteLimit?: number;
  identifier?: string; language?: string; edition?: string;
}
export interface RegularGrammarInference extends TypedProgramLinks {
  grammar: Grammar; source: GlobalConceptSource; alphabet: string[]; table: number[];
}
export function inferRegularGrammar(positives: readonly (readonly string[])[], negatives?: readonly (readonly string[])[], options?: RegularGrammarInferenceOptions): RegularGrammarInference;
export function readRegularGrammarInferenceLinks(network: LinkNetwork, root: LinkId | number): RegularGrammarInference;
export function refineRegularGrammarInference(network:LinkNetwork,root:LinkIdValue,positives?:readonly (readonly string[])[],negatives?:readonly (readonly string[])[],options?:{identifier?:string}):RegularGrammarInference;
export function readRefinedRegularGrammarInferenceLinks(network:LinkNetwork,root:LinkIdValue):RegularGrammarInference;
export function acceptInferredGrammarSymbols(model: RegularGrammarInference, symbols: readonly string[]): boolean;
export function decodeTypedProgramLinks(
  network: LinkNetwork,
  root: LinkIdValue,
  options?: { nodeLimit?: number; depthLimit?: number },
): CheckedTypedProgram;
/** Emits from current links after scope and semantic validation, without source restoration. */
export function emitTypedProgramLinks(
  network: LinkNetwork,
  root: LinkIdValue,
  target: string,
  options?: TypedProgramLinksEmissionOptions,
): TypedProgramLinksEmission;

export const PROGRAM_REPRESENTATION_SCHEMA_VERSION: 1;
export const PROGRAM_SNAPSHOT_SCHEMA_VERSION: 1;
export const SEMANTIC_CONSTRUCTS: readonly string[];

export interface ProgramProjectSource {
  path: string;
  source: string;
}

export interface ProgramProjectContext {
  root?: string;
  files?: readonly string[];
  dependencies?: readonly string[];
  extensions?: readonly string[];
  /** The analyzed program's path within the project; enables project-aware semantics. */
  entry?: string;
  /** The project's files (manifests and modules) by project-relative path. */
  sources?: readonly ProgramProjectSource[];
}

export interface NormalizedProgramProjectContext {
  readonly root: string;
  readonly files: readonly string[];
  readonly dependencies: readonly string[];
  readonly extensions: readonly string[];
  readonly entry?: string;
  readonly sources?: readonly ProgramProjectSource[];
}

/** An entry module request resolved to a project file. */
export interface ProgramProjectModule extends ProgramSourceRange {
  readonly request: string;
  readonly module: string;
  readonly file: string;
}

/** A fact read from a project file: manifests, packages, dependencies, load paths. */
export interface ProgramProjectFact extends ProgramSourceRange {
  readonly kind: string;
  readonly name: string;
  readonly file: string;
}

/** An entry range linked to the declaration it names, identified as `<file>#<qualified name>`. */
export interface ProgramProjectReference extends ProgramSourceRange {
  readonly role: 'import' | 'reference' | 'attribute' | 'macro' | 'notation' | 'template-tag' | 'tactic' |
    'assertion' | 'const-assertion';
  readonly name: string;
  readonly symbol: string;
  readonly targetKind: string;
  readonly traits: readonly string[];
  readonly file: string;
  readonly declaration: ProgramSourceRange;
}

/** A macro, notation or tagged template use with the source it expands to. */
export interface ProgramExpansion extends ProgramSourceRange {
  readonly name: string;
  readonly kind: 'macro-rules' | 'notation' | 'tagged-template';
  readonly expansion: string;
  readonly target: string;
}

export interface ProgramSourceRange {
  readonly start: number;
  readonly end: number;
}

export interface ProgramSnapshotFragment {
  readonly byteStart: number;
  readonly byteEnd: number;
  readonly text: string;
}

export interface ProgramSnapshot {
  readonly schemaVersion: 1;
  readonly language: LanguageSupport['name'];
  readonly project: NormalizedProgramProjectContext;
  readonly fragments: readonly ProgramSnapshotFragment[];
}

export interface ProgramScope extends ProgramSourceRange {
  readonly id: string;
  readonly parent: string | null;
  readonly depth: number;
}

export interface ProgramBinding {
  readonly id: string;
  readonly name: string;
  readonly kind: string;
  readonly scope: string;
  readonly declaration: ProgramSourceRange;
  readonly references: readonly ProgramSourceRange[];
}

export interface ProgramConstruct {
  readonly kind: string;
  readonly status: 'represented' | 'not-present' | 'not-applicable' | 'unavailable';
  readonly evidence: readonly ({ term?: string; kind?: string; name?: string; file?: string } & ProgramSourceRange)[];
  readonly rationale?: string;
}

export class BindingRenameError extends Error {}
export class ProgramTransformationError extends Error {}

export class ProgramRepresentation {
  static fromSnapshot(snapshot: ProgramSnapshot | string): ProgramRepresentation;
  readonly schemaVersion: 1;
  readonly language: LanguageSupport['name'];
  readonly source: string;
  readonly project: NormalizedProgramProjectContext;
  readonly network: LinkNetwork;
  readonly scopes: readonly ProgramScope[];
  readonly bindings: readonly ProgramBinding[];
  readonly unresolvedReferences: readonly ({ name: string } & ProgramSourceRange)[];
  readonly sourceMappings: readonly ({ linkId: number; term: string } & ProgramSourceRange)[];
  readonly modules: readonly object[];
  readonly types: readonly object[];
  readonly extensions: readonly object[];
  readonly proofs: readonly object[];
  readonly diagnostics: readonly object[];
  readonly constructs: readonly ProgramConstruct[];
  readonly projectModules: readonly ProgramProjectModule[];
  readonly projectFacts: readonly ProgramProjectFact[];
  readonly projectReferences: readonly ProgramProjectReference[];
  readonly expansions: readonly ProgramExpansion[];
  emit(): string;
  snapshot(): ProgramSnapshot;
  serializeSnapshot(): string;
  querySyntax(term: string): readonly ProgramSourceRange[];
  query_syntax(term: string): readonly ProgramSourceRange[];
  replace(range: ProgramSourceRange, replacement: string): ProgramRepresentation;
  insert(offset: number, inserted: string): ProgramRepresentation;
  delete(range: ProgramSourceRange): ProgramRepresentation;
  clone(range: ProgramSourceRange, destination: number): ProgramRepresentation;
  move(range: ProgramSourceRange, destination: number): ProgramRepresentation;
  renameBinding(bindingId: string, replacement: string): ProgramRepresentation;
  rename_binding(bindingId: string, replacement: string): ProgramRepresentation;
  normalized(): object;
}

export function analyzeProgram(
  source: string,
  language: string,
  project?: ProgramProjectContext,
  options?: DecoratorOptions,
): ProgramRepresentation;
export const analyze_program: typeof analyzeProgram;
export function constructProgram(
  source: string,
  language: string,
  project?: ProgramProjectContext,
): ProgramRepresentation;
export function constructProgramFromFragments(
  fragments: Iterable<unknown>,
  language: string,
  project?: ProgramProjectContext,
): ProgramRepresentation;

/** A reference is its name; a link is its name or null followed by its values. */
export type LinksNotationReading = string | [string | null, ...LinksNotationReading[]];

export class LinkNetwork {
  constructor();
  static parse(text: string, language: string, configuration?: ParseConfiguration): LinkNetwork;
  static parseLinksNotation(
    text: string,
    configuration?: ParseConfiguration,
  ): { network: LinkNetwork; links: LinkId[] };
  static parseLosslessText(
    text: string,
    language: string,
    configuration?: ParseConfiguration,
  ): LinkNetwork;
  static parseWithRegistry(
    registry: ParserRegistry,
    text: string,
    language: string,
    configuration?: ParseConfiguration,
  ): LinkNetwork;
  static parse_with_registry(
    registry: ParserRegistry,
    text: string,
    language: string,
    configuration?: ParseConfiguration,
  ): LinkNetwork;
  static parseBytes(bytes: Uint8Array | ArrayLike<number>, format: string): LinkNetwork;
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
  insertSourceToken(
    language: string,
    text: string,
    span?: SourceSpan,
    flags?: LinkFlags,
  ): LinkId;
  insertSyntaxNode(
    language: string,
    term: string,
    children?: Array<LinkId | number>,
    metadata?: { named?: boolean; span?: SourceSpan; flags?: LinkFlags },
  ): LinkId;
  insertConceptExpression(concept: string, language: string, text: string): LinkId;
  /** Assigns a concept record (by record, identity or former name) and returns its concept link. */
  insertConceptRecord(nameOrRecord: string | ConceptRecord): LinkId;
  /** Assigns every concept record. */
  seedConceptRecords(): ConceptRecordSeedReport;
  /** Merges the concepts of `source`, renaming former identities and assigning concept records. */
  importConceptOntology(source: LinkNetwork): ConceptOntologyImportReport;
  linksNotationReading(id: LinkId): LinksNotationReading;
  linksNotationText(ids: LinkId[]): string;
  link(id: LinkId | number): Link | undefined;
  links(): Link[];
  /** Original links; omission selects concrete syntax. Resolve provenance through this complete network. */
  projectedLinks(projection?: NetworkProjectionValue): Link[];
  projected_links(projection?: NetworkProjectionValue): Link[];
  len(): number;
  queryLinks(query: LinkQuery): Link[];
  find(query: LinkQuery): QueryMatch[];
  replace(matches: QueryMatch[], rule: ReplacementRule, options?: DecoratorOptions): ReplacementReport;
  applySubstitution(rule: SubstitutionRule): SubstitutionReport;
  applyLinkCliSubstitutionText(source: string): SubstitutionReport;
  toLino(): string;
  snapshot(version: number, provenance: string): NetworkSnapshot;
  verifyFullMatch(): VerificationReport;
  reconstructText(): string;
  embeddedRegions(): EmbeddedRegion[];
  parseGrammars(): ParseGrammar[];
  embedded_regions(): EmbeddedRegion[];
  reconstructBytes(): Uint8Array;
  renderSource(language: string): string;
  reconstructTextAsWithRules(
    targetLanguage: string,
    configuration: ParseConfiguration,
    rules: TranslationRuleSet,
  ): string;
  intoFluent(): FluentPipeline;
  capturedText(id: LinkId | number): string;
}

export class Link {
  id(): LinkId;
  references(): LinkId[];
  metadata(): LinkMetadata;
}

export interface EmbeddedRegionLimits {
  depthLimit?: number;
  countLimit?: number;
  byteLimit?: number;
}

export class ParseConfiguration {
  constructor(options?: {
    triviaAttachmentPolicy?: string;
    regionDetectionPolicy?: RegionDetectionPolicyValue;
    accessMode?: string;
    embeddedRegionDepthLimit?: number;
    embeddedRegionCountLimit?: number;
    embeddedRegionByteLimit?: number;
  });
  readonly triviaAttachmentPolicy: string;
  readonly regionDetectionPolicy: RegionDetectionPolicyValue;
  readonly accessMode: string;
  /** Adapter traversal bounds, independent of each grammar parser's budgets. */
  readonly embeddedRegionDepthLimit: number;
  readonly embeddedRegionCountLimit: number;
  readonly embeddedRegionByteLimit: number;
  static default(): ParseConfiguration;
  withTriviaAttachmentPolicy(policy: string): ParseConfiguration;
  withRegionDetectionPolicy(policy: RegionDetectionPolicyValue): ParseConfiguration;
  with_region_detection_policy(policy: RegionDetectionPolicyValue): ParseConfiguration;
  withAccessMode(mode: string): ParseConfiguration;
  withEmbeddedRegionLimits(limits?: EmbeddedRegionLimits): ParseConfiguration;
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
  constructor(
    name: string,
    rules?: TranslationRule[],
    languageFallbacks?: Record<string, string[]>,
  );
  withRule(rule: TranslationRule): TranslationRuleSet;
  withLanguageFallback(language: string, fallback: string): TranslationRuleSet;
  with_language_fallback(language: string, fallback: string): TranslationRuleSet;
  render(targetLanguage: string, network: LinkNetwork, rootLinkId?: LinkIdValue, options?: DecoratorOptions): string;
  decorated(decorators: DecoratorInput): TranslationRuleSet;
  toLino(): string;
  toJson(): string;
  static fromLino(source: string): TranslationRuleSet;
  static fromJson(source: string | unknown): TranslationRuleSet;
}

export type GrammarRuleKind = 'normal' | 'atomic' | 'silent' | 'token' | 'terminal' | 'nonterminal';
export type GrammarExpression =
  | { kind: 'empty' | 'any' }
  | { kind: 'literal' | 'literalInsensitive' | 'regex'; value: string }
  | { kind: 'ref'; name: string; arguments?: GrammarExpression[] }
  | { kind: 'seq'; items: GrammarExpression[] }
  | { kind: 'choice'; items: GrammarExpression[]; ordered: boolean }
  | { kind: 'repeat0' | 'repeat1' | 'optional' | 'and' | 'not'; item: GrammarExpression }
  | { kind: 'repeat'; item: GrammarExpression; min: number; max: number | null }
  | { kind: 'capture'; label: string | null; item: GrammarExpression }
  | { kind: 'charRange'; start: string; end: string }
  | {
      kind: 'charClass';
      value?: string;
      items?: Array<
        | { kind: 'char'; value: string }
        | { kind: 'range'; start: string; end: string }
        | { kind: 'category' | 'script'; value: string }
      >;
      negated?: boolean;
    }
  | {
      kind: 'byteClass';
      negated: boolean;
      items: Array<{ kind: 'byte'; value: number } | { kind: 'byteRange'; start: number; end: number }>;
    }
  | { kind: 'precedence'; level: number; associativity: 'left' | 'right' | 'none'; item: GrammarExpression }
  | { kind: 'namedPrecedence'; name: string; associativity: 'left' | 'right' | 'none'; item: GrammarExpression }
  | { kind: 'dynamicPrecedence' | 'lexicalPrecedence'; level: number; item: GrammarExpression }
  | { kind: 'longest'; items: GrammarExpression[] }
  | { kind: 'token' | 'immediateToken' | 'missing'; item: GrammarExpression }
  | { kind: 'alias'; name: string; item: GrammarExpression }
  | { kind: 'parameter'; name: string }
  | { kind: 'predicate'; item: GrammarExpression; condition: GrammarOperation }
  | { kind: 'recover'; item: GrammarExpression; synchronize: GrammarExpression }
  | { kind: 'embed'; language: string; item: GrammarExpression }
  | { kind: 'any-grammar' | 'all-grammar'; item: GrammarExpression; items: Array<GrammarSelectionCandidate | GrammarRangeSelectionCandidate | GrammarFieldSelectionCandidate> }
  | GrammarSelectionCandidate
  | GrammarRangeSelectionCandidate
  | GrammarSourceRange
  | GrammarFieldSelectionCandidate
  | GrammarSourceFieldSelector
  | { kind: 'expand'; name: string; arguments: GrammarExpression[] };

export interface GrammarSelectionCandidate {
  kind: 'grammar-candidate';
  name: string;
  /** Unknown named mappings are retained as explicit diagnostics. */
  mapping: string;
}

/** An ordered nonempty range in UTF-8 bytes relative to the matched region. */
export interface GrammarSourceRange {
  kind: 'source-range';
  start: number;
  end: number;
}

/** One explicit source view shared by every any/all candidate. The selected
 * scalar-aligned ranges must omit at least one byte from the physical region.
 */
export interface GrammarRangeSelectionCandidate {
  kind: 'grammar-range-candidate';
  name: string;
  mapping: 'included-ranges';
  ranges: GrammarSourceRange[];
}

/** A finite window of topmost physical, non-trivia capture fields. Occurrence
 * indices are unsigned 32-bit integers; first is zero-based and count is positive.
 */
export interface GrammarSourceFieldSelector {
  kind: 'source-field-selector';
  label: string;
  mode: 'include' | 'exclude';
  first: number;
  count: number;
}

/** Every candidate in one selection must use the same physical field selector. */
export interface GrammarFieldSelectionCandidate {
  kind: 'grammar-field-candidate';
  name: string;
  mapping: 'field-ranges';
  selector: GrammarSourceFieldSelector;
}

/**
 * One operation of the scanner, action and predicate language of
 * docs/grammar/feature-union.md#operation-language: `operation` names the form
 * and the remaining fields are its operands.
 */
export interface GrammarOperation {
  operation: string;
  [field: string]: unknown;
}

export interface GrammarDeclarations {
  matching?: 'generalized' | 'peg';
  imports?: string[];
  modes?: string[];
  extras?: GrammarExpression[];
  conflicts?: string[][];
  precedences?: Array<Array<{ kind: 'name' | 'rule'; value: string }>>;
  macros?: Array<{ name: string; parameters: string[]; expression: GrammarExpression }>;
  scanners?: Array<{ name: string; tokens: string[]; operations: GrammarOperation[] }>;
}
export interface GrammarRuleValue {
  name: string;
  kind: GrammarRuleKind;
  expression: GrammarExpression;
  parameters?: string[];
  channel?: string;
  modes?: string[];
  action?: GrammarOperation[];
  /** The id of the concept record the rule means. */
  concept?: string;
  /** The names the rule has in the grammars it was merged from. */
  sourceNames?: GrammarSourceName[];
  doc?: string;
}
export interface GrammarSourceName {
  source: string;
  name: string;
}
export interface NormalizedGrammar {
  schemaVersion: 1;
  start: string | null;
  sourceFormat: string | null;
  rules: GrammarRuleValue[];
  declarations?: GrammarDeclarations;
}

export class Grammar {
  constructor(
    start: string | null,
    rules: Map<string, Omit<GrammarRuleValue, 'name'> | GrammarRuleValue>,
    sourceFormat?: string | null,
    declarations?: GrammarDeclarations | null,
  );
  start: string | null;
  sourceFormat: string | null;
  declarations: GrammarDeclarations;
  rules: Map<string, GrammarRuleValue>;
  rule(name: string): GrammarRuleValue | undefined;
  ruleNames(): string[];
  rule_names(): string[];
  startRule(): GrammarRuleValue | undefined;
  start_rule(): GrammarRuleValue | undefined;
  source_format(): string | null;
  referencedNonterminals(): string[];
  referenced_nonterminals(): string[];
  undefinedNonterminals(allowed?: string[]): string[];
  undefined_nonterminals(allowed?: string[]): string[];
  normalized(): NormalizedGrammar;
}

export class GrammarBuilder {
  constructor(start: string);
  source(format: string): GrammarBuilder;
  rule(name: string, expression: GrammarExpression, kind?: GrammarRuleKind): GrammarBuilder;
  terminal(name: string, expression: GrammarExpression): GrammarBuilder;
  nonterminal(name: string, expression: GrammarExpression): GrammarBuilder;
  build(): Grammar;
  static empty(): GrammarExpression;
  static literal(value: string): GrammarExpression;
  static literalInsensitive(value: string): GrammarExpression;
  static ref(name: string): GrammarExpression;
  static seq(...items: GrammarExpression[]): GrammarExpression;
  static choice(...items: GrammarExpression[]): GrammarExpression;
  static orderedChoice(...items: GrammarExpression[]): GrammarExpression;
  static repeat0(item: GrammarExpression): GrammarExpression;
  static repeat1(item: GrammarExpression): GrammarExpression;
  static repeat(item: GrammarExpression, min: number, max?: number | null): GrammarExpression;
  static optional(item: GrammarExpression): GrammarExpression;
  static and(item: GrammarExpression): GrammarExpression;
  static not(item: GrammarExpression): GrammarExpression;
  static capture(label: string | null, item: GrammarExpression): GrammarExpression;
  static charRange(start: string, end: string): GrammarExpression;
  static charClass(
    value: string | Array<
      { kind: 'char'; value: string } | { kind: 'range'; start: string; end: string }
    >,
    negated?: boolean,
  ): GrammarExpression;
  static regex(value: string): GrammarExpression;
  static any(): GrammarExpression;
}

export const ExprBuilder: typeof GrammarBuilder;
export function emitPeggy(grammar: Grammar): string;
export interface GrammarParserOptions extends DecoratorOptions {
  /** Returns the grammar an import or an embedded language names. */
  resolveGrammar?: (name: string) => Grammar | NormalizedGrammar | null | undefined;
  /** Source-qualified root identity; with a concept space retains a root boundary. */
  grammarName?: string;
  /** Global origin membership is retained only after checking the actual source. */
  globalConceptSpace?: GlobalConceptSpace;
  startRule?: string;
  maxDepth?: number;
  stepLimit?: number;
  memoLimit?: number;
  /** The memo cells a parse may keep, across its runs and repair rounds (default 2000000). */
  memoryLimit?: number;
  ambiguity?: 'report' | 'reject';
  recovery?: 'reject' | 'accept';
  /** Repairs a failed parse into a tree with ERROR and MISSING leaves. */
  errorRecovery?: boolean;
  /** The repair points automatic recovery may add (default 32). */
  maxRepairs?: number;
  [option: string]: unknown;
}

export type SyntaxTreeNode =
  | {
      type: 'node';
      kind: string;
      field?: string;
      /** An extra of a rule that builds a node. */
      trivia?: true;
      start: number;
      end: number;
      children: SyntaxTreeNode[];
      attributes?: Record<string, string | number>;
    }
  | {
      type: 'token';
      kind: string | null;
      field?: string;
      trivia?: true;
      start: number;
      end: number;
      attributes?: Record<string, string | number>;
      text: string | null;
      hex?: string;
      lexical?: Record<string, unknown>;
    }
  | { type: 'error'; start: number; end: number; text: string | null; hex?: string; reason?: string }
  | { type: 'missing'; kind: string | null; start: number; end: number; literal?: true }
  | { type: 'embed'; language: string; start: number; end: number; root: SyntaxTreeNode;
      /** Full physical host children for an explicitly mapped selection. */
      sourceChildren?: SyntaxTreeNode[]; selection?: GrammarSelection };

/** Physical spelling only. Mapped embeds retain their source children and omit
 * decoded roots and selection metadata. Identity embed roots remain opaque to
 * field selection while their physical spelling is retained in the witness.
 */
export type GrammarPhysicalWitnessTree =
  | (Omit<Extract<SyntaxTreeNode, { type: 'node' }>, 'children'> & { children: GrammarPhysicalWitnessTree[] })
  | Extract<SyntaxTreeNode, { type: 'token' | 'error' | 'missing' }>
  | { type: 'embed'; language: string; start: number; end: number;
      sourceChildren: GrammarPhysicalWitnessTree[]; root?: never; selection?: never }
  | { type: 'embed'; language: string; start: number; end: number;
      root: GrammarPhysicalWitnessTree; sourceChildren?: never; selection?: never };

export interface GrammarFieldRangeWitness {
  start: number;
  end: number;
  children: GrammarPhysicalWitnessTree[];
}

export interface GrammarFieldRangeOccurrence {
  /** Alternating child-role strings and zero-based child indices, beginning
   * with 'children'. Identity embed roots are never traversed for occurrences.
   */
  path: Array<'children' | 'sourceChildren' | number>;
  start: number;
  end: number;
}

export interface GrammarFieldRangeSourceDerivation {
  operation: 'field-ranges';
  input: number;
  start: number;
  end: number;
  selector: GrammarSourceFieldSelector;
  occurrences: GrammarFieldRangeOccurrence[];
  ranges: Array<[number, number]>;
  witness: GrammarFieldRangeWitness;
}

export type GrammarSourcePlaneDerivation =
  | { operation: 'json-string'; input: number; start: number; end: number }
  | { operation: 'included-ranges'; input: number; start: number; end: number; ranges: Array<[number, number]> }
  | GrammarFieldRangeSourceDerivation;

/** An ordered once-owned source definition. identity equals its zero-based
 * table position; these local references are not LinkNetwork identifiers.
 */
export interface GrammarSourcePlane {
  kind: 'grammar-source';
  identity: number;
  encoding: 'UTF-8';
  byteLength: number;
  derivation: null | GrammarSourcePlaneDerivation;
}

export interface GrammarSourceMapping {
  kind: 'source-mapping';
  operation: 'json-string';
  input: number;
  output: number;
  /** Ordered [physicalStart, physicalEnd, decodedStart, decodedEnd] byte intervals. */
  intervals: Array<[number, number, number, number]>;
}

export interface GrammarIncludedRangeSourceMapping {
  kind: 'source-mapping';
  operation: 'included-ranges';
  input: number;
  output: number;
  ranges: Array<[number, number]>;
  /** Scalar copies, excluded gaps and boundary points in their original order. */
  intervals: Array<[number, number, number, number]>;
}

export interface GrammarFieldRangeSourceMapping {
  kind: 'source-mapping';
  operation: 'field-ranges';
  input: number;
  output: number;
  selector: GrammarSourceFieldSelector;
  occurrences: GrammarFieldRangeOccurrence[];
  ranges: Array<[number, number]>;
  /** Scalar copies, excluded gaps and boundary points in physical source order. */
  intervals: Array<[number, number, number, number]>;
}

export type GrammarSelectionSourceMapping =
  | GrammarSourceMapping
  | GrammarIncludedRangeSourceMapping
  | GrammarFieldRangeSourceMapping;

/** A covering source spelling, not an exact byte correspondence inside a scalar. */
export interface GrammarSourceRoute {
  output: number;
  outputStart: number;
  outputEnd: number;
  input: number;
  inputStart: number;
  inputEnd: number;
  displayAnchor: number;
}

/** Complete physical root plus the actual source definitions needed to resolve
 * mapped child references. A failed result with tree:null is not this bundle.
 */
export interface GrammarMappedTreeBundle {
  tree: SyntaxTreeNode;
  sourcePlanes: GrammarSourcePlane[];
}

export interface GrammarSelectionProvenance {
  source: string;
  language: string;
  edition: string;
  rules: Array<{ rule: string; origin: string; concept: string | null;
    declaredConcept: string | null; sourceNames: GrammarSourceName[] }>;
}
export interface GrammarSelectionAlternative {
  language: string;
  root: SyntaxTreeNode;
  provenance: GrammarSelectionProvenance;
}
export interface GrammarSelection {
  mode: 'any-grammar' | 'all-grammar' | 'grammar-root';
  mapping: string;
  start: number;
  end: number;
  candidates: Array<{ language: string; relation: 'accepted' | 'rejected' | 'unknown';
    reason?: string; mapping?: string; offset?: number; expected?: string[];
    source?: number; sourceOffset?: number; physicalRange?: [number, number];
    offsetSemantics?: 'covering-source-spelling' }>;
  /** Present only when the explicit mapping owns a separately decoded source. */
  sourceMapping?: GrammarSelectionSourceMapping;
  provenance: GrammarSelectionProvenance;
  alternatives: GrammarSelectionAlternative[];
  ambiguous: boolean;
}

export interface GrammarRejection {
  reason: 'syntax' | 'stepLimit' | 'memoryBudget' | 'nestingDepth' | 'recovered' | 'ambiguity'
    | 'unknownGrammar' | 'unsupportedEmbeddingMapping' | 'invalidEmbeddingMapping'
    | 'nonConsumingEmbeddingCycle' | 'unprovedGrammarOrigin';
  offset?: number;
  line?: number;
  column?: number;
  expected?: string[];
  limit?: number;
  offsetSemantics?: 'covering-source-spelling';
  diagnostics?: Array<{ reason: string; language?: string; start: number; end: number;
    mapping?: string; rule?: string; expected?: string[]; source?: number;
    sourceStart?: number; sourceEnd?: number; sourceRoute?: GrammarSourceRoute[];
    offsetSemantics?: 'covering-source-spelling' }>;
}

export interface GrammarParseOutcome {
  ok: boolean;
  tree: SyntaxTreeNode | null;
  ambiguities: Array<{ rule: string; start: number; end: number; languages?: string[];
    source?: number; sourceStart?: number; sourceEnd?: number; sourceRoute?: GrammarSourceRoute[];
    offsetSemantics?: 'covering-source-spelling' }>;
  rejection: GrammarRejection | null;
  /** Present only after entering an explicit mapped-source profile. Rows may
   * retain validated transforms attempted by a child grammar that rejected.
   */
  sourcePlanes?: GrammarSourcePlane[];
}

export interface GrammarParser {
  /** A mapped success requires parseTree so its source-plane registry is retained. */
  parse(source: string | Uint8Array, options?: GrammarParserOptions): SyntaxTreeNode;
  parseTree(source: string | Uint8Array, options?: GrammarParserOptions): GrammarParseOutcome;
}

export class GrammarParseError extends Error {
  constructor(rejection: GrammarRejection);
  rejection: GrammarRejection;
  reason: GrammarRejection['reason'];
  offset: number | null;
  line: number | null;
  column: number | null;
  expected: string[];
}

export class GrammarRuntimeError extends Error {
  reason: 'import' | 'embed' | 'macro' | 'parameter' | 'reference' | 'declaration' | 'operation' | 'pattern' | 'selection';
}

export function createGrammarParser(
  grammar: Grammar | NormalizedGrammar,
  options?: GrammarParserOptions,
): GrammarParser;
export function renderSyntaxTree(node: SyntaxTreeNode): string;
/** Validate mapping correspondence in ordinary plain data after generic Links
 * decode. Returns the same bundle. This does not prove grammar membership;
 * complete CSTs still require parsing with their actual grammar definitions.
 */
export function checkGrammarSelectionMappedTree<T>(bundle: T,
  options?: Pick<GrammarParserOptions, 'memoryLimit' | 'stepLimit' | 'maxDepth' | 'errorRecovery'>,
): T & GrammarMappedTreeBundle;
export function compileGrammar(
  grammar: Grammar | NormalizedGrammar,
  options?: GrammarParserOptions,
): GrammarParser;
export function parseWithGrammar(
  grammar: Grammar | NormalizedGrammar,
  source: string | Uint8Array,
  options?: GrammarParserOptions,
): SyntaxTreeNode;
export function emitJavascriptParser(grammar: Grammar): string;
export function serializeGrammar(grammar: Grammar): string;
export function deserializeGrammar(source: string | NormalizedGrammar): Grammar;

export class GrammarImportError extends Error {
  format: string;
  kind: 'parse' | 'unsupported';
  construct?: string;
}
export function importAbnf(source: string, options?: DecoratorOptions): Grammar;
export const import_abnf: typeof importAbnf;
export function importAntlr(source: string, options?: DecoratorOptions): Grammar;
export const import_antlr: typeof importAntlr;
export function importBnf(source: string, options?: DecoratorOptions): Grammar;
export const import_bnf: typeof importBnf;
export function importEbnf(source: string, options?: DecoratorOptions): Grammar;
export const import_ebnf: typeof importEbnf;
export function importGbnf(source: string, options?: DecoratorOptions): Grammar;
export const import_gbnf: typeof importGbnf;
export function importLark(source: string, options?: DecoratorOptions): Grammar;
export const import_lark: typeof importLark;
export function importPest(source: string, options?: DecoratorOptions): Grammar;
export const import_pest: typeof importPest;
export function importTreeSitterJson(source: string | unknown, options?: DecoratorOptions): Grammar;
export const import_tree_sitter_json: typeof importTreeSitterJson;

export class GrammarEmitError extends Error {
  format: string;
  kind: 'unsupported';
  construct: string;
}
export interface GrammarEmitReport {
  lossy: string[];
}
export interface GrammarEmitResult {
  source: string;
  report: GrammarEmitReport;
}
export function emitAbnf(grammar: Grammar, options?: DecoratorOptions): GrammarEmitResult;
export const emit_abnf: typeof emitAbnf;
export function emitAntlr(grammar: Grammar, options?: DecoratorOptions): GrammarEmitResult;
export const emit_antlr: typeof emitAntlr;
export function emitBnf(grammar: Grammar, options?: DecoratorOptions): GrammarEmitResult;
export const emit_bnf: typeof emitBnf;
export function emitEbnf(grammar: Grammar, options?: DecoratorOptions): GrammarEmitResult;
export const emit_ebnf: typeof emitEbnf;
export function emitGbnf(grammar: Grammar, options?: DecoratorOptions): GrammarEmitResult;
export const emit_gbnf: typeof emitGbnf;
export function emitLark(grammar: Grammar, options?: DecoratorOptions): GrammarEmitResult;
export const emit_lark: typeof emitLark;
export function emitPest(grammar: Grammar, options?: DecoratorOptions): GrammarEmitResult;
export const emit_pest: typeof emitPest;
export function emitTreeSitterJson(grammar: Grammar, options?: DecoratorOptions): GrammarEmitResult;
export const emit_tree_sitter_json: typeof emitTreeSitterJson;

export const GRAMMAR_MERGE_METHOD: 'recursive-structural-bisimulation';
/** Every imported production has a qualified origin before meaning is shared. */
export interface GlobalConceptSource {
  id: string;
  language: string;
  edition?: string;
  grammar: Grammar;
}
export interface GlobalConceptOrigin {
  alias: string;
  source: string;
  language: string;
  edition: string;
  rule: string;
  concept: string | null;
  sourceNames: Array<{ source: string; name: string }>;
}
export interface GlobalConceptProof {
  method: 'recursive-structural-bijection';
  first: string;
  second: string;
  correspondence: Array<[string, string]>;
}
export interface GlobalConceptSpace {
  schemaVersion: 1;
  sources: Array<{ id: string; language: string; edition: string; start: string | null;
    ruleOrder: string[]; context: GrammarDeclarations; sourceFormat?: string }>;
  concepts: Array<{ id: string; origins: GlobalConceptOrigin[]; definition: Omit<GrammarRuleValue, 'name'>;
    definitions: Array<{ origin: string; definition: Omit<GrammarRuleValue, 'name'> }>; proofs: number[] }>;
  mappings: Record<string, string>;
  proofs: GlobalConceptProof[];
  failures: Array<{ members: [string, string]; reason: 'unknown-origin' | 'no-one-to-one-proof' }>;
  ambiguities: Array<{ alias: string; meanings: string[] }>;
  redirects: Record<string, string>;
  formalDefinitions: Record<string, PortableFormalConceptArchive>;
}
export interface GlobalConceptOptions {
  previous?: GlobalConceptSpace | null;
  equivalences?: Array<[string, string]>;
  records?: readonly (ConceptRecord & {
    readonly formalDefinition?: FormalConceptArchive | PortableFormalConceptArchive;
  })[];
  formalDefinitions?: Readonly<Record<string, FormalConceptArchive | PortableFormalConceptArchive>>;
}
/** Explicit discovery outputs and copied formal archives in one inspectable graph. */
export interface DiscoveredConceptLinks {
  network: LinkNetwork;
  root: LinkId;
  space: GlobalConceptSpace;
}
export interface DiscoveredConceptGraph {
  sources: GlobalConceptSource[];
  space: GlobalConceptSpace;
  formalDefinitions: Record<string, FormalConceptArchive>;
}
export function createDiscoveredConceptLinks(
  sources: readonly GlobalConceptSource[], options?: GlobalConceptOptions,
): DiscoveredConceptLinks;
export function readDiscoveredConceptLinks(network: LinkNetwork, root: LinkIdValue): DiscoveredConceptGraph;

export interface ConceptViews {
  network: LinkNetwork;
  root: LinkId;
  ontology: LinkId;
  taxonomy: LinkId;
}
export interface ConceptViewDeclaration<Kind extends string> {
  members?: readonly string[];
  roots?: readonly { identity: string; typeModel?: FormalConceptArchive | PortableFormalConceptArchive }[];
  relations?: readonly { source: string; target: string; kind: Kind; model: FormalConceptArchive | PortableFormalConceptArchive }[];
}
export interface ConceptViewOptions {
  orphans?: readonly { identity: string; definition: FormalConceptArchive | PortableFormalConceptArchive; provenance?: unknown }[];
  ontology?: ConceptViewDeclaration<'association' | 'axiom'>;
  taxonomy?: ConceptViewDeclaration<'classification' | 'subsumption'>;
}
export interface ConceptViewResult {
  view: number;
  concepts: Array<{ identity: string; link: number; definitionRoots: number[]; provenanceRoots: number[]; provenance: unknown }>;
  roots: Array<{ link: number; concept: number; typeModel: number | null }>;
  relations: Array<{ link: number; source: number; target: number; kind: string; model: number }>;
  selectedRoots: number[];
  reachable: number[];
}
export function createConceptViews(base?: DiscoveredConceptLinks | null, options?: ConceptViewOptions): ConceptViews;
export function readConceptOntology(network: LinkNetwork, root: LinkIdValue, options?: { selectedRoot?: LinkIdValue }): ConceptViewResult;
export function readConceptTaxonomy(network: LinkNetwork, root: LinkIdValue, options?: { selectedRoot?: LinkIdValue }): ConceptViewResult;

export interface BinaryLinksOptions { recordLimit?: number; byteLimit?: number }
export class BinaryLinksError extends TypeError { constructor(message: string) }
export function encodeLinkNetworkBinary(network: LinkNetwork, options?: BinaryLinksOptions): Uint8Array;
export function decodeLinkNetworkBinary(bytes: Uint8Array | readonly number[], options?: BinaryLinksOptions): LinkNetwork;

/** Explicit complete positive u64 profile. Number inputs must already be safe;
 * JSON uses exact decimal text. The existing LinkId profile is unchanged. */
export type UnsignedLinkIdValue = UnsignedLinkId | LinkIdValue | bigint;
export class UnsignedLinkId {
  constructor(value: UnsignedLinkIdValue);
  static from(value: UnsignedLinkIdValue): UnsignedLinkId;
  static fromU64(value: bigint | number | string): UnsignedLinkId;
  words(): number[];
  asU64(): bigint;
  equals(other: UnsignedLinkIdValue): boolean;
  compare(other: UnsignedLinkIdValue): number;
  toString(): string;
  toJSON(): string;
  valueOf(): bigint;
}
export class UnsignedLink {
  constructor(id: UnsignedLinkIdValue, references?: UnsignedLinkIdValue[], metadata?: LinkMetadata);
  id(): UnsignedLinkId;
  references(): UnsignedLinkId[];
  metadata(): LinkMetadata;
  setReferences(references: UnsignedLinkIdValue[]): void;
  setMetadata(metadata: LinkMetadata): void;
  clone(): UnsignedLink;
}
export interface UnsignedSourceDocumentOptions extends BinaryLinksOptions {
  firstIdentity?: UnsignedLinkIdValue;
}
export interface RebasedUnsignedLinkNetwork {
  network: UnsignedLinkNetwork;
  sourceIdentities: UnsignedLinkId[];
  identities: UnsignedLinkId[];
}
export class UnsignedLinkNetwork {
  static parse(text: string, language: string, configuration?: ParseConfiguration, options?: UnsignedSourceDocumentOptions): UnsignedLinkNetwork;
  static parseWithRegistry(registry: ParserRegistry, text: string, language: string, configuration?: ParseConfiguration, options?: UnsignedSourceDocumentOptions): UnsignedLinkNetwork;
  reconstructText(options?: BinaryLinksOptions): string;
  reconstructBytes(options?: BinaryLinksOptions): Uint8Array;
  projectedLinks(projection?: NetworkProjectionValue): UnsignedLink[];
  projected_links(projection?: NetworkProjectionValue): UnsignedLink[];
  readonly nextIdentity: UnsignedLinkId | null;
  len(): number;
  links(): UnsignedLink[];
  link(id: UnsignedLinkIdValue): UnsignedLink | undefined;
  findTerm(term: string): UnsignedLinkId | undefined;
  insertLink(references?: UnsignedLinkIdValue[], metadata?: LinkMetadata): UnsignedLinkId;
  insertLinkWithOptionalId(preferred?: UnsignedLinkIdValue, references?: UnsignedLinkIdValue[], metadata?: LinkMetadata): UnsignedLinkId;
  insertTypedPoint(type: LinkTypeValue, term: string, definition?: string): UnsignedLinkId;
  insertPoint(term: string): UnsignedLinkId;
  insertObject(term: string): UnsignedLinkId;
  insertField(term: string): UnsignedLinkId;
  insertFieldRelation(parent: UnsignedLinkIdValue, label: string, child: UnsignedLinkIdValue): UnsignedLinkId;
  deleteLink(id: UnsignedLinkIdValue): void;
  clone(): UnsignedLinkNetwork;
  toLino(): string;
  static fromLino(text: string): UnsignedLinkNetwork;
}
export interface DenseUnsignedLinkNetwork {
  network: LinkNetwork;
  directory: UnsignedLinkId[];
  nextIdentity: UnsignedLinkId | null;
}
export function rebaseUnsignedLinkNetwork(source: LinkNetwork | UnsignedLinkNetwork, firstIdentity?: UnsignedLinkIdValue, options?: BinaryLinksOptions): RebasedUnsignedLinkNetwork;
export function createUnsignedSourceDocumentLinks(text: string, language: string, configuration?: ParseConfiguration, options?: UnsignedSourceDocumentOptions): RebasedUnsignedLinkNetwork;
export function createUnsignedSourceDocumentLinksWithRegistry(registry: ParserRegistry, text: string, language: string, configuration?: ParseConfiguration, options?: UnsignedSourceDocumentOptions): RebasedUnsignedLinkNetwork;
export function createDenseUnsignedLinkNetwork(source: UnsignedLinkNetwork, options?: BinaryLinksOptions): DenseUnsignedLinkNetwork;
export function restoreDenseUnsignedLinkNetwork(dense: LinkNetwork, directory: UnsignedLinkIdValue[], nextIdentity: UnsignedLinkIdValue | null, options?: BinaryLinksOptions): UnsignedLinkNetwork;
export function encodeUnsignedLinkNetworkBinary(source: UnsignedLinkNetwork, options?: BinaryLinksOptions): Uint8Array;
export function decodeUnsignedLinkNetworkBinary(bytes: Uint8Array | readonly number[], options?: BinaryLinksOptions): UnsignedLinkNetwork;
export interface RebasedUnsignedTypedProgramLinks extends RebasedUnsignedLinkNetwork {
  root: UnsignedLinkId;
}
export function rebaseUnsignedTypedProgramLinks(packet: { network: LinkNetwork | UnsignedLinkNetwork; root: UnsignedLinkIdValue }, firstIdentity?: UnsignedLinkIdValue, options?: BinaryLinksOptions): RebasedUnsignedTypedProgramLinks;
export function createUnsignedTypedProgramLinks(source: string, language: string, sourceOptions?: Parameters<typeof createTypedProgramLinks>[2], identityOptions?: UnsignedSourceDocumentOptions): RebasedUnsignedTypedProgramLinks;
export function createUnsignedRustSourceOwnerProgramLinks(source: string, sourceOptions?: {sourceVersion?: string}, identityOptions?: UnsignedSourceDocumentOptions): RebasedUnsignedTypedProgramLinks;
export function emitUnsignedTypedProgramLinks(network: UnsignedLinkNetwork, root: UnsignedLinkIdValue, target: string, emissionOptions?: TypedProgramLinksEmissionOptions, identityOptions?: BinaryLinksOptions): TypedProgramLinksEmission;
export function createUnsignedLinkIdentityProgramLinks(): TypedProgramLinks;

export type MetaLanguageArchiveKind = 'concrete syntax' | 'abstract syntax' | 'grammar'
  | 'language configuration'
  | 'global concept space' | 'formal concept definition' | 'ontology' | 'taxonomy'
  | 'resource description graph' | 'resource description dataset' | 'typed program'
  | 'discovered concepts' | 'decorators' | 'grammar inference';
export interface MetaLanguageArchiveEntry {
  identifier: string;
  kind: MetaLanguageArchiveKind;
  network: LinkNetwork;
  root: LinkIdValue;
}
export interface MetaLanguageArchiveLimits {
  nodeLimit?: number;
  referenceLimit?: number;
  byteLimit?: number;
}
export interface MetaLanguageArchive { network: LinkNetwork; root: LinkId; }
export interface ReconstructedMetaLanguageArchive {
  entries: MetaLanguageArchiveEntry[];
  networks: LinkNetwork[];
}
export interface LoadedMetaLanguageArchive extends MetaLanguageArchive, ReconstructedMetaLanguageArchive {}
export function createMetaLanguageArchive(entries: MetaLanguageArchiveEntry[], options?: MetaLanguageArchiveLimits): MetaLanguageArchive;
export function readMetaLanguageArchive(network: LinkNetwork, root: LinkIdValue, options?: MetaLanguageArchiveLimits): ReconstructedMetaLanguageArchive;
export function metaLanguageArchiveToText(archive: MetaLanguageArchive, options?: MetaLanguageArchiveLimits): string;
export function readMetaLanguageArchiveNetworkText(text: string, options?: MetaLanguageArchiveLimits): LinkNetwork;
export function readMetaLanguageArchiveText(text: string, options?: MetaLanguageArchiveLimits): LoadedMetaLanguageArchive;
export function metaLanguageArchiveToBinary(archive: MetaLanguageArchive, options?: MetaLanguageArchiveLimits): Uint8Array;
export function readMetaLanguageArchiveBinary(bytes: Uint8Array | readonly number[], options?: MetaLanguageArchiveLimits): LoadedMetaLanguageArchive;
export function extendMetaLanguageArchive(archive: MetaLanguageArchive, additions: MetaLanguageArchiveEntry[], options?: MetaLanguageArchiveLimits & { replace?: boolean; remove?: string[] }): MetaLanguageArchive;

export interface DynamicMetaLanguageOptions extends MetaLanguageArchiveLimits {
  depthLimit?: number;
  records?: ConceptRecord[];
  formalDefinitions?: Record<string, FormalConceptArchive>;
  equivalences?: [string, string][];
  entries?: MetaLanguageArchiveEntry[];
}
export class DynamicMetaLanguage {
  constructor(archive: MetaLanguageArchive, options?: MetaLanguageArchiveLimits);
  readonly archive: MetaLanguageArchive;
  readonly entries: MetaLanguageArchiveEntry[];
  readonly sources: GlobalConceptSource[];
  readonly space: GlobalConceptSpace;
  readonly formalDefinitions: Record<string, FormalConceptArchive>;
  readonly configuration: { schemaVersion: 1; records: ConceptRecord[]; equivalences: [string, string][] };
  readonly limits: Required<MetaLanguageArchiveLimits> & { depthLimit: number };
  parser(sourceIdentifier: string, options?: Record<string, unknown>): {
    parse(input: string, overrides?: Record<string, unknown>): unknown;
    parseTree(input: string, overrides?: Record<string, unknown>): unknown;
  };
  parse(sourceIdentifier: string, input: string, options?: Record<string, unknown>): unknown;
  extend(additions: GlobalConceptSource[], options?: DynamicMetaLanguageOptions & { replace?: boolean }): DynamicMetaLanguage;
}
export function createDynamicMetaLanguage(sources: GlobalConceptSource[], options?: DynamicMetaLanguageOptions): DynamicMetaLanguage;
export interface DynamicLanguageCommandOutput { exitCode: number; stdout: string | Uint8Array; stderr: string }
export function runDynamicLanguageCommand(args: string[], host: {
  readFile(path: string): string;
  readBytes(path: string): Uint8Array;
}): DynamicLanguageCommandOutput;
export function createDynamicLanguageCommandProgramLinks(): TypedProgramLinks;
export function createRustByteStringProgramLinks(): TypedProgramLinks;
/** Actual binding name and required owner (0=intrinsics,1=Map,2=Set,3=Object). */
export function readIntrinsicModuleInitializerBinding(source: string): [string, number] | null;
export function acceptIntrinsicModuleInitializerScope(forms: string[], owners: number[], bindings: string[]): boolean;
export function createJavaScriptRecordClosureParameterProgramLinks(): TypedProgramLinks;

export interface SyntaxProjectionLimits extends MetaLanguageArchiveLimits { depthLimit?: number }
export interface AbstractSyntaxOptions extends SyntaxProjectionLimits {
  treePath?: string[];
  decorators?: DecoratorInput;
}
export interface AbstractSyntaxLinks { network: LinkNetwork; root: LinkId; concreteRoot: LinkId }
export interface AbstractSyntaxNode {
  id: LinkId;
  origin: LinkId;
  properties: Record<string, string>;
  children: Array<{ role: string; container: LinkId; index: number; node: AbstractSyntaxNode }>;
}
export interface ReadAbstractSyntaxLinks {
  concreteRoot: LinkId;
  tree: AbstractSyntaxNode | null;
  settings: { schemaVersion: 1; treePath: string[]; decorators: Decorator[] };
}
export function createAbstractSyntaxLinks(network: LinkNetwork | StructuredDataLinksArchive['network'],
  root: LinkIdValue, options?: AbstractSyntaxOptions): AbstractSyntaxLinks;
export function readAbstractSyntaxLinks(network: LinkNetwork, root: LinkIdValue,
  options?: SyntaxProjectionLimits): ReadAbstractSyntaxLinks;

export interface ProjectTranslationSource {
  identifier: string;
  language: string;
  source: string;
  imports?: Record<string, string>;
  options?: SelfTranslationOptions;
}
export interface ProjectTranslationDraftOptions {
  targetLanguage: string;
  translationOptions?: SelfTranslationOptions;
  externalImports?: Record<string, Record<string, SelfTranslationSignature[]>>;
  moduleLimit?: number;
  byteLimit?: number;
  refinementLimit?: number;
}
export function draftProjectTranslation(inputs: ProjectTranslationSource[], options: ProjectTranslationDraftOptions): StructuredDataValue;
export function projectTranslationDraftToLinks(draft: StructuredDataValue, options?: StructuredDataLinksLimits): StructuredDataLinksArchive;

/** Structured RDF 1.1 terms; syntax import and OWL entailment are separate operations. */
export type ResourceDescriptionTerm =
  | { kind: 'identifier'; value: string }
  | { kind: 'blank'; value: string; sourceIdentifier?: string }
  | { kind: 'literal'; value: string; datatype?: string; language?: string };
export type ResourceDescriptionStatement = [ResourceDescriptionTerm, ResourceDescriptionTerm, ResourceDescriptionTerm];
/** Decoded terms always carry explicit blank scopes and normalized literal metadata. */
export type DecodedResourceDescriptionTerm =
  | { kind: 'identifier'; value: string }
  | { kind: 'blank'; value: string; sourceIdentifier: string }
  | { kind: 'literal'; value: string; datatype: string; language: string };
export type DecodedResourceDescriptionStatement = [
  Exclude<DecodedResourceDescriptionTerm, { kind: 'literal' }>,
  Extract<DecodedResourceDescriptionTerm, { kind: 'identifier' }>,
  DecodedResourceDescriptionTerm,
];
export interface ResourceDescriptionLinks { network: LinkNetwork; root: LinkId }
export interface ResourceDescriptionGraph {
  model: string;
  triples: DecodedResourceDescriptionStatement[];
  occurrences: Array<{ statementIndex: number; sourceIdentifier: string; position: number; sourceLanguage: string }>;
}
export function importResourceDescriptionGraph(
  triples: readonly Readonly<ResourceDescriptionStatement>[],
  options: { sourceIdentifier: string; network?: LinkNetwork },
): ResourceDescriptionLinks;
export function decodeResourceDescriptionGraph(
  network: LinkNetwork, root: LinkIdValue, options?: { nodeLimit?: number },
): ResourceDescriptionGraph;
export function mergeResourceDescriptionGraphs(inputs: readonly ResourceDescriptionLinks[]): ResourceDescriptionLinks;
export function verifyResourceDescriptionGraphWitnesses(network: LinkNetwork, root: LinkIdValue): boolean;

export interface ResourceDescriptionNTriplesLinks extends ResourceDescriptionLinks { syntaxRoot: LinkId }
export interface ResourceDescriptionNTriplesOutput { source: string; correspondence: ResourceDescriptionLinks }
export class ResourceDescriptionNTriplesError extends TypeError {
  constructor(kind: string, offset: number, message: string);
  kind: string;
  offset: number;
}
export function importResourceDescriptionNTriples(source: string, options: {
  sourceIdentifier: string; network?: LinkNetwork; correspondence?: ResourceDescriptionLinks;
}): ResourceDescriptionNTriplesLinks;
export function emitResourceDescriptionNTriples(network: LinkNetwork, root: LinkIdValue): ResourceDescriptionNTriplesOutput;
export function readResourceDescriptionNTriplesSource(network: LinkNetwork, syntaxRoot: LinkIdValue): string;

export interface FormalConceptArchive {
  readonly network: LinkNetwork;
  readonly root: LinkId;
}
export interface PortableFormalConceptArchive {
  readonly links: string;
  readonly root: number;
}
export interface FormalConceptDefinition {
  readonly schemaVersion: 1;
  readonly model: string;
  readonly shapes: readonly string[];
  readonly offsets: readonly number[];
  readonly edges: readonly number[];
}
/** Copies a checked operational definition and its declared supported model into links. */
export function encodeFormalConceptDefinition(
  value?: CheckedTypedProgram | TypedProgramLinks | object,
  options?: { model?: string; provenance?: unknown },
): FormalConceptArchive;
export function readFormalConceptDefinition(
  value: FormalConceptArchive | PortableFormalConceptArchive,
): FormalConceptDefinition;
/** Extracts the selected checked function and its actual closed recursive dependency links. */
export type FormalFunctionSelector = string | number | LinkId;
export function encodeFormalFunctionDefinition(
  value: TypedProgramLinks | PortableFormalConceptArchive,
  declaration: FormalFunctionSelector,
  options?: { provenance?: unknown },
): FormalConceptArchive;
/** Selects an actual owned Boolean declaration while preserving its original source proof. */
export function encodeFormalFunctionDeclarationValue(
  value: TypedProgramLinks | PortableFormalConceptArchive,
  declaration: FormalFunctionSelector,
  options?: { provenance?: unknown },
): FormalConceptArchive;
/** Selects a closed Number/Boolean function under an explicit shared value model. */
export function encodeSharedNumericFunctionDefinition(
  value: TypedProgramLinks | PortableFormalConceptArchive,
  declaration: FormalFunctionSelector,
  options?: { provenance?: unknown },
): FormalConceptArchive;
/** Returns a closed correspondence under the declared model, or null if none is proved. */
export function proveFormalConceptEquivalence(
  first: FormalConceptArchive | PortableFormalConceptArchive,
  second: FormalConceptArchive | PortableFormalConceptArchive,
): FormalConceptArchive | null;
export function verifyFormalConceptEquivalence(
  first: FormalConceptArchive | PortableFormalConceptArchive,
  second: FormalConceptArchive | PortableFormalConceptArchive,
  certificate: FormalConceptArchive | PortableFormalConceptArchive,
): boolean;
export const GLOBAL_CONCEPT_PROOF_METHOD: 'recursive-structural-bijection';
export function createGlobalConceptSpace(sources: readonly GlobalConceptSource[], options?: GlobalConceptOptions): GlobalConceptSpace;
export function globalConceptSpaceToLinks(space: GlobalConceptSpace): string;
export function readGlobalConceptSpaceLinks(source: string, options?: GlobalConceptOptions): GlobalConceptSpace;
/** Archive-owned identities and resolution cannot be replaced by parser options. */
export type GlobalConceptSpaceParserOptions = GrammarParserOptions & {
  resolveGrammar?: never;
  grammarName?: never;
  globalConceptSpace?: never;
};
export interface GlobalConceptSpaceParser {
  parse(source: string | Uint8Array, options?: GlobalConceptSpaceParserOptions): SyntaxTreeNode;
  parseTree(source: string | Uint8Array, options?: GlobalConceptSpaceParserOptions): GrammarParseOutcome;
}
export function createGlobalConceptSpaceParser(
  source: string, sourceIdentifier: string,
  options?: GlobalConceptSpaceParserOptions & Pick<GlobalConceptOptions, 'records'>,
): GlobalConceptSpaceParser;
export function verifyGlobalConceptCorrespondence(shapes: readonly string[], meanings: readonly string[],
  offsets: readonly number[], edges: readonly number[], first: readonly number[], second: readonly number[]): boolean;
export interface GlobalConstructTranslation {
  relation: 'translated' | 'ambiguous' | 'untranslatable' | 'unknown';
  concept: string | null;
  origins: GlobalConceptOrigin[];
  proofs: number[];
}
export function translateGlobalConstruct(space: GlobalConceptSpace, alias: string,
  language: string, options?: { edition?: string }): GlobalConstructTranslation;
export interface GrammarMergeSource {
  id: string;
  language: string;
  edition?: string;
  precedence?: number;
  grammar: Grammar;
}
export interface GrammarMergeDecision {
  kind: 'merged' | 'kept-unique' | 'renamed-for-collision' | 'homonym-kept-distinct' | 'uncertain' | 'declaration-conflict';
  name: string;
  members: string[];
  basis: string;
  definition: string | null;
}
export interface GrammarMergeNomination {
  basis: 'name-similarity' | 'identical-samples';
  members: [string, string];
  outcome: 'proven' | 'unproven';
}
export interface GrammarMergeAlternative {
  reason: 'start-rule' | 'distinct-meaning' | 'uncertain-match' | 'edition' | 'declaration-conflict';
  name: string;
  options: string[];
}
export interface GrammarMergeFailure {
  kind: 'unresolved-required-equivalence';
  members: [string, string];
  reason: 'unknown-rule' | 'different-language-or-edition' | 'not-proven';
}
export interface MergedGrammarGroup {
  key: string;
  language: string;
  edition: string;
  fingerprint: string;
  sources: string[];
  grammar: Grammar;
  identities: Record<string, string>;
  decisions: GrammarMergeDecision[];
  nominations: GrammarMergeNomination[];
  alternatives: GrammarMergeAlternative[];
}
export interface GrammarMergeResult {
  status: 'complete' | 'incomplete';
  groups: MergedGrammarGroup[];
  alternatives: GrammarMergeAlternative[];
  failures: GrammarMergeFailure[];
  reused: string[];
  recomputed: string[];
  globalConceptSpace: GlobalConceptSpace;
}
export interface GrammarMergeOptions extends DecoratorOptions {
  samples?: Record<string, string[]>;
  requiredEquivalences?: Array<[string, string]>;
  previous?: GrammarMergeResult | null;
  reconcile?: boolean;
  globalEquivalences?: Array<[string, string]>;
  conceptRecords?: readonly ConceptRecord[];
}
export class GrammarMergeError extends Error {
  failures: GrammarMergeFailure[];
}
export function mergeGrammars(sources: readonly GrammarMergeSource[], options?: GrammarMergeOptions): GrammarMergeResult;
export function assertMergeComplete(result: GrammarMergeResult): GrammarMergeResult;
export function normalizedRuleDefinition(rule: GrammarRuleValue): string;
export const GRAMMAR_ROUND_TRIP_MARKER: string;
export interface GrammarRoundTripFailure {
  kind:
    | 'sample-rejected'
    | 'sample-accepted'
    | 'marker-already-accepted'
    | 'lossy-export'
    | 'mutation-not-exported'
    | 'rules-changed'
    | 'mutation-not-visible'
    | 'export-not-stable';
  stage: 'imported' | 'exported' | 'reimported';
  detail: string;
}
export interface GrammarRoundTripReport {
  status: 'preserved' | 'broken';
  failures: GrammarRoundTripFailure[];
  mutated: Grammar;
  exported: string;
  reimported: Grammar;
}
export interface GrammarRoundTripOptions {
  importGrammar: (source: string) => Grammar;
  emitGrammar: (grammar: Grammar) => GrammarEmitResult;
  marker?: string;
  accepts?: readonly string[];
  rejects?: readonly string[];
}
export function mutateGrammarStartRule(grammar: Grammar, marker?: string): Grammar;
export function checkGrammarRoundTrip(source: string, options: GrammarRoundTripOptions): GrammarRoundTripReport;
export function canonicalRuleDefinition(rule: GrammarRuleValue): string;
export function acceptsText(grammar: Grammar, text: string): boolean;
export function carryRuleDocs(target: Grammar, source: Grammar, rename?: (name: string) => string): Grammar;
export function percentEncodeLinksText(value: string): string;
export function percentDecodeLinksText(value: string): string;
export function renderGrammarLinks(grammar: Grammar): string;
export function renderRuleLink(grammar: Grammar, rule: GrammarRuleValue): string;
export function renderRuleFieldLinks(rule: GrammarRuleValue): string[];
export function renderDeclarationLinks(declarations: Required<Omit<GrammarDeclarations, 'matching'>>): string[];
export function renderLinksExpression(expression: GrammarExpression): string;
export function parseLinksExpression(value: unknown): GrammarExpression;
export function parseGrammarLinks(source: string): Grammar;
export interface GrammarReverseFailure {
  kind: 'links-not-faithful' | 'lossy-export' | 'rules-changed' | 'doc-changed' | 'sample-rejected' | 'sample-accepted';
  stage: 'imported' | 'links' | 'exported' | 'reimported';
  detail: string;
}
export interface GrammarReverseReport {
  status: 'equivalent' | 'different';
  failures: GrammarReverseFailure[];
  links: string;
  exported: string;
  reimportedLinks: string;
}
export function checkGrammarReverseConversion(
  source: string,
  options: Omit<GrammarRoundTripOptions, 'marker'>,
): GrammarReverseReport;
export type GrammarLosslessFormat = 'abnf' | 'antlr' | 'bnf' | 'ebnf' | 'gbnf' | 'lark' | 'pest' | 'tree-sitter-json';
export const GRAMMAR_LOSSLESS_FORMATS: readonly GrammarLosslessFormat[];
export interface GrammarSourceDefinition {
  name: string;
  text: string;
  gap: string;
}
export interface GrammarLayoutDefinition extends GrammarSourceDefinition {
  fingerprint: string;
}
export interface GrammarLayout {
  format: GrammarLosslessFormat;
  prefix: string;
  members: GrammarLayoutDefinition[];
  implicit: Array<{ name: string; fingerprint: string }>;
}
export function splitGrammarSource(
  source: string,
  format: GrammarLosslessFormat,
  names: readonly string[],
): { prefix: string; members: GrammarSourceDefinition[] };
export function captureGrammarLayout(source: string, format: GrammarLosslessFormat, grammar: Grammar): GrammarLayout;
export function importGrammarLossless(source: string, format: GrammarLosslessFormat): { grammar: Grammar; layout: GrammarLayout };
export function emitGrammarLossless(grammar: Grammar, layout: GrammarLayout): GrammarEmitResult;
export function renderGrammarLayoutLinks(layout: GrammarLayout): string;
export function parseGrammarLayoutLinks(source: string): GrammarLayout;
export type GrammarDiagnosticKind =
  | 'duplicate-rule'
  | 'undefined-non-terminal'
  | 'left-recursion'
  | 'unreachable-rule'
  | 'nullable-repetition'
  | 'unused-capture';
export const GRAMMAR_DIAGNOSTIC_KINDS: readonly GrammarDiagnosticKind[];
export interface GrammarDiagnostic {
  kind: GrammarDiagnosticKind;
  severity: 'error' | 'warning';
  rule: string;
  message: string;
  [detail: string]: unknown;
}
export function validateGrammar(grammar: Grammar): GrammarDiagnostic[];
export function displayGrammarExpression(expression: GrammarExpression): string;
export type GrammarInterchangeFormat =
  | 'abnf'
  | 'antlr'
  | 'bnf'
  | 'ebnf'
  | 'gbnf'
  | 'lark'
  | 'native'
  | 'pest'
  | 'tree-sitter-json';
export const GRAMMAR_IMPORT_FORMATS: readonly GrammarInterchangeFormat[];
export const GRAMMAR_EXPORT_FORMATS: readonly GrammarInterchangeFormat[];
export function grammarImporter(format: string): ((source: string) => Grammar) | null;
export function grammarEmitter(format: string): ((grammar: Grammar) => GrammarEmitResult) | null;
export function renderNativeGrammar(grammar: Grammar): string;
export type StructuredDataValue = null | boolean | number | string | StructuredDataValue[] | { [key: string]: StructuredDataValue };
export interface StructuredDataLinksLimits { nodeLimit?: number; depthLimit?: number; byteLimit?: number; }
export interface StructuredDataLinksArchive {
  network: { link(id: LinkId): Link | undefined; links(): Link[]; toLino(): string };
  root: LinkId;
}
export function encodeStructuredDataLinks(value: StructuredDataValue, limits?: StructuredDataLinksLimits): StructuredDataLinksArchive;
export function decodeStructuredDataLinks(network: StructuredDataLinksArchive['network'], root: LinkId, limits?: StructuredDataLinksLimits): StructuredDataValue;
export function structuredDataToLino(value: StructuredDataValue, limits?: StructuredDataLinksLimits): string;
export function readStructuredDataLino(source: string, limits?: StructuredDataLinksLimits): StructuredDataValue;
export function structuredDataRecordToLino(value: StructuredDataValue, namespace?: string, limits?: StructuredDataLinksLimits): string;
export function readStructuredDataRecords(source: string, limits?: StructuredDataLinksLimits): StructuredDataValue[];

export function renderNativeExpression(expression: GrammarExpression): string;
export function parseNativeGrammar(source: string): Grammar;
export const GRAMMAR_COMMAND_USAGE: string;
export interface GrammarCommandOutput {
  exitCode: number;
  stdout: string;
  stderr: string;
}
export function runGrammarCommand(
  args: readonly string[],
  options: { readFile: (path: string) => string },
): GrammarCommandOutput;
export interface RuleAlias {
  canonical: string;
  original: string;
}
export class GrammarRenameError extends Error {
  kind: 'invalid-name' | 'unknown-rule' | 'collision';
}
export function renameGrammarRule(
  grammar: Grammar,
  from: string,
  to: string,
  options?: { namespace?: string | null; aliases?: readonly RuleAlias[] },
): { grammar: Grammar; aliases: RuleAlias[] };
export function restoreSourceNames(
  grammar: Grammar,
  aliases: readonly RuleAlias[],
  options?: { namespace?: string | null },
): Grammar;

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

/** Embedded regions an HTML or Markdown host grammar CST delimits, in source order. */
export function detectEmbeddedRegions(
  text: string,
  language: string,
  policy?: RegionDetectionPolicyValue,
): EmbeddedRegion[];
/** Embedded regions of an already parsed host tree (nodes with term, span and children). */
export function detectEmbeddedRegionsInTree(
  tree: unknown,
  text: string,
  host: 'HTML' | 'Markdown',
  policy?: RegionDetectionPolicyValue,
): EmbeddedRegion[];
/** The language of an HTML script element's content from its `type` attribute. */
export function scriptLanguage(type: string | undefined): string | null;
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

/** Complete editable source-derived JavaScript numeric conversion definitions. */
export function createStringNumberProgramLinks(): TypedProgramLinks;
/** ECMAScript StringNumericLiteral conversion, preserving binary64 signed zero. */
export function convertJavaScriptStringNumber(text: string): number;

/** Complete source-derived exact BigInt conversion definitions. */
export function createBigIntegerConversionProgramLinks(): TypedProgramLinks;
export function convertJavaScriptStringBigInteger(text: string): bigint;
export function convertJavaScriptStringBigIntegerUnits(units: number[]): bigint;
export function convertJavaScriptNumberBigInteger(value: number): bigint;

export function convertJavaScriptBooleanBigInteger(value: boolean): bigint;

/** Complete source exception constructor, class, message and presence definitions. */
export function createSourceExceptionProgramLinks(): TypedProgramLinks;
/** Complete editable UTF-16 literal, escape and unit construction definitions. */
export function createJavaScriptStringUnitProgramLinks(): TypedProgramLinks;
/** Complete editable source constructor and UTF-16 text operation definitions. */
export function createStringConstructionProgramLinks(): TypedProgramLinks;
export function createUtf16TextProgramLinks(): TypedProgramLinks;
export function createFormalStringUnitProgramLinks(): TypedProgramLinks;

/** Complete source-derived primitive comparison decisions. */
export function createJavaScriptComparisonProgramLinks(): TypedProgramLinks;
/** Complete source-derived represented binding initializer decisions. */
export function createArrayBindingProgramLinks(): TypedProgramLinks;
/** Complete source-derived String range checked-operator plans. */
export function createJavaScriptStringRangeProgramLinks(): TypedProgramLinks;

/** Complete shared optional-value and Boolean owner definitions. */
export function createOptionalValueProgramLinks(): TypedProgramLinks;

/** Full Unicode 16 default case conversion over UTF16 units. */
export function mapUnicodeCaseUtf16Text(text: readonly number[], upper: boolean): number[];
export function createUnicodeCaseProgramLinks(): TypedProgramLinks;

export function createRustUnitValueProgramLinks(): TypedProgramLinks;

export function createRustFormattingProgramLinks(): TypedProgramLinks;
/** One complete artifact identity; a digest does not prove equal meaning. */
export interface SourceDefinitionEntry { identifier:string; asset:string; bytes:number; sha256:string; root:string; definitionIdentity:string }
export interface SourceDefinitionCatalogLimits { byteLimit?:number; nodeLimit?:number; depthLimit?:number }
export function sourceDefinitionCatalog(): SourceDefinitionEntry[];
export function sourceDefinitionEntry(identifier:string): SourceDefinitionEntry|undefined;
export function createSourceDefinitionProgramLinks(identifier:string,options?:SourceDefinitionCatalogLimits): {network:LinkNetwork;root:LinkId};
export function createSourceDefinitionFormalFunctionLinks(identifier:string,declaration:string|LinkId,options?:SourceDefinitionCatalogLimits): ReturnType<typeof encodeFormalFunctionDefinition>;
export function runSourceDefinitionCommand(args:string[]): {exitCode:number;stdout:string|Uint8Array;stderr:string};
export function createJavaScriptRecordCallableParameterProgramLinks(): TypedProgramLinks;
export function createJavaScriptArrayFormalParameterProgramLinks():TypedProgramLinks;
