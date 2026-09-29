// Prints the WordNet 3.1 parts of speech of each word given on the command line.
import { loadWordNet, partsOfSpeech } from '../js/scripts/english-vocabulary.mjs';
const wordnet = loadWordNet();
for (const word of process.argv.slice(2)) console.log(word.padEnd(24), partsOfSpeech(wordnet, word).join(', ') || '-');
