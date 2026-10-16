// Portable lexical plans operate on complete, source-ordered fact columns.
// Hosts only project inspected Links/AST fields and materialize the returned
// indices. No source-language name or parser expectation enters these rules.
import { readRequiredNumberFact, readRequiredTextFact, readRequiredBooleanFact } from './fact-column-rules.js';

/** @param {string} kind @returns {number} */
export function readGrammarLexerEntryForm(kind) {
  if (kind === 'lexicalOwner') return 1;
  if (kind === 'lexicalBinding') return 2;
  if (kind === 'lexicalLiteral') return 3;
  if (kind === 'lexicalMode') return 4;
  if (kind === 'lexicalChannel') return 5;
  if (kind === 'lexicalObligation') return 6;
  if (kind === 'lexicalRule') return 7;
  if (kind === 'lexicalFragment') return 8;
  if (kind === 'lexicalType') return 9;
  return 0;
}

/** @param {string} text @returns {boolean} */
export function acceptGrammarLexerIdentifier(text) {
  if (text.length === 0) return false;
  let index = 0;
  while (index < text.length) {
    // The loop supplies exactly one UTF-16 unit, so membership cannot accept
    // an empty substring or a multi-unit spelling of an ASCII character.
    const character = text.slice(index, index + 1);
    const letter = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_'.includes(character);
    if (!letter && !(index > 0 && '0123456789'.includes(character))) return false;
    index += 1;
  }
  return true;
}

/** @param {string} text @returns {boolean} */
export function acceptGrammarLexerDecimal(text) {
  if (text.length === 0) return false;
  let index = 0;
  while (index < text.length) {
    const character = text.slice(index, index + 1);
    if (!'0123456789'.includes(character)) return false;
    index += 1;
  }
  return true;
}

/** @param {string[]} values @param {string} value @returns {boolean} */
export function containsGrammarLexerText(values, value) {
  let index = 0;
  while (index < values.length) {
    if (readRequiredTextFact(values, index) === value) return true;
    index += 1;
  }
  return false;
}

/** @param {number[]} values @param {number} value @returns {boolean} */
function containsGrammarLexerPosition(values, value) {
  let index = 0;
  while (index < values.length) {
    if (readRequiredNumberFact(values, index) === value) return true;
    index += 1;
  }
  return false;
}

/**
 * Original lexical rule positions, then -1, then original parser owner positions.
 * @param {number} section @param {string[]} sectionKinds @param {string[]} sectionNames
 * @param {string[]} tokenVocabularies @param {number[]} ruleSections @param {string[]} ruleKinds
 * @returns {number[]}
 */
export function readGrammarLexerSectionRulePositions(section, sectionKinds, sectionNames, tokenVocabularies, ruleSections, ruleKinds) {
  const selected = [];
  let index = 0;
  while (index < ruleSections.length) {
    const kind = readRequiredTextFact(ruleKinds, index);
    if (readRequiredNumberFact(ruleSections, index) === section && (kind === 'token' || kind === 'silent')) selected.push(index);
    index += 1;
  }
  selected.push(-1);
  index = 0;
  while (index < ruleSections.length) {
    const ownerSection = readRequiredNumberFact(ruleSections, index);
    if (readRequiredTextFact(ruleKinds, index) === 'normal' && (ownerSection === section
      || ownerSection >= 0 && readRequiredTextFact(sectionKinds, ownerSection) === 'parser'
        && readRequiredTextFact(tokenVocabularies, ownerSection) === readRequiredTextFact(sectionNames, section))) selected.push(index);
    index += 1;
  }
  return selected;
}

/**
 * A sole original explicit token position, -1 for an implicit combined token,
 * or -2 for an unresolved ambiguous/separate-lexer literal declaration.
 * @param {string} value @param {string} sectionKind @param {string[]} kinds
 * @param {string[]} values @param {boolean[]} literals @returns {number}
 */
export function readGrammarLexerLiteralRulePosition(value, sectionKind, kinds, values, literals) {
  let selected = -1;
  let count = 0;
  let index = 0;
  while (index < kinds.length) {
    if (readRequiredTextFact(kinds, index) === 'token' && readRequiredBooleanFact(literals, index)
      && readRequiredTextFact(values, index) === value) { selected = index; count += 1; }
    index += 1;
  }
  if (count === 1) return selected;
  if (count > 1 || sectionKind === 'lexer') return -2;
  return -1;
}

/**
 * Public token identities in original declaration order. Declared but unused
 * types remain inspectable metadata without synthetic public productions.
 * @param {string[]} declared @param {string[]} references @param {string[]} names
 * @param {string[]} legacyTypes @param {string[]} emittedTypes @param {string[]} dispositions
 * @returns {string[]}
 */
export function readGrammarLexerBindingNames(declared, references, names, legacyTypes, emittedTypes, dispositions) {
  const bindings = [];
  let index = 0;
  while (index < declared.length) {
    const name = readRequiredTextFact(declared, index);
    if (containsGrammarLexerText(references, name) && !containsGrammarLexerText(bindings, name)) bindings.push(name);
    index += 1;
  }
  index = 0;
  while (index < names.length) {
    const name = readRequiredTextFact(names, index);
    const legacy = readRequiredTextFact(legacyTypes, index);
    const emitted = readRequiredTextFact(emittedTypes, index);
    if (!containsGrammarLexerText(bindings, name)) bindings.push(name);
    if (legacy !== '' && !containsGrammarLexerText(bindings, legacy)) bindings.push(legacy);
    if (readRequiredTextFact(dispositions, index) === 'emit' && !containsGrammarLexerText(bindings, emitted)) bindings.push(emitted);
    index += 1;
  }
  return bindings;
}

/**
 * Final type, channel and disposition followed by ordered operation/argument pairs. Unsupported
 * commands remain exact source strings in an explicit obligation pair.
 * @param {string} command @param {string} name @returns {string[]}
 */
export function readGrammarLexerCommandPlan(command, name) {
  let body = command;
  if (body.startsWith('->')) body = body.slice(2);
  let type = name;
  let channel = 'default';
  let disposition = 'emit';
  const actions = [];
  const pieces = body.split(',');
  let index = 0;
  while (index < pieces.length) {
    const piece = readRequiredTextFact(pieces, index).trim();
    const opening = piece.indexOf('(');
    let head = '';
    let argument = '';
    let call = false;
    if (opening >= 0 && piece.endsWith(')')) {
      head = piece.slice(0, opening).trim();
      argument = piece.slice(opening + 1, piece.length - 1).trim();
      call = acceptGrammarLexerIdentifier(head) && (acceptGrammarLexerIdentifier(argument) || acceptGrammarLexerDecimal(argument));
    }
    if (piece === 'skip') { type = 'SKIP'; disposition = 'skip'; }
    else if (piece === 'popMode') actions.push('popMode', '');
    else if (call && head === 'type') { type = argument; disposition = 'emit'; }
    else if (call && head === 'channel') {
      let zero = argument.length > 0;
      let offset = 0;
      while (offset < argument.length) {
        if (argument.slice(offset, offset + 1) !== '0') zero = false;
        offset += 1;
      }
      channel = zero || argument === 'DEFAULT_TOKEN_CHANNEL' ? 'default' : argument;
    }
    else if (call && (head === 'pushMode' || head === 'mode')) {
      actions.push(head === 'mode' ? 'setMode' : 'pushMode', argument === 'DEFAULT_MODE' ? 'default' : argument);
    } else if (piece !== '') actions.push('unsupported', piece);
    index += 1;
  }
  return [type, channel, disposition, ...actions];
}

/** @param {string} candidate @param {string} active @returns {boolean} */
export function acceptGrammarLexerMode(candidate, active) {
  return candidate === active;
}

/** @param {string} channel @param {string} disposition @returns {boolean} */
export function acceptGrammarLexerTrivia(channel, disposition) {
  return disposition === 'skip' || channel !== 'default';
}

/** @param {string} source @param {string} emitted @param {string} disposition @returns {string} */
export function readGrammarLexerTriviaIdentity(source, emitted, disposition) {
  return disposition === 'skip' ? source : emitted;
}

/** @param {string} expected @param {string} actual @param {boolean} wildcard @returns {boolean} */
export function acceptGrammarLexerExpectedToken(expected, actual, wildcard) {
  return wildcard || expected === actual;
}

/**
 * Validate declaration rows before materializing any host maps. The result is
 * [issue, entry, action]; issue zero means every supplied row is admitted.
 * @param {string[]} kinds @param {string[]} names @param {string[]} items
 * @param {string[]} values @param {string[]} modes @param {string[]} channels
 * @param {string[]} dispositions
 * @param {number[]} actionOwners @param {string[]} actions @param {string[]} actionModes
 * @returns {number[]}
 */
export function readGrammarLexerDeclarationIssue(kinds, names, items, values, modes, channels, dispositions, actionOwners, actions, actionModes) {
  const count = kinds.length;
  if (names.length !== count || items.length !== count || values.length !== count || modes.length !== count || channels.length !== count || dispositions.length !== count
    || actionOwners.length !== actions.length || actionModes.length !== actions.length) return [1, -1, -1];
  const bindingNames = [];
  const ownerNames = [];
  const literalValues = [];
  const localNames = [];
  const declaredModes = ['default'];
  const declaredChannels = ['default', 'HIDDEN'];
  let index = 0;
  while (index < count) {
    const form = readGrammarLexerEntryForm(readRequiredTextFact(kinds, index));
    if (form === 0) return [2, index, -1];
    if (form === 1) {
      if (readRequiredTextFact(items, index) !== 'ref' || containsGrammarLexerText(ownerNames, readRequiredTextFact(names, index))) return [3, index, -1];
      ownerNames.push(readRequiredTextFact(names, index));
    }
    if (form === 2) {
      if (readRequiredTextFact(items, index) !== 'ref' || containsGrammarLexerText(bindingNames, readRequiredTextFact(names, index))) return [4, index, -1];
      bindingNames.push(readRequiredTextFact(names, index));
    }
    if (form === 3) {
      if (containsGrammarLexerText(literalValues, readRequiredTextFact(values, index))) return [5, index, -1];
      literalValues.push(readRequiredTextFact(values, index));
    }
    if (form === 4) declaredModes.push(readRequiredTextFact(names, index));
    if (form === 5) declaredChannels.push(readRequiredTextFact(names, index));
    if (form === 7 || form === 8) {
      if (containsGrammarLexerText(localNames, readRequiredTextFact(names, index))) return [6, index, -1];
      localNames.push(readRequiredTextFact(names, index));
    }
    index += 1;
  }
  index = 0;
  while (index < count) {
    if (readGrammarLexerEntryForm(readRequiredTextFact(kinds, index)) === 7) {
      if (!containsGrammarLexerText(declaredModes, readRequiredTextFact(modes, index))) return [7, index, -1];
      if (!containsGrammarLexerText(declaredChannels, readRequiredTextFact(channels, index)) && !acceptGrammarLexerDecimal(readRequiredTextFact(channels, index))) return [8, index, -1];
      const disposition = readRequiredTextFact(dispositions, index);
      if (disposition !== 'emit' && disposition !== 'skip') return [11, index, -1];
      let action = 0;
      while (action < actions.length) {
        if (readRequiredNumberFact(actionOwners, action) === index) {
          const operation = readRequiredTextFact(actions, action);
          if (operation !== 'pushMode' && operation !== 'setMode' && operation !== 'popMode') return [9, index, action];
          if (operation !== 'popMode' && !containsGrammarLexerText(declaredModes, readRequiredTextFact(actionModes, action))) return [10, index, action];
        }
        action += 1;
      }
    }
    index += 1;
  }
  return [0, -1, -1];
}

/**
 * Return unresolved obligation positions reachable from the actual token.
 * Called rules contribute predicates and match-shape obligations; their actions
 * and lexer commands do not execute during another rule's token match.
 * @param {number} count @param {number} root
 * @param {number[]} edgeSources @param {number[]} edgeTargets
 * @param {number[]} owners @param {string[]} categories @returns {number[]}
 */
export function readGrammarLexerObligationPositions(count, root, edgeSources, edgeTargets, owners, categories) {
  if (!Number.isSafeInteger(count) || count < 0 || !Number.isSafeInteger(root) || root < 0 || root >= count
    || edgeSources.length !== edgeTargets.length || owners.length !== categories.length) return [-1];
  const reached = [];
  let index = 0;
  const pending = [root];
  let cursor = 0;
  while (cursor < pending.length) {
    const current = readRequiredNumberFact(pending, cursor);
    if (!Number.isSafeInteger(current) || current < 0 || current >= count) return [-1];
    if (!containsGrammarLexerPosition(reached, current)) {
      reached.push(current);
      index = 0;
      while (index < edgeSources.length) {
        if (readRequiredNumberFact(edgeSources, index) === current) pending.push(readRequiredNumberFact(edgeTargets, index));
        index += 1;
      }
    }
    cursor += 1;
  }
  const selected = [];
  index = 0;
  while (index < owners.length) {
    const owner = readRequiredNumberFact(owners, index);
    if (Number.isSafeInteger(owner) && owner >= 0 && owner < count && containsGrammarLexerPosition(reached, owner)
      && (owner === root || readRequiredTextFact(categories, index) !== 'action' && readRequiredTextFact(categories, index) !== 'command')) selected.push(index);
    index += 1;
  }
  return selected;
}

/**
 * Graph normalization plan: [capture, final delimiter] pairs. An exact
 * unrestricted lazy middle is replaceable only at its owner's penultimate
 * sequence position, before one nonempty literal. Other lazy captures remain
 * negative delimiter -1 obligations; source node positions are never renumbered.
 * @param {string[]} kinds @param {string[]} labels @param {string[]} texts
 * @param {number[]} inner @param {number[]} edgeSources @param {number[]} edgeTargets
 * @returns {number[]}
 */
export function readGrammarLexerLazyNormalization(kinds, labels, texts, inner, edgeSources, edgeTargets) {
  const count = kinds.length;
  if (labels.length !== count || texts.length !== count || inner.length !== count || edgeSources.length !== edgeTargets.length) return [-1];
  const captures = [];
  let index = 0;
  while (index < count) {
    if (readRequiredTextFact(kinds, index) === 'capture' && readRequiredTextFact(labels, index) === 'non_greedy') {
      let delimiter = -1;
      const repeat = readRequiredNumberFact(inner, index);
      if (repeat >= 0 && repeat < count && readRequiredTextFact(kinds, repeat) === 'repeat0') {
        const atom = readRequiredNumberFact(inner, repeat);
        if (atom >= 0 && atom < count && readRequiredTextFact(kinds, atom) === 'any') {
          let owner = 0;
          while (owner < count) {
            if (readRequiredTextFact(kinds, owner) === 'seq') {
              const children = [];
              let edge = 0;
              while (edge < edgeSources.length) {
                if (readRequiredNumberFact(edgeSources, edge) === owner) children.push(readRequiredNumberFact(edgeTargets, edge));
                edge += 1;
              }
              if (children.length >= 2 && readRequiredNumberFact(children, children.length - 2) === index) {
                const last = readRequiredNumberFact(children, children.length - 1);
                if (last >= 0 && last < count && readRequiredTextFact(kinds, last) === 'literal' && readRequiredTextFact(texts, last) !== '') delimiter = last;
              }
            }
            owner += 1;
          }
        }
      }
      captures.push(index, delimiter);
    }
    index += 1;
  }
  return captures;
}
