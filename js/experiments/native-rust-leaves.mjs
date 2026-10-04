// Prints the leaves of the native Rust tree of a source (SOURCE, or a sample
// with doc comments, a nested block comment, a raw string, a lifetime, an
// escape and an attribute) as [kind, text] pairs.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL('../../parity/grammars/native/rust.lino', import.meta.url), 'utf8')));
const source = process.env.SOURCE ?? '/// d\n#[a] fn f<\'a>(x: &\'a str) /* b /* c */ */ { r#"s"#; "\\n"; } // e\n';
const leaves = (tree) => (tree.type === 'node' ? tree.children.flatMap(leaves) : [tree]);
const outcome = parser.parseTree(source);
console.log(outcome.ok, JSON.stringify(outcome.ambiguities));
console.log(JSON.stringify(leaves(outcome.tree).map(({ kind, text }) => [kind ?? null, text])));
