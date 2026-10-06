// Prints the heap a 33 kB TypeScript parse under a 100000 memo-cell budget leaves
// (the child of js/tests/issue-195-parse-memory-budget.test.js). Pass the js/ directory.
import path from 'node:path';
import { pathToFileURL } from 'node:url';
const js = path.resolve(process.argv[2] ?? 'js');
const at = (file) => pathToFileURL(path.join(js, 'src', file)).href;
const { compileGrammar } = await import(at('grammar.js'));
const { parseGrammarLinks } = await import(at('grammar-links.js'));
const { nativeGrammarText } = await import(at('native-grammar-parser.js'));
const unit = (i) => `export const table${i}: Map<string, Array<number>> = new Map([["a", [1, 2, ${i}]]]);\n`;
const source = Array.from({ length: 400 }, (_, i) => unit(i)).join('');
const parser = compileGrammar(parseGrammarLinks(nativeGrammarText('native-typescript')));
const outcome = parser.parseTree(source, { errorRecovery: true, recovery: 'accept', memoryLimit: 100000 });
const v8 = await import("node:v8");
const space = (name) => v8.getHeapSpaceStatistics().filter((s) => s.space_name.startsWith(name)).reduce((sum, s) => sum + s.space_used_size, 0) / 2 ** 20;
console.log("old", space("old_space"), "new", space("new_space"), "all", process.memoryUsage().heapUsed / 2 ** 20);
console.log(JSON.stringify({ rejection: outcome.rejection, heapMiB: process.memoryUsage().heapUsed / 2 ** 20 }));
