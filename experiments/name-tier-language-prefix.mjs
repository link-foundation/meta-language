import { importPest, mergeGrammars, sharedRuleDecisions } from '../js/src/index.js';
const a = importPest(`document = { element* }
element = { "<" ~ name ~ attribute* ~ ">" ~ document ~ "</" ~ name ~ ">" }
attribute = { name ~ "=" ~ name }
name = @{ ASCII_ALPHA+ }`);
const make = (prefix) => importPest(`${prefix}Document = { ${prefix}Element+ ~ EOI }
${prefix}Element = { "<" ~ tag ~ ${prefix}Attribute* ~ "/>" | "<" ~ tag ~ ">" ~ ${prefix}Element* ~ "</" ~ tag ~ ">" }
${prefix}Attribute = { tag ~ ("=" ~ tag)? }
tag = @{ ASCII_ALPHA ~ ASCII_ALPHANUMERIC* }`);
for (const prefix of ['html', 'xml']) {
  const r = mergeGrammars([{ id: 'native', language: 'HTML', precedence: 0, grammar: a }, { id: 'v4', language: 'HTML', precedence: 1, grammar: make(prefix) }], { reconcile: true });
  console.log(prefix, JSON.stringify(sharedRuleDecisions(r.groups[0]).map(({ name, members, basis, kind }) => ({ name, members, basis, kind }))));
}
