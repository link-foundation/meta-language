import { Grammar } from './grammar.js';
import { findGrammarDistinguishingObservation } from './grammar-active-learning-rules.js';

const defaults = Object.freeze({ maxWordLen: 8, equivalenceSamples: 256, seed: 0xa17e1eafdfa5eed5n, maxIterations: 128, useTtt: false });

function activeLearningConfig(options) {
  const config = { ...defaults, ...options };
  for (const key of ['maxWordLen', 'equivalenceSamples', 'maxIterations']) {
    if (!Number.isSafeInteger(config[key]) || config[key] < (key === 'maxIterations' ? 1 : 0)) {
      throw new RangeError('invalid grammar active learning limit ' + key);
    }
  }
  if (typeof config.seed !== 'bigint' || config.seed !== BigInt.asUintN(64, config.seed)) {
    throw new RangeError('grammar active learning seed requires an unsigned 64-bit integer');
  }
  if (typeof config.useTtt !== 'boolean') throw new TypeError('grammar active learning needs a Boolean TTT selection');
  if (config.useTtt) throw new TypeError('TTT grammar active learning is unavailable');
  return config;
}

/** Words passed to oracles retain Unicode scalar boundaries and input order. */
function validateGrammarAlphabet(alphabet) {
  if (!Array.isArray(alphabet) || alphabet.some(symbol => typeof symbol !== 'string' || !symbol.isWellFormed() || [...symbol].length !== 1)) {
    throw new TypeError('grammar active learning needs an ordered Unicode scalar alphabet');
  }
  if (new Set(alphabet).size !== alphabet.length) throw new TypeError('grammar active learning alphabet has duplicate symbols');
}

export function acceptActiveGrammarWord(hypothesis, word) {
  let state = hypothesis.start;
  const within = state => Number.isInteger(state) && state >= 0 && state < hypothesis.states;
  if (!within(state)) return false;
  for (const symbol of word) {
    const column = hypothesis.alphabet.indexOf(symbol);
    if (column < 0) return false;
    state = hypothesis.delta[state]?.[column];
    if (!within(state)) return false;
  }
  return hypothesis.accepting[state] === true;
}

function splitMix64(seed) {
  let state = seed;
  return upper => {
    state = BigInt.asUintN(64, state + 0x9e3779b97f4a7c15n);
    let value = BigInt.asUintN(64, (state ^ (state >> 30n)) * 0xbf58476d1ce4e5b9n);
    value = BigInt.asUintN(64, (value ^ (value >> 27n)) * 0x94d049bb133111ebn);
    return Number((value ^ (value >> 31n)) % BigInt(upper));
  };
}

/** Seeded equivalence candidate order matches the native regular learner. */
export function sampleActiveGrammarWords(alphabet, options = {}) {
  validateGrammarAlphabet(alphabet);
  const config = activeLearningConfig(options);
  const limit = config.equivalenceSamples;
  if (limit === 0) return [];
  const words = new Set(['']);
  for (let length = 1; length <= config.maxWordLen && words.size < limit; length += 1) {
    for (const symbol of alphabet) {
      words.add(symbol.repeat(length));
      if (words.size === limit) break;
    }
    if (!alphabet.length) break;
  }
  if (alphabet.length && words.size < limit) {
    const random = splitMix64(config.seed);
    const target = words.size + Math.floor((limit - words.size) / 2);
    for (let attempt = 0; words.size < target && attempt < limit * 32; attempt += 1) {
      const length = random(config.maxWordLen + 1);
      let word = '';
      for (let index = 0; index < length; index += 1) word += alphabet[random(alphabet.length)];
      words.add(word);
    }
    let layer = [''];
    for (let length = 1; length <= config.maxWordLen && words.size < limit; length += 1) {
      const next = [];
      for (const prefix of layer) {
        for (const symbol of alphabet) {
          const word = prefix + symbol;
          words.add(word);
          next.push(word);
          if (words.size === limit) return Array.from(words, word => [...word]);
        }
      }
      layer = next;
    }
  }
  return Array.from(words, word => [...word]);
}

export function learnActiveGrammarAutomaton(oracle, options = {}) {
  const config = activeLearningConfig(options);
  validateGrammarAlphabet(oracle.alphabet);
  const alphabet = [...oracle.alphabet];
  const prefixes = new Set(['']);
  const suffixes = new Set(['']);
  const observations = new Map();
  let membershipQueries = 0;
  let equivalenceQueries = 0;
  const ask = word => {
    const answer = oracle.membership([...word]);
    if (typeof answer !== 'boolean') throw new TypeError('grammar membership query requires a Boolean answer');
    return answer;
  };
  const observe = (prefix, suffix) => {
    let row = observations.get(prefix);
    if (!row) observations.set(prefix, row = new Map());
    if (!row.has(suffix)) {
      membershipQueries += 1;
      row.set(suffix, ask(prefix + suffix));
    }
    return row.get(suffix);
  };
  const signature = prefix => Array.from(suffixes, suffix => observe(prefix, suffix) ? '1' : '0').join('');
  const lowerPrefixes = upper => upper.flatMap(prefix => alphabet.map(symbol => prefix + symbol))
    .filter(prefix => !prefixes.has(prefix));
  const extendPrefixes = word => {
    let prefix = '';
    for (const symbol of word) {
      prefix += symbol;
      prefixes.add(prefix);
    }
  };
  const extendSuffixes = word => {
    const symbols = [...word];
    for (let index = 0; index < symbols.length; index += 1) suffixes.add(symbols.slice(index).join(''));
  };
  function distinguish(upper, rows) {
    for (let first = 0; first < upper.length; first += 1) {
      for (let second = first + 1; second < upper.length; second += 1) {
        if (rows[first] !== rows[second]) continue;
        for (const symbol of alphabet) {
          const left = Array.from(suffixes, suffix => observe(upper[first] + symbol, suffix));
          const right = Array.from(suffixes, suffix => observe(upper[second] + symbol, suffix));
          const index = findGrammarDistinguishingObservation(left, right);
          if (index >= 0) return symbol + [...suffixes][index];
        }
      }
    }
    return undefined;
  }
  function closeTable() {
    for (;;) {
      const upper = [...prefixes];
      const lower = lowerPrefixes(upper);
      const rows = upper.map(signature);
      const extensions = lower.map(signature);
      const known = new Set(rows);
      const unclosed = extensions.findIndex(row => !known.has(row));
      if (unclosed >= 0) {
        extendPrefixes(lower[unclosed]);
        continue;
      }
      const suffix = distinguish(upper, rows);
      if (suffix === undefined) return { upper, rows };
      extendSuffixes(suffix);
    }
  }
  const exact = typeof oracle.equivalence === 'function';
  const samples = exact ? [] : sampleActiveGrammarWords(alphabet, config);
  for (let round = 0; round < config.maxIterations; round += 1) {
    const { upper, rows } = closeTable();
    const uniqueRows = [...new Set(rows)];
    const representatives = uniqueRows.map(row => upper[rows.indexOf(row)]);
    const hypothesis = Object.freeze({
      alphabet: Object.freeze(alphabet),
      states: representatives.length,
      start: uniqueRows.indexOf(signature('')),
      accepting: Object.freeze(representatives.map(prefix => observe(prefix, ''))),
      delta: Object.freeze(representatives.map(prefix => Object.freeze(alphabet.map(symbol =>
        uniqueRows.indexOf(signature(prefix + symbol)))))),
    });
    equivalenceQueries += 1;
    const counterexample = exact
      ? oracle.equivalence(hypothesis, config)
      : samples.find(word => acceptActiveGrammarWord(hypothesis, word) !== ask(word.join('')));
    if (counterexample === undefined) {
      return { hypothesis, membershipQueries, equivalenceQueries, equivalence: exact ? 'oracle' : 'bounded sampling' };
    }
    if (!Array.isArray(counterexample) || counterexample.some(symbol => !alphabet.includes(symbol))) {
      throw new TypeError('grammar counterexample requires a word in the declared alphabet');
    }
    extendPrefixes(counterexample);
  }
  throw new RangeError('grammar active learning exceeded its refinement rounds: ' + config.maxIterations);
}

export function learnActiveGrammar(oracle, options) {
  const result = learnActiveGrammarAutomaton(oracle, options);
  const { hypothesis } = result;
  const rules = new Map();
  const nameOf = state => 'q' + state;
  for (let state = 0; state < hypothesis.states; state += 1) {
    const alternatives = hypothesis.accepting[state] ? [{ kind: 'empty' }] : [];
    hypothesis.delta[state].forEach((destination, column) => alternatives.push({
      kind: 'seq',
      items: [{ kind: 'literal', value: hypothesis.alphabet[column] }, { kind: 'ref', name: nameOf(destination) }],
    }));
    const name = nameOf(state);
    rules.set(name, {
      name,
      kind: 'normal',
      expression: alternatives.length === 1 ? alternatives[0] : { kind: 'choice', ordered: false, items: alternatives },
    });
  }
  return { ...result, grammar: new Grammar(nameOf(hypothesis.start), rules, 'inferred', { matching: 'generalized' }) };
}
