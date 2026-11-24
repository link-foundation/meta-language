import assert from 'node:assert/strict';
import { LinkNetwork, agreeWithNumber, analyzeMorphology, inflect } from '../../js/src/index.js';

const language = 'English';
const text = '📚 Engineers went and saw teeth.';
const network = LinkNetwork.parse(text, language);
assert.equal(network.reconstructText(), text);
assert.ok(network.verifyFullMatch().isClean());
assert.deepEqual(analyzeMorphology('went', language).map(reading => reading.lemma), ['go']);
assert.ok(analyzeMorphology('saw', language).some(({ lemma }) => lemma === 'see'));
assert.ok(analyzeMorphology('saw', language).some(({ lemma, upos }) => lemma === 'saw' && upos === 'NOUN'));
assert.equal(inflect('tooth', { Number: 'Plur' }, language), 'teeth');
assert.equal(agreeWithNumber('person', 5, language), 'people');
console.log('English dictionary readings and inverse inflections preserve the source text.');
