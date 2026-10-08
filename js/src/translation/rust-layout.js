// The layout of emitted Rust (issue #217). The emitters write each
// expression on one line, so a long array, call or macro argument list
// stays one line however long it gets. `wrapRust` breaks every line longer
// than RUST_WIDTH the way rustfmt lays out a list that does not fit: the
// bracket's items go one per line, indented four spaces past the line, and
// the closing bracket returns to the line's indentation. The layout is
// deterministic and identical in the Rust root (`translation::rust_layout`),
// so a consumer can hold its committed Rust byte-equal to the translator.
//
// Only parentheses and square brackets are broken: a brace opens a block,
// whose statements are not comma lists. Among the brackets of a line that no
// other parenthesis or square bracket encloses, the one with the longest
// content is broken first (the leftmost on a tie), and every line that
// results is laid out again, so nested lists break from the outside in. Two
// or more items each end with a comma; a single item keeps none, so a
// parenthesized expression never becomes a tuple. A line with nothing to
// break (one long string literal) stays as it is.

/** The width rustfmt lays Rust out to by default. */
export const RUST_WIDTH = 100;

const INDENT = '    ';
const CLOSER = { '(': ')', '[': ']', '{': '}', '<': '>' };

const isIdentifierCharacter = (character) => character !== undefined && /[\p{Alphabetic}\p{N}_]/u.test(character);

/**
 * The length of the string, raw string or character literal that starts at
 * `index` of `characters`, or 0 when none starts there.
 */
function literalLength(characters, index) {
  const character = characters[index];
  if (character === '"') {
    let end = index + 1;
    while (end < characters.length && characters[end] !== '"') end += characters[end] === '\\' ? 2 : 1;
    return Math.min(end + 1, characters.length) - index;
  }
  if (character === 'r' && !isIdentifierCharacter(characters[index - 1])) {
    let hashes = 0;
    while (characters[index + 1 + hashes] === '#') hashes += 1;
    if (characters[index + 1 + hashes] !== '"') return 0;
    const closing = `"${'#'.repeat(hashes)}`;
    for (let end = index + 2 + hashes; end < characters.length; end += 1) {
      if (characters.slice(end, end + closing.length).join('') === closing) return end + closing.length - index;
    }
    return characters.length - index;
  }
  if (character === "'") {
    if (characters[index + 1] === '\\') {
      let end = index + 2;
      while (end < characters.length && characters[end] !== "'") end += 1;
      return end < characters.length ? end + 1 - index : 0;
    }
    return characters[index + 2] === "'" ? 3 : 0;
  }
  return 0;
}

/**
 * The brackets of a line: for each parenthesis or square bracket closed on
 * the line, its open and close positions, whether another parenthesis or
 * square bracket encloses it, and the positions of its own commas.
 */
export function rustBrackets(characters) {
  const stack = [];
  const brackets = [];
  for (let index = 0; index < characters.length; index += 1) {
    const character = characters[index];
    const literal = literalLength(characters, index);
    if (literal > 0) {
      index += literal - 1;
      continue;
    }
    if (character === '/' && characters[index + 1] === '/') break;
    const generic = character === '<' && (isIdentifierCharacter(characters[index - 1]) || characters[index - 1] === ':');
    if (character === '(' || character === '[' || character === '{' || generic) {
      const listed = stack.some((open) => open.character === '(' || open.character === '[');
      stack.push({ character, open: index, enclosed: listed, commas: [] });
    } else if (character === ',' && stack.length > 0) {
      stack[stack.length - 1].commas.push(index);
    } else if (stack.length > 0 && character === CLOSER[stack[stack.length - 1].character]) {
      if (character === '>' && characters[index - 1] === '-') continue;
      const open = stack.pop();
      if (open.character === '(' || open.character === '[') {
        brackets.push({ open: open.open, close: index, enclosed: open.enclosed, commas: open.commas });
      }
    }
  }
  return brackets;
}

/** The bracket to break in a line that is too long, or null. */
function bracketToBreak(characters) {
  let chosen = null;
  for (const bracket of rustBrackets(characters)) {
    const length = bracket.close - bracket.open - 1;
    if (bracket.enclosed || characters.slice(bracket.open + 1, bracket.close).join('').trim() === '') continue;
    if (chosen === null || length > chosen.close - chosen.open - 1 || (length === chosen.close - chosen.open - 1 && bracket.open < chosen.open)) {
      chosen = bracket;
    }
  }
  return chosen;
}

/** The lines one long line is laid out as. */
function wrapLine(line, width) {
  const characters = Array.from(line);
  if (characters.length <= width) return [line];
  const bracket = bracketToBreak(characters);
  if (bracket === null) return [line];
  const indent = line.slice(0, line.length - line.trimStart().length);
  const cuts = [bracket.open, ...bracket.commas, bracket.close];
  const items = [];
  for (let index = 0; index + 1 < cuts.length; index += 1) {
    const item = characters.slice(cuts[index] + 1, cuts[index + 1]).join('').trim();
    if (item !== '') items.push(item);
  }
  const comma = items.length > 1 ? ',' : '';
  const lines = [
    characters.slice(0, bracket.open + 1).join('').trimEnd(),
    ...items.map((item) => `${indent}${INDENT}${item}${comma}`),
    `${indent}${characters.slice(bracket.close).join('')}`,
  ];
  return lines.flatMap((next) => wrapLine(next, width));
}

/**
 * `code` with every line longer than `width` characters broken at its
 * longest bracketed list, recursively.
 */
export function wrapRust(code, width = RUST_WIDTH) {
  return code
    .split('\n')
    .flatMap((line) => wrapLine(line, width))
    .join('\n');
}
