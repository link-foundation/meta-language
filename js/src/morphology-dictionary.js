import { createHash } from 'node:crypto';
import { readFileSync, statSync } from 'node:fs';
import { join } from 'node:path';

let source;

export function russianMorphologySource() {
  if (!source) {
    const rows = readFileSync(new URL('./data/morphology/open-corpora.lino', import.meta.url), 'utf8')
      .trim().split('\n').map(line => line.slice(1, -1).split(' '));
    const [[, , version, sha256, url], ...files] = rows;
    source = { version, sha256, url, files: new Map(files.map(([, name, size, digest]) => [name, { size: Number(size), digest }])) };
  }
  return source;
}

// OpenCorpora's compiled dictionary 2.4 stores Darts units and a completion
// guide. A record key ends with byte 1 and a base64-encoded big-endian tuple.
class WordDictionary {
  constructor(bytes, records = 0) {
    const size = bytes.readUInt32LE(0);
    const end = 4 + size * 4;
    if (end + 4 > bytes.length || bytes.readUInt32LE(end) !== size ||
        end + 4 + size * 2 !== bytes.length) throw new Error('invalid compiled word dictionary');
    this.units = bytes.subarray(4, end);
    this.guide = bytes.subarray(end + 4);
    this.records = records;
  }

  transition(index, label) {
    const unit = this.units.readUInt32LE(index * 4);
    const offset = (unit >>> 10) << ((unit & 512) >>> 6);
    const next = (index ^ offset ^ label) >>> 0;
    if (next * 4 + 4 <= this.units.length &&
        (this.units.readUInt32LE(next * 4) & 0x800000ff) === label) return next;
    return undefined;
  }

  follow(index, key) {
    for (const label of Buffer.from(key)) {
      index = this.transition(index, label);
      if (index === undefined) break;
    }
    return index;
  }

  value(key) {
    const index = this.follow(0, key);
    if (index === undefined) return 0;
    const unit = this.units.readUInt32LE(index * 4);
    if (!(unit & 256)) return 0;
    const offset = (unit >>> 10) << ((unit & 512) >>> 6);
    return this.units.readUInt32LE(((index ^ offset) >>> 0) * 4) & 0x7fffffff;
  }

  *payloads(index, bytes = []) {
    if (this.units.readUInt32LE(index * 4) & 256) {
      const payload = Buffer.from(Buffer.from(bytes).toString('ascii'), 'base64');
      if (payload.length === this.records) yield payload;
    }
    if (bytes.length >= 16) return;
    for (let byte = this.guide[index * 2]; byte;) {
      const child = this.transition(index, byte);
      if (child === undefined) throw new Error('invalid word dictionary completion');
      bytes.push(byte);
      yield* this.payloads(child, bytes);
      bytes.pop();
      byte = this.guide[child * 2 + 1];
    }
  }

  *matches(remaining, spelling = '', index = 0) {
    if (!remaining) {
      const key = this.transition(index, 1);
      if (key !== undefined) for (const payload of this.payloads(key)) yield [spelling, payload];
      return;
    }
    const character = String.fromCodePoint(remaining.codePointAt(0));
    for (const alternative of character === 'е' ? ['е', 'ё'] : [character]) {
      const next = this.follow(index, alternative);
      if (next !== undefined) yield* this.matches(remaining.slice(character.length), spelling + alternative, next);
    }
  }
}

function loadParadigms(bytes) {
  const count = bytes.readUInt16LE(0);
  const paradigms = [];
  let offset = 2;
  for (let index = 0; index < count; index += 1) {
    const width = bytes.readUInt16LE(offset);
    offset += 2;
    if (!width || width % 3 !== 0 || offset + width * 2 > bytes.length) throw new Error('invalid compiled morphology paradigm');
    paradigms.push(Uint16Array.from({ length: width }, (_, item) => bytes.readUInt16LE(offset + item * 2)));
    offset += width * 2;
  }
  if (offset !== bytes.length) throw new Error('unexpected compiled morphology paradigm data');
  return paradigms;
}

export function openRussianMorphology(directory) {
  const { files } = russianMorphologySource();
  const read = name => {
    const { size, digest } = files.get(name);
    const file = join(directory, name);
    if (statSync(file).size !== size) throw new Error(`unexpected compiled morphology size for ${name}`);
    const bytes = readFileSync(file);
    if (createHash('sha256').update(bytes).digest('hex') !== digest) throw new Error(`unexpected compiled morphology digest for ${name}`);
    return bytes;
  };
  const metadata = Object.fromEntries(JSON.parse(read('meta.json')));
  if (metadata.language_code !== 'ru' || metadata.format_version !== '2.4') throw new Error('expected compiled Russian morphology format 2.4');
  const words = new WordDictionary(read('words.dawg'), 4);
  const probabilities = new WordDictionary(read('p_t_given_w.intdawg'));
  const paradigms = loadParadigms(read('paradigms.array'));
  const suffixes = JSON.parse(read('suffixes.json'));
  const tags = JSON.parse(read('gramtab-opencorpora-int.json'));
  const prefixes = metadata.compile_options.paradigm_prefixes;
  let endings;

  function paradigmForm(paradigm, index) {
    const length = paradigm.length / 3;
    return { prefix: prefixes[paradigm[length * 2 + index]], suffix: suffixes[paradigm[index]], tag: tags[paradigm[length + index]] };
  }

  function reading(spelling, paradigmIndex, formIndex, weight) {
    const paradigm = paradigms[paradigmIndex];
    const form = paradigmForm(paradigm, formIndex);
    const root = spelling.slice(form.prefix.length, spelling.length - form.suffix.length);
    const normal = paradigmForm(paradigm, 0);
    return { lemma: normal.prefix + root + normal.suffix, ...form, spelling, paradigmIndex, root, weight, formIndex };
  }

  function analyze(word) {
    if (word.length > 128) return [];
    const exact = Array.from(words.matches(word), ([spelling, payload]) => reading(spelling, payload.readUInt16BE(0), payload.readUInt16BE(2), 1));
    if (exact.length) {
      const priors = exact.map(({ tag }) => probabilities.value(`${word}:${tag}`));
      if (priors.some(Boolean)) exact.forEach((result, index) => { result.weight = priors[index]; });
      return exact.map(result => ({ ...result, source: 'dictionary' }));
    }
    endings ??= prefixes.map((_, index) => new WordDictionary(read(`prediction-suffixes-${index}.dawg`), 8));
    const predictions = [];
    for (let index = 0; index < prefixes.length; index += 1) {
      if (!word.startsWith(prefixes[index])) continue;
      const previousCount = predictions.length;
      const maximum = Math.min(metadata.compile_options.max_suffix_length, word.length - prefixes[index].length - 1);
      for (let length = maximum; length > 0; length -= 1) {
        const stem = word.slice(0, -length);
        for (const [ending, payload] of endings[index].matches(word.slice(-length))) {
          const result = reading(stem + ending, payload.readUInt16BE(4), payload.readUInt16BE(6), payload.readUInt32BE(0));
          if (result.root && result.prefix === prefixes[index]) predictions.push({ ...result, source: 'suffix' });
        }
        if (predictions.length > previousCount) break;
      }
    }
    return predictions;
  }

  function inflections(analysis) {
    const paradigm = paradigms[analysis.paradigmIndex];
    return Array.from({ length: paradigm.length / 3 }, (_, index) => {
      const { prefix, suffix, tag } = paradigmForm(paradigm, index);
      return { form: prefix + analysis.root + suffix, tag };
    });
  }

  return { analyze, inflections };
}
