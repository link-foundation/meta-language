import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { openRussianMorphology } from '../src/morphology-dictionary.js';

const [directory, option] = process.argv.slice(2);
assert.ok(directory && (!option || option === '--check'), 'provide an extracted OpenCorpora dictionary directory');
const dictionary = openRussianMorphology(directory);
const lexemes = [
  'инженер', 'разработать', 'разрабатывать', 'разработка', 'новый', 'новое', 'сервис', 'перевести',
  'их', 'они', 'он', 'она', 'я', 'ты', 'вы', 'мы', 'и', 'в', 'на', 'к', 'из', 'от', 'с', 'по', 'до',
  'облако', 'год', 'человек', 'день', 'ребёнок', 'время', 'путь', 'ночь', 'стол', 'кошка', 'книга',
  'слово', 'замок', 'ёлка', 'идти', 'быть', 'делать', 'работать', 'читать', 'сказать', 'знать',
];
const forms = new Set(lexemes);
for (const lemma of lexemes) {
  for (const reading of dictionary.analyze(lemma)) {
    if (reading.lemma !== lemma) continue;
    for (const { form, tag } of dictionary.inflections(reading)) {
      if (!tag.includes('Abbr') && form && !/\s/u.test(form)) forms.add(form);
    }
  }
}
const rows = [];
for (const form of [...forms].sort()) {
  for (const reading of dictionary.analyze(form)) {
    if (reading.source === 'dictionary' && reading.spelling === form) {
      rows.push(`(opencorpora ${form} ${reading.lemma} ${reading.weight} ${reading.tag.replaceAll(' ', ',')})`);
    }
  }
}
const result = `${rows.join('\n')}\n`;
for (const path of ['../../js/src/data/morphology/russian.lino', '../../rust/src/data/morphology/russian.lino']) {
  const file = new URL(path, import.meta.url);
  if (option) assert.equal(readFileSync(file, 'utf8'), result);
  else writeFileSync(file, result);
}
