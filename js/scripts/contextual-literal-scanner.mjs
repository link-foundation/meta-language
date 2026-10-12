// Generate scanner decisions from literal, queue and lexical policy data.
// The executor consumes only the resulting common Links operations.
import { parseTreeSitterPattern, renderTreeSitterPattern } from '../src/grammar-importers/tree-sitter-native.js';

const encode = (value) => [...Buffer.from(value)].map((byte) => /[A-Za-z0-9_.-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`).join('') || '%';
const literal = (value) => `(literal ${encode(value)})`;
const pattern = (value) => renderTreeSitterPattern(parseTreeSitterPattern(value));
const branch = (condition, body, otherwise = null) => `(if ${condition} (then ${body})${otherwise === null ? '' : ` (else ${otherwise})`})`;
const equal = (first, second) => `(equal ${first} ${second})`;
const integer = (number) => `(integer ${number})`;
const variable = (name) => `(variable ${name})`;
const top = (name) => `(top ${name})`;
const set = (name, value) => `(set ${name} ${value})`;
const next = (value) => `(next ${value})`;
const valid = (token) => `(valid ${token})`;
const available = (token) => `(some (valid ${token}) (expected (ref ${token})))`;
const some = (...conditions) => `(some ${conditions.join(' ')})`;
const all = (...conditions) => `(all ${conditions.join(' ')})`;
const not = (condition) => `(not ${condition})`;
const present = (stack) => `(greater (depth ${stack}) (integer 0))`;
const emit = (token) => `(emit ${token})`;

/** Stacked delimited tokens, queued line bodies and whitespace-sensitive lexemes. */
export function contextualLiteralScanner({ name, tokens, contentToken, endToken, openGateToken,
  literals, delimiters = [], lexical = [], lineBreakToken, noLineBreakToken, shortInterpolationToken,
  interpolation = '#{', shortInterpolationPattern, escape = '\\', whitespace = '[ \\t\\r\\n]',
  heredoc, interpolationStartToken = null, interpolationEndToken = null, interpolationClosing = '}',
  lineContinuationPattern = null, sharedLexicalDecisions = [] }) {
  const identifiers = [name, ...tokens, contentToken, endToken, openGateToken, lineBreakToken, noLineBreakToken, shortInterpolationToken];
  if (identifiers.some((value) => typeof value !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(value)) || new Set(tokens).size !== tokens.length) throw new TypeError('contextual literal policies need valid unique token names');
  if (!Array.isArray(literals) || !literals.length || !Array.isArray(lexical) || !Array.isArray(delimiters)) throw new TypeError('contextual literals need literal and lexical policy lists');
  for (const entry of literals) {
    if (!tokens.includes(entry.token) || typeof entry.prefix !== 'string' || typeof entry.interpolated !== 'boolean' || typeof entry.words !== 'boolean' || (entry.delimiter !== undefined && typeof entry.delimiter !== 'string')) throw new TypeError('invalid contextual literal opening policy');
  }
  for (const entry of lexical) {
    if (!tokens.includes(entry.token) || typeof entry.pattern !== 'string') throw new TypeError('invalid contextual lexical policy');
  }
  if (!Array.isArray(sharedLexicalDecisions)) throw new TypeError('shared lexical decisions need token groups');
  const lexicalGroups = new Map();
  for (const group of sharedLexicalDecisions) {
    if (!Array.isArray(group) || group.length < 2 || group.some((token) => !tokens.includes(token) || !lexical.some((entry) => entry.token === token) || lexicalGroups.has(token)) || new Set(group).size !== group.length) throw new TypeError('shared lexical decisions need distinct disjoint lexical token groups');
    for (const token of group) lexicalGroups.set(token, group);
  }
  const contexts = `${name}_contexts`;
  const openingsStack = `${name}_openings`;
  const closingsStack = `${name}_closings`;
  const activeBodyInterpolation = `${name}_body_interpolation`;
  const levels = `${name}_levels`;
  const seen = `${name}_seen`;
  const leading = `${name}_leading`;
  const spaces = pattern(whitespace);
  const horizontal = pattern('[ \\t]');
  const newline = pattern('[\\r\\n]');
  const short = pattern(shortInterpolationPattern);
  const clearSeen = set(seen, integer(0));
  const haveContent = equal(variable(seen), integer(1));
  const contentOrFail = branch(haveContent, `mark ${emit(contentToken)}`, 'fail');
  // Literal tokens and prefixes can share the same body behavior. Remember
  // that behavior once, so every later token does not test an opening policy
  // for each of the language's quotation spellings.
  const bodyModes = [];
  const bodyContexts = new Map();
  const policies = literals.map((entry) => {
    const key = JSON.stringify([entry.interpolated, entry.words, entry.suffixPattern ?? null]);
    let context = bodyContexts.get(key);
    if (context === undefined) {
      context = bodyModes.length + 1;
      bodyContexts.set(key, context);
      bodyModes.push({ ...entry, context });
    }
    return { ...entry, context };
  });
  const modes = policies.flatMap((entry) => entry.delimiter === undefined ? delimiters.map((delimiter) => ({ ...entry, ...delimiter, delimiterGuard: delimiter.guard })) : [{ ...entry, opening: entry.delimiter, closing: entry.delimiter }]);
  const closingExpression = `(predicate any (equal matched (top ${closingsStack})))`;
  const openingExpression = `(predicate any (equal matched (top ${openingsStack})))`;
  const body = bodyModes.map((mode) => {
    const close = next(closingExpression);
    const nested = not(equal(top(openingsStack), top(closingsStack)));
    const decrement = `(push ${levels} (subtract (top ${levels}) (integer 1)))`;
    const increment = `(push ${levels} (add (top ${levels}) (integer 1)))`;
    const replaceLevel = (operation) => `${set(`${name}_level`, top(levels))} (pop ${levels}) ${operation.replace(`(top ${levels})`, variable(`${name}_level`))}`;
    const emitEnd = `${mode.suffixPattern ? `(consume (repeat0 ${pattern(mode.suffixPattern)})) ` : ''}mark (pop ${contexts}) (pop ${levels}) (pop ${openingsStack}) (pop ${closingsStack}) ${emit(endToken)}`;
    const finish = `mark ${branch(haveContent, emit(contentToken))} (consume ${closingExpression}) ${emitEnd}`;
    const finishAtInputEnd = branch(equal(top(closingsStack), `(text %)`), `mark ${branch(haveContent, emit(contentToken))} ${emitEnd}`);
    const special = mode.interpolated ? `${branch(next(literal(interpolation)), contentOrFail)} ${branch(next(literal(escape)), contentOrFail)} ` : '';
    const escaped = mode.interpolated ? 'advance' : branch(next(literal(escape)), `advance (if atEnd (then fail)) advance`, 'advance');
    const move = branch(all(nested, close), `${replaceLevel(decrement)} advance`, branch(all(nested, next(openingExpression)), `${replaceLevel(increment)} advance`, escaped));
    const wordBoundary = mode.words ? `${branch(next(spaces), contentOrFail)} ` : '';
    const contentMove = mode.interpolated ? branch(next(literal('#')), `mark advance ${branch(next(short), `${branch(haveContent, emit(contentToken))} mark ${emit(shortInterpolationToken)}`)}`, move) : move;
    return branch(equal(top(contexts), integer(mode.context)), `${clearSeen} (while (not atEnd) (do ${wordBoundary}${branch(all(close, equal(top(levels), integer(1))), finish)} ${special}${contentMove} ${set(seen, integer(1))})) ${finishAtInputEnd} fail`);
  }).join(' ');
  // Check token/context eligibility once per policy, before selecting a
  // delimiter. Repeating parser expectation queries for each punctuation
  // delimiter consumes the parse budget without changing the decision.
  const openingDecision = (entry) => {
    const prefix = entry.prefix + (entry.delimiter ?? '');
    const condition = all(...(prefix ? [next(literal(prefix))] : []), ...(interpolationStartToken ? [some(not(present(contexts)), equal(top(contexts), integer(0)))] : []), ...(entry.gate === false ? [] : [available(openGateToken)]), ...(entry.guard ? [entry.guard.replaceAll('$leading', variable(leading)).replace(/\(valid ([A-Za-z_]+)\)/gu, (_, token) => available(token))] : []));
    const choices = modes.filter((mode) => mode.prefix === entry.prefix && mode.token === entry.token && mode.context === entry.context && (entry.delimiter === undefined || mode.opening === entry.delimiter)).map((mode) => {
      const condition = all(next(literal(mode.prefix + mode.opening)), ...(mode.delimiterGuard ? [mode.delimiterGuard.replaceAll('$leading', variable(leading)).replace(/\(valid ([A-Za-z_]+)\)/gu, (_, token) => available(token))] : []));
      // An empty closing delimiter describes an input-ended literal. Its
      // opening whitespace must not create a nested delimiter context.
      const rememberedOpening = mode.closing === '' ? '' : mode.opening;
      return branch(condition, `(consume ${literal(mode.prefix + mode.opening)}) (push ${contexts} (integer ${mode.context})) (push ${openingsStack} (text ${encode(rememberedOpening)})) (push ${closingsStack} (text ${encode(mode.closing)})) (push ${levels} (integer 1)) ${emit(mode.token)}`);
    }).join(' ');
    return branch(condition, choices);
  };
  const openingGroups = new Map();
  for (const entry of policies.toSorted((first, second) => second.prefix.length - first.prefix.length)) {
    if (!openingGroups.has(entry.token)) openingGroups.set(entry.token, []);
    openingGroups.get(entry.token).push(entry);
  }
  // Select the requested token once, then preserve its ordered policies.
  // A successful selection does not visit the unrelated token groups.
  const openings = [...openingGroups].reduceRight((otherwise, [token, entries]) => branch(valid(token), `${entries.map(openingDecision).join(' ')} fail`, otherwise), null);
  const lexicalDecision = (entry, shared = false) => {
    const tokenPattern = pattern(entry.pattern);
    const suffix = entry.markBeforeSuffix ? ` ${literal(entry.markBeforeSuffix)}` : '';
    // Applicability must not consume or fail the whole scanner: a later
    // lexical policy may be the fallback for the same token and spelling.
    const probe = entry.following ? `(seq ${tokenPattern}${suffix} (and ${pattern(entry.following)}))` : tokenPattern;
    const eligibility = shared ? [next(probe), available(entry.token)] : [next(probe)];
    return branch(all(...eligibility, ...(entry.guard ? [entry.guard.replaceAll('$leading', variable(leading)).replace(/\(valid ([A-Za-z_]+)\)/gu, (_, token) => available(token))] : [])), `(consume ${tokenPattern})${entry.markBeforeSuffix ? ` mark (consume ${literal(entry.markBeforeSuffix)})` : ''}${entry.after ? ` (if (not ${entry.after}) (then fail))` : ''} ${emit(entry.token)}`);
  };
  const decisionGroups = new Map();
  for (const entry of lexical) {
    const group = lexicalGroups.get(entry.token) ?? entry.token;
    if (!decisionGroups.has(group)) decisionGroups.set(group, []);
    decisionGroups.get(group).push(entry);
  }
  const lexicalDecisions = [...decisionGroups].reduceRight((otherwise, [group, entries]) => {
    if (typeof group === 'string') return branch(valid(group), entries.map((entry) => lexicalDecision(entry)).join(' '), otherwise);
    // A competing parser request must observe the same ordered decision.
    // Emitting the preferred available token also rejects a request for a
    // lower-priority alternative, as a scanner with shared validity bits does.
    const decisions = entries.map((candidate) => lexicalDecision(candidate, true)).join(' ');
    return branch(some(...group.map(valid)), `${decisions} fail`, otherwise);
  }, null) ?? '';
  const line = branch(all(valid(lineBreakToken), not(available(noLineBreakToken))), `mark (consume ${literal('\n')}) (while (next ${spaces}) (do advance)) ${branch(next(literal('.')), `advance ${branch(not(next(literal('.'))), 'fail')}`)} ${branch(some(next(literal('&')), next(literal('#'))), 'fail')} ${emit(lineBreakToken)}`);
  let hereRules = '';
  let hereOperations = '';
  let beginBody = '';
  if (heredoc) {
    const { startToken, bodyStartToken, contentToken: hereContent, endToken: hereEnd, opening, modifiers, quotes, labelPattern, quotedLabelPattern, allowEnd } = heredoc;
    if (![startToken, bodyStartToken, hereContent, hereEnd].every((token) => tokens.includes(token)) || !Array.isArray(modifiers) || !Array.isArray(quotes) || typeof allowEnd !== 'boolean') throw new TypeError('invalid queued body descriptor');
    const labels = `${name}_labels`;
    const indentation = `${name}_indentation`;
    const interpolationStack = `${name}_interpolation`;
    const started = `${name}_started`;
    const queueStacks = [labels, indentation, interpolationStack, started];
    const select = `(while (greater (depth ${labels}) (integer 1)) (do ${queueStacks.map((stack) => `(push ${stack}_pending (top ${stack})) (pop ${stack})`).join(' ')}))`;
    const restore = `(while (greater (depth ${labels}_pending) (integer 0)) (do ${queueStacks.map((stack) => `(push ${stack} (top ${stack}_pending)) (pop ${stack}_pending)`).join(' ')}))`;
    const modes = [{ name: 'bare', quote: '', interpolated: true }, ...quotes];
    const starts = [];
    for (const [modifierIndex, modifier] of modifiers.entries()) {
      for (const [modeIndex, mode] of modes.entries()) {
        const labelRule = `${name}_${modifier.name ?? modifierIndex}_${mode.name ?? modeIndex}_label`;
        const label = pattern(mode.quote ? quotedLabelPattern.replaceAll('$quote', mode.quote.replace(/[\\\]\-^]/gu, '\\$&')) : labelPattern);
        const action = `(push ${labels} matched) (push ${indentation} (integer ${modifier.indented ? 1 : 0})) (push ${interpolationStack} (integer ${mode.interpolated ? 1 : 0})) (push ${started} (integer 0))`;
        hereRules += `(rule ${labelRule} silent ${label} (action ${action}))\n`;
        starts.push(`(seq ${literal(opening + modifier.text + mode.quote)} (ref ${labelRule})${mode.quote ? ` ${literal(mode.quote)}` : ''})`);
      }
    }
    hereRules += `(rule ${startToken} token (choice ordered ${starts.join(' ')}))\n`;
    const matching = `(predicate ${pattern(labelPattern)} (equal matched (top ${labels})))`;
    // Quoted labels are exact remembered text, and can contain spaces.
    const quotedMatching = `(predicate ${pattern('[^\\r\\n]+')} (equal matched (top ${labels})))`;
    const word = `(choice ordered ${matching} ${quotedMatching})`;
    const padding = `(predicate (repeat0 ${horizontal}) (equal (top ${indentation}) (integer 1)))`;
    const closing = `(seq (optional ${padding}) ${word} (and (seq (repeat0 ${horizontal}) (choice unordered ${newline} (not (class negated))))))`;
    const ready = `${name}_end_ready`;
    const boundary = all(some(equal('column', integer(0)), equal(variable(ready), integer(1))), next(closing));
    const popQueue = queueStacks.map((stack) => `(pop ${stack})`).join(' ');
    const hasHereContent = equal(variable(`${name}_here_seen`), integer(1));
    const emitHereContent = `${restore} ${emit(hereContent)}`;
    const atEnd = allowEnd ? `${branch(hasHereContent, `mark ${emitHereContent}`)} ${popQueue} ${restore} ${emit(hereEnd)}` : 'fail';
    const hereMove = branch(all(equal(top(interpolationStack), integer(1)), next(literal('#'))), `mark advance ${branch(next(short), `${branch(hasHereContent, emitHereContent)} mark ${restore} ${emit(shortInterpolationToken)}`)}`, 'advance');
    const bodyScanner = `${set(activeBodyInterpolation, top(interpolationStack))} ${set(`${name}_here_seen`, integer(0))} (while (not atEnd) (do ${branch(boundary, `(consume (optional ${padding})) ${set(ready, integer(1))} mark ${branch(hasHereContent, emitHereContent)} (consume ${word}) mark ${set(ready, integer(0))} ${set(activeBodyInterpolation, integer(0))} ${popQueue} ${restore} ${emit(hereEnd)}`)} ${branch(all(equal(top(interpolationStack), integer(1)), some(next(literal(interpolation)), next(literal(escape)))), `${branch(hasHereContent, `mark ${emitHereContent}`)} fail`)} ${hereMove} mark ${set(`${name}_here_seen`, integer(1))})) ${atEnd}`;
    hereOperations = branch(all(present(labels), not(valid(openGateToken)), some(valid(hereContent), valid(hereEnd), valid(shortInterpolationToken))), `${select} ${bodyScanner}`);
    beginBody = branch(all(present(labels), available(bodyStartToken), next(newline)), `${select} ${branch(equal(top(started), integer(0)), `(pop ${started}) (push ${started} (integer 1)) ${set(activeBodyInterpolation, top(interpolationStack))} ${restore} ${emit(bodyStartToken)}`)} ${restore}`);
  }
  if (lineContinuationPattern !== null && (typeof lineContinuationPattern !== 'string' || !lineContinuationPattern || new RegExp(`^(?:${lineContinuationPattern})$`, 'u').test(''))) throw new TypeError('line continuations need a consuming pattern');
  const continuation = lineContinuationPattern === null ? null : pattern(lineContinuationPattern);
  const skipPlainWhitespace = `${beginBody} ${branch(next(literal('\n')), line)} (skip ${spaces}) ${set(leading, integer(1))}`;
  const skipNextWhitespace = continuation === null ? skipPlainWhitespace : branch(next(continuation), `(skip ${continuation}) ${set(leading, integer(1))}`, skipPlainWhitespace);
  const skipWhitespace = `${set(leading, integer(0))} (while ${continuation === null ? next(spaces) : some(next(spaces), next(continuation))} (do ${skipNextWhitespace}))`;
  const transient = [seen, leading, `${name}_here_seen`, `${name}_level`];
  const clearTransient = transient.map((key) => set(key, integer(0))).join(' ');
  const operations = `${branch(all(present(contexts), not(valid(openGateToken)), some(valid(contentToken), valid(endToken), valid(shortInterpolationToken))), body)} ${hereOperations} ${skipWhitespace} ${lexicalDecisions} ${openings} fail`.replace(/\(emit ([A-Za-z_]+)\)/gu, (_, token) => `${clearTransient} ${emit(token)}`);
  const interpolationAllowed = some(...bodyModes.flatMap((mode) => mode.interpolated ? [equal(top(contexts), integer(mode.context))] : []), all(not(present(contexts)), equal(variable(activeBodyInterpolation), integer(1))));
  const boundaryRules = interpolationStartToken ? `(rule ${interpolationStartToken} token (predicate ${literal(interpolation)} ${interpolationAllowed}) (action (push ${contexts} (integer 0))))\n(rule ${interpolationEndToken} token ${literal(interpolationClosing)} (action (if (not (equal (top ${contexts}) (integer 0))) (then fail)) (pop ${contexts})))\n` : '';
  return `(scanner ${name} (tokens ${tokens.filter((token) => ![heredoc?.startToken, interpolationStartToken, interpolationEndToken].includes(token)).join(' ')}) (operations ${operations}))\n${hereRules}${boundaryRules}`;
}
