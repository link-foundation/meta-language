// Generate section indentation, queued virtual separators and lexical content
// as shared Links operations. The descriptor owns source-specific token names,
// delimiters, integer widths and closing policies; the runtime has no language
// callback. Elm's pinned scanner is the initial authoring source.
import { parseGrammarLinks, renderDeclarationLinks } from '../src/grammar-links.js';
import { parseTreeSitterPattern, renderTreeSitterPattern } from '../src/grammar-importers/tree-sitter-native.js';

const integer = (value) => ({ operation: 'integer', value });
const variable = (name) => ({ operation: 'variable', name });
const depth = (stack) => ({ operation: 'depth', stack });
const top = (stack) => ({ operation: 'top', stack });
const literal = (value) => ({ kind: 'literal', value });
const next = (item) => ({ operation: 'next', item });
const equal = (left, right) => ({ operation: 'equal', left, right });
const less = (left, right) => ({ operation: 'less', left, right });
const greater = (left, right) => ({ operation: 'greater', left, right });
const all = (...conditions) => ({ operation: 'all', conditions });
const some = (...conditions) => ({ operation: 'some', conditions });
const not = (condition) => ({ operation: 'not', condition });
const set = (name, value) => ({ operation: 'set', variable: name, value });
const assign = (name, value) => set(name, integer(value));
const advance = { operation: 'advance' };
const mark = { operation: 'mark' };
const fail = { operation: 'fail' };
const atEnd = { operation: 'atEnd' };
const column = { operation: 'column' };
const consume = (text) => ({ operation: 'consume', item: literal(text) });
const push = (stack, value) => ({ operation: 'push', stack, value });
const pop = (stack) => ({ operation: 'pop', stack });
const branch = (condition, consequent, alternative) => ({
  operation: 'if', condition, consequent,
  ...(alternative === undefined ? {} : { alternative }),
});
const loop = (condition, body) => ({ operation: 'while', condition, body });
const increment = (name, amount) => set(name, {
  operation: 'add', left: variable(name), right: integer(amount),
});
const clear = (stack) => loop(greater(depth(stack), integer(0)), [pop(stack)]);

/** Generate a complete queued section scanner, including its lexical tokens. */
export function sectionLayoutScanner({
  name, endDeclarationToken, openSectionToken, endSectionToken, minusToken,
  shaderContentToken, commentContentToken, stringContentToken,
  maximumDepth = 256, indentationModulo = 256, sectionEndWord = 'in',
  sectionClosers = [')', ',', '}'], commentOpening = '{-', commentClosing = '-}',
  lineCommentOpening = '--', multilineClosing = '"""', stringEscape = '\\',
  shaderClosing = '|]', minusFollowing = '[A-Za-z(\\u0080-\\u{10ffff}]',
}) {
  const tokens = [endDeclarationToken, openSectionToken, endSectionToken, minusToken,
    shaderContentToken, commentContentToken, stringContentToken];
  if ([name, ...tokens].some((text) => typeof text !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(text))
    || new Set(tokens).size !== tokens.length) throw new TypeError('section layout needs distinct scanner identifiers');
  if (!Number.isSafeInteger(maximumDepth) || maximumDepth < 2 || !Number.isSafeInteger(indentationModulo) || indentationModulo < 2) throw new TypeError('section layout needs bounded stack and integer widths');
  if ([commentOpening, commentClosing, lineCommentOpening, multilineClosing, shaderClosing, sectionEndWord].some((text) => typeof text !== 'string' || [...text].length < 2)
    || [...sectionEndWord].length !== 2 || [...commentOpening].length !== 2 || [...commentClosing].length !== 2
    || [...lineCommentOpening].length !== 2 || [...shaderClosing].length !== 2 || [...multilineClosing].length !== 3
    || typeof stringEscape !== 'string' || [...stringEscape].length !== 1
    || !Array.isArray(sectionClosers) || !sectionClosers.length || sectionClosers.some((text) => [...text].length !== 1)) throw new TypeError('section layout delimiters have incompatible widths');
  const available = (token) => some({ operation: 'valid', token }, {
    operation: 'expected', item: { kind: 'ref', name: token },
  });
  const requested = (token) => ({ operation: 'valid', token });
  const indents = `${name}_indentations`;
  const pending = `${name}_pending`;
  const generated = `${name}_generated`;
  const width = `${name}_line_indentation`;
  const temporaryColumn = `${name}_opening_column`;
  const localNames = ['newline', 'section_end', 'can_mark', 'scan', 'content', 'nesting', 'comments', 'settle'].map((suffix) => `${name}_${suffix}`);
  const [newline, ending, canMark, scanning, hasContent, nesting, comments, settling] = localNames;
  const yes = (name) => equal(variable(name), integer(1));
  const emit = (token) => [...localNames.map((name) => assign(name, 0)), assign(temporaryColumn, 0), { operation: 'emit', token }];
  const space = { kind: 'charClass', negated: false, items: [' ', '\r', '\n'].map((value) => ({ kind: 'char', value })) };
  const horizontal = { kind: 'charClass', negated: false, items: [' ', '\r'].map((value) => ({ kind: 'char', value })) };
  const commentSpace = { kind: 'charClass', negated: false, items: [' ', '\r', '\n', '\t'].map((value) => ({ kind: 'char', value })) };
  const firstWord = [...sectionEndWord][0];
  const secondWord = [...sectionEndWord][1];
  const firstComment = [...commentOpening][0];
  const secondComment = [...commentOpening][1];
  const firstCommentClose = [...commentClosing][0];
  const secondCommentClose = [...commentClosing][1];
  const firstLineComment = [...lineCommentOpening][0];
  const secondLineComment = [...lineCommentOpening][1];
  const quote = [...multilineClosing][0];
  const following = parseGrammarLinks(`(grammar (start character))\n(rule character normal ${renderTreeSitterPattern(parseTreeSitterPattern(minusFollowing))})\n`).rule('character').expression;
  if ([...multilineClosing].some((character) => character !== quote)) throw new TypeError('multiline closing must repeat one quote');
  const queueEmission = [
    branch(all(greater(depth(pending), integer(0)), equal(top(pending), integer(0)), requested(endDeclarationToken)), [pop(pending), ...emit(endDeclarationToken)]),
    branch(all(greater(depth(pending), integer(0)), equal(top(pending), integer(1)), requested(endSectionToken)), [pop(pending), ...emit(endSectionToken)]),
  ];
  const nestedComment = [
    assign(nesting, 1),
    loop(all(greater(variable(nesting), integer(0)), not(atEnd)), [
      branch(next(literal(commentOpening)), [consume(commentOpening), increment(nesting, 1)], [
        branch(next(literal(commentClosing)), [consume(commentClosing), increment(nesting, -1)], [advance]),
      ]),
    ]),
    assign(nesting, 0),
  ];
  const remeasureAfterComment = [loop(all(not(atEnd), next(commentSpace)), [
    branch(next(literal('\n')), [advance,
      loop(next(literal(' ')), [advance]), set(width, column)], [advance]),
  ])];
  const multilineContent = [assign(hasContent, 0), loop(not(atEnd), [
    branch(next(literal(quote)), [mark, advance,
      branch(next(literal(quote)), [advance,
        branch(next(literal(quote)), [branch(not(yes(hasContent)), [fail]), ...emit(stringContentToken)]),
      ]), assign(hasContent, 1)], [
      branch(next(literal(stringEscape)), [mark, branch(not(yes(hasContent)), [fail]), ...emit(stringContentToken)], [assign(hasContent, 1), advance]),
    ]),
  ]), mark, branch(not(yes(hasContent)), [fail]), ...emit(stringContentToken)];
  const spacesAndComments = [
    assign(scanning, 1),
    loop(yes(scanning), [branch(next(horizontal), [advance], [
      branch(next(literal('\n')), [advance, assign(newline, 1), loop(next(literal(' ')), [advance]), set(width, column)], [
        branch(all(not(available(commentContentToken)), next(literal(firstLineComment))), [advance,
          branch(all(available(minusToken), next(following)), [branch(not(yes(canMark)), [fail]), mark, ...emit(minusToken)]),
          branch(all(next(literal(secondLineComment)), yes(newline)), [assign(canMark, 0), advance,
            loop(all(not(atEnd), not(next(literal('\n')))), [advance])], [fail]),
        ], [branch(atEnd, [
          branch(requested(endSectionToken), emit(endSectionToken)),
          branch(requested(endDeclarationToken), emit(endDeclarationToken)),
          assign(scanning, 0),
        ], [assign(scanning, 0)])]),
      ]),
    ])]),
  ];
  const closeSection = [branch(greater(depth(indents), integer(0)), [pop(indents)]), ...emit(endSectionToken)];
  const detectEndWord = branch(all(available(endSectionToken), next(literal(firstWord))), [advance,
    branch(next(literal(secondWord)), [advance,
      branch(some(next(space), atEnd), [branch(yes(newline), [assign(ending, 1)], closeSection)]),
    ]),
  ]);
  const openSection = branch(all(available(openSectionToken), not(atEnd)), [
    branch(not(less(depth(indents), integer(maximumDepth))), [fail]),
    set(temporaryColumn, column), loop(not(less(variable(temporaryColumn), integer(indentationModulo))), [
      set(temporaryColumn, { operation: 'subtract', left: variable(temporaryColumn), right: integer(indentationModulo) }),
    ]), push(indents, variable(temporaryColumn)), ...emit(openSectionToken),
  ]);
  const commentContent = branch(available(commentContentToken), [branch(not(yes(canMark)), [fail]), mark,
    assign(scanning, 1), loop(all(yes(scanning), not(atEnd)), [
      branch(next(literal(firstCommentClose)), [mark, advance,
        branch(next(literal(secondCommentClose)), [assign(scanning, 0)])], [
        branch(next(literal(firstComment)), [advance,
          branch(next(literal(secondComment)), [advance, ...nestedComment, mark])], [advance]),
      ]),
    ]), ...emit(commentContentToken),
  ]);
  const inspectCommentsBeforeDedent = branch(all(next(literal(firstComment)), not(available(commentContentToken)), greater(depth(indents), integer(0)), less(variable(width), top(indents))), [
    advance, branch(next(literal(secondComment)), [assign(canMark, 0), advance, ...nestedComment, ...remeasureAfterComment,
      assign(comments, 1), loop(all(yes(comments), next(literal(firstComment))), [advance,
        branch(next(literal(secondComment)), [advance, ...nestedComment, ...remeasureAfterComment], [assign(comments, 0)]),
      ]),
    ]),
  ]);
  const settleIndentation = [clear(generated), inspectCommentsBeforeDedent, assign(settling, 1),
    loop(all(yes(settling), greater(depth(indents), integer(0)), not(greater(variable(width), top(indents)))), [
      branch(equal(variable(width), top(indents)), [
        branch(yes(ending), [pop(indents), push(generated, integer(1)), assign(ending, 0)], [
          branch(next(literal(firstLineComment)), [advance, branch(next(literal(secondLineComment)), [assign(settling, 0)])]),
          branch(all(yes(settling), next(literal(firstComment))), [advance, branch(next(literal(secondComment)), [assign(settling, 0)])]),
          branch(yes(settling), [push(generated, integer(0))]),
        ]), assign(settling, 0),
      ], [pop(indents), push(generated, integer(1)),
        branch(all(yes(ending), some(equal(depth(indents), integer(0)), greater(variable(width), top(indents)))), [assign(ending, 0)]),
      ]),
    ]),
    branch(all(yes(ending), greater(depth(indents), integer(0))), [pop(indents), push(generated, integer(1)), assign(ending, 0)]),
    loop(greater(depth(generated), integer(0)), [push(pending, top(generated)), pop(generated)]),
    ...queueEmission,
    branch(all(atEnd, requested(endSectionToken)), emit(endSectionToken)),
  ];
  const shaderContent = branch(available(shaderContentToken), [branch(not(yes(canMark)), [fail]),
    loop(not(atEnd), [branch(next(literal([...shaderClosing][0])), [mark, advance,
      branch(next(literal([...shaderClosing][1])), [advance, ...emit(shaderContentToken)])], [advance])]),
    mark, ...emit(shaderContentToken),
  ]);
  const operations = [
    branch(all(...tokens.map(available)), [fail]),
    branch(equal(depth(indents), integer(0)), [push(indents, integer(0))]),
    ...queueEmission, clear(pending), clear(generated),
    branch(available(stringContentToken), multilineContent),
    assign(newline, 0), assign(ending, 0), assign(canMark, 1), mark,
    ...spacesAndComments, detectEndWord,
    branch(all(available(endSectionToken), some(...sectionClosers.map((text) => next(literal(text))))), closeSection),
    openSection, commentContent,
    branch(yes(newline), settleIndentation), shaderContent, fail,
  ];
  return `${renderDeclarationLinks({ imports: [], modes: [], extras: [], conflicts: [], precedences: [], macros: [], scanners: [{ name, tokens, operations }] }).join('\n')}\n`;
}
