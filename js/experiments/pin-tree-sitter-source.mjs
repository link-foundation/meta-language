// Pins a tree-sitter grammar checkout for the import pipeline: writes the
// gzipped src/grammar.json and test corpus (JSON of each corpus file) to
// parity/grammars/sources/ and prints their sha256 (taken before
// compression) for the parity/grammars/sources.json entry. A checkout with
// several grammars over one corpus (tree-sitter-typescript's typescript and
// tsx) names their directories; each DIRECTORY/src/grammar.json is written
// as NAME.DIRECTORY.grammar.json.gz.
//
//   node js/experiments/pin-tree-sitter-source.mjs CHECKOUT REPOSITORY REVISION NAME [DIRECTORY...]
import { createHash } from 'node:crypto';
import { copyFileSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { gzipSync } from 'node:zlib';

const [checkout, repository, revision, name, ...directories] = process.argv.slice(2);
const target = new URL('../../parity/grammars/sources/', import.meta.url).pathname;
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const grammars = directories.length === 0
  ? [[name, readFileSync(join(checkout, 'src/grammar.json'))]]
  : directories.map((directory) => [`${name}.${directory}`, readFileSync(join(checkout, directory, 'src/grammar.json'))]);
const corpusDirectory = join(checkout, 'test/corpus');
const files = Object.fromEntries(readdirSync(corpusDirectory).filter((file) => file.endsWith('.txt')).sort()
  .map((file) => [file, readFileSync(join(corpusDirectory, file), 'utf8')]));
const corpus = Buffer.from(`${JSON.stringify({ repository, revision, path: 'test/corpus', files }, null, 1)}\n`);
for (const [file, grammar] of grammars) writeFileSync(join(target, `${file}.grammar.json.gz`), gzipSync(grammar, { level: 9 }));
writeFileSync(join(target, `${name}.corpus.json.gz`), gzipSync(corpus, { level: 9 }));
copyFileSync(join(checkout, 'LICENSE'), join(target, `${name}.LICENSE`));
const grammarHashes = Object.fromEntries(grammars.map(([file, grammar]) => [file, sha256(grammar)]));
console.log(JSON.stringify({ grammars: grammarHashes, corpus: sha256(corpus), files: Object.keys(files) }, null, 1));
