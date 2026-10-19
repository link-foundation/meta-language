// Authoring import closure: all supporting definitions become one executable
// native grammar before packaging. File acquisition is injected; installed
// parsing needs no file system resolver or foreign engine.
import { createHash } from 'node:crypto';
import { Grammar } from '../src/grammar.js';
import { grammarImporter } from '../src/grammar-interchange.js';
import { parseGrammarLinks, renderGrammarLinks } from '../src/grammar-links.js';
import { grammarDeclarations } from '../src/grammar-feature-forms.js';

const digest = (source) => createHash('sha256').update(source).digest('hex');

export function importGrammarSourceBundle(entry, sources) {
  const records = new Map();
  const active = [];
  function read(id) {
    if (records.has(id)) return records.get(id);
    const source = sources instanceof Map ? sources.get(id) : Object.hasOwn(sources, id) ? sources[id] : undefined;
    if (!source || typeof source.source !== 'string') throw new Error(`supporting grammar ${id} is not registered in the source bundle`);
    const importer = source.format === 'links' ? parseGrammarLinks : grammarImporter(source.format);
    if (!importer) throw new Error(`supporting grammar ${id} has unknown format ${source.format}`);
    const grammar = importer(source.source);
    const record = { id, format: source.format, sha256: digest(source.source), grammar };
    records.set(id, record);
    return record;
  }

  function resolve(id) {
    if (active.includes(id)) throw new Error(`supporting grammar import cycle: ${[...active, id].join(' -> ')}`);
    active.push(id);
    const record = read(id);
    const local = grammarDeclarations(record.grammar);
    const rules = new Map();
    const declarations = { modes: [], extras: [], conflicts: [], precedences: [], macros: new Map(), scanners: [] };
    const include = (resolved) => {
      for (const [name, rule] of resolved.rules) rules.set(name, rule);
      for (const name of ['modes', 'extras', 'conflicts', 'precedences', 'scanners']) declarations[name].push(...resolved.declarations[name]);
      for (const [name, macro] of resolved.declarations.macros) declarations.macros.set(name, macro);
    };
    for (const imported of local.imports) include(resolve(imported));
    for (const rule of record.grammar.rules.values()) {
      rules.set(rule.name, { ...rule, sourceNames: [...(rule.sourceNames ?? []), { source: id, name: rule.name }] });
    }
    for (const name of ['modes', 'extras', 'conflicts', 'precedences', 'scanners']) declarations[name].push(...local[name]);
    for (const macro of local.macros) declarations.macros.set(macro.name, macro);
    active.pop();
    return { rules, declarations };
  }
  const { rules, declarations } = resolve(entry);
  const root = read(entry).grammar;
  const grammar = new Grammar(root.start, rules, root.sourceFormat, {
    ...declarations, macros: [...declarations.macros.values()], matching: root.declarations.matching,
    settling: root.declarations.settling,
  });
  for (const [name, sourceRule] of rules) {
    if (sourceRule.doc !== undefined) grammar.rules.set(name, Object.freeze({ ...grammar.rule(name), doc: sourceRule.doc }));
  }
  // Linked grammar serialization is authoritative: replay is built only from
  // these links, not a cached original source or an injected import callback.
  const links = renderGrammarLinks(grammar);
  const provenance = [...records.values()].map(({ id, format, sha256, grammar: sourceGrammar }) => ({
    id, format, sha256, imports: [...grammarDeclarations(sourceGrammar).imports],
    originalNames: sourceGrammar.ruleNames(),
  }));
  return { grammar: parseGrammarLinks(links), links, provenance };
}
