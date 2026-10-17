// Candidate inline delimiter transitions from the authenticated Markdown source.
// The caller must supply the complete simultaneous source-token eligibility;
// ordinary requested-token equality is not an equivalent scanner context.

/** Generate the entire inline scanner as shared Links operations. */
export function markdownInlineScanner({name, tokens, eligibility}) {
  if (eligibility !== 'complete-source-symbol-context') throw new TypeError('inline delimiter scanners require a complete source-symbol context');
  if (typeof name !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(name)
    || !Array.isArray(tokens) || tokens.length !== 15 || new Set(tokens).size !== 15
    || tokens.some(token => typeof token !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(token))) {
    throw new TypeError('inline delimiter scanners retain fifteen distinct ordered source token names');
  }
  const [error, trigger, codeOpen, codeClose, starOpen, underscoreOpen, starClose, underscoreClose,
    lastWhitespace, lastPunctuation, strikeOpen, strikeClose, latexOpen, latexClose, unclosed] = tokens;
  const value = variable => `(variable ${name}_${variable})`;
  const number = integer => `(integer ${integer})`;
  const set = (variable, expression) => `(set ${name}_${variable} ${expression})`;
  const increment = variable => set(variable, `(add ${value(variable)} ${number(1)})`);
  const decrement = variable => set(variable, `(subtract ${value(variable)} ${number(1)})`);
  const wrapIncrement = variable => `${increment(variable)} (if (greater ${value(variable)} ${number(255)}) (then ${set(variable, number(0))}))`;
  const equal = (left, right) => `(equal ${left} ${right})`;
  const not = condition => `(not ${condition})`;
  const all = (...conditions) => `(all ${conditions.join(' ')})`;
  const some = (...conditions) => `(some ${conditions.join(' ')})`;
  const branch = (condition, operations, alternative = '') => `(if ${condition} (then ${operations})${alternative ? ` (else ${alternative})` : ''})`;
  const available = token => `(expected (ref ${token}))`;
  const literal = character => `(literal ${[...Buffer.from(character)].map(byte => `%${byte.toString(16).toUpperCase().padStart(2, '0')}`).join('')})`;
  const next = character => `(next ${literal(character)})`;
  const punctuation = `(next (class plain ${Array.from({length: 94}, (_, index) => index + 33)
    .filter(code => code <= 47 || code >= 58 && code <= 64 || code >= 91 && code <= 96 || code >= 123)
    .map(code => `(char %${code.toString(16).toUpperCase()})`).join(' ')}))`;
  const whitespace = some('atEnd', next('\n'), next('\r'), next(' '), next('\t'));
  // Keep all four original serialized bytes, including unused flag bits.
  const readOpeningFlag = `${set('flag_remainder', value('flags'))} (while (greater ${value('flag_remainder')} ${number(7)}) (do ${set('flag_remainder', `(subtract ${value('flag_remainder')} ${number(8)})`)}))`;
  const opening = `(greater ${value('flag_remainder')} ${number(3)})`;
  const clearOpening = `${readOpeningFlag} ${branch(opening, set('flags', `(subtract ${value('flags')} ${number(4)})`))}`;
  const setOpening = `${readOpeningFlag} ${branch(not(opening), set('flags', `(add ${value('flags')} ${number(4)})`))}`;
  const leaf = (character, remembered, openToken, closeToken) => {
    const marker = next(character);
    const run = `${set('run_width', number(0))} (while ${marker} (do advance ${wrapIncrement('run_width')})) mark`;
    const close = branch(all(equal(value('run_width'), value(remembered)), available(closeToken)),
      `${set(remembered, number(0))} (emit ${closeToken})`);
    // The remembered opening width wraps at 256, while future run counts do not.
    const inspect = `${set('close_width', number(0))} ${set('found_closer', number(0))} `
      + `(while ${all(not('atEnd'), equal(value('found_closer'), number(0)))} (do `
      + branch(marker, increment('close_width'), branch(equal(value('close_width'), value('run_width')),
        set('found_closer', number(1)), set('close_width', number(0))))
      + ` ${branch(equal(value('found_closer'), number(0)), 'advance')}))`;
    const open = branch(available(openToken), `${inspect} `
      + branch(equal(value('close_width'), value('run_width')), `${set(remembered, value('run_width'))} (emit ${openToken})`)
      + ` ${branch(available(unclosed), `(emit ${unclosed})`)}`);
    return `${run} ${close} ${open} fail`;
  };
  const emphasis = (character, openToken, closeToken) => {
    const continuation = branch(`(greater ${value('emphasis_remaining')} ${number(0)})`,
      `${readOpeningFlag} ${branch(all(opening, available(openToken)), `${clearOpening} ${decrement('emphasis_remaining')} (emit ${openToken})`)} `
      + branch(available(closeToken), `${decrement('emphasis_remaining')} (emit ${closeToken})`));
    const count = `mark ${set('run_width', number(1))} (while ${next(character)} (do ${wrapIncrement('run_width')} advance))`;
    const closing = all(available(closeToken), not(available(lastWhitespace)),
      some(not(available(lastPunctuation)), punctuation, whitespace));
    const openingCondition = all(not(whitespace), some(not(punctuation), available(lastPunctuation), available(lastWhitespace)));
    const pending = branch(equal(value('run_width'), number(0)), set('emphasis_remaining', number(255)),
      set('emphasis_remaining', `(subtract ${value('run_width')} ${number(1)})`));
    // Source behavior allows this opening branch even when only closing is eligible.
    const decide = branch(some(available(openToken), available(closeToken)), `${pending} `
      + branch(closing, `${clearOpening} (emit ${closeToken})`) + ' '
      + branch(openingCondition, `${setOpening} (emit ${openToken})`));
    return `advance ${continuation} ${count} ${decide} fail`;
  };
  const operations = branch(available(trigger), `(emit ${error})`) + ' '
    + branch(next('`'), leaf('`', 'code_width', codeOpen, codeClose)) + ' '
    + branch(next('$'), leaf('$', 'latex_width', latexOpen, latexClose)) + ' '
    + branch(next('*'), emphasis('*', starOpen, starClose)) + ' '
    + branch(next('_'), emphasis('_', underscoreOpen, underscoreClose)) + ' '
    + branch(next('~'), emphasis('~', strikeOpen, strikeClose)) + ' fail';
  return `(scanner ${name} (tokens ${tokens.join(' ')}) (operations ${operations}))\n`;
}
