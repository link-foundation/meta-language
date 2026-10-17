// Finite syntax decisions are generated from the shared LiNo profile.
import { readFileSync, writeFileSync } from 'node:fs';
import { readProjectData } from './project-data.mjs';
const profile = readProjectData(new URL('../src/data/resource-description-ntriples-profile.lino', import.meta.url));
const base = profile.characterRanges.map(([low, high]) => '(code >= ' + low + ' && code <= ' + high + ')').join(' || ');
const source = '// Generated from the linked N-Triples profile. Do not edit by hand.\n'
  + '/** @param {number} code @param {boolean} first @returns {boolean} */\n'
  + 'export function resourceDescriptionNTriplesName(code, first) {\n'
  + '  if (' + base + ' || code === 95 || code === 58 || code >= 48 && code <= 57) return true;\n'
  + '  return !first && (code === 45 || code === 46 || code === 183 || code >= 768 && code <= 879 || code >= 8255 && code <= 8256);\n}\n\n'
  + '/** @param {number} code @returns {number} */\nexport function resourceDescriptionNTriplesEscape(code) {\n'
  + profile.literalEscapes.map(([code, value]) => '  if (code === ' + code + ') return ' + value + ';').join('\n')
  + '\n  return -1;\n}\n\n'
  + '/** @param {number} state @param {number} kind @returns {number} */\nexport function readResourceDescriptionNTriplesTransition(state, kind) {\n'
  + '  if (kind === 9 && (' + profile.whitespaceStates.map(state => 'state === ' + state).join(' || ') + ')) return state;\n'
  + profile.transitions.map(([state,kind,next]) => '  if (state === ' + state + ' && kind === ' + kind + ') return ' + next + ';').join('\n')
  + '\n  return -1;\n}\n\n'
  + '/** @param {number} state @param {number} kind @returns {number} */\nexport function readResourceDescriptionNTriplesRole(state, kind) {\n'
  + profile.transitions.filter(([, , ,role]) => role).map(([state,kind,,role]) => '  if (state === ' + state + ' && kind === ' + kind + ') return ' + role + ';').join('\n')
  + '\n  return 0;\n}\n\n'
  + '/** @param {number} state @returns {boolean} */\nexport function acceptResourceDescriptionNTriplesFinalState(state) {\n  return '
  + profile.finalStates.map(state => 'state === ' + state).join(' || ') + ';\n}\n';
const file = new URL('../src/resource-description-ntriples-characters.js', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(file, 'utf8') !== source) throw new Error('the N-Triples character decisions are stale');
} else writeFileSync(file, source);
