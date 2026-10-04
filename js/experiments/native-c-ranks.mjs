// Prints the (matching longest) token ranks of the native C grammar for the
// kinds given on the command line.
import { readFileSync } from 'node:fs';
import { parseGrammarLinks } from '../src/index.js';
import { loadProgram } from '../src/grammar-runtime/load.js';

const program = loadProgram(parseGrammarLinks(readFileSync('../parity/grammars/native/c.lino', 'utf8')));
for (const kind of process.argv.slice(2)) console.log(kind, JSON.stringify(program.tokenRanks.kinds.get(kind)));
