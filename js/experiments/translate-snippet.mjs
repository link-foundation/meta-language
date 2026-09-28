// Prints the translation of a JavaScript snippet: node translate-snippet.mjs '<source>' [Rust|Lean|Rocq|JavaScript]
import { translateProgram } from '../src/program-translation.js';

const target = process.argv[3] ?? 'Rust';
const result = translateProgram(process.argv[2], 'JavaScript', target);
if (result.diagnostic) console.log(result.diagnostic);
// The Rust runtime prelude is long; show the program after it.
else if (target === 'Rust') console.log(result.code.slice(result.code.indexOf('\n}\n', result.code.indexOf('pub mod ml')) + 3));
else console.log(result.code);
