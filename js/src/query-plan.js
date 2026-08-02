import {
  ByteRange,
  LinkMetadata,
  LinkType,
  Point,
  SourceSpan,
} from './primitives.js';
import { parseSqlOperation } from './query-plan-parser.js';

const encoder = new TextEncoder();

/** SQL profiles whose common subset normalizes through the built-in frontend. */
export const SQL_DIALECT_PROFILES = Object.freeze([
  Object.freeze({ key: 'sql-ansi', vendor: 'ANSI SQL' }),
  Object.freeze({ key: 'sql-postgres', vendor: 'PostgreSQL' }),
  Object.freeze({ key: 'sql-mysql', vendor: 'MySQL' }),
  Object.freeze({ key: 'sql-sqlite', vendor: 'SQLite' }),
  Object.freeze({ key: 'sql-server', vendor: 'SQL Server' }),
  Object.freeze({ key: 'sql-oracle', vendor: 'Oracle' }),
  Object.freeze({ key: 'sql-bigquery', vendor: 'BigQuery' }),
  Object.freeze({ key: 'sql-snowflake', vendor: 'Snowflake' }),
]);

export const QueryAuthorization = Object.freeze({
  Required: 'Required',
});

export const QueryPlanErrorKind = Object.freeze({
  UnsupportedLanguage: 'UnsupportedLanguage',
  InvalidConcreteSyntax: 'InvalidConcreteSyntax',
  Syntax: 'Syntax',
  Semantic: 'Semantic',
  Frontend: 'Frontend',
});

/** Fail-closed query frontend error. */
export class QueryPlanError extends Error {
  constructor(kind, message, offset = undefined) {
    super(offset === undefined ? message : `${message} at byte ${offset}`);
    this.name = 'QueryPlanError';
    this.kind = kind;
    this.offset = offset;
  }

  static frontend(message) {
    return new QueryPlanError(QueryPlanErrorKind.Frontend, message);
  }
}

/** Source evidence retained separately from canonical operation semantics. */
export class SourceEvidence {
  constructor(language, span, syntaxLinkId = undefined) {
    this.language = String(language);
    this.span = span;
    this.syntaxLinkId = syntaxLinkId;
  }

  static synthetic(language) {
    return new SourceEvidence(
      language,
      new SourceSpan(new ByteRange(0, 0), new Point(0, 0), new Point(0, 0)),
    );
  }

  static forSource(language, source) {
    const byteLength = encoder.encode(source).length;
    const lines = source.split('\n');
    return new SourceEvidence(
      language,
      new SourceSpan(
        new ByteRange(0, byteLength),
        new Point(0, 0),
        new Point(lines.length - 1, encoder.encode(lines.at(-1)).length),
      ),
    );
  }
}

/** Canonical operation plus non-authorizing source evidence. */
export class QueryPlan {
  constructor(operation, evidence) {
    this._operation = deepClone(operation);
    this._evidence = evidence;
  }

  operation() {
    return deepClone(this._operation);
  }

  evidence() {
    return this._evidence;
  }

  authorization() {
    return QueryAuthorization.Required;
  }

  canonicalJson() {
    return deepClone(this._operation);
  }

  canonical_json() {
    return this.canonicalJson();
  }

  /** Declares this plan as engine-neutral semantic links in `network`. */
  declareIn(network) {
    const references = this._evidence.syntaxLinkId !== undefined
      && network.link(this._evidence.syntaxLinkId) !== undefined
      ? [this._evidence.syntaxLinkId]
      : [];
    const root = network.insertLink(
      references,
      LinkMetadata.new()
        .withLinkType(LinkType.Semantic)
        .withNamed(true)
        .withTerm('query-plan')
        .withDefinition(JSON.stringify(this._operation)),
    );
    const links = [root];
    const operation = insertSemantic(
      network,
      root,
      `query-operation:${this._operation.kind}`,
      undefined,
      links,
    );
    declareOperation(network, operation, this._operation, links);
    return Object.freeze({ root, links: Object.freeze(links) });
  }

  declare_in(network) {
    return this.declareIn(network);
  }
}

/** Built-in SQL common-subset frontend. */
export class BuiltInSqlFrontend {
  lower(source, language) {
    return lowerSql(source, language);
  }
}

/** Extensible case-insensitive source frontend registry. */
export class QueryPlanRegistry {
  constructor() {
    const sql = new BuiltInSqlFrontend();
    this._frontends = new Map(
      SQL_DIALECT_PROFILES.map((profile) => [profile.key, sql]),
    );
  }

  register(language, frontend) {
    if (typeof frontend?.lower !== 'function') {
      throw new TypeError('query frontend must provide lower(source, language)');
    }
    this._frontends.set(String(language).toLowerCase(), frontend);
    return this;
  }

  isRegistered(language) {
    return this._frontends.has(String(language).toLowerCase());
  }

  is_registered(language) {
    return this.isRegistered(language);
  }

  lower(source, language) {
    const frontend = this._frontends.get(String(language).toLowerCase());
    if (frontend === undefined) {
      throw unsupportedLanguage(language);
    }
    return frontend.lower(source, language);
  }
}

/** Lowers one complete SQL statement for a supported vendor profile. */
export function lowerSql(source, language) {
  const profile = SQL_DIALECT_PROFILES.find(
    (candidate) => candidate.key.toLowerCase() === String(language).toLowerCase(),
  );
  if (profile === undefined) {
    throw unsupportedLanguage(language);
  }
  const operation = parseSqlOperation(String(source));
  return new QueryPlan(operation, SourceEvidence.forSource(language, String(source)));
}

export const lower_sql = lowerSql;

function unsupportedLanguage(language) {
  return new QueryPlanError(
    QueryPlanErrorKind.UnsupportedLanguage,
    `no query frontend is registered for \`${language}\``,
  );
}

function insertSemantic(network, parent, term, definition, links) {
  let metadata = LinkMetadata.new()
    .withLinkType(LinkType.Semantic)
    .withNamed(true)
    .withTerm(term);
  if (definition !== undefined) {
    metadata = metadata.withDefinition(String(definition));
  }
  const id = network.insertLink([parent], metadata);
  links.push(id);
  return id;
}

function declareOperation(network, parent, operation, links) {
  switch (operation.kind) {
    case 'select':
      for (const item of operation.projection) {
        const projection = insertSemantic(
          network,
          parent,
          'query-projection',
          item.alias ?? undefined,
          links,
        );
        declareExpression(network, projection, item.expression, links);
      }
      if (operation.from !== null) {
        declareSource(network, parent, 'query-source', operation.from, links);
      }
      declareOptionalFilter(network, parent, operation.filter, links);
      for (const expression of operation.groupBy) {
        const group = insertSemantic(network, parent, 'query-group', undefined, links);
        declareExpression(network, group, expression, links);
      }
      for (const item of operation.orderBy) {
        const order = insertSemantic(
          network,
          parent,
          'query-order',
          item.direction,
          links,
        );
        declareExpression(network, order, item.expression, links);
      }
      if (operation.limit !== null) {
        insertSemantic(network, parent, 'query-limit', operation.limit, links);
      }
      if (operation.offset !== null) {
        insertSemantic(network, parent, 'query-offset', operation.offset, links);
      }
      break;
    case 'insert':
      declareSource(network, parent, 'query-target', operation.into, links);
      for (const row of operation.rows) {
        const rowLink = insertSemantic(network, parent, 'query-insert-row', undefined, links);
        for (const expression of row) {
          declareExpression(network, rowLink, expression, links);
        }
      }
      break;
    case 'update':
      declareSource(network, parent, 'query-target', operation.table, links);
      for (const assignment of operation.assignments) {
        const assignmentLink = insertSemantic(
          network,
          parent,
          'query-assignment',
          assignment.column.join('.'),
          links,
        );
        declareExpression(network, assignmentLink, assignment.value, links);
      }
      declareOptionalFilter(network, parent, operation.filter, links);
      break;
    case 'delete':
      declareSource(network, parent, 'query-target', operation.from, links);
      declareOptionalFilter(network, parent, operation.filter, links);
      break;
    default:
      throw QueryPlanError.frontend(`unsupported canonical operation: ${operation.kind}`);
  }
}

function declareOptionalFilter(network, parent, expression, links) {
  if (expression !== null) {
    const filter = insertSemantic(network, parent, 'query-filter', undefined, links);
    declareExpression(network, filter, expression, links);
  }
}

function declareSource(network, parent, term, source, links) {
  insertSemantic(network, parent, term, source.path.join('.'), links);
}

function declareExpression(network, parent, expression, links) {
  let term;
  let definition;
  switch (expression.kind) {
    case 'column':
      term = 'query-column';
      definition = expression.path.join('.');
      break;
    case 'literal':
      term = 'query-literal';
      definition = JSON.stringify(expression.value);
      break;
    case 'parameter':
      term = 'query-parameter';
      definition = expression.name;
      break;
    case 'wildcard':
      term = 'query-wildcard';
      break;
    case 'unary':
      term = `query-unary:${expression.operator}`;
      break;
    case 'binary':
      term = `query-binary:${expression.operator}`;
      break;
    case 'aggregate':
      term = `query-aggregate:${expression.function}`;
      break;
    case 'function':
      term = 'query-function';
      definition = expression.name;
      break;
    case 'extension':
      term = 'query-extension';
      definition = `${expression.namespace}:${expression.name}`;
      break;
    default:
      throw QueryPlanError.frontend(`unsupported canonical expression: ${expression.kind}`);
  }
  const expressionLink = insertSemantic(network, parent, term, definition, links);
  if (expression.kind === 'unary') {
    declareExpression(network, expressionLink, expression.operand, links);
  } else if (expression.kind === 'binary') {
    declareExpression(network, expressionLink, expression.left, links);
    declareExpression(network, expressionLink, expression.right, links);
  } else if (expression.kind === 'aggregate') {
    declareExpression(network, expressionLink, expression.expression, links);
  } else if (expression.kind === 'function' || expression.kind === 'extension') {
    for (const argument of expression.arguments) {
      declareExpression(network, expressionLink, argument, links);
    }
  }
}

function deepClone(value) {
  return JSON.parse(JSON.stringify(value));
}
