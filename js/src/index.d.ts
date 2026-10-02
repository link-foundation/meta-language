export * from './query-algebra.js';

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
): ProgramTranslation;
export function decodeProgramTranslation(
  code: string,
  targetLanguage: string,
): DecodedProgramTranslation;
export function readTranslationProvenance(
  code: string,
  targetLanguage: string,
): ReadTranslationProvenance;

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

export class ParseConfiguration {
  readonly triviaAttachmentPolicy: string;
  readonly regionDetectionPolicy: RegionDetectionPolicyValue;
  readonly accessMode: string;
  static default(): ParseConfiguration;
  withTriviaAttachmentPolicy(policy: string): ParseConfiguration;
  withRegionDetectionPolicy(policy: RegionDetectionPolicyValue): ParseConfiguration;
  with_region_detection_policy(policy: RegionDetectionPolicyValue): ParseConfiguration;
  withAccessMode(mode: string): ParseConfiguration;
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
  render(targetLanguage: string, network: LinkNetwork, rootLinkId?: LinkIdValue): string;
  toLino(): string;
  toJson(): string;
  static fromLino(source: string): TranslationRuleSet;
  static fromJson(source: string | unknown): TranslationRuleSet;
}

export type GrammarRuleKind = 'normal' | 'atomic' | 'silent' | 'token' | 'terminal' | 'nonterminal';
export type GrammarExpression =
  | { kind: 'empty' | 'any' }
  | { kind: 'literal' | 'literalInsensitive' | 'regex'; value: string }
  | { kind: 'ref'; name: string }
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
        { kind: 'char'; value: string } | { kind: 'range'; start: string; end: string }
      >;
      negated?: boolean;
    };
export interface GrammarRuleValue {
  name: string;
  kind: GrammarRuleKind;
  expression: GrammarExpression;
}
export interface NormalizedGrammar {
  schemaVersion: 1;
  start: string | null;
  sourceFormat: string | null;
  rules: GrammarRuleValue[];
}

export class Grammar {
  constructor(
    start: string | null,
    rules: Map<string, Omit<GrammarRuleValue, 'name'> | GrammarRuleValue>,
    sourceFormat?: string | null,
  );
  start: string | null;
  sourceFormat: string | null;
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
export function compileGrammar(grammar: Grammar, options?: Record<string, unknown>): unknown;
export function parseWithGrammar(
  grammar: Grammar,
  source: string,
  options?: Record<string, unknown>,
): unknown;
export function emitJavascriptParser(grammar: Grammar): string;
export function serializeGrammar(grammar: Grammar): string;
export function deserializeGrammar(source: string | NormalizedGrammar): Grammar;

export class GrammarImportError extends Error {
  format: string;
  kind: 'parse' | 'unsupported';
  construct?: string;
}
export function importAbnf(source: string): Grammar;
export const import_abnf: typeof importAbnf;
export function importAntlr(source: string): Grammar;
export const import_antlr: typeof importAntlr;
export function importBnf(source: string): Grammar;
export const import_bnf: typeof importBnf;
export function importEbnf(source: string): Grammar;
export const import_ebnf: typeof importEbnf;
export function importGbnf(source: string): Grammar;
export const import_gbnf: typeof importGbnf;
export function importLark(source: string): Grammar;
export const import_lark: typeof importLark;
export function importPest(source: string): Grammar;
export const import_pest: typeof importPest;
export function importTreeSitterJson(source: string | unknown): Grammar;
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
export function emitAbnf(grammar: Grammar): GrammarEmitResult;
export const emit_abnf: typeof emitAbnf;
export function emitAntlr(grammar: Grammar): GrammarEmitResult;
export const emit_antlr: typeof emitAntlr;
export function emitBnf(grammar: Grammar): GrammarEmitResult;
export const emit_bnf: typeof emitBnf;
export function emitEbnf(grammar: Grammar): GrammarEmitResult;
export const emit_ebnf: typeof emitEbnf;
export function emitGbnf(grammar: Grammar): GrammarEmitResult;
export const emit_gbnf: typeof emitGbnf;
export function emitLark(grammar: Grammar): GrammarEmitResult;
export const emit_lark: typeof emitLark;
export function emitPest(grammar: Grammar): GrammarEmitResult;
export const emit_pest: typeof emitPest;
export function emitTreeSitterJson(grammar: Grammar): GrammarEmitResult;
export const emit_tree_sitter_json: typeof emitTreeSitterJson;

export const GRAMMAR_MERGE_METHOD: 'recursive-structural-bisimulation';
export interface GrammarMergeSource {
  id: string;
  language: string;
  edition?: string;
  precedence?: number;
  grammar: Grammar;
}
export interface GrammarMergeDecision {
  kind: 'merged' | 'kept-unique' | 'renamed-for-collision' | 'homonym-kept-distinct' | 'uncertain';
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
  reason: 'start-rule' | 'distinct-meaning' | 'uncertain-match' | 'edition';
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
}
export interface GrammarMergeOptions {
  samples?: Record<string, string[]>;
  requiredEquivalences?: Array<[string, string]>;
  previous?: GrammarMergeResult | null;
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
