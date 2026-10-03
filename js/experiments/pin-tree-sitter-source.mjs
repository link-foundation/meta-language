// Pins a tree-sitter grammar checkout for the import pipeline: writes the
// gzipped src/grammar.json and test corpus (JSON of each corpus file) to
// parity/grammars/sources/ and prints their sha256 (taken before
// compression) for the parity/grammars/sources.json entry.
//
//   node js/experiments/pin-tree-sitter-source.mjs CHECKOUT REPOSITORY REVISION NAME
import { createHash } from 'node:crypto';
import { copyFileSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { gzipSync } from 'node:zlib';

const [checkout, repository, revision, name] = process.argv.slice(2);
const target = new URL('../../parity/grammars/sources/', import.meta.url).pathname;
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const grammar = readFileSync(join(checkout, 'src/grammar.json'));
const corpusDirectory = join(checkout, 'test/corpus');
const files = Object.fromEntries(readdirSync(corpusDirectory).filter((file) => file.endsWith('.txt')).sort()
  .map((file) => [file, readFileSync(join(corpusDirectory, file), 'utf8')]));
const corpus = Buffer.from(`${JSON.stringify({ repository, revision, path: 'test/corpus', files }, null, 1)}\n`);
writeFileSync(join(target, `${name}.grammar.json.gz`), gzipSync(grammar, { level: 9 }));
writeFileSync(join(target, `${name}.corpus.json.gz`), gzipSync(corpus, { level: 9 }));
copyFileSync(join(checkout, 'LICENSE'), join(target, `${name}.LICENSE`));
console.log(JSON.stringify({ grammar: sha256(grammar), corpus: sha256(corpus), files: Object.keys(files) }, null, 1));
