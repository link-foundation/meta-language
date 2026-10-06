// Mutates the committed naming registers and inventory in memory and prints
// the problems the naming check reports for each mutation.
import { loadWordNet } from '../js/scripts/english-vocabulary.mjs';
import { checkRepositoryNames, extractNameInventory, loadNamingRegisters } from '../js/scripts/issue-195-naming.mjs';

const wordnet = loadWordNet();
const { records } = loadNamingRegisters('.');
const names = extractNameInventory('.');
const base = (id) => structuredClone(records.find((record) => record.id === id));
const mutations = {
  'abbreviated identity': { records: [...records.filter((r) => r.id !== 'grammar.character-class'), { ...base('grammar.character-class'), id: 'grammar.char-class', phrase: 'char class' }] },
  'semantic duplicate (synonym)': { records: [...records, { ...base('grammar.nonterminal'), id: 'grammar.token', phrase: 'token', definition: 'A lexical unit.' }] },
  'duplicate definition': { records: [...records, { ...base('loop'), id: 'iteration', phrase: 'iteration' }] },
  'ambiguous phrase': { records: [...records, { ...base('grammar.string'), id: 'text.string', definition: 'A text.' }] },
  'unknown word': { records: [...records, { ...base('loop'), id: 'grammar.foobaz', phrase: 'foobaz', definition: 'A foobaz.' }] },
  'wrong role': { records: [...records, { ...base('operation.parse'), id: 'operation.parsing-quickly', phrase: 'parsing quickly', role: 'concept', definition: 'Parse fast.' }] },
  'source name without record': { names: [...names, { inventory: 'grammar constructs', file: 'rust/src/grammar/fidelity.rs', name: 'lookahead', record: 'grammar.lookahead' }] },
  'abbreviated inferred rule name': { names: [...names, { inventory: 'inferred rule names', file: 'rust/src/grammar/inference/advisor.rs', name: 'seq_2', record: null }] },
  'former name still in use': { names: [...names, { inventory: 'grammar constructs', file: 'rust/src/grammar/fidelity.rs', name: 'char-class', record: 'grammar.char-class' }] },
};
for (const [label, override] of Object.entries(mutations)) {
  const { problems } = checkRepositoryNames('.', wordnet, override);
  console.log(`${label}: ${problems.length}`);
  for (const problem of problems) console.log(`  ${problem.kind} | ${problem.message}`);
}
