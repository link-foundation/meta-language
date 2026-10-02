// The JavaScript side of the link-assistant/formal-ai workloads
// (requirement I195-DOWNSTREAM-FORMAL-AI-WORKLOADS). formal-ai's npm package
// declares no meta-language dependency, so js/scripts/run-formal-ai-workloads.mjs
// runs formal-ai's own data through the installed npm artifact instead: the
// grammar projection seed, the program grammar catalog and its hello-world
// programs, the link-edit rule shapes and the coding benchmark LiNo, each
// through the public API (parse to a network, query matches, a translation
// rule set from LiNo, edits verified by a full match, to_lino and from_lino).
//
// `buildFormalAiInputs` reads that data from the pinned checkout into one
// inputs file both runtimes read; `formalAiHelpers` is the self-contained
// consumer code (it is serialized into the consumer directory, so it closes
// over nothing but the `api` it is given); the Rust probe of
// js/scripts/issue-195-formal-ai-rust-probe.mjs writes the same outputs with
// formal-ai's own Rust functions, and `compareRuntimeOutputs` checks that both
// runtimes produced the same links and projections.
import { readFile } from 'node:fs/promises';
import path from 'node:path';

export { TEST_EVENT_REPORTER } from './issue-195-rml-workload-probes.mjs';

/** The formal-ai data files of the JavaScript workloads, as checkout paths. */
export const FORMAL_AI_DATA_FILES = Object.freeze({
  projectionRules: 'data/seed/grammar-projection-rules.lino',
  programGrammars: 'data/seed/program-cst-grammars.lino',
  helloWorldPrograms: 'data/seed/hello-world-programs.lino',
  linkEditRules: 'data/meta/link-edit-rules.lino',
  codingBenchmark: 'data/benchmarks/coding-modification-suite.lino',
});

/** The grammar labels formal-ai's projection owns (formal_ai::grammar_kinds::CORPORA). */
export const PROJECTION_LABELS = Object.freeze(['rust', 'javascript', 'typescript']);

// The source languages of a projection rule, by the prefix of its name.
const RULE_SOURCES = Object.freeze({ rust: ['rust'], es: ['javascript', 'typescript'], ts: ['typescript'] });

const REFUSAL_LANGUAGE = 'grammar-projection-refusal';
const NOFORM_LANGUAGE = 'grammar-projection-noform';

// formal-ai's issue 1085 source and link edits (rust/tests/unit/issue_1085_link_edit_rules.rs).
const LINK_EDIT_SOURCE = 'pub const WEB_SEARCH_PROVIDERS: &[&str] = &[\n    "wikipedia",\n    "wikidata",\n];\n\n' +
  'const QUERY_PLACEHOLDER: &str = "{query}";\nconst QUERY_PLACEHOLDER_2: &str = "second {query}";\n\n' +
  'fn describe() -> String {\n    format!("search {}", QUERY_PLACEHOLDER)\n}\n';

/** The link-edit cases: every step edits the text the previous step left. */
export const LINK_EDIT_CASES = Object.freeze([
  {
    name: 'insert a provider, then refuse it as present',
    language: 'rust',
    source: LINK_EDIT_SOURCE,
    steps: [
      {
        rule: { shape: 'insert_member', list: 'WEB_SEARCH_PROVIDERS', member: 'wikiquote' },
        expect: { outcome: 'edited', edits: 1, contains: '    "wikidata",\n    "wikiquote",\n];' },
      },
      {
        rule: { shape: 'insert_member', list: 'WEB_SEARCH_PROVIDERS', member: 'wikiquote' },
        expect: { outcome: 'refused', error: 'link_edit:member_present:WEB_SEARCH_PROVIDERS:wikiquote' },
      },
    ],
  },
  {
    name: 'replace a literal inside both string links',
    language: 'rust',
    source: LINK_EDIT_SOURCE,
    steps: [{
      rule: { shape: 'replace_literal', old: '{query}', new: '{search_query}' },
      expect: { outcome: 'edited', edits: 2, contains: 'const QUERY_PLACEHOLDER_2: &str = "second {search_query}";' },
    }],
  },
  {
    name: 'rename an identifier at its declaration and its use',
    language: 'rust',
    source: LINK_EDIT_SOURCE,
    steps: [{
      rule: { shape: 'rename_identifier', old: 'QUERY_PLACEHOLDER', new: 'TRENDS_QUERY_PLACEHOLDER' },
      expect: { outcome: 'edited', edits: 2, contains: 'format!("search {}", TRENDS_QUERY_PLACEHOLDER)' },
    }],
  },
  {
    name: 'refuse a rename without a matching link',
    language: 'rust',
    source: LINK_EDIT_SOURCE,
    steps: [{
      rule: { shape: 'rename_identifier', old: 'NOT_IN_THE_FILE', new: 'ANYTHING' },
      expect: { outcome: 'refused', error: 'link_edit:no_matching_link:rename_identifier:NOT_IN_THE_FILE' },
    }],
  },
]);

// formal-ai's issue 1138 projection samples (rust/tests/unit/issue_1138_rust_projection.rs).
const refused = (construct) => ({ outcome: 'refused', construct });
const rendered = (source) => ({ outcome: 'rendered', source });
/** The projection cases and what formal-ai's own tests expect of them. */
export const PROJECTION_CASES = Object.freeze([
  { name: 'an unknown source label', from: 'nope', target: 'rust', source: 'x', expect: { ...refused('nope'), refusals: 1 } },
  { name: 'an unknown target label', from: 'rust', target: 'nope', source: 'x', expect: { ...refused('rust'), refusals: 1 } },
  { name: 'the same label', from: 'rust', target: 'rust', source: 'x', expect: { ...refused('rust'), refusals: 1 } },
  {
    name: 'a rust function into javascript', from: 'rust', target: 'javascript',
    source: 'pub fn area(w: u32) -> u32 { w * w }', expect: rendered('function area(w) {\nw * w\n}'),
  },
  {
    name: 'a rust function into typescript', from: 'rust', target: 'typescript',
    source: 'pub fn area(w: u32) -> u32 { w * w }', expect: rendered('function area(w: u32): u32 {\nw * w\n}'),
  },
  {
    name: 'a javascript function into rust', from: 'javascript', target: 'rust',
    source: 'function add(a, b) { return a + b; }', expect: rendered('fn add(a, b) {\nreturn a + b;\n}'),
  },
  {
    name: 'a javascript variable into rust', from: 'javascript', target: 'rust',
    source: 'var total = obj.compute(1, 2);', expect: rendered('let total = obj.compute(1, 2);'),
  },
  { name: 'an object literal into rust', from: 'javascript', target: 'rust', source: 'let o = { a: 1 };', expect: refused('object') },
  {
    name: 'a try statement into rust', from: 'javascript', target: 'rust',
    source: 'try { f(); } catch (e) { g(); }', expect: refused('try_statement'),
  },
  {
    name: 'a match into javascript', from: 'rust', target: 'javascript',
    source: 'fn m(x: u32) { match x { 1 => 2, _ => 3 } }', expect: refused('match_expression'),
  },
  { name: 'an enum into typescript', from: 'rust', target: 'typescript', source: 'enum E {\nA,\nB(u32)\n}', expect: refused('enum_item') },
  { name: 'a trait into typescript', from: 'rust', target: 'typescript', source: 'trait T { fn m(&self); }', expect: refused('trait_item') },
]);

/** The link-edit cases of the LiNo data: a quoted literal of the program grammar catalog. */
function linoLinkEditCases(programGrammars) {
  return [{
    name: 'replace a quoted LiNo literal of the program grammar catalog',
    language: 'lino',
    source: programGrammars,
    steps: [{
      rule: { shape: 'replace_literal', old: 'meta-language', new: 'meta-language-candidate' },
      expect: { outcome: 'edited', edits: 1, contains: 'component "meta-language-candidate"' },
    }],
  }];
}

// A LiNo string literal as formal-ai's seeds write it: single-quoted, with \n, \t, \r and \xHH escapes.
function unescapeLiteral(raw) {
  return raw.slice(1, -1).replace(/\\(x[0-9a-fA-F]{2}|.)/gsu, (whole, escape) =>
    escape[0] === 'x' && escape.length === 3
      ? String.fromCharCode(Number.parseInt(escape.slice(1), 16))
      : { n: '\n', t: '\t', r: '\r' }[escape] ?? escape);
}

function decodeField(value) {
  try {
    return decodeURIComponent(value);
  } catch {
    return value;
  }
}

/** The rows of a seed written as `(id: parent (meta: (field: value) …))`. */
export function seedRows(text) {
  return text.split('\n').filter(Boolean).map((line) => {
    const row = /^\((\d+): (?:(\d+) )?\(meta: (.*)\)\)$/u.exec(line);
    if (!row) throw new Error(`not a seed row: ${line.slice(0, 120)}`);
    const fields = Object.fromEntries([...row[3].matchAll(/\((t|n|term|def|lang): ([^()\s]*)\)/gu)]
      .map(([, name, value]) => [name, decodeField(value)]));
    return { id: Number(row[1]), parent: row[2] ? Number(row[2]) : null, ...fields };
  });
}

/** The projection rules and the ruled, refused and no-form kinds the seed declares. */
export function projectionSeed(text) {
  const rows = seedRows(text);
  const children = (parent) => rows.filter((row) => row.parent === parent);
  const root = rows.find((row) => row.term === 'translation-rule-set');
  const rules = rows.filter((row) => row.parent === root?.id && row.term === 'translation-rule').map((row) => {
    const specification = JSON.parse(children(row.id).find((child) => child.term === 'translation-rule-match')?.def ?? '{}');
    const sexpression = specification.sexpression ?? '';
    return {
      name: row.def,
      sexpression,
      rootKind: /^\(\s*([^\s()]+)/u.exec(sexpression)?.[1] ?? null,
      sources: RULE_SOURCES[row.def.split(':')[0]] ?? [],
      targets: children(row.id).filter((child) => child.def === 'translation-rule-template').map((child) => child.lang),
    };
  });
  const declared = (language) => rows.filter((row) => row.parent === root?.id && row.lang === language)
    .map((row) => ({ kind: row.term, target: row.def }));
  return {
    rules,
    classification: {
      ruled: rules.filter(({ rootKind }) => rootKind).flatMap(({ rootKind, targets }) => targets.map((target) => ({ kind: rootKind, target }))),
      refused: declared(REFUSAL_LANGUAGE),
      noform: declared(NOFORM_LANGUAGE),
    },
  };
}

/** The hello-world programs of the given grammar labels, as `template_…` blocks declare them. */
export function helloWorldPrograms(text, labels) {
  const programs = [];
  for (const block of text.split(/\n(?=\S)/u)) {
    const head = block.split('\n')[0];
    if (!head.startsWith('template_')) continue;
    const field = (name) => new RegExp(`^  ${name} (.*)$`, 'mu').exec(block)?.[1];
    const language = field('language');
    const code = field('code');
    if (!labels.includes(language) || !code) continue;
    programs.push({ name: head, language, text: unescapeLiteral(code), source: FORMAL_AI_DATA_FILES.helloWorldPrograms });
  }
  return programs;
}

/** The grammar labels the program grammar catalog declares (its `meta_language_label` fields). */
export function programGrammarLabels(text) {
  return [...text.matchAll(/^\s*meta_language_label "([^"]+)"$/gmu)].map(([, label]) => label);
}

/** The inputs both runtimes read, from formal-ai's data at the pinned checkout. */
export async function buildFormalAiInputs(checkout) {
  const read = (file) => readFile(path.join(checkout, file), 'utf8');
  const data = Object.fromEntries(await Promise.all(Object.entries(FORMAL_AI_DATA_FILES)
    .map(async ([key, file]) => [key, await read(file)])));
  const labels = programGrammarLabels(data.programGrammars);
  const { rules, classification } = projectionSeed(data.projectionRules);
  const documents = [
    ...helloWorldPrograms(data.helloWorldPrograms, labels),
    ...Object.entries(FORMAL_AI_DATA_FILES).map(([key, file]) => ({ name: file, language: 'lino', text: data[key], source: file })),
  ];
  return {
    dataFiles: { ...FORMAL_AI_DATA_FILES },
    labels,
    projectionLabels: [...PROJECTION_LABELS],
    documents,
    rules,
    classification,
    ruleSetText: data.projectionRules,
    linkEditRulesText: data.linkEditRules,
    linkEdits: [...LINK_EDIT_CASES, ...linoLinkEditCases(data.programGrammars)],
    projections: PROJECTION_CASES,
  };
}

/**
 * The consumer code of the JavaScript workloads over the installed package
 * `api`. Self-contained: the runner writes `(${formalAiHelpers})(api)` into the
 * consumer directory. Every output mirrors what the Rust probe writes with
 * formal-ai's functions, so the two runtimes compare field by field.
 */
export function formalAiHelpers(api) {
  const { LinkNetwork, LinkQuery, ReplacementRule, TranslationRuleSet } = api;
  const refusalLanguage = 'grammar-projection-refusal';
  const noformLanguage = 'grammar-projection-noform';
  const message = (error) => String(error?.message ?? error);
  const sameList = (left, right) => left.length === right.length && left.every((value, index) => value === right[index]);
  const byteRange = (metadata) => metadata.span?.byteRange ?? null;
  const syntaxLinks = (network) => [...network.links()].filter((link) => link.metadata().linkType === 'Syntax');
  const nodeLabel = (metadata) => {
    const range = byteRange(metadata);
    return range ? `${metadata.term} ${range.start}..${range.end}` : `${metadata.term} ?`;
  };
  const parse = (text, language) => LinkNetwork.parse(text, language);

  function namedNodes(network) {
    return syntaxLinks(network)
      .filter((link) => link.metadata().named && byteRange(link.metadata()))
      .map((link) => nodeLabel(link.metadata()))
      .sort();
  }

  // formal_ai::grammar_kinds::named_kind_histogram: named syntax links by kind.
  function namedKinds(network) {
    const histogram = new Map();
    for (const link of syntaxLinks(network)) {
      const metadata = link.metadata();
      if (!metadata.named) continue;
      const kind = metadata.term ?? '?';
      histogram.set(kind, (histogram.get(kind) ?? 0) + 1);
    }
    return Object.fromEntries([...histogram].sort(([left], [right]) => (left < right ? -1 : left > right ? 1 : 0)));
  }

  function documentOutput({ language, text }) {
    const network = parse(text, language);
    const nodes = namedNodes(network);
    const output = {
      language,
      roundTrip: network.reconstructText() === text,
      clean: network.verifyFullMatch().isClean(),
      namedKinds: namedKinds(network),
      namedNodes: nodes,
    };
    try {
      const reloaded = LinkNetwork.fromLino(network.toLino());
      const again = namedNodes(reloaded);
      output.reload = { namedNodes: again.length, sameNamedNodes: sameList(again, nodes), roundTrip: reloaded.reconstructText() === text };
    } catch (error) {
      output.reload = { error: message(error) };
    }
    return output;
  }

  function ruleSetOutput(text) {
    let set;
    try {
      set = TranslationRuleSet.fromLino(text);
    } catch (error) {
      return { error: message(error) };
    }
    const output = { rules: set.rules.map((rule) => rule.name) };
    try {
      output.reloadRules = TranslationRuleSet.fromLino(set.toLino()).rules.map((rule) => rule.name);
    } catch (error) {
      output.reloadRules = { error: message(error) };
    }
    return output;
  }

  function seedNetworkOutput(text) {
    try {
      const links = [...LinkNetwork.fromLino(text).links()].map((link) => link.metadata());
      return {
        translationRules: links.filter(({ term }) => term === 'translation-rule').length,
        refusalRows: links.filter(({ language }) => language === refusalLanguage).length,
        noformRows: links.filter(({ language }) => language === noformLanguage).length,
      };
    } catch (error) {
      return { error: message(error) };
    }
  }

  // One match as `kind start..end {capture=kind start..end, …}`; the root capture `match` is the match itself.
  function matchLabel(network, match) {
    const captures = [...match.captures]
      .filter(([name]) => name !== 'match')
      .map(([name, id]) => `${name}=${nodeLabel(network.link(id).metadata())}`)
      .sort();
    return `${nodeLabel(network.link(match.linkId).metadata())} {${captures.join(', ')}}`;
  }

  // The match of every projection rule over the documents of its source languages. The package
  // query language needs a root capture, so a query it rejects as written runs as `(root) @match`.
  function queryOutputs(rules, documents) {
    const networks = new Map();
    const outputs = {};
    for (const rule of rules) {
      const output = { asWritten: null, adapted: null, documents: {} };
      let query = null;
      try {
        query = LinkQuery.fromSexpression(rule.sexpression);
      } catch (error) {
        output.asWritten = message(error);
        output.adapted = `(${rule.rootKind}) @match`;
        try {
          query = LinkQuery.fromSexpression(output.adapted);
        } catch (adaptedError) {
          output.error = message(adaptedError);
        }
      }
      for (const document of documents.filter(({ language }) => rule.sources.includes(language))) {
        if (!query) continue;
        if (!networks.has(document.name)) networks.set(document.name, parse(document.text, document.language));
        const network = networks.get(document.name);
        output.documents[document.name] = network.find(query).map((match) => matchLabel(network, match)).sort();
      }
      outputs[rule.name] = output;
    }
    return outputs;
  }

  // formal_ai::agentic_coding::link_edit_rules::parse_rule_shapes.
  function ruleShapes(text) {
    const shapes = [];
    for (const line of text.split('\n')) {
      const trimmed = line.trim();
      if (trimmed.startsWith('rule ')) {
        shapes.push({ rule: trimmed.slice(5).trim(), nodeKinds: [], anchorKind: null, listKind: null, elementKind: null });
        continue;
      }
      const shape = shapes.at(-1);
      if (!shape) continue;
      const value = (prefix) => (trimmed.startsWith(prefix) ? trimmed.slice(prefix.length).trim() : null);
      if (value('node_kind ') !== null) shape.nodeKinds.push(value('node_kind '));
      else if (value('anchor_kind ') !== null) shape.anchorKind = value('anchor_kind ');
      else if (value('list_kind ') !== null) shape.listKind = value('list_kind ');
      else if (value('element_kind ') !== null) shape.elementKind = value('element_kind ');
    }
    return shapes;
  }

  class LinkEditError extends Error {}
  const refuse = (text) => {
    throw new LinkEditError(text);
  };

  // The byte ranges of the syntax links of `kinds`, ordered and without duplicates.
  function spansOfKinds(network, kinds) {
    const seen = new Set();
    return syntaxLinks(network)
      .map((link) => link.metadata())
      .filter((metadata) => kinds.includes(metadata.term) && byteRange(metadata))
      .map((metadata) => ({ start: byteRange(metadata).start, end: byteRange(metadata).end }))
      .sort((left, right) => left.start - right.start || left.end - right.end)
      .filter(({ start, end }) => !seen.has(`${start}:${end}`) && seen.add(`${start}:${end}`));
  }

  // formal-ai's member insertion: after the last element of the first list that follows the anchor.
  function memberInsertion(network, bytes, shape, list, member) {
    const slice = ({ start, end }) => bytes.subarray(start, end).toString('utf8');
    const anchor = spansOfKinds(network, [shape.anchorKind ?? 'identifier']).find((range) => slice(range) === list) ??
      refuse(`link_edit:no_matching_link:insert_member:${list}`);
    const array = spansOfKinds(network, [shape.listKind ?? 'array_expression'])
      .filter((range) => range.start >= anchor.end && !slice({ start: anchor.end, end: range.start }).includes(';'))
      .sort((left, right) => left.start - right.start)[0] ?? refuse(`link_edit:no_list_after:${list}`);
    const quoted = member.startsWith('"') ? member : `"${member}"`;
    const elements = spansOfKinds(network, [shape.elementKind ?? 'string_literal'])
      .filter((range) => range.start >= array.start && range.end <= array.end);
    if (elements.some((range) => slice(range) === quoted)) refuse(`link_edit:member_present:${list}:${member}`);
    if (elements.length === 0) {
      const open = slice(array).indexOf('[');
      const at = array.start + (open === -1 ? 0 : Buffer.byteLength(slice(array).slice(0, open + 1)));
      return { start: at, end: at, text: quoted };
    }
    const last = elements.at(-1);
    const gap = elements.length > 1 ? slice({ start: elements.at(-2).end, end: last.start }) : '';
    return { start: last.end, end: last.end, text: `${gap || ', '}${quoted}` };
  }

  // The byte ranges the rule edits, as formal_ai::agentic_coding::apply_link_edit selects them.
  function selectEdits(network, bytes, shape, rule, linoLiteral) {
    const slice = ({ start, end }) => bytes.subarray(start, end).toString('utf8');
    if (rule.shape === 'insert_member') return [memberInsertion(network, bytes, shape, rule.list, rule.member)];
    if (!rule.old) refuse('link_edit:empty_pattern');
    const needle = Buffer.from(rule.old);
    const edits = [];
    if (rule.shape === 'rename_identifier') {
      for (const range of spansOfKinds(network, shape.nodeKinds)) {
        if (slice(range) === rule.old) edits.push({ ...range, text: rule.new });
      }
    } else if (linoLiteral) {
      const quoted = Buffer.from(`"${rule.old}"`);
      for (let at = bytes.indexOf(quoted); at !== -1; at = bytes.indexOf(quoted, at + quoted.length)) {
        edits.push({ start: at + 1, end: at + 1 + needle.length, text: rule.new });
      }
    } else {
      for (const range of spansOfKinds(network, shape.nodeKinds)) {
        const inside = bytes.subarray(range.start, range.end);
        for (let at = inside.indexOf(needle); at !== -1; at = inside.indexOf(needle, at + needle.length)) {
          edits.push({ start: range.start + at, end: range.start + at + needle.length, text: rule.new });
        }
      }
    }
    if (edits.length === 0) refuse(`link_edit:no_matching_link:${rule.shape}:${rule.old}`);
    return edits;
  }

  // The package has no byte-range edit, so each edit goes through the smallest syntax link that
  // contains it (strictly, for an insertion): its captured text is replaced by a ReplacementRule.
  function applyThroughLinks(network, bytes, edits) {
    const containers = syntaxLinks(network)
      .map((link) => ({ id: link.id(), term: link.metadata().term, range: byteRange(link.metadata()) }))
      .filter(({ range }) => range);
    const containing = (edit) => containers
      .filter(({ range }) => (edit.start === edit.end
        ? range.start < edit.start && edit.end < range.end
        : range.start <= edit.start && edit.end <= range.end))
      .sort((left, right) => (left.range.end - left.range.start) - (right.range.end - right.range.start))[0];
    const groups = [];
    for (const edit of edits) {
      const container = containing(edit) ?? refuse(`link_edit:edit_rejected:${edit.start}:${edit.end}`);
      const group = groups.find(({ range }) => range.start <= container.range.start && container.range.end <= range.end);
      if (group) {
        group.edits.push(edit);
        continue;
      }
      const inner = groups.filter(({ range }) => container.range.start <= range.start && range.end <= container.range.end);
      for (const nested of inner) groups.splice(groups.indexOf(nested), 1);
      groups.push({ ...container, edits: [edit, ...inner.flatMap(({ edits: nested }) => nested)] });
    }
    groups.sort((left, right) => right.range.start - left.range.start);
    for (const group of groups) {
      let text = bytes.subarray(group.range.start, group.range.end);
      for (const edit of [...group.edits].sort((left, right) => right.start - left.start)) {
        text = Buffer.concat([
          text.subarray(0, edit.start - group.range.start),
          Buffer.from(edit.text),
          text.subarray(edit.end - group.range.start),
        ]);
      }
      const match = network.find(LinkQuery.byType('Syntax').withTerm(group.term))
        .find(({ linkId }) => linkId.value === group.id.value);
      const report = match ? network.replace([match], ReplacementRule.capturedText('match', text.toString('utf8'))) : null;
      if (!report || report.replacements.length === 0) {
        const first = group.edits[0];
        refuse(`link_edit:edit_rejected:${first.start}:${first.end}`);
      }
    }
  }

  // formal_ai::agentic_coding::apply_link_edit through the package's JavaScript API.
  function applyLinkEdit(source, language, rule, shapes) {
    const shape = shapes.find(({ rule: name }) => name === rule.shape) ?? refuse(`link_edit:rule_undeclared:${rule.shape}`);
    const network = parse(source, language);
    const roundTripBefore = network.reconstructText() === source;
    const linoLiteral = language.toLowerCase() === 'lino' && rule.shape === 'replace_literal';
    if (!linoLiteral && syntaxLinks(network).length === 0) refuse(`link_edit:not_parsed:${language}`);
    const bytes = Buffer.from(network.reconstructText());
    const selected = selectEdits(network, bytes, shape, rule, linoLiteral)
      .sort((left, right) => right.start - left.start)
      .filter((edit, index, all) => index === 0 || all[index - 1].start !== edit.start);
    applyThroughLinks(network, bytes, selected);
    return {
      text: network.reconstructText(),
      edits: selected.length,
      roundTripBefore,
      cleanAfter: network.verifyFullMatch().isClean(),
    };
  }

  function linkEditOutputs(cases, rulesText) {
    const shapes = ruleShapes(rulesText);
    const outputs = {};
    for (const { name, language, source, steps } of cases) {
      let current = source;
      outputs[name] = {
        steps: steps.map(({ rule }) => {
          try {
            const edited = applyLinkEdit(current, language, rule, shapes);
            current = edited.text;
            const reparsed = parse(edited.text, language);
            return {
              outcome: 'edited',
              source: edited.text,
              edits: edited.edits,
              roundTripBefore: edited.roundTripBefore,
              cleanAfter: edited.cleanAfter,
              reparsed: { roundTrip: reparsed.reconstructText() === edited.text, namedNodes: namedNodes(reparsed) },
            };
          } catch (error) {
            if (!(error instanceof LinkEditError)) return { outcome: 'error', error: message(error) };
            return { outcome: 'refused', error: error.message };
          }
        }),
      };
    }
    return outputs;
  }

  // formal_ai::rust_projection::projection_from: the rule set and the seed network loaded through the package.
  function loadProjection(text) {
    const ruleSet = TranslationRuleSet.fromLino(text);
    const network = LinkNetwork.fromLino(text);
    const links = [...network.links()];
    const root = links.find((link) => link.metadata().linkType === 'Semantic' && link.metadata().term === 'translation-rule-set');
    if (!root) throw new Error('seed has no translation-rule-set root');
    const childOf = (parent) => (link) => link.references()[0]?.value === parent.id().value;
    const rows = links.filter(childOf(root));
    const ruleLinks = rows.filter((link) => link.metadata().term === 'translation-rule')
      .sort((left, right) => left.id().value - right.id().value);
    let rootKinds = ruleLinks.map((rule) => {
      const definition = links.filter(childOf(rule)).find((link) => link.metadata().term === 'translation-rule-match')?.metadata().definition;
      try {
        return /^\(\s*([^\s()]+)/u.exec(JSON.parse(definition ?? '{}').sexpression ?? '')?.[1] ?? null;
      } catch {
        return null;
      }
    });
    if (rootKinds.length !== ruleSet.rules.length) rootKinds = ruleSet.rules.map(() => null);
    const table = () => new Map();
    const add = (map, kind, target) => map.set(kind, [...(map.get(kind) ?? []), target]);
    const ruled = table();
    ruleSet.rules.forEach((rule, index) => {
      if (rootKinds[index]) for (const { language } of rule.templates) add(ruled, rootKinds[index], language);
    });
    const refusedKinds = table();
    const noformKinds = table();
    for (const link of rows) {
      const { language, term, definition } = link.metadata();
      if (!term || !definition) continue;
      if (language === refusalLanguage) add(refusedKinds, term, definition);
      if (language === noformLanguage) add(noformKinds, term, definition);
    }
    const declared = (map, kind, target) => (map.get(kind) ?? []).some((value) => value === target || value === 'any');
    return {
      ruleSet,
      ruleCount: ruleSet.rules.length,
      refusalCount: [...refusedKinds.values()].reduce((sum, targets) => sum + targets.length, 0),
      noformCount: [...noformKinds.values()].reduce((sum, targets) => sum + targets.length, 0),
      isRuled: (kind, target) => (ruled.get(kind) ?? []).includes(target),
      isRefused: (kind, target) => declared(refusedKinds, kind, target),
      isNoform: (kind, target) => declared(noformKinds, kind, target),
    };
  }

  // formal_ai::rust_projection::project through the package's JavaScript API.
  function project(from, target, source, labels, projection) {
    if (!labels.includes(from) || !labels.includes(target) || from === target) {
      return { outcome: 'refused', refusals: [from] };
    }
    if (projection.error) return { outcome: 'error', error: `the grammar projection seed does not load: ${projection.error}` };
    const network = parse(source, from);
    const named = syntaxLinks(network).filter((link) => link.metadata().named);
    const kinds = [...new Set(named.map((link) => link.metadata().term).filter(Boolean))].sort();
    const uncovered = kinds.filter((kind) => !projection.value.isRuled(kind, target) &&
      !projection.value.isRefused(kind, target) && !projection.value.isNoform(kind, target));
    if (uncovered.length > 0) return { outcome: 'refused', refusals: uncovered };
    const referenced = new Set(named.flatMap((link) => link.references().map((id) => id.value)));
    const root = named.find((link) => !referenced.has(link.id().value));
    try {
      return { outcome: 'rendered', source: projection.value.ruleSet.render(target, network, root?.id()) };
    } catch (error) {
      return { outcome: 'error', error: message(error) };
    }
  }

  function loadedProjection(text) {
    try {
      return { value: loadProjection(text) };
    } catch (error) {
      return { error: message(error) };
    }
  }

  function projectionOutputs(cases, labels, seedText) {
    const projection = loadedProjection(seedText);
    return Object.fromEntries(cases.map(({ name, from, target, source }) => [name, project(from, target, source, labels, projection)]));
  }

  /** Whether a projection output is what formal-ai's tests expect of it. */
  function projectionMeetsExpectation(output, expect) {
    if (output?.outcome !== expect.outcome) return false;
    if (expect.outcome === 'rendered') return output.source === expect.source;
    return output.refusals.includes(expect.construct) && (expect.refusals === undefined || output.refusals.length === expect.refusals);
  }

  /** Whether a link-edit step output is what formal-ai's tests expect of it. */
  function linkEditMeetsExpectation(output, expect) {
    if (output?.outcome !== expect.outcome) return false;
    if (expect.outcome === 'refused') return output.error === expect.error;
    return output.edits === expect.edits && output.source.includes(expect.contains);
  }

  // A probe check as the Rust probe writes it: the first failures, or the detail when none.
  function check(name, failures, detail) {
    return { name, holds: failures.length === 0, detail: failures.length === 0 ? detail : failures.slice(0, 8) };
  }

  function languageFailures(documents) {
    return documents.flatMap(({ name, language, text }) => {
      const foreign = syntaxLinks(parse(text, language)).filter((link) => link.metadata().language !== language).length;
      return foreign > 0 ? [`${name}: ${foreign} syntax links carry another language`] : [];
    });
  }

  function kindsOf(documents, label) {
    return new Set(documents
      .filter(({ language, name }) => language === label && name.startsWith('template_'))
      .flatMap(({ text }) => Object.keys(namedKinds(parse(text, label)))));
  }

  // formal-ai's projection kinds checked against the classification its seed declares (as the Rust probe does).
  function classificationFailures(projection, inputs) {
    const { ruled, refused, noform } = inputs.classification;
    const ruledKinds = new Set(ruled.map(({ kind }) => kind));
    const failures = [];
    for (const { kind, target } of ruled) {
      if (!projection.isRuled(kind, target)) failures.push(`${kind} is not ruled for ${target}`);
    }
    for (const { kind, target: declared } of refused) {
      for (const target of declared === 'any' ? inputs.projectionLabels : [declared]) {
        if (!projection.isRefused(kind, target)) failures.push(`${kind} is not refused for ${target}`);
        if (!ruledKinds.has(kind) && projection.isRuled(kind, target)) failures.push(`the refused-only ${kind} is ruled for ${target}`);
      }
    }
    for (const { kind, target } of noform) {
      if (!projection.isNoform(kind, target)) failures.push(`${kind} is not no-form for ${target}`);
      if (projection.isRuled(kind, target)) failures.push(`the no-form ${kind} is ruled for ${target}`);
    }
    const counts = [
      ['rules', projection.ruleCount, inputs.rules.length],
      ['refusals', projection.refusalCount, refused.length],
      ['no-form rows', projection.noformCount, noform.length],
    ];
    for (const [name, actual, expected] of counts) {
      if (actual !== expected) failures.push(`${actual} ${name}, the seed declares ${expected}`);
    }
    return failures;
  }

  function seedClassificationFailures(inputs) {
    const projection = loadedProjection(inputs.ruleSetText);
    return projection.error ? [`the seed does not load: ${projection.error}`] : classificationFailures(projection.value, inputs);
  }

  function projectionFailures(cases, labels, seedText) {
    const projected = projectionOutputs(cases, labels, seedText);
    return cases.filter(({ name, expect }) => !projectionMeetsExpectation(projected[name], expect))
      .map(({ name }) => `${name}: ${JSON.stringify(projected[name])}`);
  }

  function linkEditFailures(cases, rulesText) {
    const edited = linkEditOutputs(cases, rulesText);
    return cases.flatMap(({ name, steps }) => steps.flatMap(({ expect }, index) => {
      const output = edited[name].steps[index];
      return linkEditMeetsExpectation(output, expect) ? [] : [`${name}: ${JSON.stringify(output ?? null).slice(0, 400)}`];
    }));
  }

  /** The distinctionsPreserved checks, named and computed as the Rust probe computes them. */
  function distinctionChecks(inputs) {
    const kinds = Object.fromEntries(['rust', 'javascript', 'typescript'].map((label) => [label, kindsOf(inputs.documents, label)]));
    const kindFailures = [];
    const kindDetail = {};
    for (const [left, right] of [['typescript', 'javascript'], ['rust', 'javascript'], ['rust', 'typescript']]) {
      const only = [...kinds[left]].filter((kind) => !kinds[right].has(kind));
      if (only.length === 0) kindFailures.push(`no ${left} kind is missing from ${right}`);
      kindDetail[`${left} without ${right}`] = only.length;
    }
    const { classification } = inputs;
    return [
      check("every syntax link keeps its document's language label", languageFailures(inputs.documents),
        { documents: inputs.documents.length }),
      check('rust, javascript and typescript keep kinds of their own', kindFailures, kindDetail),
      check('ruled, refused and no-form kinds stay distinct per target', seedClassificationFailures(inputs), {
        ruled: classification.ruled.length,
        refused: classification.refused.length,
        noform: classification.noform.length,
      }),
      check('projections render ruled constructs and refuse the rest by name',
        projectionFailures(inputs.projections, inputs.projectionLabels, inputs.ruleSetText), { cases: inputs.projections.length }),
      check('link edits stay distinct from their refusals', linkEditFailures(inputs.linkEdits, inputs.linkEditRulesText),
        { cases: inputs.linkEdits.length }),
    ];
  }

  /** Every output the Rust probe also writes, from the shared inputs. */
  function outputs(inputs) {
    return {
      documents: Object.fromEntries(inputs.documents.map((document) => [document.name, documentOutput(document)])),
      ruleSet: ruleSetOutput(inputs.ruleSetText),
      seedNetwork: seedNetworkOutput(inputs.ruleSetText),
      queries: queryOutputs(inputs.rules, inputs.documents),
      linkEdits: linkEditOutputs(inputs.linkEdits, inputs.linkEditRulesText),
      projections: projectionOutputs(inputs.projections, inputs.projectionLabels, inputs.ruleSetText),
    };
  }

  return {
    parse,
    namedNodes,
    namedKinds,
    syntaxLinks,
    documentOutput,
    ruleSetOutput,
    seedNetworkOutput,
    queryOutputs,
    ruleShapes,
    linkEditOutputs,
    loadedProjection,
    projectionOutputs,
    projectionMeetsExpectation,
    linkEditMeetsExpectation,
    seedClassificationFailures,
    projectionFailures,
    linkEditFailures,
    distinctionChecks,
    outputs,
  };
}

function canonical(value) {
  if (Array.isArray(value)) return `[${value.map(canonical).join(',')}]`;
  if (value && typeof value === 'object') {
    return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(',')}}`;
  }
  return JSON.stringify(value ?? null);
}

const excerpt = (value) => {
  const text = canonical(value);
  return text.length > 240 ? `${text.slice(0, 240)}…` : text;
};

// One check over every entry of a section: the projected values of both runtimes must be equal.
function compareSection(name, javascript, rust, project) {
  const keys = [...new Set([...Object.keys(javascript ?? {}), ...Object.keys(rust ?? {})])].sort();
  const differing = keys.filter((key) => canonical(project(javascript?.[key])) !== canonical(project(rust?.[key])));
  if (!javascript || !rust) {
    return { name, holds: false, detail: `the ${javascript ? 'rust' : 'javascript'} run wrote no outputs` };
  }
  if (differing.length === 0) return { name, holds: true, detail: { compared: keys.length } };
  const [first] = differing;
  return {
    name,
    holds: false,
    detail: `${differing.length} of ${keys.length} differ; first ${first}: javascript ${excerpt(project(javascript[first]))}` +
      ` rust ${excerpt(project(rust[first]))}`,
  };
}

const matchRoots = (documents) => (documents
  ? Object.fromEntries(Object.entries(documents).map(([name, labels]) => [name, [...new Set(labels.map((label) => label.split(' {')[0]))]]))
  : null);

/**
 * The sharedConceptsReused checks: the JavaScript consumer and formal-ai's Rust
 * functions, run on the same inputs, produced the same links and projections.
 */
export function compareRuntimeOutputs(javascript, rust) {
  const pick = (section) => [javascript?.[section], rust?.[section]];
  // A single-valued section, compared whole; a runtime without outputs stays missing.
  const whole = (section) => [javascript && { value: javascript[section] }, rust && { value: rust[section] }];
  return [
    compareSection('both runtimes parse the same named syntax trees', ...pick('documents'), (document) => document?.namedNodes),
    compareSection('both runtimes round-trip and fully match the same documents', ...pick('documents'),
      (document) => document && { roundTrip: document.roundTrip, clean: document.clean }),
    compareSection('both runtimes count the same named kinds', ...pick('documents'), (document) => document?.namedKinds),
    compareSection('both runtimes keep the same links through to_lino and from_lino', ...pick('documents'), (document) => document?.reload),
    compareSection('both runtimes load the same translation rule set', ...whole('ruleSet'), (value) => value),
    compareSection('both runtimes load the same seed network', ...whole('seedNetwork'), (value) => value),
    compareSection('both runtimes accept the same match s-expressions as written', ...pick('queries'),
      (query) => query && query.asWritten === null),
    compareSection('both runtimes match the same root links', ...pick('queries'), (query) => matchRoots(query?.documents)),
    compareSection('both runtimes bind the same captures', ...pick('queries'), (query) => query?.documents),
    compareSection('both runtimes apply the same link edits', ...pick('linkEdits'), (edit) => edit),
    compareSection('both runtimes project the same sources', ...pick('projections'), (projection) => projection),
  ];
}
