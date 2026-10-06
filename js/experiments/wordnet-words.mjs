// Prints the WordNet parts of speech of each word given.
//   node experiments/wordnet-words.mjs WORD...
import { loadWordNet, partsOfSpeech } from '../scripts/english-vocabulary.mjs';
const wn = loadWordNet();
for (const w of process.argv.slice(2)) console.log(w, partsOfSpeech(wn, w).join(','));
