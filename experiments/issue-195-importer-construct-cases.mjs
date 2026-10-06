// Tries candidate construct-coverage cases for the shared importer corpus:
// import, membership through the interpreter, same-format emit, re-import.
import { isDeepStrictEqual } from 'node:util';
import * as ml from '../js/src/index.js';
import { renderGrammarRule } from '../js/tests/support/render-grammar-expression.js';

export const candidates = [
  {
    format: 'abnf',
    source: 'assignment = %i"let" 1*SP name *SP %s"=" *SP value [ %x3B ]\nname = ALPHA *7( ALPHA / DIGIT / %s"_" )\nvalue = 1*3DIGIT / DQUOTE *( %x20-21 / %x23-7E ) DQUOTE\n',
    start: 'assignment',
    rules: ['assignment', 'name', 'value'],
    accepts: ['let x = 12;', 'LET ab_1="hi"', 'Let  q=7'],
    rejects: ['let 1x = 2;', 'let x = 1234;', 'letx = 1', 'let abcdefghi = 1'],
  },
  {
    format: 'bnf',
    source: '<assignment> ::= "let " <name> <spaces> "=" <spaces> <value> <end>\n<name> ::= <letter> <name> | <letter>\n<spaces> ::= " " <spaces> | ""\n<value> ::= <digit> <value> | <digit> | \'"\' <letters> \'"\'\n<letters> ::= <letter> <letters> | ""\n<end> ::= ";" | ""\n<letter> ::= "a" | "b" | "x" | "q"\n<digit> ::= "1" | "2" | "7"\n',
    start: 'assignment',
    rules: ['assignment', 'name', 'spaces', 'value', 'letters', 'end', 'letter', 'digit'],
    accepts: ['let x = 12;', 'let ab="ab"', 'let q=7'],
    rejects: ['let 1x = 2;', 'letx = 1', 'let x = "a'],
  },
  {
    format: 'ebnf',
    source: 'assignment = "let", " ", name, { " " }, "=", { " " }, value, [ ";" ] ;\nname = letter, ( letter | digit | "_" )* ;\nvalue = digit, [ digit, [ digit ] ] | \'"\', { letter }, \'"\' ;\nletter = "a" | "b" | "x" | "q" ;\ndigit = "1" | "2" | "7" ;\n',
    start: 'assignment',
    rules: ['assignment', 'name', 'value', 'letter', 'digit'],
    accepts: ['let x = 12;', 'let ab_1="ab"', 'let q=777'],
    rejects: ['let 1x = 2;', 'letx = 1', 'let x = 1212'],
  },
  {
    format: 'pest',
    source: 'assignment = { ^"let" ~ " "+ ~ !keyword ~ name ~ " "* ~ "=" ~ " "* ~ value ~ ";"? }\nkeyword = { "let" ~ !alnum }\nname = @{ alpha ~ (alnum | "_"){0,7} }\nvalue = ${ digit{1,3} ~ !digit | "\\"" ~ (!"\\"" ~ ANY)* ~ "\\"" }\nalnum = _{ alpha | digit }\nalpha = _{ \'a\'..\'z\' | \'A\'..\'Z\' }\ndigit = _{ \'0\'..\'9\' }\n',
    start: 'assignment',
    rules: ['assignment', 'keyword', 'name', 'value', 'alnum', 'alpha', 'digit'],
    accepts: ['let x = 12;', 'LET ab_1="hi"', 'Let  q=7'],
    rejects: ['let let = 1', 'let x = 1234;', 'letx = 1', 'let x = "a'],
  },
  {
    format: 'tree-sitter-json',
    source: JSON.stringify({
      name: 'assignment',
      rules: {
        assignment: { type: 'SEQ', members: [
          { type: 'STRING', value: 'let' },
          { type: 'REPEAT1', content: { type: 'STRING', value: ' ' } },
          { type: 'FIELD', name: 'name', content: { type: 'SYMBOL', name: 'identifier' } },
          { type: 'REPEAT', content: { type: 'STRING', value: ' ' } },
          { type: 'STRING', value: '=' },
          { type: 'REPEAT', content: { type: 'STRING', value: ' ' } },
          { type: 'FIELD', name: 'value', content: { type: 'SYMBOL', name: 'value' } },
          { type: 'CHOICE', members: [{ type: 'STRING', value: ';' }, { type: 'BLANK' }] },
        ] },
        identifier: { type: 'TOKEN', content: { type: 'SEQ', members: [
          { type: 'PATTERN', value: '[a-z_]' },
          { type: 'REPEAT', content: { type: 'PATTERN', value: '[a-z0-9_]' } },
        ] } },
        value: { type: 'CHOICE', members: [
          { type: 'PREC', value: 1, content: { type: 'ALIAS', content: { type: 'SYMBOL', name: 'number' }, named: true, value: 'integer' } },
          { type: 'SYMBOL', name: 'string' },
        ] },
        number: { type: 'TOKEN', content: { type: 'REPEAT1', content: { type: 'PATTERN', value: '[0-9]' } } },
        string: { type: 'SEQ', members: [
          { type: 'STRING', value: '"' },
          { type: 'REPEAT', content: { type: 'PATTERN', value: '[^"\\\\]' } },
          { type: 'IMMEDIATE_TOKEN', content: { type: 'STRING', value: '"' } },
        ] },
      },
    }),
    start: 'assignment',
    rules: ['assignment', 'identifier', 'value', 'number', 'string'],
    accepts: ['let x = 12;', 'let ab_1="hi"', 'let  q=7'],
    rejects: ['let 1x = 2;', 'letx = 1', 'let x = "a'],
  },
];

const pairs = {
  abnf: [ml.importAbnf, ml.emitAbnf],
  bnf: [ml.importBnf, ml.emitBnf],
  ebnf: [ml.importEbnf, ml.emitEbnf],
  pest: [ml.importPest, ml.emitPest],
  'tree-sitter-json': [ml.importTreeSitterJson, ml.emitTreeSitterJson],
};
const accepts = (grammar, text) => { try { ml.parseWithGrammar(grammar, text); return true; } catch { return false; } };

if (import.meta.url === `file://${process.argv[1]}`) {
  for (const fixture of candidates) {
    const [importer, emitter] = pairs[fixture.format];
    console.log(`== ${fixture.format}`);
    let grammar;
    try { grammar = importer(fixture.source); } catch (error) { console.log('  IMPORT FAILED', error.message); continue; }
    console.log('  start', grammar.startRule()?.name, 'names', grammar.ruleNames().join(','), 'undefined', grammar.undefinedNonterminals());
    for (const name of fixture.rules) console.log(`  ${name} = ${renderGrammarRule(grammar.rule(name))}`);
    for (const text of fixture.accepts) if (!accepts(grammar, text)) console.log('  SHOULD ACCEPT', JSON.stringify(text));
    for (const text of fixture.rejects) if (accepts(grammar, text)) console.log('  SHOULD REJECT', JSON.stringify(text));
    let emitted;
    try { emitted = emitter(grammar); } catch (error) { console.log('  EMIT FAILED', error.message); continue; }
    console.log('  lossy', emitted.report.lossy);
    console.log(emitted.source.replace(/^/gm, '    | '));
    const again = importer(emitted.source);
    for (const name of fixture.rules) {
      if (!isDeepStrictEqual(again.rule(name), grammar.rule(name))) {
        console.log(`  REIMPORT DIFF ${name} = ${renderGrammarRule(again.rule(name))}`);
      }
    }
    for (const text of fixture.accepts) if (!accepts(again, text)) console.log('  REIMPORT SHOULD ACCEPT', JSON.stringify(text));
    for (const text of fixture.rejects) if (accepts(again, text)) console.log('  REIMPORT SHOULD REJECT', JSON.stringify(text));
  }
}
