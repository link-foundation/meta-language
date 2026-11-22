import {tokenize} from '../src/translation/lexer.js';

export function removeTrailingRustLayoutWhitespace(source) {
  const { tokens, comments } = tokenize(source, 'Rust');
  const protectedSpans = [...tokens, ...comments].sort((left, right) => left.start - right.start);
  let index = 0;
  return source.replace(/[ \t]+(?=\r?$)/gmu, (whitespace, position) => {
    while (index < protectedSpans.length && protectedSpans[index].end <= position) index += 1;
    return protectedSpans[index]?.start < position + whitespace.length ? whitespace : '';
  });
}
