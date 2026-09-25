// Compares the shared trigram identifier with franc-min on the inventory
// fixtures and assorted sentences; they must agree whenever the text has no
// astral characters and fits in franc's sample window.
import { francAll } from 'franc-min';
import { identifyLanguage } from '../src/language-identification.js';
import { LANGUAGE_TRIGRAMS } from '../src/language-trigrams.js';
import { readFileSync } from 'node:fs';

const codes = Object.keys(LANGUAGE_TRIGRAMS.languages);
const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url)));
const samples = [
  ...inventory.languages.filter((entry) => entry.family === 'natural').map((entry) => entry.source),
  'Hawaii is a state.', 'Гавайи это штат.', 'Le chat mange la souris.', 'O gato come o rato.',
  'El gato come al ratón.', 'The cat eats the mouse.', 'मैं घर जाता हूँ।', 'আমি বাড়ি যাই।', '我喜欢学习。',
  'یہ ایک کتاب ہے۔', 'هذا كتاب.', '12345', '', '!!!', 'ok', 'Ωμέγα',
];
let disagreements = 0;
for (const sample of samples) {
  const [[code, score]] = francAll(sample, { only: codes, minLength: 1 });
  const franc = score > 0 ? LANGUAGE_TRIGRAMS.languages[code] : undefined;
  const ours = identifyLanguage(sample);
  const mark = franc === ours ? 'ok ' : 'DIFF';
  if (franc !== ours) disagreements += 1;
  console.log(mark, JSON.stringify(sample), franc, ours);
}
console.log('disagreements', disagreements);
