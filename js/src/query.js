import { LinkType } from './primitives.js';

export class LinkQuery {
  constructor({
    linkType = undefined,
    term = undefined,
    language = undefined,
    named = undefined,
    sexpression = undefined,
  } = {}) {
    this.linkType = linkType;
    this.term = term;
    this.language = language;
    this.named = named;
    this.sexpression = sexpression;
  }

  static byType(linkType) {
    return new LinkQuery({ linkType });
  }

  static byTerm(term) {
    return new LinkQuery({ term });
  }

  static fromSexpression(source) {
    const root = source.match(/\(\s*([^()\s]+)\s*\)\s*@([A-Za-z_][\w-]*)/m);
    if (!root) {
      throw new SyntaxError('S-expression query must start with `(node_type) @capture`');
    }

    const predicates = [];
    const predicatePattern =
      /\(\s*#(eq\?|not-eq\?)\s+@([A-Za-z_][\w-]*)\s+"((?:\\.|[^"])*)"\s*\)/gm;
    let match = predicatePattern.exec(source);
    while (match) {
      predicates.push({
        operator: match[1],
        capture: match[2],
        value: unescapeString(match[3]),
      });
      match = predicatePattern.exec(source);
    }

    return new LinkQuery({
      sexpression: {
        nodeType: root[1],
        capture: root[2],
        predicates,
      },
    });
  }

  withTerm(term) {
    return new LinkQuery({ ...this, term });
  }

  withLanguage(language) {
    return new LinkQuery({ ...this, language });
  }

  withNamed(named = true) {
    return new LinkQuery({ ...this, named });
  }

  matchesMetadata(metadata) {
    if (this.linkType !== undefined && metadata.linkType !== this.linkType) {
      return false;
    }
    if (this.term !== undefined && metadata.term !== this.term) {
      return false;
    }
    if (this.language !== undefined && metadata.language !== this.language) {
      return false;
    }
    if (this.named !== undefined && metadata.named !== this.named) {
      return false;
    }
    if (this.sexpression && metadata.term !== this.sexpression.nodeType) {
      return false;
    }
    return true;
  }
}

export class QueryCaptures {
  constructor(entries = []) {
    this.entries = new Map(entries);
  }

  get(name) {
    return this.entries.get(name);
  }

  set(name, linkId) {
    this.entries.set(name, linkId);
  }

  has(name) {
    return this.entries.has(name);
  }

  [Symbol.iterator]() {
    return this.entries[Symbol.iterator]();
  }
}

export class QueryMatch {
  constructor(linkId, captures = new QueryCaptures()) {
    this.linkId = linkId;
    this.captures = captures;
  }
}

export function queryByConceptTerm(term) {
  return LinkQuery.byType(LinkType.Concept).withTerm(term);
}

function unescapeString(value) {
  return value.replace(/\\"/g, '"').replace(/\\\\/g, '\\');
}
