// Translates meta-language's own modules between JavaScript, TypeScript and
// Rust through meta-language links. The source is parsed losslessly with its
// native grammar; a translation into the same language writes the links back
// byte for byte, and a translation into the other language translates each
// top-level item the portable core can express and carries every other item
// verbatim in a marked comment. Both kinds of item keep their source as
// provenance, so translating an unedited translation back restores the
// source byte for byte.
import { createHash } from 'node:crypto';

import { LinkNetwork } from './network.js';
import { parseProgrammingLanguage } from './programming-language-parser.js';
import { checkProgram } from './translation/check.js';
import { TranslationError } from './translation/diagnostics.js';
import { emitJavaScript } from './translation/emit-javascript.js';
import { emitRust } from './translation/emit-rust.js';
import { parseJavaScript } from './translation/javascript.js';
import { parseRust } from './translation/rust.js';

/** The languages self-translation reads and writes. */
export const SELF_TRANSLATION_LANGUAGES = Object.freeze(['JavaScript', 'TypeScript', 'Rust']);

const HEADER = '// meta-language:self-translation:v1 ';
const CARRIED = '// meta-language:carried ';
const TRANSLATED = '// meta-language:translated ';
const SOURCE_LINE = '// |';
const PRELUDE_BEGIN = '// meta-language:prelude begin';
const PRELUDE_END = '// meta-language:prelude end';
// Portable-core Rust keeps the source's parentheses and names.
const RUST_ALLOW = '#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]';
const ALIASES = new Map([
  ['javascript', 'JavaScript'], ['js', 'JavaScript'], ['mjs', 'JavaScript'],
  ['typescript', 'TypeScript'], ['ts', 'TypeScript'],
  ['rust', 'Rust'], ['rs', 'Rust'],
]);
const COMMENTS = new Set(['comment', 'line_comment', 'block_comment']);

/** Thrown for an unknown language or a source the links do not reproduce. */
export class SelfTranslationError extends Error {
  constructor(message) {
    super(message);
    this.name = 'SelfTranslationError';
  }
}

/** The self-translation language `language` names, or `null`. */
export function selfTranslationLanguage(language) {
  return ALIASES.get(String(language).toLowerCase()) ?? null;
}

function required(language) {
  const name = selfTranslationLanguage(language);
  if (!name) throw new SelfTranslationError(`self-translation reads and writes ${SELF_TRANSLATION_LANGUAGES.join(', ')}, not ${language}`);
  return name;
}

// The family a language's code is written in: TypeScript items are read and
// written as JavaScript.
const family = (language) => (language === 'Rust' ? 'Rust' : 'JavaScript');
const sha256 = (text) => createHash('sha256').update(Buffer.from(text, 'utf8')).digest('hex');

/**
 * Translates `source` from `sourceLanguage` to `targetLanguage`, each one of
 * {@link SELF_TRANSLATION_LANGUAGES} or an alias of one.
 *
 * The result is `{ sourceLanguage, targetLanguage, code, items }`. Each item
 * `{ term, start, end, status, reason }` is a top-level item of the source
 * (byte offsets) with its status: `kept` (same language), `translated`,
 * `restored` (provenance gave back its source), `comment`, `provenance` (a
 * self-translation header or prelude) or `carried`, with the reason it was
 * carried.
 */
export function selfTranslate(source, sourceLanguage, targetLanguage) {
  const from = required(sourceLanguage);
  const to = required(targetLanguage);
  const text = String(source);
  const reconstructed = LinkNetwork.parse(text, from).reconstructText();
  if (reconstructed !== text) {
    throw new SelfTranslationError(`the ${from} links of the source do not reproduce it`);
  }
  const bytes = Buffer.from(text, 'utf8');
  const items = topLevelItems(text, from, bytes);
  if (family(from) === family(to)) {
    return freeze(from, to, reconstructed, items.map(({ term, start, end }) => ({ term, start, end, status: 'kept', reason: null })));
  }
  const blocks = [];
  const preludes = [];
  const recorded = [];
  let gap = '';
  for (const group of groupItems(items, bytes)) {
    const out = translateGroup(group, from, to);
    for (const prelude of out.preludes ?? []) if (!preludes.includes(prelude)) preludes.push(prelude);
    for (const { term, start, end } of group.items) recorded.push({ term, start, end, status: out.status, reason: out.reason ?? null });
    if (out.code !== null) {
      blocks.push(`${blocks.length ? breaks(gap) : ''}${out.code}`);
      gap = group.after;
    } else if (!gap) {
      // A dropped group keeps the layout before it.
      gap = group.after;
    }
  }
  const body = `${blocks.join('')}\n`;
  // An unedited translation back gives the source its header describes.
  const header = items.find((item) => item.comment && item.text.startsWith(HEADER));
  if (header && header.text.includes(` source=${to} `) && header.text.includes(` sha256=${sha256(body)} `)) {
    return freeze(from, to, body, recorded);
  }
  if (family(to) === 'Rust' && recorded.some(({ status }) => status === 'translated')) preludes.unshift(RUST_ALLOW);
  const lines = [`${HEADER}source=${from} target=${to} sha256=${sha256(text)} bytes=${bytes.length}`, ''];
  if (preludes.length) lines.push(PRELUDE_BEGIN, ...preludes.flatMap((prelude, index) => (index ? ['', prelude] : [prelude])), PRELUDE_END, '');
  return freeze(from, to, `${lines.join('\n')}\n${body}`, recorded);
}

function freeze(sourceLanguage, targetLanguage, code, items) {
  return Object.freeze({
    sourceLanguage,
    targetLanguage,
    code,
    items: Object.freeze(items.map((item) => Object.freeze(item))),
  });
}

// The layout between two emitted blocks: its line breaks, at least one.
function breaks(gap) {
  return '\n'.repeat(Math.max(1, (gap.match(/\n/gu) ?? []).length));
}

/** The top-level items of `text`, with their text and the layout after each. */
function topLevelItems(text, language, bytes) {
  const parsed = parseProgrammingLanguage(text, language);
  const children = (parsed?.tree.children ?? [])
    .filter((child) => child.term !== 'whitespace')
    .map((child) => {
      const { start } = child.span.byteRange;
      let { end } = child.span.byteRange;
      // A Rust line comment ends with its line break, which is layout here.
      if (COMMENTS.has(child.term) && bytes[end - 1] === 0x0a) end -= bytes[end - 2] === 0x0d ? 2 : 1;
      return { term: child.term, start, end };
    });
  return children.map((item, index) => ({
    ...item,
    comment: COMMENTS.has(item.term),
    text: bytes.subarray(item.start, item.end).toString('utf8'),
    after: bytes.subarray(item.end, children[index + 1]?.start ?? bytes.length).toString('utf8'),
  }));
}

const lineBreak = (layout) => /^\r?\n$/u.test(layout);
const sourceLines = (lines) => lines.map(({ text }) => text.slice(SOURCE_LINE.length).replace(/^ /u, ''));

/**
 * Groups the items: a self-translation header, a prelude block, a carried or
 * translated item with its provenance, and an item with the comments directly
 * before it are one group each.
 */
function groupItems(items, bytes) {
  const groups = [];
  const textBetween = (first, last) => bytes.subarray(first.start, last.end).toString('utf8');
  for (let index = 0; index < items.length; index += 1) {
    const item = items[index];
    const take = (end, fields) => {
      groups.push({ ...fields, items: items.slice(index, end + 1), after: items[end].after });
      index = end;
    };
    // The `// |` source lines that follow the item at `at` line by line.
    const linesAfter = (at) => {
      let end = at;
      while (end + 1 < items.length && items[end + 1].comment && items[end + 1].text.startsWith(SOURCE_LINE) && lineBreak(items[end].after)) end += 1;
      return end;
    };
    if (item.comment && item.text.startsWith(HEADER)) {
      take(index, { kind: 'provenance' });
    } else if (item.comment && item.text === PRELUDE_BEGIN) {
      let end = index;
      while (end + 1 < items.length && items[end].text !== PRELUDE_END) end += 1;
      take(end, { kind: 'provenance' });
    } else if (item.comment && item.text.startsWith(CARRIED)) {
      const end = linesAfter(index);
      const [language] = item.text.slice(CARRIED.length).split(' ');
      take(end, { kind: 'carried', language, marker: item.text, lines: sourceLines(items.slice(index + 1, end + 1)) });
    } else if (item.comment && item.text.startsWith(TRANSLATED)) {
      const fields = Object.fromEntries(item.text.slice(TRANSLATED.length).split(' ').slice(2).map((field) => field.split('=')));
      const linesEnd = linesAfter(index);
      const count = Number(fields.items);
      const last = linesEnd + count;
      if (Number.isSafeInteger(count) && count > 0 && last < items.length) {
        const code = textBetween(items[linesEnd + 1], items[last]);
        const [language] = item.text.slice(TRANSLATED.length).split(' ');
        const lines = sourceLines(items.slice(index + 1, linesEnd + 1));
        if (sha256(code) === fields.sha256) {
          take(last, { kind: 'carried', language, lines });
          continue;
        }
        // An edited translation is translated again; its provenance is dropped.
        take(linesEnd, { kind: 'provenance' });
        continue;
      }
      take(index, { kind: 'comment', text: item.text, term: item.term });
    } else {
      // Comments directly before an item document it and travel with it.
      let end = index;
      while (items[end].comment && end + 1 < items.length && lineBreak(items[end].after) && !isMarker(items[end + 1])) end += 1;
      // A run of comments no item follows is one comment group.
      take(end, { kind: items[end].comment ? 'comment' : 'item', text: textBetween(item, items[end]), term: items[end].term });
    }
  }
  return groups;
}

const isMarker = (item) => item.comment && [HEADER, CARRIED, TRANSLATED, PRELUDE_BEGIN].some((marker) => item.text.startsWith(marker));

function translateGroup(group, from, to) {
  if (group.kind === 'provenance') return { code: null, status: 'provenance' };
  if (group.kind === 'carried') {
    if (family(group.language) === family(to)) return { code: group.lines.join('\n'), status: 'restored' };
    const marker = group.items[0].text;
    return { code: [marker, ...group.lines.map(sourceLine)].join('\n'), status: 'carried', reason: 'carried from another language' };
  }
  const { text, term } = group;
  if (group.kind === 'comment') {
    // A copied comment has no provenance, so it must also fit the source.
    if (group.items.every((item) => commentFits(item.text, from) && commentFits(item.text, to))) return { code: text, status: 'comment' };
    return carry(text, term, from, 'comment the target cannot hold');
  }
  let emitted;
  try {
    // The Rust frontend reads a program, so an item alone gets an empty main.
    const program = checkProgram(family(from) === 'Rust'
      ? parseRust(/\bfn\s+main\s*\(/u.test(text) ? text : `${text}\nfn main() {}\n`)
      : parseJavaScript(text));
    if (program.main.effects.length > 0) return carry(text, term, from, 'top-level statement');
    emitted = (family(to) === 'Rust' ? emitRust : emitJavaScript)(program);
  } catch (error) {
    if (!(error instanceof TranslationError)) throw error;
    return carry(text, term, from, error.kind);
  }
  if (emitted.definitions.length === 0) return carry(text, term, from, 'no definition');
  const exported = family(to) === 'JavaScript' && (family(from) === 'Rust' ? /^pub(\([^)]*\))?\s/mu : /^export\s/mu).test(text);
  const code = emitted.definitions.map((definition) => (exported ? `export ${definition}` : definition)).join('\n\n');
  const count = emitted.definitions.length;
  const marker = `${TRANSLATED}${from} ${term} items=${count} sha256=${sha256(code)}`;
  return {
    code: [marker, ...text.split(/\r?\n/u).map(sourceLine), code].join('\n'),
    preludes: emitted.preludes,
    status: 'translated',
  };
}

function carry(text, term, from, reason) {
  return {
    code: [`${CARRIED}${from} ${term} (${reason})`, ...text.split(/\r?\n/u).map(sourceLine)].join('\n'),
    status: 'carried',
    reason,
  };
}

const sourceLine = (line) => (line === '' ? SOURCE_LINE : `${SOURCE_LINE} ${line}`);

// A comment keeps its text when the target reads it as an ordinary comment:
// a Rust doc comment needs an item after it, and Rust block comments nest.
function commentFits(text, to) {
  if (text.startsWith('/*') && /\/\*|\*\//u.test(text.slice(2, -2))) return false;
  return family(to) !== 'Rust' || !/^(\/\/[/!]|\/\*[*!])/u.test(text);
}
