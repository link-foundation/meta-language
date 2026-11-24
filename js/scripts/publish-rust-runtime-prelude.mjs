import {mkdirSync, writeFileSync} from 'node:fs';
import {tokenize} from '../src/translation/lexer.js';

const maximumLines = 900;

export function publishRustRuntimePrelude(directory, stem, source) {
  if (!/^[a-z][a-z0-9-]*$/u.test(stem)) throw new TypeError('invalid Rust runtime stem');
  const lines = source.match(/[^\n]*\n|[^\n]+$/gu) ?? [];
  const {tokens, comments} = tokenize(source, 'Rust');
  const spans = [...tokens, ...comments].sort((left, right) => left.start - right.start);
  // A template is included as text. Keep its exact bytes and never bisect a
  // lexical token, even when the enclosing runtime module spans several files.
  const parts = [];
  let offset = 0, start = 0, tokenIndex = 0, boundary = 0, declarationBoundary = 0;
  for (const [index, line] of lines.entries()) {
    offset += line.length;
    while (spans[tokenIndex]?.end <= offset) tokenIndex += 1;
    if (index + 1 - start > maximumLines) {
      if (boundary <= start) throw new RangeError('Rust runtime token exceeds the source file limit');
      const at = declarationBoundary > start + Math.floor(maximumLines / 2) ? declarationBoundary : boundary;
      parts.push(lines.slice(start, at).join(''));
      start = at;
    }
    if (spans[tokenIndex]?.start >= offset || tokenIndex === spans.length) {
      boundary = index + 1;
      if (!line.trim() && /^(?:#\[|pub (?:const fn|fn|mod|enum|struct) )/u.test(lines[boundary] ?? '')) {
        declarationBoundary = index;
        while (declarationBoundary > start && !lines[declarationBoundary - 1].trim()) declarationBoundary -= 1;
      }
    }
  }
  parts.push(lines.slice(start).join(''));
  if (parts.join('') !== source || parts.length > 128) {
    throw new RangeError('Rust runtime source needs a bounded lossless partition');
  }
  const paths = parts.map((_, index) => `${stem}/part_${index + 1}.rs`);
  mkdirSync(new URL(`${stem}/`, directory), {recursive: true});
  for (const [index, path] of paths.entries()) writeFileSync(new URL(path, directory), parts[index]);
  writeFileSync(new URL(`${stem}.rs`, directory),
    '// Generated source prelude. The emitter inserts these original bytes in order.\n'
    + 'concat!(\n' + paths.map(path => `    include_str!("${path}"),\n`).join('') + ')\n');
}
