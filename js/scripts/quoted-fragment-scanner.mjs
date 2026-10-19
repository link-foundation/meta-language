// Stateless fragments between grammar-owned quotes retain marked lookahead,
// interpolation/escape boundaries and optional line-anchored closing runs.
const literal = value => `(literal ${[...Buffer.from(value)].map(byte => /[A-Za-z0-9_.-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`).join('') || '%'})`;
const branch = (condition, yes, no = '') => `(if ${condition} (then ${yes})${no ? ` (else ${no})` : ''})`;
const loop = (condition, body) => `(while ${condition} (do ${body}))`;
const next = value => `(next ${literal(value)})`;

export function quotedFragmentScanner({ name, token, closing, interpolation = '#{',
  supportsInterpolation = true, escape = '\\', closingAfterNewline = false,
  whitespace = [' ', '\t', '\n', '\r'] }) {
  if ([name, token].some(value => typeof value !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(value))
    || typeof closing !== 'string' || !closing || new Set([...closing]).size !== 1
    || typeof interpolation !== 'string' || [...interpolation].length !== 2
    || typeof escape !== 'string' || [...escape].length !== 1
    || ![supportsInterpolation, closingAfterNewline].every(value => typeof value === 'boolean')
    || !Array.isArray(whitespace) || !whitespace.length) throw new TypeError('invalid quoted fragment profile');
  const width = [...closing].length;
  const character = [...closing][0];
  const [interpolationPrefix, interpolationEnd] = [...interpolation];
  const seen = `${name}_content`, line = `${name}_newline`, count = `${name}_closing_width`;
  const set = (key, value) => `(set ${key} (integer ${value}))`;
  const value = key => `(variable ${key})`;
  const equals = (key, number) => `(equal ${value(key)} (integer ${number}))`;
  const emit = `${branch(equals(seen, 0), 'fail')} (emit ${token})`;
  const spaces = `(choice unordered ${whitespace.map(literal).join(' ')})`;
  // The common operation language has no imperative break: the candidate
  // loop remembers an unmatched prefix and stops before the next iteration.
  const measuring = `${set(count, 1)} ${set(`${name}_measuring`, 1)} ${loop(`(all (less ${value(count)} (integer ${width})) (equal (variable ${name}_measuring) (integer 1)))`, `advance ${branch(next(character), `(set ${count} (add ${value(count)} (integer 1)))`, set(`${name}_measuring`, 0))}`)}`;
  const close = `${measuring} ${branch(`(all ${equals(count, width)}${closingAfterNewline ? ` ${equals(line, 1)}` : ''})`, emit)}`;
  const escaped = `advance ${closingAfterNewline ? branch(next('\n'), '', branch(supportsInterpolation ? '(equal (integer 1) (integer 1))' : next(character), emit)) : supportsInterpolation ? emit : branch(next(character), emit)}`;
  const nonClosing = branch(next(interpolationPrefix), `advance ${supportsInterpolation ? branch(next(interpolationEnd), emit) : ''}`,
    branch(next(escape), escaped, branch('(some atEnd (next (literal %00)))', emit, 'advance')));
  const linePrefix = branch('(some (next (literal %0A)) (next (literal %0D)))', `advance ${set(seen, 1)} ${set(line, 1)} ${loop(`(next ${spaces})`, 'advance')}`);
  return `(scanner ${name} (tokens ${token}) (operations (if (not (valid ${token})) (then fail)) ${set(seen, 0)} ${loop('(equal (integer 1) (integer 1))', `${set(line, 0)} ${linePrefix} mark ${branch(next(character), close, nonClosing)} ${set(seen, 1)}`)} fail))\n`;
}
