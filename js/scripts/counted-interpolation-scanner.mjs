// Counted quote and interpolation profiles compile to the common scanner
// operation language. Grammar-owned expressions stay outside the scanner.
const literal = value => `(literal ${[...Buffer.from(value)].map(byte => /[A-Za-z0-9_.-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`).join('') || '%'})`;
const integer = value => `(integer ${value})`;
const branch = (condition, yes, no = '') => `(if ${condition} (then ${yes})${no ? ` (else ${no})` : ''})`;
const all = (...items) => `(all ${items.join(' ')})`;
const some = (...items) => `(some ${items.join(' ')})`;
const equal = (left, right) => `(equal ${left} ${right})`;
const next = value => `(next ${literal(value)})`;
const valid = token => `(valid ${token})`;
const set = (name, value) => `(set ${name} ${value})`;
const variable = name => `(variable ${name})`;
const top = name => `(top ${name})`;
const depth = name => `(depth ${name})`;
const push = (name, value) => `(push ${name} ${value})`;
const pop = name => `(pop ${name})`;
const emit = name => `(emit ${name})`;
const loop = (condition, body) => `(while ${condition} (do ${body}))`;

/** A profile with separate prefix, quote, brace and content token identities. */
export function countedInterpolationScanner({ name, optionalToken, regularStartToken,
  verbatimStartToken, rawStartToken, startQuoteToken, endQuoteToken, openBraceToken,
  closeBraceToken, contentToken, rawQuoteStartToken, rawQuoteEndToken, rawContentToken,
  quote = '"', prefix = '$', verbatimPrefix = '@', openBrace = '{', closeBrace = '}',
  escape = '\\', semicolon = ';', countModulo = 256, minimumRawQuotes = 3 }) {
  const tokens = [optionalToken, regularStartToken, verbatimStartToken, rawStartToken,
    startQuoteToken, endQuoteToken, openBraceToken, closeBraceToken, contentToken,
    rawQuoteStartToken, rawQuoteEndToken, rawContentToken];
  if ([name, ...tokens].some(value => typeof value !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(value))
    || new Set(tokens).size !== tokens.length || [quote, prefix, verbatimPrefix, openBrace, closeBrace, escape, semicolon].some(value => typeof value !== 'string' || [...value].length !== 1)
    || !Number.isSafeInteger(countModulo) || countModulo < 4 || !Number.isSafeInteger(minimumRawQuotes) || minimumRawQuotes < 3) throw new TypeError('invalid counted interpolation profile');
  const key = suffix => `${name}_${suffix}`;
  const quotes = key('quotes'), braces = key('braces'), dollars = key('prefixes'), types = key('types');
  const quoteCount = key('quote_count'), braceCount = key('brace_count'), prefixCount = key('prefix_count');
  const rawQuotes = key('raw_quotes'), advanced = key('advanced'), verbatim = key('verbatim'), startingType = key('starting_type');
  const increase = counter => `${set(counter, `(add ${variable(counter)} ${integer(1)})`)} ${branch(equal(variable(counter), integer(countModulo)), set(counter, integer(0)))}`;
  const assignTop = (stack, value) => `${pop(stack)} ${push(stack, value)}`;
  const increaseTop = stack => `${set(quoteCount, top(stack))} ${increase(quoteCount)} ${assignTop(stack, variable(quoteCount))}`;
  const whitespace = '(class plain (char %20) (char %09) (char %0A) (char %0B) (char %0C) (char %0D))';
  const skipWhitespace = loop(`(next ${whitespace})`, `(skip ${whitespace})`);
  const countQuotes = `${set(quoteCount, integer(0))} ${loop(next(quote), `advance ${increase(quoteCount)}`)}`;
  const present = `(greater ${depth(types)} ${integer(0)})`;
  const regular = equal(top(types), integer(1)), verbatimType = equal(top(types), integer(2)), raw = equal(top(types), integer(4));
  const save = type => `${push(types, integer(type))} ${push(dollars, variable(prefixCount))} ${push(braces, integer(0))} ${push(quotes, integer(0))}`;
  const clear = [types, dollars, braces, quotes].map(pop).join(' ');
  const rawStart = branch(valid(rawQuoteStartToken), `${skipWhitespace} ${branch(next(quote), `${countQuotes} ${branch(`(not (less ${variable(quoteCount)} ${integer(minimumRawQuotes)}))`, `${set(rawQuotes, variable(quoteCount))} ${emit(rawQuoteStartToken)}`)}`)}`);
  const rawEnd = branch(all(valid(rawQuoteEndToken), next(quote)), `${countQuotes} ${branch(equal(variable(quoteCount), variable(rawQuotes)), `${set(rawQuotes, integer(0))} ${emit(rawQuoteEndToken)}`)} ${branch(`(greater ${variable(quoteCount)} ${integer(0)})`, set(advanced, integer(1)))}`);
  const rawContent = branch(valid(rawContentToken), `${loop(`(all (not atEnd) (not ${next('\0')}))`, `${branch(next(quote), `mark ${countQuotes} ${branch(equal(variable(quoteCount), variable(rawQuotes)), emit(rawContentToken))}`)} advance ${set(advanced, integer(1))}`)} mark ${emit(rawContentToken)}`);
  const starts = branch(some(valid(regularStartToken), valid(verbatimStartToken), valid(rawStartToken)), `${skipWhitespace} ${set(prefixCount, integer(0))} ${set(verbatim, integer(0))} ${set(startingType, integer(1))} ${branch(next(verbatimPrefix), `${set(verbatim, integer(1))} advance`)} ${loop(all(next(prefix), equal(variable(quoteCount), integer(0))), `advance ${increase(prefixCount)}`)} ${branch(all(`(greater ${variable(prefixCount)} ${integer(0)})`, some(next(quote), next(verbatimPrefix))), `${branch(some(equal(variable(verbatim), integer(1)), next(verbatimPrefix)), `${branch(next(verbatimPrefix), `advance ${set(verbatim, integer(1))}`)}`)} mark advance ${branch(all(next(quote), equal(variable(verbatim), integer(0))), `advance ${branch(next(quote), `${set(startingType, integer(4))} ${save(4)}`)}`, branch(equal(variable(verbatim), integer(1)), save(2), save(1)))} ${branch(equal(variable(verbatim), integer(1)), emit(verbatimStartToken), branch(equal(variable(startingType), integer(4)), emit(rawStartToken), emit(regularStartToken)))}`)}`);
  const startQuote = branch(all(valid(startQuoteToken), present), `${branch(some(regular, verbatimType), branch(next(quote), `advance ${increaseTop(quotes)}`), loop(next(quote), `advance ${increaseTop(quotes)}`))} ${branch(`(greater ${top(quotes)} ${integer(0)})`, emit(startQuoteToken))}`);
  const endQuote = branch(all(valid(endQuoteToken), present), `${countQuotes} ${branch(equal(variable(quoteCount), top(quotes)), `${clear} ${emit(endQuoteToken)}`)} ${branch(`(greater ${variable(quoteCount)} ${integer(0)})`, set(advanced, integer(1)))}`);
  const open = branch(all(some(valid(openBraceToken), all(valid(contentToken), `(expected (ref ${openBraceToken}))`)), present), `${loop(all(next(openBrace), `(less ${variable(braceCount)} ${top(dollars)})`), `advance ${increase(braceCount)}`)} ${branch(all(`(greater ${variable(braceCount)} ${integer(0)})`, equal(variable(braceCount), top(dollars)), `(not ${next(openBrace)})`), `${assignTop(braces, variable(braceCount))} ${emit(openBraceToken)}`)}`);
  const close = branch(all(valid(closeBraceToken), present), `${set(braceCount, integer(0))} ${loop(`(next ${whitespace})`, 'advance')} ${loop(next(closeBrace), `advance ${increase(braceCount)} ${branch(equal(variable(braceCount), top(braces)), `${assignTop(braces, integer(0))} ${emit(closeBraceToken)}`)}`)} fail`);
  const returnContent = `${branch(equal(variable(advanced), integer(0)), 'fail')} ${emit(contentToken)}`;
  const braceBoundary = `${branch(next(openBrace), `mark ${loop(all(next(openBrace), `(less ${variable(braceCount)} ${top(braces)})`), `advance ${increase(braceCount)}`)} ${branch(all(equal(variable(braceCount), top(braces)), some(equal(variable(braceCount), integer(0)), `(not ${next(openBrace)})`)), returnContent)}`)}`;
  const rawBoundary = `${branch(next(quote), `mark advance ${branch(next(quote), `advance ${set(quoteCount, integer(2))} ${loop(next(quote), `${increase(quoteCount)} advance`)} ${branch(equal(variable(quoteCount), top(quotes)), returnContent)}`)}`)} ${braceBoundary}`;
  // A doubled verbatim quote continues directly, before the common advance.
  const verbatimLoop = `${branch(next(quote), `mark advance ${branch(next(quote), 'advance', returnContent)}`, `${braceBoundary} ${branch(`(not ${next(openBrace)})`, set(braceCount, integer(0)))} advance ${set(advanced, integer(1))}`)}`;
  const regularBoundary = `${branch(some(next(escape), next('\n'), next(quote)), `mark ${returnContent}`)} ${braceBoundary}`;
  const commonAdvance = `${branch(`(not ${next(openBrace)})`, set(braceCount, integer(0)))} advance ${set(advanced, integer(1))}`;
  const content = branch(all(valid(contentToken), present), `${loop(`(all (not atEnd) (not ${next('\0')}))`, branch(raw, `${rawBoundary} ${commonAdvance}`, branch(verbatimType, verbatimLoop, `${regularBoundary} ${commonAdvance}`)))} mark ${returnContent}`);
  const optional = branch(valid(optionalToken), `${branch(next(semicolon), 'advance')} ${emit(optionalToken)}`);
  const operations = `${set(braceCount, integer(0))} ${set(quoteCount, integer(0))} ${set(advanced, integer(0))} ${optional} ${rawStart} ${rawEnd} ${rawContent} ${starts} ${startQuote} ${endQuote} ${open} ${close} ${content} fail`;
  return `(scanner ${name} (tokens ${tokens.join(' ')}) (operations ${operations}))\n`;
}
