// Concrete syntax styles for the complete generated mapping family. Number
// addition must round before division; f64::midpoint changes that behavior.
import assert from 'node:assert/strict';
import { tokenize } from '../src/translation/lexer.js';
import { readRustLibraryItems } from './rust-library-packaging.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';

/** Prove the actual owner and scope of each mechanical mapping-library style. */
export function readGrammarSelectionMappingLibraryStyles(source) {
  const items = readRustLibraryItems(source).items;
  const whole = tokenize(source, 'Rust').tokens;
  const protectedNames = new Set(['f64', 'usize', 'Vec', 'Box', 'Option', 'Clone', 'UnicodeEscape']);
  for (let at = 0; at < whole.length; at += 1) {
    if (['type', 'struct', 'trait', 'mod', 'enum'].includes(whole[at].value)
      && protectedNames.has(whole[at + 1]?.value) && whole[at + 1].value !== 'UnicodeEscape') return { source, actions: [] };
    if (['use', 'impl'].includes(whole[at].value)) {
      let end = at + 1;
      while (whole[end] && ![';', '{'].includes(whole[end].value) && whole[end].kind !== 'eof') end += 1;
      if (whole.slice(at, end).some(token => whole[at].value === 'use'
        ? protectedNames.has(token.value) || token.value === '*' : token.value === 'UnicodeEscape')) return { source, actions: [] };
    }
  }
  // Numeric token.value omits the Rust type suffix; ownership/arithmetic
  // proofs must retain the actual literal rather than equating 2f32 and 2f64.
  const spelling = value => tokenize(value, 'Rust').tokens.filter(token => token.kind !== 'eof')
    .map(token => token.kind === 'number' ? token.raw : token.value).join('');
  const resultOwners = items.filter(item => item.kind === 'enum' && item.name === 'UnicodeEscape');
  const resultOwned = resultOwners.length === 1 && spelling(resultOwners[0].source)
    === '#[derive(Clone,Debug,PartialEq)]pubenumUnicodeEscape{Scalar(f64,f64),Malformed,Unsupported(f64,f64),}';
  const arrayOwners = items.filter(item => item.kind === 'mod' && item.name === 'ml_array');
  const arrayOwned = arrayOwners.length === 1 && spelling(readSourceTextRustBlock(arrayOwners[0].source, 'fn', 'optional_at'))
    === 'pubfnoptional_at<T:Clone>(values:&[T],index:Option<usize>)->Option<T>{index.and_then(|index|values.get(index)).cloned()}';
  const actions = [];
  let adapted = source;
  for (const item of items.filter(item => item.kind === 'fn' && (item.name === 'decode_unicode_escape'
    || item.name === 'read_grammar_selection_mapped_cover' || /^ml_read_grammar_selection_mapped_cover_loop[0-9]+$/u.test(item.name)))) {
    if (items.filter(candidate => candidate.kind === 'fn' && candidate.name === item.name).length !== 1) continue;
    const block = readSourceTextRustBlock(item.source, 'fn', item.name);
    const { tokens, comments } = tokenize(block, 'Rust');
    const matching = new Map(), stack = [];
    for (let at = 0; at < tokens.length; at += 1) {
      if (tokens[at].kind !== 'punct') continue;
      if (['(', '[', '{'].includes(tokens[at].value)) stack.push(at);
      else if ([')', ']', '}'].includes(tokens[at].value)) {
        const open = stack.pop();
        assert.equal(tokens[open]?.value, ({ ')': '(', ']': '[', '}': '{' })[tokens[at].value]);
        matching.set(open, at); matching.set(at, open);
      }
    }
    assert.equal(stack.length, 0);
    const words = (start, end) => tokens.slice(start, end)
      .map(token => token.kind === 'number' ? token.raw : token.value).join('');
    const clean = (start, end) => !comments.some(comment => comment.start >= tokens[start].start && comment.end <= tokens[end - 1].end);
    const fn = tokens.findIndex(token => token.value === 'fn'), parameters = fn + 2;
    const headerEnd = matching.get(parameters), header = words(parameters + 1, headerEnd);
    const body = tokens.findIndex((token, index) => index > headerEnd && token.value === '{');
    if (body < 0) continue;
    const edits = [];
    const identifiers = new Set(tokens.filter(token => token.kind === 'identifier').map(token => token.value));
    const sameName = name => tokens.filter(token => token.kind === 'identifier' && token.value === name).length;
    // Only actual enum constructors, complete if/else alternatives and blocks
    // with leading lets establish an owned result. A loan or unknown call does
    // not establish that a match consumes its boxed payload.
    const ownedResult = (start, end, owner) => {
      if (start >= end) return false;
      if (tokens[start].value === '{' && matching.get(start) === end - 1) {
        start += 1; end -= 1;
        while (tokens[start]?.value === 'let') {
          while (start < end && tokens[start].value !== ';') start = matching.get(start) > start ? matching.get(start) + 1 : start + 1;
          if (start >= end) return false;
          start += 1;
        }
        return ownedResult(start, end, owner);
      }
      if (tokens[start].value === 'if') {
        for (let at = start + 1; at < end; at += 1) {
          if (tokens[at].value === '{' && tokens[matching.get(at) + 1]?.value === 'else') {
            const close = matching.get(at);
            return ownedResult(at, close + 1, owner) && ownedResult(close + 2, end, owner);
          }
          if (matching.get(at) > at) at = matching.get(at);
        }
        return false;
      }
      return words(start, start + 4) === 'self::' + owner + '::'
        && tokens[start + 4]?.kind === 'identifier' && tokens[start + 5]?.value === '('
        && matching.get(start + 5) === end - 1
        && spelling(items.find(candidate => candidate.kind === 'enum' && candidate.name === owner)?.source ?? '')
          .includes(tokens[start + 4].value + '(');
    };
    if (item.name === 'decode_unicode_escape' && resultOwned) {
      for (let at = body + 1; at < tokens.length - 1; at += 1) {
        if (words(at, at + 2) !== 'self::' || !/^MlDecodeUnicodeEscapeJoin[0-9]+$/u.test(tokens[at + 2]?.value ?? '')
          || tokens[at + 3]?.value !== '::' || tokens[at + 5]?.value !== '(') continue;
        const owner = tokens[at + 2].value, variant = tokens[at + 4].value;
        const close = matching.get(at + 5), payload = tokens[at + 6]?.value;
        if (![payload, payload + ','].includes(words(at + 6, close)) || tokens[close + 1]?.value !== '=>'
          || tokens[close + 2]?.value !== '{') continue;
        const end = matching.get(close + 2);
        if (words(close + 3, end) !== 'let' + payload + '=*' + payload + ';' + payload + '.clone()'
          || !clean(close + 3, end)) continue;
        const declarations = items.filter(candidate => candidate.kind === 'enum' && candidate.name === owner);
        if (declarations.length !== 1 || !spelling(declarations[0].source).includes(variant + '(Box<self::UnicodeEscape>)')) continue;
        let matchOpen = at - 1;
        while (matchOpen >= 0 && !(tokens[matchOpen].value === '{' && matching.get(matchOpen) > end
          && tokens[matchOpen - 2]?.value === 'match')) matchOpen -= 1;
        const scrutinee = tokens[matchOpen - 1]?.value;
        if (matchOpen < 0 || sameName(scrutinee) !== 2) continue;
        const binding = tokens.findIndex((token, index) => token.value === 'let' && tokens[index + 1]?.value === scrutinee && tokens[index + 2]?.value === '=');
        if (binding < 0 || binding >= matchOpen) continue;
        let initializerEnd = binding + 3;
        while (initializerEnd < matchOpen && tokens[initializerEnd].value !== ';') initializerEnd = matching.get(initializerEnd) > initializerEnd ? matching.get(initializerEnd) + 1 : initializerEnd + 1;
        if (!ownedResult(binding + 3, initializerEnd, owner)) continue;
        // Both the copy and its otherwise redundant terminal alias disappear.
        // The primitive-only derived enum has no custom Clone/Drop behavior.
        edits.push([tokens[close + 3].start, tokens[end - 1].end, '*' + payload]);
      }
    }
    const columnOwned = arrayOwned && /(?:^|,)intervals:&\[Vec<f64>\](?:,|$)/u.test(header);
    if (item.name !== 'decode_unicode_escape' && columnOwned) {
      for (let at = body + 1; at < tokens.length - 1; at += 1) {
        if (words(at, at + 3) !== '.unwrap_or_else(') continue;
        const end = matching.get(at + 2);
        if (words(at + 3, end) !== '||Vec::<f64>::new()' || !clean(at + 3, end) || tokens[at - 1]?.value !== ')') continue;
        let first = matching.get(at - 1), last = at;
        while (tokens[first]?.value === '(' && matching.get(first) === last - 1) { first += 1; last -= 1; }
        if (words(first, first + 6) !== 'self::ml_array::optional_at('
          || tokens[first + 6]?.value !== 'intervals' || tokens[first + 7]?.value !== ','
          || matching.get(first + 5) !== last - 1) continue;
        edits.push([tokens[at + 3].start, tokens[end - 1].end, 'Vec::<f64>::new']);
      }
      let shadowed = false;
      for (let at = body + 1; at < tokens.length - 1; at += 1) {
        if (tokens[at].kind === 'macro' || ['match', 'for', 'fn', '|'].includes(tokens[at].value)) shadowed = true;
        if (tokens[at].value !== 'let') continue;
        let equal = at + 1;
        while (equal < tokens.length && !['=', ';', '}'].includes(tokens[equal].value)) equal += 1;
        if (tokens.slice(at + 1, equal).some(token => ['low', 'high'].includes(token.value))) shadowed = true;
      }
      if (!shadowed && /^ml_read_grammar_selection_mapped_cover_loop[0-9]+$/u.test(item.name)
        && /(?:^|,)(?:mut)?low:f64(?:,|$)/u.test(header) && /(?:^|,)(?:mut)?high:f64(?:,|$)/u.test(header)) {
        for (let at = body + 1; at < tokens.length - 1; at += 1) {
          if (words(at, at + 3) !== 'letmiddle=') continue;
          let end = at + 3;
          while (end < tokens.length && tokens[end].value !== ';') end = matching.get(end) > end ? matching.get(end) + 1 : end + 1;
          if (words(at + 3, end) !== '((low+high)/2f64).floor()' || !clean(at, end + 1)) continue;
          let sum = 'ml_mapping_sum', suffix = 1;
          while (identifiers.has(sum)) sum = 'ml_mapping_sum_' + suffix++;
          identifiers.add(sum);
          // Preserve add -> divide -> floor, including overflow, signed zero
          // and each separate rounding point; never introduce midpoint().
          const original = block.slice(tokens[at].start, tokens[end].end);
          const prefix = 'let ' + sum + ' = low + high;\n';
          const replacement = prefix + 'let middle = (' + sum + ' / 2f64).floor();';
          assert.equal(replacement.slice(prefix.length).replace('(' + sum + ' / 2f64)', '((low + high) / 2f64)'),
            'let middle = ((low + high) / 2f64).floor();', 'the fresh binding has the exact original arithmetic inverse');
          assert.equal(spelling(original), spelling('let middle = ((low + high) / 2f64).floor();'));
          edits.push([tokens[at].start, tokens[end].end, replacement]);
        }
      }
    }
    let styled = block, boundary = block.length;
    for (const [start, end, replacement] of edits.sort((left, right) => right[0] - left[0])) {
      assert.ok(end <= boundary, 'mapping source styles have disjoint original token spans');
      styled = styled.slice(0, start) + replacement + styled.slice(end); boundary = start;
    }
    if (styled !== block) {
      assert.equal(adapted.split(block).length, 2, 'the complete function action has exactly one source occurrence');
      actions.push({ op: 'replace', field: 'source', from: block, to: styled });
      adapted = adapted.replace(block, styled);
    }
  }
  return { source: adapted, actions };
}
