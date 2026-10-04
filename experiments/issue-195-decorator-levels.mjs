// Probes every decorator level of the JavaScript runtime on a small input.
import {
  analyzeProgram, compileGrammar, DecoratorSet, emitGbnf, importGbnf, LinkNetwork, LinkQuery, LinkType,
  mergeGrammars, ParseConfiguration, ReplacementRule, translateNativeConstruct, TranslationRule, TranslationRuleSet,
} from '../js/src/index.js';

const set = (lino) => DecoratorSet.fromLino(lino);
const gbnf = 'root ::= item item\nitem ::= "a" | "b"\n';

const imported = importGbnf(gbnf, { decorators: set('(decorator rename (level importer) (when (name item)) (set name letter))') });
console.log('importer', imported.ruleNames(), imported.start);

const parser = compileGrammar(importGbnf(gbnf), {
  decorators: set('(decorator kinds (level executor) (when (kind item)) (set kind letter))'),
});
console.log('executor', JSON.stringify(parser.parse('ab')));

const recovering = compileGrammar(importGbnf(gbnf), { errorRecovery: true, recovery: 'accept', decorators: set('(decorator why (level recovery) (when (type missing)) (set kind absent))') });
console.log('recovery', JSON.stringify(recovering.parseTree('a').tree));

const merged = mergeGrammars([
  { id: 'one', language: 'demo', grammar: importGbnf(gbnf) },
  { id: 'two', language: 'demo', grammar: importGbnf(gbnf) },
], { decorators: set('(decorator basis (level merge-decision) (when (kind merged)) (set basis reviewed))') });
console.log('merge-decision', JSON.stringify(merged.groups[0].decisions));

console.log('concept-mapping', JSON.stringify(translateNativeConstruct('native-json', 'pair', 'native-json5')),
  JSON.stringify(translateNativeConstruct('native-json', 'pair', 'native-json5', { decorators: set('(decorator settle (level concept-mapping) (when (rule pair)) (set rules property))') })));

const program = analyzeProgram('const a = 1;\n', 'JavaScript', {}, { decorators: set('(decorator terms (level cst-to-ast) (when (term number)) (set term numeral))') });
console.log('cst-to-ast', JSON.stringify(program.sourceMappings.map(({ term, start, end }) => [term, start, end])));

const network = LinkNetwork.parse('const oldName = call(oldName);\n', 'JavaScript', ParseConfiguration.default());
const matches = network.find(LinkQuery.fromSexpression('(identifier) @target (#eq? @target "oldName")'));
network.replace(matches, ReplacementRule.capturedText('target', 'newName'), { decorators: set('(decorator suffix (level transformation) (replace new Name Title))') });
console.log('transformation', JSON.stringify(network.reconstructText()));

console.log('emitter', JSON.stringify(emitGbnf(importGbnf(gbnf), { decorators: set('(decorator spaced (level emitter) (replace line %3A%3A%3D %3A%3A%3D%20))') }).source));

const shell = new LinkNetwork();
const root = shell.insertSyntaxNode('Shell', 'command', [shell.insertSourceToken('Shell', 'ls')]);
const rules = new TranslationRuleSet('shell-to-js', [new TranslationRule('command', new LinkQuery({ linkType: LinkType.Syntax, language: 'Shell' }).withTerm('command'))
  .withReferenceCapture('body', 0).withTemplate('JavaScript', 'await $`{body}`;')]);
console.log('translation-rule', JSON.stringify(rules.render('JavaScript', shell, root, { decorators: set('(decorator bare (level translation-rule) (replace text await%20 %20))') })));
