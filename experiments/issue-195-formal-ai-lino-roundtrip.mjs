// Reproduces the formal-ai Links Notation parity failures in the JavaScript runtime.
// Usage: node experiments/issue-195-formal-ai-lino-roundtrip.mjs <file.lino>
import { readFileSync } from 'node:fs';
import { LinkNetwork, LinkType, TranslationRuleSet } from '../js/src/index.js';

const text = readFileSync(process.argv[2], 'utf8');
const network = LinkNetwork.parse(text, 'lino');
const named = (net) => [...net.links()].filter((link) => link.metadata().linkType === LinkType.Syntax && link.metadata().named).length;
console.log('parsed named', named(network), 'roundTrip', network.reconstructText() === text);
try {
  const reloaded = LinkNetwork.fromLino(network.toLino());
  console.log('reloaded named', named(reloaded), 'roundTrip', reloaded.reconstructText() === text);
} catch (error) { console.log('reload error', error.message); }
try { console.log('seed links', [...LinkNetwork.fromLino(text).links()].length); } catch (error) { console.log('fromLino error', error.message); }
try { console.log('rules', TranslationRuleSet.fromLino(text).rules.length); } catch (error) { console.log('rules error', error.message); }
