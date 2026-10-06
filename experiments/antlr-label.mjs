// Imports an ANTLR rule with `# Label` alternatives and prints the native grammar.
import { importAntlr, renderNativeGrammar } from '../js/src/index.js';
const source = process.argv[2] ?? "grammar E;\ne : e '+' e # Add\n  | INT    # Int // number\n  ;\nINT : [0-9]+ ;\n";
const imported = importAntlr(source);
console.log(renderNativeGrammar(imported.grammar ?? imported));
