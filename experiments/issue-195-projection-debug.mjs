// Renders one formal-ai projection case through the JavaScript package and
// prints the rules that claim each link, to compare with the Rust renderer.
// Usage: node experiments/issue-195-projection-debug.mjs <from> <to> <source>
import { readFileSync } from 'node:fs';
import { LinkNetwork, ParseConfiguration, TranslationRuleSet } from '../js/src/index.js';

const [from, to, source] = process.argv.slice(2);
const text = readFileSync(new URL('../parity/fixtures/formal-ai-regressions/data/seed/grammar-projection-rules.lino', import.meta.url), 'utf8');
const ruleSet = TranslationRuleSet.fromLino(text);
const network = LinkNetwork.parse(source, from, ParseConfiguration.default());
for (const rule of ruleSet.rules) {
  const matches = network.queryMatches(rule.query);
  if (matches.length === 0) continue;
  const templates = rule.templates.map((template) => template.language).join(',');
  for (const match of matches) {
    const link = network.link(match.linkId);
    console.log(rule.name ?? '?', `[${templates}]`, link.metadata().term, JSON.stringify([...match.captures].map(([name, id]) => `${name}=${network.link(id).metadata().term}`)));
  }
}
const named = [...network.links()].filter((link) => link.metadata().linkType === 'Syntax' && link.metadata().named);
const referenced = new Set(named.flatMap((link) => link.references().map((id) => id.value)));
const root = named.find((link) => !referenced.has(link.id().value));
console.log('root', root?.metadata().term);
console.log(JSON.stringify(ruleSet.render(to, network, root?.id())));
